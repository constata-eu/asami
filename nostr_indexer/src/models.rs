use sqlx::postgres::{PgPoolOptions, PgConnectOptions};
pub use sqlx_models_orm::{Db, model};
use std::str::FromStr;

pub use sqlx::types::chrono::{DateTime, Utc, TimeZone};
pub type UtcDateTime = DateTime<Utc>;

use nostr_sdk::nostr::{
  UncheckedUrl,
  key::XOnlyPublicKey,
  Event,
  EventId,
  Tag,
  Kind,
};
use url::Url;
use tokio::time::{sleep, Duration};

#[derive(Clone)]
pub struct Site {
  pub db: Db,
}

impl Site {
  pub async fn new() -> anyhow::Result<Self> {
    let conn_string = std::env::var("DATABASE_URL")?;
    let options = PgConnectOptions::from_str(&conn_string)?;
    //let mut options = PgConnectOptions::from_str(&conn_string)?;
    //options.disable_statement_logging();
    let pool_options = PgPoolOptions::new().max_connections(100);
    let pool = pool_options.connect_with(options).await?;
    let db = Db{ pool, transaction: None };

    Ok(Self { db })
  }
}

model!{
  state: Site,
  table: pubkeys,
  struct DbPubkey {
    #[sqlx_model_hints(varchar)]
    id: String,
    #[sqlx_model_hints(timestamptz, default)]
    first_found_at: UtcDateTime,
  },
  has_many{
    DbEvent(pubkey_id), // The events this pubkey has authored.
    DbPubkeySource(pubkey_id), // The events that point to this pubkey, as the author or in their tags.
  }
}

impl DbPubkeyHub {
  pub async fn create_if_missing(&self, id: String) -> sqlx::Result<DbPubkey> {
    if let Some(existing) = self.find_optional(&id).await? {
      return Ok(existing)
    };

    self.insert(InsertDbPubkey{ id }).save().await
  }

  pub async fn add(&self, event_id: &EventId, id: &XOnlyPublicKey) -> anyhow::Result<DbPubkey> {
    let db_pubkey = self.create_if_missing(id.to_string()).await?;

    self.state.db_pubkey_source().insert(InsertDbPubkeySource{
      pubkey_id: db_pubkey.attrs.id.clone(),
      event_id: event_id.to_string(),
    }).save().await?;

    Ok(db_pubkey)
  }
}

model!{
  state: Site,
  table: events,
  struct DbEvent {
    #[sqlx_model_hints(varchar)]
    id: String,
    #[sqlx_model_hints(text)]
    payload: String,
    #[sqlx_model_hints(timestamptz)]
    created_at: UtcDateTime,
    #[sqlx_model_hints(varchar)]
    pubkey_id: String,
  },
  belongs_to{
    DbPubkey(pubkey_id), // The pubkey that authored the event.
  },
  has_many {
    DbPubkeySource(event_id), // All the pubkeys mentioned in this event.
    DbEventSource(event_id), // All the relays that provided this event.
  }
}

impl DbEventHub {
  pub async fn add(&self, event: Event, relay_id: &str) -> anyhow::Result<(DbEvent, Vec<EventId>)> {
    let mut other_event_ids = vec![];

    if let Some(existing) = self.find_optional(&event.id.to_string()).await? {
      return Ok((existing, other_event_ids));
    }

    let db_pubkey = self.state.db_pubkey().add(&event.id, &event.pubkey).await?;

    let created_at = Utc.timestamp_millis_opt(event.created_at.as_i64()).single()
      .ok_or_else(|| anyhow::anyhow!("Invalid creation date for event {}", event.id))?;

    let db_event = self.insert(InsertDbEvent{
      id: event.id.to_string(),
      payload: serde_json::to_string(&event)?,
      created_at: created_at,
      pubkey_id: db_pubkey.attrs.id,
    }).save().await?;

    self.state.db_event_source().insert(InsertDbEventSource{
      event_id: db_event.attrs.id.clone(),
      relay_id: relay_id.to_string(),
    }).save().await?;

    if event.kind == Kind::RecommendRelay {
      self.state.db_relay().add(&event.id, &UncheckedUrl::new(event.content.clone())).await?;
    }

    for tag in &event.tags {
      match tag {
        Tag::Event(other_event_id, maybe_relay_url, _) => {
          if self.state.db_event().find_optional(&other_event_id.to_string()).await?.is_none() {
            other_event_ids.push(other_event_id.clone());
          }

          if let Some(url) = maybe_relay_url {
            self.state.db_relay().add(&event.id, url).await?;
          }
        },
        Tag::PubKey(pubkey, maybe_relay_url) => {
          self.state.db_pubkey().add(&event.id, pubkey).await?;

          if let Some(url) = maybe_relay_url {
            self.state.db_relay().add(&event.id, url).await?;
          }
        },
        Tag::RelayMetadata(url, _) => {
          self.state.db_relay().add(&event.id, url).await?;
        },
        Tag::A { public_key, relay_url, .. } => {
          self.state.db_pubkey().add(&event.id, public_key).await?;

          if let Some(url) = relay_url {
            self.state.db_relay().add(&event.id, url).await?;
          }
        },
        Tag::Relay(url) => {
          self.state.db_relay().add(&event.id, url).await?;
        },
        Tag::ContactList { pk, relay_url, ..} => {
          self.state.db_pubkey().add(&event.id, pk).await?;

          if let Some(url) = relay_url {
            self.state.db_relay().add(&event.id, url).await?;
          }
        },
        Tag::Delegation { delegator_pk, .. } => {
          self.state.db_pubkey().add(&event.id, delegator_pk).await?;
        },
        Tag::Relays(urls) => {
          for url in urls {
            self.state.db_relay().add(&event.id, url).await?;
          }
        },
        _ => (),
      }
    }

    Ok((db_event, other_event_ids))
  }
}

