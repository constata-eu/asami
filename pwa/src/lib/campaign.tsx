
export const contentId = (campaign) => JSON.parse(campaign.briefingJson)

export const viewPostUrl = (campaign) => {
  if(campaign.site == "INSTAGRAM"){
    return `https://instagram.com/p/${contentId(campaign)}`;
  } else {
    return `https://x.com/user/status/${contentId(campaign)}`;
  }
}

export const validateCampaignLink = (url) => {
  let error = null;

  try {
    const u = new URL(url);
    const path = u.pathname.replace(/\/$/, '').split("/");
    const contentId = path[path.length - 1];

    const is_x = (u.host.match(/\.?x\.com$/) || u.host.match(/\.?twitter\.com$/)) && contentId.match(/^\d+$/);
		const is_ig = u.host.match(/\.?instagram.com$/) && contentId.match(/^[\d\w\-_]+$/);

		if (!is_x && !is_ig) {
			return "not_a_post_url";
		}

  } catch {
		return "invalid_url";
	}
	
	return null;
}
