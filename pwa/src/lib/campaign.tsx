export const campaignBriefing = (campaign) => JSON.parse(campaign.briefingJson);

export const contentId = (campaign) => {
  const briefing = campaignBriefing(campaign);
  return briefing.kind === "x_repost" ? briefing.tweet_id : null;
};

export const isXRepostCampaign = (campaign) => campaign.kind === "X_REPOST";
export const isCommunityGiveawayCampaign = (campaign) =>
  campaign.kind === "COMMUNITY_GIVEAWAY";

export const campaignLabel = (campaign) =>
  isXRepostCampaign(campaign) ? contentId(campaign) : "Community giveaway";

export const viewPostUrl = (campaign) => {
  return `https://x.com/user/status/${contentId(campaign)}`;
};

export const validateCampaignLink = (url) => {
  let error = null;

  try {
    const u = new URL(url);
    const path = u.pathname.replace(/\/$/, '').split("/");
    const contentId = path[path.length - 1];

    const is_x = (u.host.match(/\.?x\.com$/) || u.host.match(/\.?twitter\.com$/)) && contentId.match(/^\d+$/);

    if (!is_x) {
        return "not_a_post_url";
    }

  } catch {
		return "invalid_url";
  }
	
  return null;
};
