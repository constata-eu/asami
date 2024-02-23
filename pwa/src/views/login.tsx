/// <reference types="vite-plugin-svgr/client" />
import { useEffect } from 'react';
import {
  Alert, Avatar, AlertTitle, Badge, Divider, Card, CardActions, CardContent, CardHeader,
  Dialog, DialogActions, DialogContent, DialogTitle,
  IconButton, Box, Button, Container, Paper, styled,
  Toolbar, Typography, Skeleton, useMediaQuery
} from '@mui/material';
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
  useGetList
} from 'react-admin';
import { TwitterTweetEmbed } from 'react-twitter-embed';
import { useNavigate } from 'react-router-dom';
import { BareLayout } from './layout';
import { Head1, Head2, CardTitle, BulletPoint, yellow, dark, red } from '../components/theme';
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

const Login = () => {
  const [role, setRole] = useStore('user.role', 'advertiser');
  const [open, setOpen] = useSafeSetState(false);
  const [error, setError] = useSafeSetState();
  const [pubDataProvider, setPubDataProvider] = useSafeSetState();

  useEffect(() => {
    async function initApp() {
      const prov = await publicDataProvider();
      setPubDataProvider(prov);
    }
    initApp();
  }, []);

  if (!pubDataProvider) {
    return <></>;
  }


  const loginAs = async (role) => {
    try {
      setRole(role);
      setOpen(true);
    } catch (e) {
      setError(e.message)
    }
  }

  return (
    <CoreAdminContext dataProvider={pubDataProvider} authProvider={null}>
      <BareLayout>
        <Box p="1em">
          <LoginSelector open={open} setOpen={setOpen} />

          <Box sx={{columnCount: { xs: 1, sm: 2, md: 3, lg: 4, xl: 5}, columnGap:"1em"}}>
            <Box p="1em" mb="1em">
              <AsamiLogo width="100%" height="100%"/>
              <Box my="1em">
                <Button
                  color="primary"
                  onClick={() => loginAs("member")}
                  fullWidth
                  id="button-login-as-member"
                >
                  Login &ndash; Signup
                </Button>
                <Button href="/#/about" color="inverted" fullWidth id="button-about-us" > About ASAMI</Button>
                <Button
                  href="https://x.com/asami_club"
                  target="_blank"
                  startIcon={ <XIcon /> }
                  color="inverted"
                  size="small"
                  fullWidth
                  id="button-visit-x"
                >
                  asami_club
                </Button>
                <Button
                  href="https://instagram.com/asamiclub"
                  target="_blank"
                  startIcon={ <InstagramIcon /> }
                  color="inverted"
                  size="small"
                  fullWidth
                  id="button-visit-instagram"
                >
                  asamiclub
                </Button>
              </Box>
            </Box>

            <PublicCampaignList loginAs={loginAs} />
          </Box>

        </Box>
      </BareLayout>
    </CoreAdminContext>
  );
};

const PublicCampaignList = ({loginAs}) => {
  const listContext = useListController({
    debounce: 500,
    disableSyncWithLocation: true,
    filter: { finishedEq: false },
    perPage: 40,
    queryOptions: {
      refetchInterval: 100000,
    },
    resource: "Campaign",
  });

  if (listContext.isLoading){
    return <></>;
  }

  if (listContext.total == 0 ) {
    return <>
      <Card id="campaign-list-empty" sx={{my:"3em"}}>
        <CardContent>
          <Head2>We're all out of campaigns.</Head2>
          <Typography>Check back soon though!</Typography>
        </CardContent>
      </Card>
    </>;
  }

  const items = flatten(chunk(listContext.data, 5).map((i) => [{yourPostHere: true}, ...i]));

  return <>
    { items.map((item, index) => {
      if(item.yourPostHere) {
        return <YourPostHere key={`your-post-here-${index}`} loginAs={loginAs}/>
      } else if (item.site == "X") {
        return <PublicXCampaign key={item.id} item={item} loginAs={loginAs}/>;
      } else {
        return <PublicInstagramCampaign key={item.id} item={item} loginAs={loginAs}/>;
      }
    })}
  </>;
}

const YourPostHere = ({loginAs}) =>
  <Card sx={{ border: "1px solid", borderColor: yellow, p: "1em", marginBottom: "1em", breakInside: "avoid", flex: "1 1 250px" }} >
    <Head2 >Your post here</Head2>
    <Typography>
      Set up a reward for club members to boost your Instagram or X content.
    </Typography>
    <Box mt="1em" >
      <Button onClick={() => loginAs("advertiser")} className="submit-your-post" color="secondary" fullWidth size="large" variant="contained" >
        Submit your post
      </Button>
    </Box>
  </Card>

const PublicCardHeader = ({loginAs, item, buttonLabel, icon}) => 
  <>
    <CardHeader
      avatar={ <Avatar>{icon}</Avatar> }
      title={ `Pays up to $${formatEther(item.remaining)}` }
      subheader={ `$${formatEther(item.priceScoreRatio)} per ✨` }
    />
    <Box px="10px">
      <Button onClick={() => loginAs("member") } fullWidth size="large" variant="contained" >
        { buttonLabel }
      </Button>
    </Box>
  </>

