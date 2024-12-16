//use api::models::*;

api_test! { api_creates_and_sends_email_login (mut c)
    let mut client = c.test_app.client().await;
    let _result: gql::create_email_login_link::ResponseData = client.gql(
        &gql::CreateEmailLoginLink::build_query(
            gql::create_email_login_link::Variables{ email: "yo@nubis.im".to_string()}
        ),
        vec![],
    )
    .await;

    c.app().one_time_token().send_email_tokens().await?;
}

/* TODO
browser_test! { creates_login_link_from_frontend (mut d)
    d.goto("http://127.0.0.1:5173").await;
    wait_here();
    // I would like to test that a user using a mobile browser
    // that tries to log-in with X, sees a 'warning' about the action
    // possibly failing. And a button to continue logging in with X.
    todo!("I should write this test");
}

browser_test! { shows_warning_when_trying_to_log_in_with_x_mobile (mut d)
    d.goto("http://127.0.0.1:5173").await;
    wait_here();
    // I would like to test that a user using a mobile browser
    // that tries to log-in with X, sees a 'warning' about the action
    // possibly failing. And a button to continue logging in with X.
    todo!("I should write this test");
}
*/
