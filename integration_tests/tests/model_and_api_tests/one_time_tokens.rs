use super::*;

app_test! { sends_one_time_tokens (a)
    // Creates a new one time token and someone uses it to log in.
    a.app.one_time_token().create_for_email(
      "alice@example.com".into(),
      api::Lang::Es,
      None
    ).await.unwrap();


    // The token is not longer usable after the first time.

    // The previous person adds a secondary login email.

    // A new person comes in, and claims that previous email
    // There's now only one auth method, for the most recent verifier of that email.
}
