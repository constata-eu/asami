use anyhow::Result;
use ethers::types::{Address, U256};
use integration_tests::{TestEnvironment, MEMBER_ONE, MEMBER_THREE, MEMBER_TWO};

#[tokio::test]
async fn club_distributes_gifts_and_serves_members() -> Result<()> {
    let mut club = TestEnvironment::start().await?;

    club.add_member(MEMBER_ONE, 1, "@rootstock").await?;
    club.add_member(MEMBER_TWO, 2, "Satoshi Friend").await?;
    club.add_member(MEMBER_THREE, 3, MEMBER_THREE).await?;
    club.configure_asset(Address::zero(), true, tokens(1)).await?;
    club.set_sender_name("Money on Chain").await?;

    club.send_native(tokens(10)).await?;
    club.run_cycles(4).await?;
    assert_eq!(
        club.count("SELECT count(*) FROM disbursements WHERE status = 'completed'").await?,
        1
    );
    assert_eq!(
        club.count("SELECT count(*) FROM payouts WHERE status = 'confirmed'").await?,
        3
    );

    // A gift that the owner already withdrew is retained as a terminal,
    // auditable failure instead of being recalculated or announced as paid.
    club.send_native(tokens(2)).await?;
    club.withdraw_native(tokens(3)).await?;
    club.run_cycle().await?;
    assert_eq!(
        club.count(
            "SELECT count(*) FROM disbursements WHERE status = 'failed' AND failure_reason = 'funds_not_available'"
        )
        .await?,
        1
    );

    // Below-minimum transfers stay indexed and are reconsidered when the
    // administrator lowers the asset minimum.
    let accepted = club.deploy_token("Accepted Gift", "GIFT").await?;
    club.configure_asset(accepted, true, tokens(100)).await?;
    club.mint_and_send_token(accepted, tokens(10)).await?;
    club.run_cycle().await?;
    assert_eq!(
        club.count("SELECT count(*) FROM incoming_transfers WHERE processing_status = 'below_minimum'")
            .await?,
        1
    );
    club.configure_asset(accepted, true, tokens(5)).await?;
    club.run_cycles(4).await?;

    // Low gas stops on the first unpaid intent, queues one English message,
    // and resumes safely after the automator is refilled.
    club.mint_and_send_token(accepted, tokens(6)).await?;
    club.set_automator_balance(U256::zero()).await?;
    club.run_cycle().await?;
    assert_eq!(
        club.count("SELECT count(*) FROM payouts WHERE status = 'blocked_low_gas'").await?,
        1
    );
    assert_eq!(
        club.count("SELECT count(*) FROM tweet_queue WHERE kind = 'low_gas'").await?,
        1
    );
    club.run_cycle().await?;
    assert_eq!(
        club.count("SELECT count(*) FROM tweet_queue WHERE kind = 'low_gas'").await?,
        1
    );
    club.set_automator_balance(tokens(10)).await?;
    club.run_cycles(4).await?;

    // Arbitrary tokens are indexed but never treated as accepted. Discovery
    // creates one English notification job exactly once.
    let unknown = club.deploy_token("Unknown Gift", "WHAT").await?;
    club.mint_and_send_token(unknown, tokens(7)).await?;
    club.run_cycle().await?;
    assert_eq!(club.count("SELECT count(*) FROM seen_token_alerts").await?, 1);
    assert_eq!(
        club.count("SELECT count(*) FROM tweet_queue WHERE kind = 'new_unconfigured_token'").await?,
        1
    );
    club.run_cycle().await?;
    assert_eq!(
        club.count("SELECT count(*) FROM tweet_queue WHERE kind = 'new_unconfigured_token'").await?,
        1
    );

    // Exercise the embedded frontend through the real Rust API server and the
    // repository's pinned Chromium/ChromeDriver fixture.
    club.start_web(true).await?;
    assert_eq!(club.browser().count(".member-card").await?, 3);
    assert_eq!(club.browser().count(".avatar-photo").await?, 1);
    assert_eq!(club.browser().count(".avatar-identicon").await?, 3);
    assert_eq!(club.browser().count(".gratitude-title").await?, 2);
    assert_eq!(club.browser().count(".gratitude-panel img").await?, 0);
    assert_eq!(club.browser().text(".member-name").await?, MEMBER_THREE);
    assert_eq!(
        club.browser().attribute("a[href='https://x.com/rootstock']", "href").await?,
        Some("https://x.com/rootstock".to_string())
    );
    let address_link = format!("https://rootstock.blockscout.com/address/{MEMBER_THREE}");
    assert_eq!(
        club.browser().attribute(&format!("a[href='{address_link}']"), "href").await?,
        Some(address_link)
    );
    assert_eq!(club.browser().count(".member-name a").await?, 2);

    // Every successful member payout has one persisted attempt and receipt.
    assert_eq!(
        club.count(
            "SELECT count(*) FROM payouts WHERE status = 'confirmed' AND successful_transaction_hash IS NOT NULL"
        )
        .await?,
        9
    );
    assert_eq!(
        club.count("SELECT count(*) FROM payout_attempts WHERE receipt_status = 1").await?,
        9
    );
    // Rootstock rejects EIP-1559 submissions. All automator payouts must be
    // signed as legacy transactions even when the development node supports both.
    assert_eq!(club.count_legacy_payout_transactions().await?, 9);

    club.stop().await;
    Ok(())
}

fn tokens(amount: u64) -> U256 {
    U256::exp10(18) * amount
}
