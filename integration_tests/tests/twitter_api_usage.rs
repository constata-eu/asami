#[macro_use]
pub mod support;
use api::models::*;

/*
app_test!{ verifies_and_appraises_on_twitter (a) 
  let nubis = a.client().await;
  nubis.account().await.create_handle_request(models::Site::X, "nubis_bruno").await.unwrap();

  a.app.handle_request().verify_and_appraise_x().await?;
}
*/
