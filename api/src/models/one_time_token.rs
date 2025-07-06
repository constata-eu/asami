/* One time tokens are just single use strings for the one time token authentication method.
 * The authentication strategy will only check that the string exists and has not been used.
 * This token's id is referenced in the lookup key of (at least one) AuthMethod.
 * OneTimeTokens are sent via email or possibly other login methods.
 * They create an authentication method when using the first time.
 * If the authentication method already belonged to someone else,
 * the old one will be removed.
 */

use validators::{models::Host, prelude::*};

use super::*;

model! {
  state: App,
  table: one_time_tokens,
  #[derive(juniper::GraphQLObject)]
  #[graphql(name = "OneTimeToken")]
  struct OneTimeToken {
    #[sqlx_model_hints(int4, default)]
    id: i32,
    #[sqlx_model_hints(varchar)]
    value: String,
    #[sqlx_model_hints(varchar)]
    lookup_key: String, // The email address, phone number, etc.
    #[sqlx_model_hints(boolean, default)]
    used: bool,
    // The user-id can be used to link a newly created auth method for this token
    // to an existing user, but it's not used yet.
    #[sqlx_model_hints(int4)]
    user_id: Option<i32>,
    #[sqlx_model_hints(timestamptz, op_gt)]
    expires_at: DateTime<Utc>,
    #[sqlx_model_hints(timestamptz, default, op_is_set)]
    sent_at: Option<DateTime<Utc>>,
    #[sqlx_model_hints(text, op_is_set)]
    email: Option<String>,
    #[sqlx_model_hints(language)]
    lang: lang::Lang,
  }
}

#[derive(Validator)]
#[validator(email(
    comment(Disallow),
    ip(Allow),
    local(Allow),
    at_least_two_labels(Allow),
    non_ascii(Allow)
))]
pub struct Email {
    pub local_part: String,
    pub need_quoted: bool,
    pub domain_part: Host,
}

impl OneTimeTokenHub {
    pub fn rand_value() -> String {
        use rand::{thread_rng, Rng};
        use rand::distributions::Alphanumeric;
        thread_rng()
            .sample_iter(&Alphanumeric)
            .take(6)
            .collect::<String>()
            .to_uppercase()
    }

    pub fn default_expiration() -> DateTime<Utc> {
        Utc::now() + chrono::Duration::minutes(20)
    }

    pub async fn create_for_session_migration(&self, user_id: i32) -> AsamiResult<OneTimeToken> {
        let value = Self::rand_value();
        let lookup_key = format!("session_migration:{value}");

        self.state.auth_method().insert(InsertAuthMethod{
            user_id,
            kind: AuthMethodKind::OneTimeToken,
            lookup_key: lookup_key.clone(),
        }).save().await?;
        
        Ok(self
            .insert(InsertOneTimeToken {
                value,
                lookup_key,
                user_id: None,
                email: None,
                lang: Lang::En,
                expires_at: Self::default_expiration(),
            })
            .save()
            .await?)
    }

    pub async fn create_for_email(
        &self,
        email: String,
        lang: lang::Lang,
        user_id: Option<i32>,
    ) -> AsamiResult<OneTimeToken> {
        let canonical = email.to_lowercase();
        Email::parse_string(&canonical)?;
        let value = Self::rand_value();

        Ok(self
            .insert(InsertOneTimeToken {
                value,
                lookup_key: format!("email:{canonical}"),
                user_id,
                email: Some(canonical),
                expires_at: Self::default_expiration(),
                lang,
            })
            .save()
            .await?)
    }

    pub async fn send_email_tokens(&self) -> AsamiResult<()> {
        let all = self.select().sent_at_is_set(false).email_is_set(true).all().await?;
        for token in all {
            let Some(email) = token.email() else {
                continue;
            };

            let subject = match token.lang() {
                lang::Lang::Es => "Link de inicio de sesión en asami.club",
                _ => "Your login link for asami.club",
            };

            let content = match token.lang() {
                lang::Lang::Es => format!("<a href=\"{}/#/one_time_token_login?token={}\">Visita este link</a> para ingresar a asami.club, tienes 10 minutos. Si no estás intentando iniciar sesión, puedes ignorar este correo.", self.state.settings.pwa_host, token.value()),
                _ => format!("<a href=\"{}/#/one_time_token_login?token={}\">Visit this link</a> to log-in to asami.club, you have 10 minutes. If you're not trying to log-in, you can ignore this email.", self.state.settings.pwa_host, token.value()),
            };

            self.state.send_mail(email, subject, &content).await?;

            token.update().sent_at(Some(Utc::now())).save().await?;
        }

        Ok(())
    }
}
