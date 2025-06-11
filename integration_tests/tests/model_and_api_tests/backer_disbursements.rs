use super::*;

#[tokio::test]
#[serial_test::file_serial]
async fn pays_out_backers() {
    TestHelper::run(|h| async move {
      h.test_app.sync_events_until("Backers and disbursements are created", || async {
        h.test_app.app.backer_disbursement().select().count().await.unwrap() > 0
        &&
        h.test_app.app.backer().select().count().await.unwrap() > 0
      }).await;

      dbg!(h.test_app.app.backer_disbursement().find(1).await.unwrap());

      dbg!(h.test_app.app.backer().select().all().await.unwrap());

      // Run task that stores stakes. X2

      // Wait for an on-chain-job that supposedly pays out all backers buut is skipped.

      // Simulate a new disbursement directly in the DB.

      // This disbursment has paid all backers that were staking.

      // They have pending payments.

      // An on-chain-job runs, and pays them.

      // Run task that stores stakes. No new stakes created.

      // Simulate a new disbursement. This one has no payments as all stakes were paid.

      // An on-chain-job runs, get skipped.
    }).await;
}
