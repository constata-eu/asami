use super::*;

model! {
  state: App,
  table: auth_methods,
  struct AuthMethod {
    #[sqlx_model_hints(int4, default)]
    id: i32,
    user_id: i32,
    #[sqlx_model_hints(auth_method_kind)]
    kind: AuthMethodKind,
    #[sqlx_model_hints(Varchar)]
    lookup_key: String,
  },
  belongs_to {
    User(user_id)
  }
}

make_sql_enum![
    "auth_method_kind",
    pub enum AuthMethodKind {
        X,
        Facebook,
        Nostr,
        Eip712,
        OneTimeToken,
    }
];

impl AuthMethodKind {
    pub fn from_text(s: &str) -> Option<AuthMethodKind> {
        match s {
            "X" => Some(AuthMethodKind::X),
            "Facebook" => Some(AuthMethodKind::Facebook),
            "Nostr" => Some(AuthMethodKind::Nostr),
            "Eip712" => Some(AuthMethodKind::Eip712),
            "OneTimeToken" => Some(AuthMethodKind::OneTimeToken),
            _ => None,
        }
    }
}
