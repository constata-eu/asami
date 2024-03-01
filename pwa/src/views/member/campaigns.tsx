/// <reference types="vite-plugin-svgr/client" />
import { useEffect, useContext } from 'react';
import {
  Alert, Avatar, AlertTitle, Badge, Divider, Card, CardActions, CardContent, CardHeader,
  Dialog, DialogActions, DialogContent, DialogTitle,
  IconButton, Box, Button, Container, Paper, styled,
  Toolbar, Typography, Skeleton, useMediaQuery
} from '@mui/material';
import { makeXUrl,  } from '../lib/auth_provider';
import { ethers, parseUnits, formatEther, toBeHex, zeroPadValue, parseEther } from "ethers";
import { publicDataProvider } from "../lib/data_provider";
import {  
  useCheckAuth,
  useSafeSetState,
  useStore,
  useListController,
  defaultExporter,
  ListContextProvider,
  CoreAdminContext,
  useNotify,
  useGetList,
  useTranslate,
  I18nContext,
  Confirm
} from 'react-admin';

import { TwitterTweetEmbed } from 'react-twitter-embed';
import { useNavigate } from 'react-router-dom';
import { BareLayout, DeckCard } from '../layout';
import { Head1, Head2, CardTitle, BulletPoint, yellow, dark, red, light, green } from '../../components/theme';
import logo from '../assets/asami.png';
import RootstockLogo from '../assets/rootstock.svg?react';
import AsamiLogo from '../assets/logo.svg?react';
import { useContracts } from "../components/contracts_context";
import FacebookLogin from '@greatsumini/react-facebook-login';
import { Settings } from '../settings';
import InstagramIcon from '@mui/icons-material/Instagram';
import XIcon from '@mui/icons-material/X';
import FacebookIcon from '@mui/icons-material/Facebook';
import WalletIcon from '@mui/icons-material/Wallet';
import truncate from 'lodash/truncate';
import chunk from 'lodash/chunk';
import flatten from 'lodash/flatten';

export const CampaignHeader = ({campaign, icon, children}) => {
  const translate = useTranslate();

  return (<>
    <CardHeader
      avatar={ <Avatar sx={{ bgcolor: light }} >{icon}</Avatar> }
      title={ translate("member_campaigns.header.title", {amount: formatEther(campaign.remaining)}) }
      subheader={ translate("member_campaigns.header.subheader", {amount: formatEther(campaign.priceScoreRatio)}) }
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

export const XCampaign = ({campaign, prefsContext, setPreference}) => {
  const translate = useTranslate();
  const repostUrl = `https://twitter.com/intent/retweet?tweet_id=${campaign.contentId}&related=asami_club`;
  const attemptedOn = prefsContext.data.find((x) => x.campaignId == campaign.id)?.attemptedOn;

  return <DeckCard id={`campaign-container-${campaign.id}`} elevation={attemptedOn ? 1 : 10}>
    <CampaignHeader
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

export const IgCampaign = ({campaign, prefsContext, setPreference}) => {
  const translate = useTranslate();
  const notify = useNotify();
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
      <Box my="1em" display="flex" flexDirection="column" alignItems="center">
        <img style={{maxWidth: "100%", maxHeight: "400px"}} src={dataUri} />
        { !!caption && <Typography sx={{lineBreak:"anywhere" }}>{ truncate(caption, {length: 120}) }
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
        { !!caption ?
            translate("member_campaigns.ig.instruction_with_caption")
          :
            translate("member_campaigns.ig.instruction_without_caption")
        }
        </Typography>
        <ul>
          <li>It should be a post, not a story</li>
          <li>Do not use filters</li>
          <li>The post must be public.</li>
          <li>Your account must not be private.</li>
        </ul>
        <Box display="flex" gap="1em">
          <Button fullWidth
            variant="contained"
            target="_blank" href={ dataUri }
            rel="noopener noreferrer"
            download={filename}
          >
            Download image
          </Button> 
          { !!caption &&
            <Button
              fullWidth
              onClick={() => copyText() }
              variant="contained"
            >
              Copy text
            </Button> 
          }
        </Box>
      </DialogContent>
    </Dialog>
  </>;
}
