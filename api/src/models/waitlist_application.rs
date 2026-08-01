use super::*;

make_sql_enum![
    "waitlist_application_status",
    pub enum WaitlistApplicationStatus {
        Pending,
        Reviewed,
        Accepted,
        Rejected,
    }
];

model! {
  state: App,
  table: waitlist_applications,
  struct WaitlistApplication {
    #[sqlx_model_hints(int4, default, op_in)]
    id: i32,
    #[sqlx_model_hints(text, op_like, op_ilike)]
    name: String,
    #[sqlx_model_hints(text, op_like, op_ilike)]
    email: String,
    #[sqlx_model_hints(text)]
    link_1: String,
    #[sqlx_model_hints(text)]
    link_2: String,
    #[sqlx_model_hints(text)]
    link_3: Option<String>,
    #[sqlx_model_hints(text)]
    note: String,
    #[sqlx_model_hints(int4, default)]
    invitation_id: Option<i32>,
    #[sqlx_model_hints(waitlist_application_status, default, op_in)]
    status: WaitlistApplicationStatus,
    #[sqlx_model_hints(text, default)]
    review_notes: Option<String>,
    #[sqlx_model_hints(timestamptz, default)]
    created_at: UtcDateTime,
    #[sqlx_model_hints(timestamptz, default, op_is_set)]
    reviewed_at: Option<UtcDateTime>,
  }
}

#[derive(Debug, Clone)]
pub struct ReviewWaitlistApplication {
    pub status: WaitlistApplicationStatus,
    pub review_notes: Option<String>,
    pub accept_with_invitation: Option<CreateInvitation>,
}

impl WaitlistApplicationHub {
    pub async fn review(&self, id: i32, input: ReviewWaitlistApplication) -> AsamiResult<WaitlistApplication> {
        let tx = self.state.transactional().await?;
        let application = tx.waitlist_application().find(id).await?;

        if input.status == WaitlistApplicationStatus::Accepted && input.accept_with_invitation.is_none() {
            return Err(Error::validation(
                "accept_with_invitation",
                "invite_required_for_acceptance",
            ));
        }

        let invitation_id = if let Some(invite) = input.accept_with_invitation {
            if application.invitation_id().is_some() {
                return Err(Error::validation("waitlist_application", "already_has_invitation"));
            }

            Some(*InvitationHub::create_with_account_in_tx(&tx, invite).await?.id())
        } else {
            *application.invitation_id()
        };

        let reviewed_at = match input.status {
            WaitlistApplicationStatus::Pending => None,
            _ => Some(Utc::now()),
        };

        let updated = application
            .update()
            .status(if invitation_id.is_some() {
                WaitlistApplicationStatus::Accepted
            } else {
                input.status
            })
            .review_notes(input.review_notes)
            .reviewed_at(reviewed_at)
            .invitation_id(invitation_id)
            .save()
            .await?;

        tx.commit().await?;
        Ok(updated)
    }
}
