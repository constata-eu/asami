use super::*;

make_sql_enum![
  "site",
  pub enum Site {
    X,
    Instagram,
    LinkedIn,
    Facebook,
    TikTok,
    Youtube,
    Nostr,
    Bluesky,
    Reserved1,
    Reserved2,
    Reserved3,
    Reserved4,
    Reserved5,
    Reserved6,
  }
];

impl Site {
  pub fn from_on_chain(o: u8) -> Self {
    match o {
      0 => Self::X,
      1 => Self::Instagram,
      2 => Self::Instagram,
      3 => Self::Nostr,
      _ => panic!("mismatched site on contract"),
    }
  }
}
