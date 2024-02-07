use super::*;

make_sql_enum!["site", pub enum Site {
  X,
  Instagram,
  Nostr
}];

impl Site {
  pub fn from_on_chain(o: u8) -> Self {
    match o {
      0 => Self::X,
      1 => Self::Nostr,
      2 => Self::Instagram,
      _ => panic!("mismatched site on contract")
    }
  }
}
