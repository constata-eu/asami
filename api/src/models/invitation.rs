use super::*;
use rand::{distr::Alphanumeric, rng, RngExt};

model! {
  state: App,
  table: invitations,
  struct Invitation {
    #[sqlx_model_hints(int4, default, op_in)]
    id: i32,
    #[sqlx_model_hints(varchar)]
    inviter_account_id: String,
    #[sqlx_model_hints(varchar)]
    account_id: String,
    #[sqlx_model_hints(varchar, op_like)]
    token: String,
    #[sqlx_model_hints(timestamptz, default)]
    created_at: UtcDateTime,
    #[sqlx_model_hints(timestamptz, default, op_is_set)]
    expires_at: Option<UtcDateTime>,
    #[sqlx_model_hints(timestamptz, default, op_is_set)]
    accepted_at: Option<UtcDateTime>,
    #[sqlx_model_hints(timestamptz, default, op_is_set)]
    revoked_at: Option<UtcDateTime>,
  },
  belongs_to {
    Account(inviter_account_id),
  }
}

#[derive(Debug, Clone)]
pub struct CreateInvitation {
    pub inviter_account_id: String,
    pub account_name: String,
    pub lang: Lang,
    pub alignment: i32,
    pub trust: i32,
    pub contribution: i32,
    pub advocacy: i32,
    pub judgment: i32,
    pub rootstock_joined_at: UtcDateTime,
    pub comment: Option<String>,
    pub priority: bool,
}

impl InvitationHub {
    pub(crate) async fn create_with_account_in_tx(state: &App, input: CreateInvitation) -> AsamiResult<Invitation> {
        let inviter = state.account().find(&input.inviter_account_id).await?;

        if *inviter.available_invites() <= 0 {
            return Err(Error::validation("available_invites", "no_available_invites"));
        }

        let invited_account = state
            .account()
            .insert(InsertAccount {
                addr: None,
                lang: input.lang,
                profile_pic: None,
            })
            .save()
            .await?
            .update()
            .name(input.account_name)
            .acceptance_status(AcceptanceStatus::Active)
            .invited_by_account_id(Some(input.inviter_account_id.clone()))
            .save()
            .await?;

        let invited_user = state
            .user()
            .insert(InsertUser {
                name: "invited_user".to_string(),
            })
            .save()
            .await?;

        state
            .account_user()
            .insert(InsertAccountUser {
                account_id: invited_account.id().clone(),
                user_id: *invited_user.id(),
            })
            .save()
            .await?;

        let token = Self::rand_token();
        let lookup_key = format!("invitation:{token}");

        state
            .auth_method()
            .insert(InsertAuthMethod {
                user_id: *invited_user.id(),
                kind: AuthMethodKind::OneTimeToken,
                lookup_key: lookup_key.clone(),
            })
            .save()
            .await?;

        state
            .one_time_token()
            .insert(InsertOneTimeToken {
                value: token.clone(),
                lookup_key,
                user_id: Some(*invited_user.id()),
                email: None,
                lang: input.lang,
                expires_at: Utc::now() + Duration::days(30),
            })
            .save()
            .await?;

        state
            .community_member()
            .upsert_score(
                UpsertCommunityMemberScore {
                    account_id: input.inviter_account_id.clone(),
                    member_id: invited_account.id().clone(),
                    priority: input.priority,
                    alignment: input.alignment,
                    trust: input.trust,
                    contribution: input.contribution,
                    advocacy: input.advocacy,
                    judgment: input.judgment,
                    rootstock_joined_at: input.rootstock_joined_at,
                    comment: input.comment,
                },
                true,
            )
            .await?;

        let invitation = state
            .invitation()
            .insert(InsertInvitation {
                inviter_account_id: input.inviter_account_id.clone(),
                account_id: invited_account.id().clone(),
                token,
            })
            .save()
            .await?;

        state
            .account()
            .find(&input.inviter_account_id)
            .await?
            .update()
            .available_invites(inviter.available_invites() - 1)
            .save()
            .await?;

        Ok(invitation)
    }

    pub fn rand_token() -> String {
        rng().sample_iter(&Alphanumeric).take(24).map(char::from).collect()
    }

    pub async fn create_with_account(&self, input: CreateInvitation) -> AsamiResult<Invitation> {
        let tx = self.state.transactional().await?;
        let invitation = Self::create_with_account_in_tx(&tx, input).await?;

        tx.commit().await?;

        Ok(invitation)
    }

    pub async fn accept_token(&self, token: &str) -> AsamiResult<()> {
        if let Some(invitation) = self
            .select()
            .token_eq(token.to_string())
            .accepted_at_is_set(false)
            .revoked_at_is_set(false)
            .optional()
            .await?
        {
            invitation.update().accepted_at(Some(Utc::now())).save().await?;
        }

        Ok(())
    }

    pub async fn revoke(&self, invitation_id: i32, actor_account_id: &str, is_admin: bool) -> AsamiResult<Invitation> {
        let invitation = self.find(invitation_id).await?;

        if !is_admin && invitation.inviter_account_id() != actor_account_id {
            return Err(Error::validation("invitation", "cannot_revoke_other_users_invitation"));
        }

        if invitation.accepted_at().is_some() {
            return Err(Error::validation("invitation", "cannot_revoke_accepted_invitation"));
        }

        if invitation.revoked_at().is_some() {
            return Err(Error::validation("invitation", "invitation_already_revoked"));
        }

        let tx = self.state.transactional().await?;
        let invitation = tx.invitation().find(invitation_id).await?;
        let revoked = invitation.update().revoked_at(Some(Utc::now())).save().await?;

        if let Some(token) = tx
            .one_time_token()
            .select()
            .lookup_key_eq(format!("invitation:{}", revoked.token()))
            .optional()
            .await?
        {
            token.update().used(true).expires_at(Utc::now()).save().await?;
        }

        tx.account()
            .find(revoked.account_id())
            .await?
            .update()
            .acceptance_status(AcceptanceStatus::Guestlist)
            .save()
            .await?;

        let inviter = tx.account().find(revoked.inviter_account_id()).await?;
        let next = inviter.available_invites() + 1;
        inviter.update().available_invites(next).save().await?;

        tx.commit().await?;
        Ok(revoked)
    }
}