model!{
  state: Site,
  table: relays,
  struct DbRelay {
    #[sqlx_model_hints(varchar)]
    id: String,
    #[sqlx_model_hints(timestamptz, default)]
    first_found_at: UtcDateTime,
    #[sqlx_model_hints(timestamptz, default)]
    banned_until: Option<UtcDateTime>,
  },
  queries {
    relays_found_after("first_found_at > $1 limit 10", d: UtcDateTime)
  },
  has_many{
    DbRelaySource(relay_id), // All the events that mentioned this relay.
    DbEventSource(relay_id), // All the events found on this relay.
    DbRelayLogEntry(relay_id),
  }
}

impl DbRelayHub {
  pub async fn create_if_missing(&self, id: String) -> sqlx::Result<DbRelay> {
    if let Some(existing) = self.find_optional(&id).await? {
      return Ok(existing);
    }

    Ok(self.insert(InsertDbRelay{ id }).save().await?)
  }

  pub async fn add(&self, event_id: &EventId, unchecked_url: &UncheckedUrl) -> anyhow::Result<DbRelay> {
    let url = Url::try_from(unchecked_url.clone())?.to_string();
    let db_relay = self.create_if_missing(url).await?;

    self.state.db_relay_source().insert(InsertDbRelaySource{
      relay_id: db_relay.attrs.id.clone(),
      event_id: event_id.to_string(),
    }).save().await?;

    Ok(db_relay)
  }
}

#[derive(Debug, thiserror::Error)]
pub enum EventProcessingError {
  /*
  #[error(transparent)]
  IntoInner(#[from] std::io::IntoInnerError<std::io::BufWriter<Vec<u8>>>),
    /*
    {
      Ok(x) => x,
      Err(tungstenite::Error::Http(response)) => {
        self.log("tungstenite_connect_http_error", response.body.map(|b| String::from_utf8_lossy(b) ) ).await?;
      },
      e => self.log("other_connect_error", format!("{e:?}") ).await?,
    };
    */
      let Ok(msg) = socket.read_message() else {
        log_short(&site, &relay, "error_reading_socket_message", None).await?;
        continue
      };
      let Ok(msg_text) = msg.to_text() else {
        log_short(&site, &relay, "error_converting_message_to_text", None).await?;
        continue
      };

  */
}

impl DbRelay {
  pub async fn run(relay_id: String) {
    loop {
      let Ok(site) = Site::new().await else {
        println!("{relay_id}: Could not establish db connection. Retrying soon.");
        sleep(Duration::from_secs(60 * 5)).await;
        continue;
      }

      let Ok(relay) = site.db_relay().find(&relay_id).await? else {
        println!("{relay_id}: Relay not found! That's super strange. I'm out.");
        break;
      }

      let result = relay.process_events().await;
      println!("{relay_id}: Work in websocket needs to start over because {result:?}");

      if let Err(e) = result {
        let logging = site.db_relay_log_entry().log(Severity::Error, &relay_id, e.text(), e.json()).await;
        if let Err(oops) = logging {
          println!("{relay_id}: Oops, could not log error because of {oops:?}");
        }
      }

      sleep(Duration::from_secs(60 * 5)).await;
    }
  }

