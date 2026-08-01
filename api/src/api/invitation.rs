use super::{
    models::{self, *},
    *,
};

#[derive(Debug, GraphQLObject, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Invitation {
    id: i32,
    inviter_account_id: i32,
    account_id: i32,
    account_name: String,
    token: String,
    invite_url: String,
    created_at: UtcDateTime,
    expires_at: Option<UtcDateTime>,
    accepted_at: Option<UtcDateTime>,
    revoked_at: Option<UtcDateTime>,
}

#[derive(Clone, Debug, Default, GraphQLInputObject, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InvitationFilter {
    ids: Option<Vec<i32>>,
    id_eq: Option<i32>,
    inviter_account_id_eq: Option<i32>,
    account_id_eq: Option<i32>,
    accepted_at_is_set: Option<bool>,
    revoked_at_is_set: Option<bool>,
}

#[rocket::async_trait]
impl Showable<models::Invitation, InvitationFilter> for Invitation {
    fn sort_field_to_order_by(field: &str) -> Option<models::InvitationOrderBy> {
        match field {
            "id" => Some(InvitationOrderBy::Id),
            "createdAt" => Some(InvitationOrderBy::CreatedAt),
            "acceptedAt" => Some(InvitationOrderBy::AcceptedAt),
            "revokedAt" => Some(InvitationOrderBy::RevokedAt),
            _ => None,
        }
    }

    fn filter_to_select(_context: &Context, filter: Option<InvitationFilter>) -> FieldResult<models::SelectInvitation> {
        if let Some(f) = filter {
            Ok(models::SelectInvitation {
                id_in: f.ids,
                id_eq: f.id_eq,
                inviter_account_id_eq: f.inviter_account_id_eq.map(i32_to_hex),
                account_id_eq: f.account_id_eq.map(i32_to_hex),
                accepted_at_is_set: f.accepted_at_is_set,
                revoked_at_is_set: f.revoked_at_is_set,
                ..Default::default()
            })
        } else {
            Ok(Default::default())
        }
    }

    fn select_by_id(_context: &Context, id: i32) -> FieldResult<models::SelectInvitation> {
        Ok(models::SelectInvitation {
            id_eq: Some(id),
            ..Default::default()
        })
    }

    async fn resource(context: &Context, id: i32) -> FieldResult<Self> {
        let invitation = context.app.invitation().find(id).await?;
        if !context.current_session()?.0.admin() && invitation.inviter_account_id() != &context.account_id()? {
            return Err(Error::service("authentication", "asami_authentication_needed").into_field_error());
        }
        Invitation::db_to_graphql(context, invitation).await.map_err(|e| e.into_field_error())
    }

    async fn collection(
        context: &Context,
        page: Option<i32>,
        per_page: Option<i32>,
        sort_field: Option<String>,
        sort_order: Option<String>,
        filter: Option<InvitationFilter>,
    ) -> FieldResult<Vec<Self>> {
        let filter = match filter {
            Some(mut f) => {
                if !context.current_session()?.0.admin() {
                    f.inviter_account_id_eq = Some(
                        hex_to_i32(&context.account_id()?)
                            .map_err(|e| Error::runtime(&e.to_string()).into_field_error())?,
                    );
                }
                Some(f)
            }
            None if !context.current_session()?.0.admin() => Some(InvitationFilter {
                inviter_account_id_eq: Some(
                    hex_to_i32(&context.account_id()?)
                        .map_err(|e| Error::runtime(&e.to_string()).into_field_error())?,
                ),
                ..Default::default()
            }),
            None => None,
        };

        <Self as Showable<models::Invitation, InvitationFilter>>::base_collection(
            context, page, per_page, sort_field, sort_order, filter,
        )
        .await
    }

    async fn db_to_graphql(context: &Context, d: models::Invitation) -> AsamiResult<Self> {
        let invited = context.app.account().find(d.account_id()).await?;
        Ok(Self {
            id: *d.id(),
            inviter_account_id: hex_to_i32(d.inviter_account_id())?,
            account_id: hex_to_i32(d.account_id())?,
            account_name: invited.name().clone(),
            invite_url: format!(
                "{}/#/one_time_token_login?token={}",
                context.app.settings.pwa_host,
                d.token()
            ),
            token: d.token().clone(),
            created_at: *d.created_at(),
            expires_at: d.expires_at().clone(),
            accepted_at: d.accepted_at().clone(),
            revoked_at: d.revoked_at().clone(),
        })
    }
}

#[derive(Clone, GraphQLInputObject, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateInvitationInput {
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

impl CreateInvitationInput {
    pub async fn process(self, context: &Context) -> FieldResult<Invitation> {
        let created = context
            .app
            .invitation()
            .create_with_account(models::CreateInvitation {
                inviter_account_id: context.account_id()?,
                account_name: self.account_name,
                lang: context.lang,
                alignment: self.alignment,
                trust: self.trust,
                contribution: self.contribution,
                advocacy: self.advocacy,
                judgment: self.judgment,
                rootstock_joined_at: self.rootstock_joined_at,
                comment: self.comment,
                priority: self.priority,
            })
            .await
            .map_err(|e| e.into_field_error())?;

        Invitation::db_to_graphql(context, created).await.map_err(|e| e.into_field_error())
    }
}

pub async fn revoke_invitation(context: &Context, id: i32) -> FieldResult<Invitation> {
    let revoked = context
        .app
        .invitation()
        .revoke(id, &context.account_id()?, *context.current_session()?.0.admin())
        .await
        .map_err(|e| e.into_field_error())?;

    Invitation::db_to_graphql(context, revoked).await.map_err(|e| e.into_field_error())
}
