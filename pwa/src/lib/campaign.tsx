export function viewPostUrl(campaign) {
  if(campaign.site == "INSTAGRAM"){
    return `https://instagram.com/p/${campaign.contentId}`;
  } else {
    return `https://x.com/user/status/${campaign.contentId}`;
  }
}
