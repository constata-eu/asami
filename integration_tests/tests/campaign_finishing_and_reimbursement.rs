#[macro_use]
mod support;

/*
ToDo:
- Campaigns with old valid_until are not offered to users.
- Campaigns with old valid_until are not sent in collab_requests. (We need collab request validations). 
- Campaigns are periodically finished. 

use graphql_client::GraphQLQuery;
use support::{
  *,
  gql::{
    *,
    create_handle_request as chr,
    all_handle_requests as ahr,
    all_collabs as ac,
  }
};
use api::models::*;
*/

