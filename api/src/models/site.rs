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

impl Site {
    pub fn can_do_campaign_kind(&self, kind: &CampaignKind) -> bool {
        *self == Self::X && kind == &CampaignKind::XRepost
            || *self == Self::Instagram && kind == &CampaignKind::IgClonePost
    }
}
