use super::{models::*, *};

#[derive(Debug, GraphQLObject, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[graphql(description = "A contract call made by the campaign manager or oracle.")]
pub struct Stats {
    #[graphql(description = "Unique numeric identifier of this resource")]
    pub id: i32,
    #[graphql(description = "Handles that made at least one collab")]
    pub total_active_handles: i32,
    #[graphql(description = "How many collabs were made")]
    pub total_collabs: i32,
    #[graphql(description = "How many campaigns were run")]
    pub total_campaigns: i32,
    #[graphql(description = "How much money was paid in rewards")]
    pub total_rewards_paid: String,
    #[graphql(description = "Date in which these stats were calculated")]
    pub date: UtcDateTime,
}
