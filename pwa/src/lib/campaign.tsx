export const viewPostUrl = (campaign) => {
  if(campaign.site == "INSTAGRAM"){
    return `https://instagram.com/p/${campaign.contentId}`;
  } else {
    return `https://x.com/user/status/${campaign.contentId}`;
  }
}

export const parseCampaignSiteAndContentId = (url) => {
  const result = {
    error: null,
    site: null,
    contentId: null
  };

  try {
    const u = new URL(url);
    const path = u.pathname.replace(/\/$/, '').split("/");
    const contentId = path[path.length - 1];

    if ( (u.host.match(/\.?x\.com$/) || u.host.match(/\.?twitter\.com$/)) && contentId.match(/^\d+$/) ) {
      result.site = "X";
    } else if (u.host.match(/\.?instagram.com$/) && contentId.match(/^[\d\w\-_]+$/)) {
      result.site = "INSTAGRAM";
    } else {
      result.error = "not_a_post_url";
    }
    result.contentId = contentId;

  } catch {
    result.error = "invalid_url";
  }

  return result;
}

export const defaultValidUntil = () => {
  let currentDate = new Date();
  currentDate.setTime(currentDate.getTime() + (30 * 24 * 60 * 60 * 1000));
  return currentDate;
}

