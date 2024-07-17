/* One time tokens are just single use strings for the one time token authentication method.
 * The authentication strategy will only check that the string exists and has not been used.
 * This token's id is referenced in the lookup key of (at least one) AuthMethod.
 * OneTimeTokens are sent via email or possibly other login methods.
 * They create an authentication method when using the first time.
 * If the authentication method already belonged to someone else,
 * the old one will be removed.
 */

use super::*;
use validators::prelude::*;
use validators::models::Host;
use chbs::{config::BasicConfig, prelude::*};

model! {
  state: App,
  table: one_time_tokens,
  struct OneTimeToken {
    #[sqlx_model_hints(int4, default)]
    id: i32,
    #[sqlx_model_hints(varchar)]
    value: String,
    #[sqlx_model_hints(varchar)]
    lookup_key: String, // The email address, phone number, etc.
    #[sqlx_model_hints(boolean, default)]
    used: bool,
    #[sqlx_model_hints(i32)]
    user_id: Option<i32>,
    #[sqlx_model_hints(timestamptz, default)]
    expires_at: DateTime<Utc>,
    #[sqlx_model_hints(timestamptz, default)]
    sent_at: Option<DateTime<Utc>>,
  }
}

#[derive(Validator)]
#[validator(email(comment(Disallow), ip(Allow), local(Allow), at_least_two_labels(Allow), non_ascii(Allow)))]
pub struct Email {
    pub local_part: String,
    pub need_quoted: bool,
    pub domain_part: Host,
}

impl OneTimeTokenHub {
    pub async fn create_for_email(&self, email: &str, user_id: Option<i32>) -> AsamiResult<OneTimeToken> {
        Email::parse_string(email)?;
        let mut config = BasicConfig::default();
        config.separator = "+".into();
        config.capitalize_first = false.into();
        let value = config.to_scheme().generate()

        Ok(self.insert(InsertOneTimeToken{
            value,
            lookup_key: format!("email:{email}"),
            user_id,
        }).await?)
    }
}
