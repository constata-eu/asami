use super::*;

#[tokio::test(flavor = "multi_thread")]
#[serial_test::file_serial]
async fn creates_campaign_using_stripe() {
    TestHelper::with_web(|h| async move {
        let advertiser = h.user().await;
        advertiser.login_to_web_with_otp().await;
        h.web().navigate("/dashboard?role=advertiser").await;
        h.web().click("#open-start-campaign-stripe-dialog").await;
        h.web().fill_in("input[name='contentUrl']",
            "https://x.com/rootstock_io/status/1922301351725261191"
        ).await;
        h.web().fill_in("input[name='budget']", "200").await;
        h.web().click("#submit-start-campaign-form").await;
        h.web().click("#campaign-done-close").await;
        h.web().wait_for_text("td.column-status span", "Awaiting Payment").await;

        h.web().wait_for("#btn-go-to-checkout-for-1").await;

        TestApp::try_until(100, 50, "No campaign created", || async {
            h.test_app.app.campaign().select().count().await.unwrap() > 0
        }).await;


        let campaign = h.test_app.app.campaign().select().one().await.unwrap();

        send_test_stripe_event_sync(
            &h.test_app.app.settings.stripe.events_secret,
            advertiser.account().await.stripe_customer_id().as_deref().unwrap(),
            *campaign.id()).unwrap();

        h.web().wait_until_gone("#btn-go-to-checkout-for-1").await;

        h.test_app.start_mining().await;
        h.test_app.app.campaign().create_managed_on_chain_campaigns().await.unwrap();
        h.test_app.stop_mining().await;

        h.test_app
            .sync_events_until("Campaign should be published", || async {
                *campaign.reloaded().await.unwrap().status() == models::CampaignStatus::Published 
            })
            .await;
        h.web().wait_for_text("td.column-status span", "Live Campaign").await;
    })
    .await;
}

#[tokio::test(flavor = "multi_thread")]
#[serial_test::file_serial]
async fn admin_address_has_an_account_too() {
    TestHelper::run(|h| async move {
        assert_eq!(h.a().app.account().select().count().await.unwrap(), 0);
        let account = h.a().app.get_admin_own_account().await.unwrap();
        assert_eq!(
            *account.addr().as_ref().unwrap(),
            h.a().app.on_chain.client.address().encode_hex()
        );
        assert_eq!(
            *account.addr().as_ref().unwrap(),
            "0x000000000000000000000000ed4e67213c7a375af60893fe8e0852d0f7040913"
        );

        let reloaded = h.a().app.get_admin_own_account().await.unwrap();
        assert_eq!(account, reloaded);
    })
    .await;
}

pub fn send_test_stripe_event_sync(
    webhook_secret: &str,
    customer: &str,
    campaign_id: i32,
) -> Result<(), Box<dyn std::error::Error>> {
    use hmac::{Hmac, Mac};
    use sha2::Sha256;
    use std::time::{SystemTime, UNIX_EPOCH};

    use serde_json::json;
    type HmacSha256 = Hmac<Sha256>;

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)?
        .as_secs()
        .to_string();

    let payload = json!({
        "id": "evt_test_payment_intent_succeeded",
        "object": "event",
        "type": "payment_intent.succeeded",
        "data": {
            "object": {
                "id": "pi_test_123456789",
                "object": "payment_intent",
                "amount": 2000,
                "currency": "usd",
                "status": "succeeded",
                "metadata": {
                    "campaign_id": campaign_id.to_string()
                },
                "capture_method": "automatic",
                "confirmation_method": "automatic",
                "customer": customer,
                "created": timestamp.parse::<u64>()?,
                "livemode": false,
                "payment_method_types": ["card"]
            }
        },
        "livemode": false,
        "created": timestamp.parse::<u64>()?,
    });

    let payload_string = payload.to_string();
    let signed_payload = format!("{}.{}", timestamp, payload_string);

    let mut mac = HmacSha256::new_from_slice(webhook_secret.as_bytes())?;
    mac.update(signed_payload.as_bytes());
    let signature = hex::encode(mac.finalize().into_bytes());

    let stripe_signature_header = format!("t={},v1={}", timestamp, signature);

    let response_result = ureq::post("http://127.0.0.1:8000/handle_stripe_events")
        .set("Content-Type", "application/json")
        .set("Stripe-Signature", &stripe_signature_header)
        .send_string(&payload_string);

    match response_result {
        Ok(response) => {
            let status = response.status();
            let body = response.into_string().unwrap_or_else(|_| "<could not read body>".into());
            println!("→ Sent fake event. Status: {}, Body: {}", status, body);
        }
        Err(ureq::Error::Status(code, response)) => {
            let body = response.into_string().unwrap_or_else(|_| "<could not read body>".into());
            eprintln!("→ Error response. Status: {}, Body: {}", code, body);
        }
        Err(e) => {
            eprintln!("→ Request failed: {}", e);
        }
    }
    Ok(())
}

