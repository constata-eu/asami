use super::*;

#[tokio::test]
#[serial_test::file_serial]
async fn stores_asami_value_series() {
    std::env::set_var(
        "ROCKET_RSK",
        format!(
            "{{ readonly_mainnet_rpc_url={} }}",
            std::env::var("MAINNET_READONLY_RPC").unwrap()
        ),
    );

    TestHelper::run(|h| async move {
        use models::SeriesName::*;
        h.a().app.db.execute(models::sqlx::query("DELETE FROM value_series")).await.unwrap();

        for variant in [
            AsamiDocPrice,
            AsamiSupply,
            AsamiAssignedTokens,
            AsamiIssuanceRate,
            AsamiFeePool,
        ] {
            let before = h.a().app.value_series().select().count().await.unwrap();
            let one = h.a().app.value_series().store(variant).await.unwrap();
            let two = h.a().app.value_series().store(variant).await.unwrap();
            let after = h.a().app.value_series().select().count().await.unwrap();
            assert_eq!(before + 1, after);
            assert_eq!(&one, &two);
            assert_eq!(h.a().app.value_series().get(variant).await.unwrap(), one);
        }
    })
    .await;
}
