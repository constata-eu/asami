use super::*;
use chrono::Duration;
use image_hasher::{HashAlg, HasherConfig, ImageHash};
use std::{
    collections::{HashMap, HashSet},
    io::Cursor,
};

model! {
  state: App,
  table: ig_crawls,
  struct IgCrawl {
    #[sqlx_model_hints(int4, default)]
    id: i32,
    #[sqlx_model_hints(timestamptz, default)]
    created_at: DateTime<Utc>,
    #[sqlx_model_hints(ig_crawl_status, default)]
    status: IgCrawlStatus,
    #[sqlx_model_hints(text)]
    input: String,
    #[sqlx_model_hints(varchar, default)]
    apify_id: Option<String>,
    #[sqlx_model_hints(boolean, default)]
    processed_for_campaign_rules: bool,
    #[sqlx_model_hints(boolean, default)]
    processed_for_handle_requests: bool,
    #[sqlx_model_hints(boolean, default)]
    processed_for_collabs: bool,
    #[sqlx_model_hints(varchar, default)]
    log_text: String,
  },
  queries {
    recent_ig_crawls("now() - created_at < $1", cooldown: Duration ),
  },
  has_many {
    IgCrawlResult(crawl_id),
  }
}

impl IgCrawlHub {
    pub async fn do_everything(&self) -> anyhow::Result<()> {
        self.schedule_new().await?;
        self.submit_scheduled().await?;
        self.collect_all_responses().await?;
        self.process_for_campaign_rules().await?;
        self.process_for_handle_requests().await?;
        self.process_for_collabs().await?;
        Ok(())
    }

    pub fn settings(&self) -> &InstagramConfig {
        &self.state.settings.instagram
    }

    pub async fn schedule_new(&self) -> anyhow::Result<()> {
        let cooldown = Duration::minutes(self.settings().crawl_cooldown_minutes);

        if self.recent_ig_crawls(cooldown).optional().await?.is_some() {
            return Ok(());
        }

        let mut direct_urls: HashSet<String> = HashSet::new();

        direct_urls.extend(
            self.state
                .handle()
                .select()
                .site_eq(Site::Instagram)
                .status_in(vec![HandleStatus::Unverified, HandleStatus::Active])
                .all()
                .await?
                .into_iter()
                .map(|h| format!("https://www.instagram.com/{}", h.attrs.username)),
        );

        for c in self
            .state
            .campaign()
            .select()
            .campaign_kind_eq(CampaignKind::IgClonePost)
            .budget_gt(weihex("0"))
            .all()
            .await?
        {
            if c.is_missing_ig_rules().await? {
                direct_urls.insert(format!("https://www.instagram.com/p/{}", c.content_id()?));
            }
        }

        let input = serde_json::to_string(&serde_json::json![{
          "addParentData": true,
          "directUrls": direct_urls,
          "enhanceUserSearchWithFacebookPage": false,
          "isUserTaggedFeedURL": false,
          "resultsLimit": 20,
          "resultsType": "details",
          "searchLimit": 1,
          "searchType": "hashtag"
        }])
        .map_err(|_| Error::Runtime("could build input json string".to_string()))?;

        self.insert(InsertIgCrawl { input }).save().await?;

        Ok(())
    }

    pub async fn submit_scheduled(&self) -> anyhow::Result<()> {
        let api_url = "https://api.apify.com/v2/acts/apify~instagram-scraper/runs";
        let token = &self.state.settings.instagram.apify_key;

        for c in self.select().status_eq(IgCrawlStatus::Scheduled).all().await? {
            let result = ureq_agent()
                .post(&format!("{api_url}?token={token}"))
                .set("Content-Type", "application/json")
                .send_string(c.input());

            match result {
                Ok(response) => {
                    let apify_id = response.into_json::<ApifyActorRun>()?.data.id;
                    c.update().status(IgCrawlStatus::Submitted).apify_id(Some(apify_id)).save().await?;
                }
                Err(e) => {
                    let msg = if let ureq::Error::Status(_, r) = e {
                        r.into_string()?
                    } else {
                        e.to_string()
                    };
                    c.update().status(IgCrawlStatus::Cancelled).log_text(msg).save().await?;
                }
            }
        }

        Ok(())
    }