const PublicXCampaign = ({loginAs, item}) => {
  return <Card sx={{ marginBottom: "1em", breakInside: "avoid", flex: "1 1 250px" }} key={item.id} id={`campaign-container-${item.id}`}>
    <PublicCardHeader icon={<XIcon/>} item={item} loginAs={loginAs} buttonLabel="Repost to earn" />
    <TwitterTweetEmbed tweetId={item.contentId} options={{ theme: "dark", align: "center", width: "250px", conversation: "none"}} />
  </Card>;
}

const PublicInstagramCampaign = ({loginAs, item}) => {
  const notify = useNotify();
  const {data, isLoading} = useGetList(
    "IgCampaignRule",
    { filter: {campaignIdEq: item.id}, perPage: 1,}
  );

  if (isLoading || !data[0]) {
    return null;
  }

  const dataUri = "data:image/jpeg;base64,"+data[0].image;
  const filename = `campaign_${data[0].campaignId}.jpg`;

  return <Card sx={{ marginBottom: "1em", flex: "1 1 250px"}} key={item.id} id={`campaign-container-${item.id}`}>
    <PublicCardHeader icon={<InstagramIcon/>} item={item} loginAs={loginAs} buttonLabel="Repost to earn" />

    <CardContent>
      <Box display="flex" flexDirection="column" alignItems="center">
        <a href={ dataUri } target="_blank" download={filename}>
          <img style={{maxWidth: "100%", maxHeight: "400px"}} src={dataUri} />
        </a>
        { !!data[0].caption && <Typography>{ truncate(data[0].caption, {length: 120}) }</Typography> }
      </Box>
    </CardContent>
  </Card>;
}

const LoginSelector = ({open, setOpen}) => {
  const { signLoginMessage } = useContracts();
  const navigate = useNavigate();
  const [oauthVerifier, setOauthVerifier] = useStore('user.oauthChallenge');

  const startXLogin = async (method) => {
    setOpen(false);
    const { url, verifier } = await makeXUrl();
    localStorage.setItem("oauthVerifier", verifier);
    document.location.href = url;
  };

  const startEip712Login = async (method) => {
    setOpen(false);
    const code = await signLoginMessage();
    navigate(`/eip712_login?code=${code}`);
  };

  return (<Dialog open={open} onClose={() => setOpen(false)} fullWidth maxWidth="sm">
    <Alert severity="error" icon={false}>
      <Typography sx={{ color: "inherit" }} fontSize="1.2em" fontFamily="LeagueSpartanBlack" letterSpacing="-0.03em">
        Consent to transfer your data outside the EU
      </Typography>
      Using ASAMI means agreeing to permanent disclosure and storage of your data globally.
      This non-revocable policy is essential for our club's fairness and transparency.
      Before logging in, please assess the potential privacy and reputation risks of disclosing your social media handles and rewards.
      <Button href="https://eur-lex.europa.eu/legal-content/EN/TXT/?uri=CELEX%3A32016R0679" sx={{mt:"1em"}} id="no-login-button" fullWidth color="error" variant="outlined">
        I do not consent!
      </Button>
    </Alert>
    <DialogContent>
      <Box display="flex" gap="1em" flexDirection="column">
        <Box>
          <Typography mb="1em" fontSize="1.2em" fontFamily="LeagueSpartanBlack" letterSpacing="-0.03em">
            Consent and login with your social account.
          </Typography>
          <Box display="flex" gap="1em" alignItems="center" mb="1em">
            <XIcon/>
            <Button
              id="x-login-button"
              fullWidth
              variant="contained"
              onClick={startXLogin}
            >
              Consent with X
            </Button>
          </Box>
          <Box display="flex" gap="1em" alignItems="center" mb="1em">
            <FacebookIcon/>
            <FacebookLogin
              appId={ Settings.facebook.appId }
              useRedirect
              render={ ({onClick}) =>
                <Button
                  onClick={onClick}
                  id="facebook-login-button"
                  fullWidth
                  variant="contained"
                >
                  Consent with Facebook
                </Button> 
              }
              scope="public_profile"
              dialogParams={{
                redirect_uri: Settings.facebook.redirectUri,
                state: "FacebookLoginState",
                response_type: "code"
              }}
            />
          </Box>
        </Box>
        <Box>
          <Typography fontSize="1.2em" fontFamily="LeagueSpartanBlack" letterSpacing="-0.03em">
            Consent and login with a WEB3 wallet
          </Typography>
          <Typography mb="1em">
            To spend your rewards, get ASAMI tokens, and vote.
          </Typography>
          <Box display="flex" gap="1em" alignItems="center" mb="1em">
            <WalletIcon/>
            <Button id="wallet-login-button" fullWidth variant="contained" onClick={startEip712Login}>
              Consent and connect
            </Button>
          </Box>
        </Box>
      </Box>
    </DialogContent>
  </Dialog>);
}

export default Login;

