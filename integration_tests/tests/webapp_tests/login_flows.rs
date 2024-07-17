use ::api::models::*;

app_test! { api_creates_and_sends_email_login (a) 
    let result: gql::create_email_login_link::ResponseData = self
        .gql(
            &gql::CreateEmailLoginLink::build_query("alice@example.com"),
            vec![],
        )
        .await;

    dbg!(&result);
}

test! { sends_mail
    // Replace this with your actual SendGrid API key or retrieve it from an environment variable
    let sendgrid_api_key = "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";

    // Create the JSON body
    let body = serde_json::json!({
        "personalizations": [{
            "to": [{
                "email": "yo@nubis.im"
            }]
        }],
        "from": {
            "email": "hola@asami.club"
        },
        "subject": "Sending with SendGrid is Fun",
        "content": [{
            "type": "text/plain",
            "value": "Visit asami.club https://asami.club"
        }]
    });

    // Perform the POST request
    ureq::post("https://api.sendgrid.com/v3/mail/send")
        .set("Authorization", &format!("Bearer {}", sendgrid_api_key))
        .set("Content-Type", "application/json")
        .send_json(body)
        .unwrap();

}

browser_test! { shows_warning_when_trying_to_log_in_with_x_mobile (mut d)
    d.goto("http://127.0.0.1:5173").await;
    //wait_here();
    // I would like to test that a user using a mobile browser
    // that tries to log-in with X, sees a 'warning' about the action
    // possibly failing. And a button to continue logging in with X.
    todo!("I should write this test");
}
