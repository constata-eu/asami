/// <reference types="vite-plugin-svgr/client" />
import { Alert, Avatar, CardContent, CardHeader, Dialog, DialogContent, Box, Button, Typography } from '@mui/material';
import { formatEther } from "ethers";
import { useSafeSetState, useNotify, useGetList, useTranslate, Confirm } from 'react-admin';
import { TwitterTweetEmbed } from 'react-twitter-embed';
import { DeckCard } from '../layout';
import { light } from '../../components/theme';
import InstagramIcon from '@mui/icons-material/Instagram';
import XIcon from '@mui/icons-material/X';
import truncate from 'lodash/truncate';

export const CampaignHeader = ({handle, campaign, icon, children}) => {
  const translate = useTranslate();

  return (<>
    <CardHeader
      avatar={ <Avatar sx={{ bgcolor: light }} >{icon}</Avatar> }
      title={ <Typography variant="h6">{translate("member_campaigns.header.title", {amount: formatEther(handle.price)}) }</Typography>}
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
  const repostUrl = `https://twitter.com/intent/retweet?tweet_id=${campaign.contentId}&related=asami_club`;
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
          id={`button-repost-${campaign.Id}`}
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
      <TwitterTweetEmbed tweetId={campaign.contentId} options={{ theme: "dark", align: "center", width: "250px", conversation: "none"}} />
    </Box>
    { !attemptedOn &&
      <CardContent sx={{pt: 0, pb: "1em !important" }}>
        <ConfirmHideCampaign campaignId={campaign.id} setPreference={setPreference} />
      </CardContent>
    }
  </DeckCard>;
}

export const IgCampaign = ({handle, campaign, prefsContext, setPreference}) => {
  const translate = useTranslate();
  const {data, isLoading} = useGetList(
    "IgCampaignRule",
    { filter: {campaignIdEq: campaign.id}, perPage: 1,}
  );
  const attemptedOn = prefsContext.data.find((x) => x.campaignId == campaign.id)?.attemptedOn;

  if (isLoading || !data[0]) {
    return null;
  }

  const dataUri = "data:image/jpeg;base64,"+data[0].image;
  const caption = data[0].caption;

  return <DeckCard id={`campaign-container-${campaign.id}`} elevation={attemptedOn ? 1 : 10}>
    <CampaignHeader
      icon={<InstagramIcon/>}
      campaign={campaign}
      handle={handle}
    >
      { attemptedOn &&
        <Alert id={`alert-repost-attempted-${campaign.id}`}
          variant="outlined" color="warning" severity="info"
          sx={{border:"none", p: 0}}
        >
          { translate("member_campaigns.ig.attempted") }
        </Alert>
      }
      <IgCampaignInstructions
        campaignId={campaign.id}
        setPreference={setPreference}
        dataUri={dataUri}
        caption={caption}
        attemptedOn={attemptedOn}
      />
    </CampaignHeader>

    <CardContent sx={{ pt: "0px" }}>
      <Box mt="1em" display="flex" flexDirection="column" alignItems="center">
        <img style={{maxWidth: "100%", maxHeight: "400px"}} src={dataUri} />
        { !!caption && <Typography sx={{lineBreak:"anywhere", mt: "1em" }}>{ truncate(caption, {length: 120}) }
        </Typography> }
      </Box>
    </CardContent>
    { !attemptedOn &&
      <CardContent sx={{pt: 0, pb: "1em !important" }}>
        <ConfirmHideCampaign campaignId={campaign.id} setPreference={setPreference} />
      </CardContent>
    }
  </DeckCard>;
}

const IgCampaignInstructions = ({campaignId, setPreference, dataUri, caption, attemptedOn}) => {
  const notify = useNotify();
  const translate = useTranslate();
  const [open, setOpen] = useSafeSetState(false);

  let [size, variant, buttonLabel, color] = attemptedOn ?
    ["small", "text", "member_campaigns.ig.see_instructions_again", "secondary"] :
    ["large","contained", "member_campaigns.ig.main_button", "primary"];

  const filename = `campaign_${campaignId}.jpg`;
  const copyText = async () => {
    notify("member_campaigns.ig.caption_copied", { anchorOrigin: { vertical: 'top', horizontal: 'center' }});
    await navigator.clipboard.writeText(caption);
  }

  const onClose = () => {
    setOpen(false);
    setPreference(campaignId, false, true);
  }

  return <>
    <Button
      id={`button-repost-${campaignId}`}
      sx={{mb: "0.5em" }}
      onClick={() => setOpen(true) }
      fullWidth
      color={color}
      size={size}
      variant={variant}
    >
      { translate(buttonLabel) }
    </Button>

    <Dialog open={open} onClose={onClose} fullWidth maxWidth="sm">
      <DialogContent>
        <Typography>
        { caption ?
            translate("member_campaigns.ig.instruction_with_caption")
          :
            translate("member_campaigns.ig.instruction_without_caption")
        }
        </Typography>
        <ul>
          <li>{ translate("member_campaigns.ig.post_not_story") }</li>
          <li>{ translate("member_campaigns.ig.no_filters") }</li>
          <li>{ translate("member_campaigns.ig.public_post") }</li>
          <li>{ translate("member_campaigns.ig.no_private_account") }</li>
          <li>{ translate("member_campaigns.ig.follow_rules") }</li>
        </ul>
        <Box display="flex" gap="1em">
          <Button fullWidth
            variant="contained"
            target="_blank" href={ dataUri }
            rel="noopener noreferrer"
            download={filename}
          >
            { translate("member_campaigns.ig.download_image") }
          </Button> 
          { !!caption &&
            <Button
              fullWidth
              onClick={() => copyText() }
              variant="contained"
            >
              { translate("member_campaigns.ig.copy_text") }
            </Button> 
          }
        </Box>
      </DialogContent>
    </Dialog>
  </>;
}
