use super::{
    models::{self, *},
    *,
};

#[derive(Debug, GraphQLObject, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WaitlistApplication {
    id: i32,
    name: String,
    email: String,
    link_1: String,
    link_2: String,
    link_3: Option<String>,
    note: String,
    invitation_id: Option<i32>,
    invite_url: Option<String>,
    status: WaitlistApplicationStatus,
    review_notes: Option<String>,
    created_at: UtcDateTime,
    reviewed_at: Option<UtcDateTime>,
}

#[derive(Debug, Clone, Default, GraphQLInputObject, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WaitlistApplicationFilter {
    ids: Option<Vec<i32>>,
    id_eq: Option<i32>,
    status_eq: Option<WaitlistApplicationStatus>,
    name_ilike: Option<String>,
    email_ilike: Option<String>,
}

#[rocket::async_trait]
impl Showable<models::WaitlistApplication, WaitlistApplicationFilter> for WaitlistApplication {
    async fn resource(context: &Context, id: i32) -> FieldResult<Self> {
        if !context.current_session()?.0.admin() {
            return Err(Error::service("authentication", "asami_authentication_needed").into_field_error());
        }

        <Self as Showable<models::WaitlistApplication, WaitlistApplicationFilter>>::base_resource(context, id).await
    }

    async fn collection(
        context: &Context,
        page: Option<i32>,
        per_page: Option<i32>,
        sort_field: Option<String>,
        sort_order: Option<String>,
        filter: Option<WaitlistApplicationFilter>,
    ) -> FieldResult<Vec<Self>> {
        if !context.current_session()?.0.admin() {
            return Err(Error::service("authentication", "asami_authentication_needed").into_field_error());
        }

        <Self as Showable<models::WaitlistApplication, WaitlistApplicationFilter>>::base_collection(
            context, page, per_page, sort_field, sort_order, filter,
        )
        .await
    }

    async fn count(context: &Context, filter: Option<WaitlistApplicationFilter>) -> FieldResult<ListMetadata> {
        if !context.current_session()?.0.admin() {
            return Err(Error::service("authentication", "asami_authentication_needed").into_field_error());
        }

        <Self as Showable<models::WaitlistApplication, WaitlistApplicationFilter>>::base_count(context, filter).await
    }

    fn sort_field_to_order_by(field: &str) -> Option<models::WaitlistApplicationOrderBy> {
        match field {
            "id" => Some(WaitlistApplicationOrderBy::Id),
            "name" => Some(WaitlistApplicationOrderBy::Name),
            "email" => Some(WaitlistApplicationOrderBy::Email),
            "status" => Some(WaitlistApplicationOrderBy::Status),
            "createdAt" => Some(WaitlistApplicationOrderBy::CreatedAt),
            "reviewedAt" => Some(WaitlistApplicationOrderBy::ReviewedAt),
            _ => None,
        }
    }

    fn filter_to_select(
        _context: &Context,
        filter: Option<WaitlistApplicationFilter>,
    ) -> FieldResult<models::SelectWaitlistApplication> {
        if let Some(f) = filter {
            Ok(models::SelectWaitlistApplication {
                id_in: f.ids,
                id_eq: f.id_eq,
                status_eq: f.status_eq,
                name_ilike: into_like_search(f.name_ilike),
                email_ilike: into_like_search(f.email_ilike),
                ..Default::default()
            })
        } else {
            Ok(Default::default())
        }
    }

    fn select_by_id(_context: &Context, id: i32) -> FieldResult<models::SelectWaitlistApplication> {
        Ok(models::SelectWaitlistApplication {
            id_eq: Some(id),
            ..Default::default()
        })
    }

    async fn db_to_graphql(context: &Context, d: models::WaitlistApplication) -> AsamiResult<Self> {
        Ok(Self {
            id: d.attrs.id,
            name: d.attrs.name,
            email: d.attrs.email,
            link_1: d.attrs.link_1,
            link_2: d.attrs.link_2,
            link_3: d.attrs.link_3,
            note: d.attrs.note,
            invitation_id: d.attrs.invitation_id,
            invite_url: if let Some(invitation_id) = d.attrs.invitation_id {
                let invitation = context.app.invitation().find(invitation_id).await?;
                Some(format!(
                    "{}/#/one_time_token_login?token={}",
                    context.app.settings.pwa_host,
                    invitation.token()
                ))
            } else {
                None
            },
            status: d.attrs.status,
            review_notes: d.attrs.review_notes,
            created_at: d.attrs.created_at,
            reviewed_at: d.attrs.reviewed_at,
        })
    }
}

#[derive(Clone, GraphQLInputObject, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateWaitlistApplicationInput {
    name: String,
    email: String,
    link_1: String,
    link_2: String,
    link_3: Option<String>,
    note: String,
}

impl CreateWaitlistApplicationInput {
    pub async fn process(self, context: &Context) -> FieldResult<WaitlistApplication> {
        let created = context
            .app
            .waitlist_application()
            .insert(models::InsertWaitlistApplication {
                name: self.name,
                email: self.email.trim().to_lowercase(),
                link_1: self.link_1,
                link_2: self.link_2,
                link_3: self.link_3,
                note: self.note,
            })
            .save()
            .await?;

        WaitlistApplication::db_to_graphql(context, created).await.map_err(|e| e.into_field_error())
    }
}

#[derive(Clone, GraphQLInputObject, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EditWaitlistApplicationInput {
    status: WaitlistApplicationStatus,
    review_notes: Option<String>,
    accept_with_invitation: Option<AcceptWaitlistApplicationInput>,
}

#[derive(Clone, GraphQLInputObject, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AcceptWaitlistApplicationInput {
    account_name: String,
    alignment: i32,
    trust: i32,
    contribution: i32,
    advocacy: i32,
    judgment: i32,
    rootstock_joined_at: UtcDateTime,
    comment: Option<String>,
    priority: bool,
}

impl EditWaitlistApplicationInput {
    pub async fn process(self, context: &Context, id: i32) -> FieldResult<WaitlistApplication> {
        let inviter_account_id = context.account_id()?;
        let updated = context
            .app
            .waitlist_application()
            .review(
                id,
                models::ReviewWaitlistApplication {
                    status: self.status,
                    review_notes: self.review_notes,
                    accept_with_invitation: self.accept_with_invitation.map(|invite| models::CreateInvitation {
                        inviter_account_id: inviter_account_id.clone(),
                        account_name: invite.account_name,
                        lang: context.lang,
                        alignment: invite.alignment,
                        trust: invite.trust,
                        contribution: invite.contribution,
                        advocacy: invite.advocacy,
                        judgment: invite.judgment,
                        rootstock_joined_at: invite.rootstock_joined_at,
                        comment: invite.comment,
                        priority: invite.priority,
                    }),
                },
            )
            .await
            .map_err(|e| e.into_field_error())?;

        WaitlistApplication::db_to_graphql(context, updated).await.map_err(|e| e.into_field_error())
    }
}