    pub async fn collect_all_responses(&self) -> anyhow::Result<()> {
        let api_runs_url = "https://api.apify.com/v2/actor-runs";
        let api_datasets_url = "https://api.apify.com/v2/datasets";
        let token = &self.state.settings.instagram.apify_key;

        for crawl in self.select().status_eq(IgCrawlStatus::Submitted).all().await? {
            let run_id = crawl
                .apify_id()
                .as_ref()
                .ok_or_else(|| Error::Runtime(format!("submitted crawl had no apify id {:?}", &crawl.attrs)))?;

            let result = ureq_agent().get(&format!("{api_runs_url}/{run_id}?token={token}")).call();

            let response = match result {
                Ok(r) => r,
                Err(e) => {
                    let msg = if let ureq::Error::Status(_, r) = e {
                        r.into_string()?
                    } else {
                        e.to_string()
                    };
                    crawl.update().log_text(msg).save().await?;
                    continue;
                }
            };

            let meta = response.into_json::<ApifyActorRun>()?.data;

            match meta.status.as_str() {
                "FAILED" | "TIMED-OUT" | "ABORTED" => {
                    crawl
                        .update()
                        .status(IgCrawlStatus::Cancelled)
                        .log_text(format!("crawl did not complete {:?}", &meta))
                        .save()
                        .await?;
                }
                "SUCCEEDED" => {
                    let dataset_id = meta.default_dataset_id;
                    let items: Vec<IgResult> = ureq_agent()
                        .get(&format!("{api_datasets_url}/{dataset_id}/items?token={token}"))
                        .call()?
                        .into_json()?;

                    for i in items {
                        let json_string = serde_json::to_string(&i)
                            .map_err(|_| Error::Runtime("IgProfile not serializable?".into()))?;

                        self.state
                            .ig_crawl_result()
                            .insert(InsertIgCrawlResult {
                                crawl_id: crawl.attrs.id,
                                json_string,
                            })
                            .save()
                            .await?;
                    }

                    crawl.update().status(IgCrawlStatus::Responded).save().await?;
                }
                _ => {
                    crawl.update().log_text(format!("crawl still working {:?}", &meta)).save().await?;
                }
            }
        }

        Ok(())
    }

    pub async fn process_for_campaign_rules(&self) -> anyhow::Result<()> {
        for crawl in self
            .select()
            .processed_for_campaign_rules_eq(false)
            .status_eq(IgCrawlStatus::Responded)
            .all()
            .await?
        {
            for result in crawl.ig_crawl_result_scope().processed_for_campaign_rules_eq(false).all().await? {
                result.process_for_campaign_rules().await?;
            }
            crawl.update().processed_for_campaign_rules(true).save().await?;
        }

        Ok(())
    }

    pub async fn process_for_handle_requests(&self) -> anyhow::Result<()> {
        let (_, verification_image_hash) = get_url_image_hash(&self.state.settings.instagram.verification_image_url)?;

        let verification_caption_regex = regex::Regex::new(&format!(
            r#"^[\n\r\s]*{} \[(\d*)\]"#,
            self.settings().verification_caption
        ))?;

        for crawl in self
            .select()
            .processed_for_handle_requests_eq(false)
            .status_eq(IgCrawlStatus::Responded)
            .all()
            .await?
        {
            for result in crawl.ig_crawl_result_scope().processed_for_handle_requests_eq(false).all().await? {
                result.process_for_handle_requests(&verification_image_hash, &verification_caption_regex).await?;
            }
            crawl.update().processed_for_handle_requests(true).save().await?;
        }

        Ok(())
    }

    pub async fn process_for_collabs(&self) -> anyhow::Result<()> {
        let mut campaigns = vec![];

        for campaign in self
            .state
            .campaign()
            .select()
            .campaign_kind_eq(CampaignKind::IgClonePost)
            .budget_gt(weihex("0"))
            .all()
            .await?
        {
            if let Some(rule) = self.state.ig_campaign_rule().select().campaign_id_eq(campaign.id()).optional().await? {
                campaigns.push((campaign, rule));
            }
        }

        for crawl in self.select().processed_for_collabs_eq(false).status_eq(IgCrawlStatus::Responded).all().await? {
            for result in crawl.ig_crawl_result_scope().processed_for_collabs_eq(false).all().await? {
                result.process_for_collabs(&campaigns).await?;
            }
            crawl.update().processed_for_collabs(true).save().await?;
        }

        Ok(())
    }
}

model! {
  state: App,
  table: ig_crawl_results,
  struct IgCrawlResult {
    #[sqlx_model_hints(int4, default)]
    id: i32,
    #[sqlx_model_hints(int4)]
    crawl_id: i32,
    #[sqlx_model_hints(text)]
    json_string: String,
    #[sqlx_model_hints(boolean, default)]
    processed_for_campaign_rules: bool,
    #[sqlx_model_hints(boolean, default)]
    processed_for_handle_requests: bool,
    #[sqlx_model_hints(boolean, default)]
    processed_for_collabs: bool,
    #[sqlx_model_hints(varchar, default)]
    log_text: String,
    #[sqlx_model_hints(timestamptz, default)]
    created_at: DateTime<Utc>,
  },
  belongs_to {
    IgCrawl(crawl_id),
  }
}

