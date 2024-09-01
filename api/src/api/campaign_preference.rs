use super::{
    models::{self, *},
    *,
};

#[derive(Debug, GraphQLObject, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[graphql(description = "Internal UX preferences for specific campaigns")]
pub struct CampaignPreference {
    #[graphql(description = "Unique numeric identifier of this resource")]
    id: i32,
    #[graphql(description = "The campaign this preferences apply to.")]
    campaign_id: i32,
    #[graphql(description = "Date in which the user marked to not be interested in this campaign.")]
    not_interested_on: Option<UtcDateTime>,
    #[graphql(description = "Date in which the user attempted to retweet this campaign.")]
    attempted_on: Option<UtcDateTime>,
}

#[derive(Debug, Clone, Default, GraphQLInputObject, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CampaignPreferenceFilter {
    ids: Option<Vec<i32>>,
    id_eq: Option<i32>,
    campaign_id_eq: Option<i32>,
}

#[rocket::async_trait]
impl Showable<models::CampaignPreference, CampaignPreferenceFilter> for CampaignPreference {
    fn sort_field_to_order_by(field: &str) -> Option<models::CampaignPreferenceOrderBy> {
        match field {
            "id" => Some(CampaignPreferenceOrderBy::Id),
            _ => None,
        }
    }

    fn filter_to_select(
        context: &Context,
        filter: Option<CampaignPreferenceFilter>,
    ) -> FieldResult<models::SelectCampaignPreference> {
        let account_id = context.account_id()?;

        if let Some(f) = filter {
            Ok(models::SelectCampaignPreference {
                account_id_eq: Some(account_id),
                id_in: f.ids,
                campaign_id_eq: f.campaign_id_eq,
                id_eq: f.id_eq,
                ..Default::default()
            })
        } else {
            Ok(models::SelectCampaignPreference {
                account_id_eq: Some(account_id),
                ..Default::default()
            })
        }
    }

    fn select_by_id(context: &Context, id: i32) -> FieldResult<models::SelectCampaignPreference> {
        Ok(models::SelectCampaignPreference {
            id_eq: Some(id),
            account_id_eq: Some(context.account_id()?),
            ..Default::default()
        })
    }

    async fn db_to_graphql(_context: &Context, d: models::CampaignPreference) -> AsamiResult<Self> {
        Ok(CampaignPreference {
            id: d.attrs.id,
            campaign_id: d.attrs.campaign_id,
            not_interested_on: d.attrs.not_interested_on,
            attempted_on: d.attrs.attempted_on,
        })
    }
}

#[derive(Clone, GraphQLInputObject, Serialize, Deserialize)]
#[graphql(description = "The input for creating a new CampaignPreference.")]
#[serde(rename_all = "camelCase")]
pub struct CreateCampaignPreferenceInput {
    pub campaign_id: i32,
    pub not_interested: bool,
    pub attempted: bool,
}

impl CreateCampaignPreferenceInput {
    pub async fn process(self, context: &Context) -> FieldResult<CampaignPreference> {
        let maybe = context
            .app
            .campaign_preference()
            .select()
            .account_id_eq(context.account_id()?)
            .campaign_id_eq(self.campaign_id)
            .optional()
            .await?;

        let not_interested_on = if self.not_interested { Some(Utc::now()) } else { None };
        let attempted_on = if self.attempted { Some(Utc::now()) } else { None };

        let preference = if let Some(p) = maybe {
            p.update().not_interested_on(not_interested_on).attempted_on(attempted_on).save().await?
        } else {
            context
                .app
                .campaign_preference()
                .insert(InsertCampaignPreference {
                    account_id: context.account_id()?,
                    campaign_id: self.campaign_id,
                    not_interested_on,
                    attempted_on,
                })
                .save()
                .await?
        };

        Ok(CampaignPreference::db_to_graphql(context, preference).await?)
    }
}
