use ::api::models::*;
use api::Error;

use super::*;

#[tokio::test]
#[serial_test::file_serial]
async fn weekly_score_picks_are_filled_once_per_week_without_duplicates() {
    TestHelper::run(|h| async move {
        let alice = h.collaborator(200).await;
        let bob = h.collaborator(200).await;
        let carl = h.collaborator(200).await;
        let dan = h.collaborator(200).await;

        let picks = h.a().app.weekly_score_pick().ensure_current(3).await.unwrap();
        assert_eq!(picks.len(), 3);

        let target_ids: std::collections::HashSet<String> =
            picks.iter().map(|pick| pick.target_account_id().clone()).collect();
        assert_eq!(target_ids.len(), 3);

        let expected_accounts: std::collections::HashSet<String> = [
            alice.account().await.id().clone(),
            bob.account().await.id().clone(),
            carl.account().await.id().clone(),
            dan.account().await.id().clone(),
        ]
        .into_iter()
        .collect();

        assert!(target_ids.is_subset(&expected_accounts));

        let second_pass = h.a().app.weekly_score_pick().ensure_current(3).await.unwrap();
        assert_eq!(second_pass.len(), 3);

        let second_ids: std::collections::HashSet<String> =
            second_pass.iter().map(|pick| pick.target_account_id().clone()).collect();
        assert_eq!(target_ids, second_ids);
    })
    .await
}

#[tokio::test]
#[serial_test::file_serial]
async fn score_card_cooldown_and_effective_score_aggregation_work() {
    TestHelper::run(|h| async move {
        let alice = h.collaborator(200).await;
        let bob = h.collaborator(200).await;
        let carl = h.collaborator(200).await;
        let dan = h.collaborator(200).await;

        let alice_account = alice.account().await;
        let bob_account = bob.account().await;
        let carl_account = carl.account().await;
        let dan_account = dan.account().await;

        h.a()
            .app
            .score_card()
            .create_checked(InsertScoreCard {
                scorer_account_id: alice_account.id().clone(),
                target_account_id: bob_account.id().clone(),
                priority: false,
                alignment: 6,
                trust: 6,
                contribution: 6,
                advocacy: 6,
                judgment: 6,
                rootstock_joined_at: Utc::now() - Duration::days(600),
                comment: Some("Knows the ecosystem well".to_string()),
            })
            .await
            .unwrap();

        match h
            .a()
            .app
            .score_card()
            .create_checked(InsertScoreCard {
                scorer_account_id: alice_account.id().clone(),
                target_account_id: bob_account.id().clone(),
                priority: false,
                alignment: 7,
                trust: 7,
                contribution: 7,
                advocacy: 7,
                judgment: 7,
                rootstock_joined_at: Utc::now() - Duration::days(500),
                comment: None,
            })
            .await
            .unwrap_err()
        {
            Error::Validation(field, message) => {
                assert_eq!(field, "target_account_id");
                assert_eq!(message, "score_card_cooldown_active");
            }
            other => panic!("unexpected error: {other:?}"),
        }

        h.a()
            .app
            .score_card()
            .create_checked(InsertScoreCard {
                scorer_account_id: carl_account.id().clone(),
                target_account_id: bob_account.id().clone(),
                priority: false,
                alignment: 4,
                trust: 4,
                contribution: 4,
                advocacy: 4,
                judgment: 4,
                rootstock_joined_at: Utc::now() - Duration::days(1000),
                comment: None,
            })
            .await
            .unwrap();

        h.a()
            .app
            .score_card()
            .create_checked(InsertScoreCard {
                scorer_account_id: dan_account.id().clone(),
                target_account_id: bob_account.id().clone(),
                priority: true,
                alignment: 10,
                trust: 10,
                contribution: 10,
                advocacy: 10,
                judgment: 10,
                rootstock_joined_at: Utc::now() - Duration::days(1200),
                comment: Some("Priority override".to_string()),
            })
            .await
            .unwrap();

        let effective = h
            .a()
            .app
            .score_card()
            .effective_score_for_target(bob_account.id())
            .await
            .unwrap()
            .unwrap();

        assert_eq!(effective.scorecards_received, 3);
        assert_eq!(effective.scorecards_issued, 0);
        assert_eq!(effective.rootstock_joined_at.unwrap().date_naive(), (Utc::now() - Duration::days(1200)).date_naive());
        assert_eq!(effective.base_alignment.unwrap(), 7.5);
        assert_eq!(effective.base_trust.unwrap(), 7.5);
        assert_eq!(effective.base_contribution.unwrap(), 7.5);
        assert_eq!(effective.base_advocacy.unwrap(), 7.5);
        assert_eq!(effective.base_judgment.unwrap(), 7.5);

        let creator_scores = h
            .a()
            .app
            .score_card()
            .creator_scores_for_targets(alice_account.id(), &[bob_account.id().clone()])
            .await
            .unwrap();

        let creator_score = creator_scores.get(bob_account.id()).unwrap();
        assert_eq!(creator_score.final_alignment.unwrap(), 6.75);
        assert_eq!(creator_score.final_trust.unwrap(), 6.75);
        assert_eq!(creator_score.final_contribution.unwrap(), 6.75);
        assert_eq!(creator_score.final_advocacy.unwrap(), 6.75);
        assert_eq!(creator_score.final_judgment.unwrap(), 6.75);
    })
    .await
}