impl IgCrawlResult {
    pub async fn process_for_campaign_rules(self) -> anyhow::Result<()> {
        let ig_result = serde_json::from_str(self.json_string())
            .map_err(|_| Error::Runtime("stored invalid IgResult in DB".into()))?;

        let IgResult::IgPost(post) = ig_result else {
            return Ok(());
        };

        let Some(campaign) = self
            .state
            .campaign()
            .select()
            .campaign_kind_eq(CampaignKind::IgClonePost)
            .briefing_json_eq(&serde_json::to_string(&post.short_code)?)
            .optional()
            .await?
        else {
            return Ok(());
        };

        if !campaign.is_missing_ig_rules().await? {
            return Ok(());
        }

        let (image, hash) = get_url_image_hash(&post.display_url)?;

        self.state
            .ig_campaign_rule()
            .insert(InsertIgCampaignRule {
                campaign_id: campaign.attrs.id,
                display_url: post.display_url,
                image_hash: hash.to_base64(),
                image,
                caption: post.caption,
            })
            .save()
            .await?;

        Ok(())
    }

    pub async fn process_for_handle_requests(
        mut self,
        verification_hash: &ImageHash,
        caption_regex: &regex::Regex,
    ) -> anyhow::Result<()> {
        let ig_result = serde_json::from_str(self.json_string())
            .map_err(|_| Error::Runtime("stored invalid IgResult in DB".into()))?;

        let IgResult::IgProfile(profile) = ig_result else {
            return Ok(());
        };

        let maybe_handle = self
            .state
            .handle()
            .select()
            .username_ilike(profile.username)
            .site_eq(Site::Instagram)
            .status_eq(HandleStatus::Unverified)
            .optional()
            .await?;

        match maybe_handle {
            None => {
                self.log("Skipped. No pending handle request").await?;
            }
            Some(handle) => {
                for post in &profile.latest_posts {
                    if let Some(capture) = caption_regex.captures(post.caption.trim()) {
                        let Ok(account_id) = capture[1].parse::<String>().map(weihex) else {
                            self.log_post(post, "no discernable account id in caption").await?;
                            continue;
                        };

                        if &account_id != handle.account_id() {
                            self.log_post(post, "account id in caption did not match request account").await?;
                            continue;
                        }
                    } else {
                        self.log_post(post, "post had no matching caption").await?;
                        continue;
                    }

                    let posted_hash = match get_url_image_hash(&post.display_url) {
                        Ok((_, h)) => h,
                        Err(e) => {
                            self.log_post(post, &format!("could not fetch display_url at this time: {e:?}")).await?;
                            continue;
                        }
                    };

                    let distance = verification_hash.dist(&posted_hash);
                    if distance > 200 {
                        self.log_post(post, &format!("Distance was {}", distance)).await?;
                        continue;
                    }

                    let score = U256::from(profile.followers_count) * wei("85") / wei("100");
                    handle.verify(profile.id).await?.set_score(score).await?;

                    self.log_post(post, "verified and appraised").await?;
                    break;
                }
            }
        }

        self.update().processed_for_handle_requests(true).save().await?;

        Ok(())
    }

    pub async fn process_for_collabs(mut self, campaigns: &[(Campaign, IgCampaignRule)]) -> anyhow::Result<()> {
        let ig_result = serde_json::from_str(self.json_string())
            .map_err(|_| Error::Runtime("stored invalid IgResult in DB".into()))?;

        let IgResult::IgProfile(profile) = ig_result else {
            return Ok(());
        };

        let maybe_handle = self
            .state
            .handle()
            .select()
            .username_eq(profile.username.clone())
            .site_eq(Site::Instagram)
            .order_by(HandleOrderBy::Id)
            .desc(true)
            .optional()
            .await?;

        match maybe_handle {
            None => {
                self.log("Skipped. Handle not synced yet. If it exists, we'll get it next time.").await?;
            }
            Some(handle) => {
                if *handle.status() != HandleStatus::Active {
                    self.log("Handle is inactive at the moment.").await?;
                    return Ok(());
                }

                let mut post_hashes = HashMap::new();

                for post in &profile.latest_posts {
                    for (campaign, rule) in campaigns {
                        if !post.caption.trim().starts_with(rule.attrs.caption.trim()) {
                            self.log_post(post, &format!("did not match caption for {}", campaign.id())).await?;
                            continue;
                        }

                        let posted_hash = match post_hashes
                            .entry(&post.id)
                            .or_insert_with(|| get_url_image_hash(&post.display_url).map(|x| x.1))
                        {
                            Ok(h) => h,
                            Err(e) => {
                                let description = format!("could not fetch display_url {:?} {:?}", post, e);
                                self.log_post(post, &description).await?;
                                continue;
                            }
                        };

                        let distance = rule.get_image_hash()?.dist(posted_hash);
                        if distance > 200 {
                            self.log_post(post, &format!("Distance was {}", distance)).await?;
                            continue;
                        }

                        let Some(reward) = handle.reward_for(&campaign) else {
                            self.log_post(post, "User reward for this campaign was none.").await?;
                            continue;
                        };

                        match campaign.make_collab(&handle, reward, &profile.username).await {
                            Err(Error::Validation(field, value)) => {
                                self.log_post(post, &format!("could be a collab, but was invalid {} {}", field, value))
                                    .await?;
                            }
                            Err(e) => return Err(e.into()),
                            Ok(collab) => {
                                self.log_post(post, &format!("Made collab request {}", collab.attrs.id)).await?;
                            }
                        }
                    }
                }
            }
        }

        self.update().processed_for_collabs(true).save().await?;

        Ok(())
    }

