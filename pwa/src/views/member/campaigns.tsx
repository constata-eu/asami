/// <reference types="vite-plugin-svgr/client" />
import { Alert, Avatar, CardContent, CardHeader, Dialog, DialogContent, Box, Button, Typography } from '@mui/material';
import { formatEther } from "ethers";
import { useSafeSetState, useNotify, useGetList, useTranslate, Confirm } from 'react-admin';
import { TwitterTweetEmbed } from 'react-twitter-embed';
import { DeckCard } from '../layout';
import { light } from '../../components/theme';
import XIcon from '@mui/icons-material/X';
import truncate from 'lodash/truncate';
import { contentId } from '../../lib/campaign';

export const CampaignHeader = ({campaign, icon, children}) => {
  const translate = useTranslate();

  return (<>
    <CardHeader
      avatar={ <Avatar sx={{ bgcolor: light }} >{icon}</Avatar> }
      title={ <Typography variant="h6">{translate("member_campaigns.header.title", {amount: formatEther(campaign.youWouldReceive || 0)}) }</Typography>}
      subheader={ translate("member_campaigns.header.subheader") }
    />
    <Box px="10px">
      { children }
    </Box>
  </>);
};

const ConfirmHideCampaign = ({campaignId, setPreference }) => {
  const translate = useTranslate();
  const [open, setOpen] = useSafeSetState(false);
  const [hide, setHide] = useSafeSetState(false);
  
  const handleConfirm = () => {
    setPreference(campaignId, true, false)
    setOpen(false);
    setHide(true);
  }

  if (hide) { return null; }

  return <>
    <Button fullWidth id={`open-hide-campaign-${campaignId}`}
      size="small" color="inverted" onClick={() => setOpen(true) }
    >
      { translate("member_campaigns.hide_campaign.button") }
    </Button>
    <Confirm
      isOpen={open}
      title={ translate("member_campaigns.hide_campaign.title") }
      content={ translate("member_campaigns.hide_campaign.content") }
      onConfirm={handleConfirm}
      onClose={() => setOpen(false)}
    />
  </>;
}

export const XCampaign = ({handle, campaign, prefsContext, setPreference}) => {
  const translate = useTranslate();
  const repostUrl = `https://twitter.com/intent/retweet?tweet_id=${contentId(campaign)}&related=asami_club`;
	const attemptedOn = prefsContext.data.find((x) => x.campaignId == campaign.id)?.attemptedOn;

  return <DeckCard id={`campaign-container-${campaign.id}`} elevation={attemptedOn ? 1 : 10}>
    <CampaignHeader
      handle={handle}
      icon={<XIcon/>}
      campaign={campaign}
    >
      { attemptedOn ?
        <Alert id={`alert-repost-attempted-${campaign.id}`}
          variant="outlined" color="warning" severity="info"
          sx={{mb: "1em", border:"none", p: 0}}
        >
          { translate("member_campaigns.x.attempted") }
        </Alert>
        :
        <Button
          id={`button-repost-${campaign.id}`}
          sx={{mb: "0.5em" }}
          onClick={() => setPreference(campaign.id, false, true)}
          fullWidth
          size="large"
          variant="contained"
          href={repostUrl}
          target="_blank"
        >
          { translate("member_campaigns.x.main_button") }
        </Button>
      }
    </CampaignHeader>
    <Box minHeight="250px" mb="1em">
      <TwitterTweetEmbed tweetId={contentId(campaign)} options={{ theme: "dark", align: "center", width: "250px", conversation: "none"}} />
    </Box>
    { !attemptedOn &&
      <CardContent sx={{pt: 0, pb: "1em !important" }}>
        <ConfirmHideCampaign campaignId={campaign.id} setPreference={setPreference} />
      </CardContent>
    }
  </DeckCard>;
}
