/*
 - Auto-log errors using thiserror.

 - Auto-ban misbehaving relays.
     Update banned status while working.

 - Do not request all pubkey catch-up at once.

 - re-enable newest relay spawning in main.
 */

use nostr_sdk::nostr::{
  ClientMessage,
  Filter,
  RelayMessage,
  SubscriptionId,
  key::XOnlyPublicKey,
};
use tungstenite::{connect, Message as WsMessage};
use itertools::Itertools;
use std::str::FromStr;
use tokio::task::spawn;

mod models;
use models::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  dotenv::dotenv()?;
  env_logger::init();

  let site = Site::new().await?;

  // Seed the first relay.
  site.db_relay().create_if_missing("wss://nos.lol/".to_string()).await?;

  let mut newest_relay_date = Utc.timestamp_opt(0,0).earliest()
    .expect("The literal 0,0 timestamp is always ok");

  let newer_relays = site.db_relay().relays_found_after(newest_relay_date).all().await?;
  newest_relay_date = newer_relays.last()
    .map(|p| p.first_found_at().clone() )
    .unwrap_or(newest_relay_date);

  for relay in newer_relays {
    spawn(DbRelay::run(relay.attrs.id.clone()));
  }

  std::future::pending::<()>().await;
  Ok(())
}