    pub async fn log(&mut self, line: &str) -> anyhow::Result<()> {
        self.reload().await?;
        self.clone().update().log_text(format!("{}{}\n", self.log_text(), line)).save().await?;
        Ok(())
    }

    pub async fn log_post(&mut self, post: &IgPost, line: &str) -> anyhow::Result<()> {
        self.log(&format!("Post {}: {}", &post.id, line)).await
    }

    pub fn settings(&self) -> &InstagramConfig {
        &self.state.settings.instagram
    }
}

model! {
  state: App,
  table: ig_campaign_rules,
  struct IgCampaignRule {
    #[sqlx_model_hints(int4, default)]
    id: i32,
    #[sqlx_model_hints(int4)]
    campaign_id: i32,
    #[sqlx_model_hints(text)]
    display_url: String,
    #[sqlx_model_hints(bytea)]
    image: Vec<u8>,
    #[sqlx_model_hints(text)]
    image_hash: String,
    #[sqlx_model_hints(text)]
    caption: String,
  },
  belongs_to {
    Campaign(campaign_id),
  }
}

impl IgCampaignRule {
    pub fn get_image_hash(&self) -> anyhow::Result<ImageHash> {
        ImageHash::from_base64(self.image_hash())
            .map_err(|_| Error::service("db", "invalid_image_hash_on_campaign_rule_table").into())
    }
}

pub fn get_url_image_hash(url: &str) -> anyhow::Result<(Vec<u8>, ImageHash)> {
    use image::io::Reader as ImageReader;
    use std::io::Read;

    let hasher = HasherConfig::new().hash_alg(HashAlg::DoubleGradient).hash_size(100, 100).to_hasher();
    let resp = ureq_agent().get(url).call()?;

    let len: usize = 1024 * 1024;
    let mut bytes: Vec<u8> = Vec::with_capacity(len);
    resp.into_reader().take(len as u64).read_to_end(&mut bytes)?;

    let image = ImageReader::with_format(Cursor::new(&bytes), image::ImageFormat::Jpeg)
        .decode()
        .map_err(|e| Error::service("image_hasher", &e.to_string()))?;

    Ok((bytes, hasher.hash_image(&image)))
}

pub fn ureq_agent() -> ureq::Agent {
    ureq::AgentBuilder::new().timeout(std::time::Duration::from_secs(5)).build()
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum IgResult {
    IgProfile(IgProfile),
    IgPost(IgPost),
    IgError(IgError),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IgProfile {
    id: String,
    username: String,
    is_business_account: bool,
    joined_recently: bool,
    private: bool,
    verified: bool,
    profile_pic_url: String,
    followers_count: u64,
    follows_count: u64,
    posts_count: u64,
    latest_posts: Vec<IgPost>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IgPost {
    id: String,
    #[serde(rename = "type")]
    post_type: String,
    short_code: String,
    caption: String,
    dimensions_height: u64,
    dimensions_width: u64,
    display_url: String,
    owner_username: String,
    owner_id: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IgError {
    input_url: Option<String>,
    error: String,
    username: Option<String>,
    url: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApifyActorRun {
    data: ApifyActorData,
}
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApifyActorData {
    id: String,
    status: String,
    default_dataset_id: String,
}

#[derive(Copy, Clone, Debug, PartialEq, Deserialize, Serialize, sqlx::Type, GraphQLEnum)]
#[sqlx(type_name = "ig_crawl_status", rename_all = "snake_case")]
pub enum IgCrawlStatus {
    Scheduled,
    Submitted,
    Responded,
    Cancelled,
}

impl sqlx::postgres::PgHasArrayType for IgCrawlStatus {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("_ig_crawl_status")
    }
}