  async fn process_events(self) -> Result<(), EventProcessingError> {
    if self.is_banned() {
      while self.reloaded().await?.is_banned() {
        self.log("waiting_until_unbanned", self.banned_until());
        sleep(Duration::from_secs(60 * 15)).await;
      }
    }

    let (mut socket, response) = match connect(&relay)?;

    self.log("connected", None).await?;

    let main_subscription = SubscriptionId::new("main_subscription");
    socket.write_message(WsMessage::Text(
      ClientMessage::new_req(main_subscription.clone(), vec![Filter::new().limit(1000)]).as_json()
    ))?;

    let pubkeys_limit = 50;
    let mut pubkeys_offset = 0;
    let mut fetch_pubkeys = true;

    loop {
      if fetch_pubkeys {
        let pubkeys = self.state.db_pubkey().select()
          .order_by(DbPubkeyOrderBy::FirstFoundAt)
          .limit(pubkeys_limit)
          .offset(pubkeys_offset)
          .all().await?;

        if !pubkeys.is_empty() {
          self.log("suscribing_for_pubkey_catch_up", pubkey_offset).await?;

          socket.write_message(WsMessage::Text(
            ClientMessage::new_req(
              SubscriptionId::new(&format!("pubkeys_from_{pubkeys_offset}")),
              vec![Filter::new().pubkeys(pubkeys)]
            ).as_json()
          ))?;
        }

        pubkeys_offset += pubkeys_limit;
        fetch_pubkeys = false;
      }

      let msg = socket.read_message()?;
      let msg_text = msg.to_text()?;

      match RelayMessage::from_json(msg_text) {
        Ok(handled_message) => match handled_message {
          RelayMessage::Event { event, .. } => {
            (_, other_event_id) = site.db_event().add(*event, &relay).await?;
            if !other_event_ids.is_empty() {
              self.suscribe(SubscriptionId::generate(), Filter::new().events(other_event_ids))?;
            }
          },
          RelayMessage::EndOfStoredEvents(subscription_id) => {
            if subscription_id != main_subscription {
              socket.write_message(WsMessage::Text(ClientMessage::close(subscription_id).as_json()))?;
              if subscription_id.to_string().starts_with("pubkeys") {
                fetch_pubkeys = false;
              }
              self.log("closing_subscription", Some(subscription_id)).await;
            }
          },
          RelayMessage::Notice{ message } => println!("{relay} Relay: Message {message}"),
          RelayMessage::Ok { event_id, status, message } => println!("{relay} Relay: OK {event_id} {status} {message}"),
          RelayMessage::Auth {..} => println!("{relay} Relay: Unexpected auth message"),
          RelayMessage::Count {..} => println!("{relay} Relay: Unexpected count message"),
          RelayMessage::Empty {..} => println!("{relay} Relay: Empty message"),
        },
        Err(e) => {
          self.log("got_unexpected_message", e).await?;
        }
      }
    }
  }

  fn suscribe(socket: i32, id: SubscriptionId, filter: Filter) -> anyhow::Result<()> {
    socket.write_message(WsMessage::Text( ClientMessage::new_req(id, vec![filter]).as_json() ))?;
  }

  async fn log<S: Serialize>(&self, text: &str, json: Option<S>) -> anyhow::Result<LogEntry> {
    self.state.db_relay_log_entry().log(Severity::Debug, relay_id, text, json).await
  }
}

// Tells that 'event' mentioned 'pubkey'.
model!{
  state: Site,
  table: pubkey_sources,
  struct DbPubkeySource {
    #[sqlx_model_hints(int4, default)]
    id: i32,
    #[sqlx_model_hints(varchar)]
    event_id: String,
    #[sqlx_model_hints(varchar)]
    pubkey_id: String,
    #[sqlx_model_hints(timestamptz, default)]
    created_at: UtcDateTime,
  },
  belongs_to{
    DbEvent(event_id),
    DbPubkey(pubkey_id),
  }
}

// Tells that 'event' mentioned 'relay'.
model!{
  state: Site,
  table: relay_sources,
  struct DbRelaySource {
    #[sqlx_model_hints(int4, default)]
    id: i32,
    #[sqlx_model_hints(varchar)]
    relay_id: String,
    #[sqlx_model_hints(varchar)]
    event_id: String,
    #[sqlx_model_hints(timestamptz, default)]
    created_at: UtcDateTime,
  },
  belongs_to{
    DbRelay(relay_id),
    DbEvent(event_id),
  }
}

// Tells that 'relay' provided 'event'.
model!{
  state: Site,
  table: event_sources,
  struct DbEventSource {
    #[sqlx_model_hints(int4, default)]
    id: i32,
    #[sqlx_model_hints(varchar)]
    event_id: String,
    #[sqlx_model_hints(varchar)]
    relay_id: String,
    #[sqlx_model_hints(timestamptz, default)]
    created_at: UtcDateTime,
  },
  belongs_to{
    DbEvent(event_id),
    DbRelay(relay_id),
  }
}

// Log entries with relay activity, connection, disconnection, etc.
model!{
  state: Site,
  table: event_sources,
  struct DbRelayLogEntries {
    #[sqlx_model_hints(int4, default)]
    id: i32,
    #[sqlx_model_hints(timestamptz, default)]
    created_at: UtcDateTime,
    #[sqlx_model_hints(varchar)]
    relay_id: String,
    #[sqlx_model_hints(varchar)]
    severity: Severity,
    #[sqlx_model_hints(varchar)]
    summary: String,
    #[sqlx_model_hints(text)]
    json: String,
  },
  belongs_to{
    DbRelay(relay_id),
  }
}

impl DbRelayLogEntryHub {
  pub async fn log<S: Serialize>(&self, severity: Severity, relay_id: &str, summary: &str, json: Option<S>) -> anyhow::Result<DbRelayLogEntry> {
    Ok(
      self.insert(InsertDbRelayLogEntry{
        relay_id: relay_id.to_string(),
        summary: summary.to_string(),
        json: serde_json::to_string(json)?
      }).await?
    )
  }
}

#[derive(sqlx::Type, Copy, Clone, Debug, Deserialize, PartialEq, Serialize)]
#[sqlx(type_name = "severity", rename_all = "lowercase")]
pub enum Severity {
  Trace,
  Debug,
  Info,
  Warning,
  Error,
}