// -> Has a 'paid' flag.
// -> Has a column for stripe's payment code, clearing code, or other details.
// -> Once marked paid, a campaign is created (the campaign id is stored.)
// -> Campaign creation is funded by the admin address itself, so it needs doc.
// -> All draft campaigns require DOC to be available in the admin address?
// -> These campaigns may take up to 24 hours to become active.
// -> Any campaign that gets marked paid shoots out an email and report.
//      - this report includes the DOC balance in the admin address and the total balance of all DRAFT and PAID campaigns.

// I need a function that sets-up the admin as a user on its own.

/*
CampaignRequestStatus

requested: Request just received.
paid: Payment on stripe done.
draft: The inner campaign exists, and is in draft status.
    -> Admin address pays the doc and marks submitted.
submitted: The inner campaign exists, and is in submitted status.
published: The inner campaign exists, and is in published status.
created: A draft campaign has been created.
failed


*/

/*
use models::CampaignStatus;

use super::*;
browser_test! { makes_campaign (mut d)
    d.test_app().start_mining().await;

    d.goto("http://127.0.0.1:5173").await;
    d.click("#button-login-as-member").await;
    d.click("#wallet-login-button").await;
    d.link_wallet_and_sign_login().await;
    d.wait_for("#member-dashboard").await;
    d.click("#button-pay-to-amplify").await;
    d.wait_for("#advertiser-dashboard").await;
    d.wait_for("#advertiser-claim-account-pending").await;

    d.test_app().wait_for_job(
        "Claiming account",
        models::OnChainJobKind::PromoteSubAccounts,
        models::OnChainJobStatus::Settled
    ).await;

    d.wait_for("#advertiser-claim-account-done").await;

    d.wait_for("#open-start-campaign-dialog").await;
    d.click("#open-start-campaign-dialog").await;
    wait_here();
    d.fill_in("#contentUrl", "https://x.com/asami_club/status/1716421161867710954?s=20").await;
    d.fill_in("#budget", "20").await;

    d.click("#submit-start-campaign-form").await;
    d.wait_for("#approval-waiter").await;

    try_until(10, 200, "No other window opened", || async {
        d.driver.windows().await.unwrap().len() == 2
    }).await;
    let mut handles = d.driver.windows().await.unwrap();
    d.driver.switch_to_window(handles[1].clone()).await.expect("to switch window zero");

    d.wait_for(".token-allowance-container").await;

    d.fill_in("#custom-spending-cap", "500").await;
    d.click("button[data-testid=page-container-footer-next]").await;
    d.wait_for(".review-spending-cap").await;
    d.click("button[data-testid=page-container-footer-next]").await;
    d.driver.switch_to_window(handles[0].clone()).await.unwrap();

    d.wait_until_gone("#approval-waiter").await;
    d.wait_for("#creation-waiter").await;

    try_until(10, 200, "No other window opened", || async {
        d.driver.windows().await.unwrap().len() == 2
    }).await;
    handles = d.driver.windows().await.unwrap();
    d.driver.switch_to_window(handles[1].clone()).await.expect("to switch window zero");
    d.click("button[data-testid=page-container-footer-next]").await;

    d.driver.switch_to_window(handles[0].clone()).await.unwrap();
    d.wait_for("#campaign-done").await;
    d.click("#campaign-done-close").await;

    d.wait_for("#campaign-list").await;
    d.wait_for_text("#campaign-list td.column-status", "Publishing soon").await;

    d.test_app().sync_events_until("Campaign is published", || async {
        d.app().campaign().select().status_eq(CampaignStatus::Published)
            .count().await.unwrap() > 0
    }).await;

    d.wait_for_text("#campaign-list td.column-status", "Has 20.0 DOC").await;

    d.test_app().sync_events_until("Campaign is published", || async {
        d.app().campaign().select().status_eq(CampaignStatus::Published)
            .count().await.unwrap() == 1
    }).await;

    d.test_app().stop_mining().await;
}

browser_test! { advertiser_always_needs_a_wallet (mut d)
    d.signup_with_one_time_token().await;
    d.click("#balance-card-claim-account-button").await;
    d.link_wallet_and_sign_login().await;
    d.wait_for(".MuiSnackbarContent-message").await;
    d.wait_for("#advertiser-claim-account-pending").await;
    d.test_app().wait_for_job(
        "Claiming account",
        models::OnChainJobKind::PromoteSubAccounts,
        models::OnChainJobStatus::Settled
    ).await;
    d.wait_for("#open-start-campaign-dialog").await;
}
*/
