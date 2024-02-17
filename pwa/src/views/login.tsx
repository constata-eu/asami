/// <reference types="vite-plugin-svgr/client" />
import { useEffect } from 'react';
import {
  Alert, Avatar, AlertTitle, Badge, Divider, Card, CardActions, CardContent, CardHeader,
  Dialog, DialogActions, DialogContent, DialogTitle,
  IconButton, Box, Button, Container, Paper, styled,
  Toolbar, Typography, Skeleton, useMediaQuery
} from '@mui/material';
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

  /*
    <Button
      onClick={() => loginAs("member")}
      sx={{margin: "auto 0 0 0"}}
      variant="contained"
      size="large"
      fullWidth
      id="button-login-as-member"
    >
      Connect as member
    </Button>
  */

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
                <Button
                  color="inverted"
                  fullWidth
                  id="button-login-as-member"
                >
                  About ASAMI
                </Button>
                <Button
                  startIcon={ <XIcon /> }
                  color="inverted"
                  size="small"
                  fullWidth
                  id="button-login-as-member"
                >
                  asami_club
                </Button>
                <Button
                  startIcon={ <InstagramIcon /> }
                  color="inverted"
                  size="small"
                  fullWidth
                  id="button-login-as-member"
                >
                  asamiclub
                </Button>
              </Box>
            </Box>

            <PublicCampaignList />
          </Box>

          <Divider light sx={{ mt: "5em", mb: "2em" }}/>

          <Button href="https://rootstock.io/" target="_blank" sx={{textDecoration: "none", mb: "2em"}}>
            <Box display="flex" flexDirection="column">
              <Typography fontSize="1em" textTransform="uppercase" fontFamily="LeagueSpartanBold">
                Built with
              </Typography>
              <RootstockLogo/>
            </Box>
          </Button>
        </Box>
      </BareLayout>
    </CoreAdminContext>
  );
};

const YourPostHere = () =>
  <Card sx={{ border: "1px solid", borderColor: yellow, p: "1em", marginBottom: "1em", breakInside: "avoid", flex: "1 1 250px" }} >
    <Head2 >Your post here</Head2>
    <Typography>
      Set up a reward for club members to boost your Instagram or X content.
    </Typography>
    <Box mt="1em" >
      <Button color="secondary" fullWidth size="large" variant="contained" >
        Submit your post
      </Button>
    </Box>
  </Card>

const PublicCampaignList = () => {
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
    { items.map((item) => {
      if(item.yourPostHere) {
        return <YourPostHere/>
      } else if (item.site == "X") {
        return <PublicXCampaign key={item.id} item={item} />;
      } else {
        return <PublicInstagramCampaign key={item.id} item={item} />;
      }
    })}
  </>;
}

const PublicXCampaign = ({item}) => {
  return <Card sx={{ marginBottom: "1em", breakInside: "avoid", flex: "1 1 250px" }} key={item.id} id={`campaign-container-${item.id}`}>
    <CardHeader
      avatar={ <Avatar><XIcon/></Avatar> }
      title="Pays up to $200"
      subheader="0.001 per point"
    />
    <Box px="10px">
      <Button fullWidth size="large" variant="contained" >
        Repost to Earn
      </Button>
    </Box>

    <TwitterTweetEmbed tweetId={item.contentId} options={{ theme: "dark", align: "center", width: "250px", conversation: "none"}} />
  </Card>;
}

const PublicInstagramCampaign = ({item}) => {
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

  return <Card sx={{ marginBottom: "1em", flex: "1 1 400px"}} key={item.id} id={`campaign-container-${item.id}`}>
    <CardHeader
      avatar={ <Avatar><InstagramIcon/></Avatar> }
      title="Pays up to $200"
      subheader="0.001 per point"
    />
    <Box px="10px">
      <Button fullWidth size="large" variant="contained" >
        Post to Earn
      </Button>
    </Box>

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

  return (<Dialog open={open} fullWidth maxWidth="sm">
    <Alert severity="warning" icon={false}>
      <Typography sx={{ color: "inherit" }} fontSize="1.2em" fontFamily="LeagueSpartanBlack" letterSpacing="-0.03em">
        Consent to transfer your data outside the EU
      </Typography>
      Using ASAMI means agreeing to permanent disclosure and storage of your data globally.
      This non-revocable policy is essential for our club's fairness and transparency.
      Before logging in, please assess the potential privacy and reputation risks of disclosing your social media handles and rewards.
      <Button href="https://eur-lex.europa.eu/legal-content/EN/TXT/?uri=CELEX%3A32016R0679" sx={{mt:"1em"}} id="no-login-button" fullWidth color="warning" variant="outlined">
        I do not consent!
      </Button>
    </Alert>
    <DialogContent>
      <Box p="0em 1em 0em 0em" display="flex" gap="1em" flexDirection="column">
        <Box>
          <Typography mb="1em" fontSize="1.2em" fontFamily="LeagueSpartanBlack" letterSpacing="-0.03em">
            Login with your social account and see what asami is about.
          </Typography>
          <Button sx={{mb: "1em"}} id="x-login-button" fullWidth variant="contained" onClick={startXLogin}>Accept and login with X</Button>
          <FacebookLogin
            appId={ Settings.facebook.appId }
            useRedirect
            render={ ({onClick}) => <Button onClick={onClick} id="facebook-login-button" fullWidth variant="contained">Login with Facebook</Button> }
            scope="public_profile"
            dialogParams={{
              redirect_uri: Settings.facebook.redirectUri,
              state: "FacebookLoginState",
              response_type: "code"
            }}
          />
        </Box>
        <Divider light sx={{ m: "1em 0" }}/>
        <Box>
          <Typography fontSize="1.2em" fontFamily="LeagueSpartanBlack" letterSpacing="-0.03em">
            Login as a full member with WEB3.
          </Typography>
          <Typography>
            Get the full experience by using your own wallet whether you're an advertiser or a member of the club,
            you'll get to be part of the governance and earn passive income from your involvement.
          </Typography>
          <Button id="wallet-login-button" fullWidth variant="contained" onClick={startEip712Login}>Accept and login with WEB3</Button>
        </Box>
      </Box>
    </DialogContent>
  </Dialog>);
}

export default Login;

