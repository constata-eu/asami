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
    }
];
