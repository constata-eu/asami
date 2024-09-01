/// <reference types="vite-plugin-svgr/client" />
import { useEffect, useContext } from 'react';
import { Alert, Divider, Avatar, CardContent, CardHeader, Dialog, DialogContent, Box, Button, Typography } from '@mui/material';
import { makeXUrl } from '../lib/auth_provider';
import { etherToHex  } from '../lib/formatters';
import { formatEther } from "ethers";
import { publicDataProvider } from "../lib/data_provider";
import {
    useSafeSetState,
    useStore,
    useListController,
    CoreAdminContext,
    useGetList,
    useTranslate,
    I18nContext,
    useDataProvider,
    Form,
    TextInput,
    SaveButton,
    useNotify,
    email,
    required
} from 'react-admin';

import { TwitterTweetEmbed } from 'react-twitter-embed';
import { useNavigate } from 'react-router-dom';
import { BareLayout, DeckCard } from './layout';
import { Head2, Head3, light, green } from '../components/theme';
import AsamiLogo from '../assets/logo.svg?react';
import { useContracts } from "../components/contracts_context";
import { Settings } from '../settings';
import InstagramIcon from '@mui/icons-material/Instagram';
import XIcon from '@mui/icons-material/X';
import WalletIcon from '@mui/icons-material/Wallet';
import CampaignIcon from '@mui/icons-material/Campaign';
import truncate from 'lodash/truncate';
import chunk from 'lodash/chunk';
import flatten from 'lodash/flatten';
import CampaignListEmpty from './campaign_list_empty';
import {isMobile} from 'react-device-detect';
import AlternateEmailIcon from '@mui/icons-material/AlternateEmail';
import SendIcon from '@mui/icons-material/Send';

const Login = () => {
  const translate = useTranslate();
  const [, setRole] = useStore('user.role', 'advertiser');
  const [open, setOpen] = useSafeSetState(false);
  const [pubDataProvider, setPubDataProvider] = useSafeSetState();
  const i18nProvider = useContext(I18nContext);

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

  const loginAs = async (newRole) => {
    setRole(newRole);
    setOpen(true);
  }

  return (
    <CoreAdminContext i18nProvider={i18nProvider} dataProvider={pubDataProvider} authProvider={null}>
      <BareLayout>
        <Box p="1em" id="login-form-and-landing">
          <LoginSelector open={open} setOpen={setOpen} />

          <Box sx={{columnCount: { xs: 1, sm: 2, md: 3, lg: 4, xl: 5}, columnGap:"1em"}}>
            <Box sx={{ breakInside: "avoid", p:"1em", mb:"1em", display:"flex", flexDirection:"column", alignItems:"center" }}>
              <AsamiLogo margin="auto" width="250px" height="100%"/>
              <Box my="1em">
                <Button
                  color="primary"
                  onClick={() => loginAs("member")}
                  fullWidth
                  id="button-login-as-member"
                >
                  { translate("login.login_button") }
                </Button>
                <Button href="/#/about" color="inverted" fullWidth id="button-about-us" >
                  { translate("login.about_asami_button") }
                </Button>
                <Button
                  href={ `https://x.com/${translate("login.x_handle")}` }
                  target="_blank"
                  startIcon={ <XIcon /> }
                  color="inverted"
                  size="small"
                  fullWidth
                  id="button-visit-x"
                >
                  { translate("login.x_handle") }
                </Button>
                <Button
                  href={ `https://instagram.com/${translate("login.ig_handle")}` }
                  target="_blank"
                  startIcon={ <InstagramIcon /> }
                  color="inverted"
                  size="small"
                  fullWidth
                  id="button-visit-instagram"
                >
                  { translate("login.ig_handle") }
                </Button>
              </Box>
            </Box>

            <JoinNow key="join-now" loginAs={loginAs}/>
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
    filter: { budgetGt: etherToHex("1") },
    sort: { field: 'createdAt', order: 'DESC'},
    perPage: 20,
    queryOptions: {
      refetchInterval: 100000,
    },
    resource: "Campaign",
  });

  if (listContext.isLoading){
    return <></>;
  }

  if (listContext.total == 0 ) {
    return <CampaignListEmpty />;
  }

  const items = flatten(chunk(listContext.data, 4).map((i) => [...i, {yourPostHere: true}]));

  return <>
    { items.map((item, index) => {
      if(item.yourPostHere) {
        return <YourPostHere key={`your-post-here-${index}`} loginAs={loginAs}/>
      } else if (item.campaignKind == "XREPOST") {
        return <PublicXCampaign key={item.id} item={item} loginAs={loginAs}/>;
      } else {
        return <PublicInstagramCampaign key={item.id} item={item} loginAs={loginAs}/>;
      }
    })}
  </>;
}

const YourPostHere = ({loginAs}) => {
  const translate = useTranslate();
  
  return (<DeckCard elevation={10} >
    <CardContent>
      <Head2>{ translate("your_post_here.title") }</Head2>
      <Typography>{ translate("your_post_here.message") }</Typography>
      <Box mt="1em" >
        <Button onClick={() => loginAs("advertiser")} className="submit-your-post" color="inverted" fullWidth size="large" variant="outlined" >
          <CampaignIcon sx={{mr:"5px"}}/>
          { translate("your_post_here.button") }
        </Button>
      </Box>
    </CardContent>
  </DeckCard>);
}

const JoinNow = ({loginAs}) => {
  const translate = useTranslate();
  
  return (<DeckCard borderColor={green} elevation={10}>
    <CardContent>
      <Head2 sx={{ mb: "0.5em" }}>{ translate("join_now.title") }</Head2>
      <Typography>{ translate("join_now.message") }</Typography>
      <Head3 sx={{ mt: "1em", mb: "0.5em" }}>{ translate("join_now.no_crypto_no_problem") }</Head3>
      <Typography>{ translate("join_now.learn_later") }</Typography>
      <Box mt="1em" >
        <Button onClick={() => loginAs("member")} className="get-your-sparkles" fullWidth size="large" variant="contained" >
          { translate("join_now.button") }
        </Button>
      </Box>
    </CardContent>
  </DeckCard>);
}

const PublicCardHeader = ({loginAs, item, buttonLabel, icon}) => {
  const translate = useTranslate();

  return (
    <CardHeader
      sx={{ mb: "0", pb: "0.5em" }}
      avatar={ <Avatar sx={{ bgcolor: light }} >{icon}</Avatar> }
      title={ translate("public_card_header.title", {amount: formatEther(item.budget)}) }
      subheader={
        <Button sx={{ mt:"0.2em" }} onClick={() => loginAs("member") } fullWidth color="inverted" size="small" variant="outlined" >
          { buttonLabel }
        </Button>
      }
    />
  );
};


const PublicXCampaign = ({loginAs, item}) => {
  const translate = useTranslate();
  return <DeckCard id={`campaign-container-${item.id}`}>
    <PublicCardHeader
      icon={<XIcon/>}
      item={item}
      loginAs={loginAs}
      buttonLabel={ translate("public_x_campaign.main_button")}
    />
    <CardContent>
      <TwitterTweetEmbed tweetId={JSON.parse(item.briefingJson)} options={{ theme: "dark", align: "center", width: "250px", conversation: "none"}} />
    </CardContent>
  </DeckCard>;
}

const PublicInstagramCampaign = ({loginAs, item}) => {
  const translate = useTranslate();
  const {data, isLoading} = useGetList(
    "IgCampaignRule",
    { filter: {campaignIdEq: item.id}, perPage: 1,}
  );

  if (isLoading || !data[0]) {
    return null;
  }

  const dataUri = "data:image/jpeg;base64,"+data[0].image;
  const filename = `campaign_${data[0].campaignId}.jpg`;

  return <DeckCard id={`campaign-container-${item.id}`}>
    <PublicCardHeader
      icon={<InstagramIcon/>}
      item={item}
      loginAs={loginAs}
      buttonLabel={ translate("public_ig_campaign.main_button") }
    />

    <CardContent>
      <Box display="flex" flexDirection="column" alignItems="center">
        <a href={ dataUri } target="_blank" download={filename} rel="noreferrer">
          <img style={{maxWidth: "100%", maxHeight: "400px"}} src={dataUri} />
        </a>
        { !!data[0].caption && <Typography>{ truncate(data[0].caption, {length: 120}) }</Typography> }
      </Box>
    </CardContent>
  </DeckCard>;
}

const LoginSelector = ({open, setOpen}) => {
  const translate = useTranslate();
  const { signLoginMessage } = useContracts();
  const navigate = useNavigate();

  const startXLogin = async () => {
    setOpen(false);
    const { url, verifier } = await makeXUrl();
    localStorage.setItem("oauthVerifier", verifier);
    document.location.href = url;
  };

  const startEip712Login = async () => {
    setOpen(false);
    const code = await signLoginMessage();
    navigate(`/eip712_login?code=${code}`);
  };

  return (<Dialog open={open} onClose={() => setOpen(false)} fullWidth maxWidth="sm" slotProps={{ backdrop: {sx: {background: "rgba(0,0,0,0.9)"}}}} PaperProps={ {"variant": "outlined", sx:{ borderWidth: "10px" }} }>
    <Alert  severity="warning" icon={false}>
      <Typography sx={{ color: "inherit" }} fontSize="1.2em" fontFamily="LeagueSpartanBlack" letterSpacing="-0.03em">
        { translate("login_form.disclaimer_title") }
      </Typography>
        { translate("login_form.disclaimer_body") }
      <Box display="flex" gap="1em" flexWrap="wrap" mt="1em">
        <Button sx={{ flex: "1 1 300px"}} href="https://eur-lex.europa.eu/legal-content/EN/TXT/?uri=CELEX%3A32016R0679" id="no-login-button" color="warning" variant="outlined">
          { translate("login_form.reject_button") }
        </Button>
        <Button sx={{ flex: "1 1 100px" }} href="/#/about" id="more-on-privacy" color="warning">
          { translate("login_form.read_more") }
        </Button>
      </Box>
    </Alert>
    <Divider/>
    <DialogContent>
      <Box display="flex" gap="1em" flexDirection="column">
        <Box>
          <Typography mb="1em" fontSize="1.2em" fontFamily="LeagueSpartanBlack" letterSpacing="-0.03em">
            { translate("login_form.web2_consent_title") }
          </Typography>
          <Box display="flex" gap="1em" alignItems="center" mb="1em">
            <XIcon/>

            { isMobile ?
                <LoginWithXMobileWarning onClick={startXLogin} />
                :
                <Button
                  id="x-login-button"
                  fullWidth
                  variant="contained"
                  color="inverted"
                  onClick={startXLogin}
                >
                  { translate("login_form.consent_with_x") }
                </Button>
            }
          </Box>
          <Box display="flex" gap="1em" alignItems="center" mb="1em">
            <AlternateEmailIcon/>
            <LoginWithEmail/>
          </Box>
        </Box>
        <Box>
          <Typography fontSize="1.2em" fontFamily="LeagueSpartanBlack" letterSpacing="-0.03em">
            { translate("login_form.web3_consent_title") }
          </Typography>
          <Typography mb="1em">
            { translate("login_form.web3_consent_body") }
          </Typography>
          <Box display="flex" gap="1em" alignItems="center" mb="1em">
            <WalletIcon/>
            <Button id="wallet-login-button" fullWidth color="inverted" variant="contained" onClick={startEip712Login}>
              { translate("login_form.consent_and_connect") }
            </Button>
          </Box>
        </Box>
      </Box>
    </DialogContent>
  </Dialog>);
}

const LoginWithXMobileWarning = ({onClick}) => {
  const translate = useTranslate();
  const [open, setOpen] = useSafeSetState(false);
  return (
    <Box width="100%">
        <Button
          id="x-login-button"
          fullWidth
          variant="contained"
          color="inverted"
          onClick={ () => setOpen(true) }
        >
          { translate("login_form.consent_with_x") }
        </Button>
        <Dialog open={open} onClose={() => setOpen(false)} fullWidth maxWidth="sm" slotProps={{ backdrop: {sx: {background: "rgba(0,0,0,0.9)"}}}} PaperProps={ {"variant": "outlined", sx:{ borderWidth: "10px" }} }>
            <DialogContent>
                <Typography mb="1em" fontSize="1.2em" fontFamily="LeagueSpartanBlack" letterSpacing="-0.03em">
                    { translate("login_form.x_mobile_warning.title") }
                </Typography>
                <Typography mb="1em">{ translate("login_form.x_mobile_warning.text") }</Typography>
                <Button
                    id="x-login-button"
                    fullWidth
                    variant="contained"
                    color="inverted"
                    onClick={onClick}
                >
                    { translate("login_form.consent_with_x") }
                </Button>
            </DialogContent>
        </Dialog>
    </Box>);
}

const LoginWithEmail = ({onClick}) => {
  const translate = useTranslate();
  const [open, setOpen] = useSafeSetState(false);
  const [sent, setSent] = useSafeSetState(false);
  const dataProvider = useDataProvider();

  const onSubmit = async (values) => {
    try {
        await dataProvider.create('EmailLoginLink', { data: { email: values.email }});
        setSent(true);
    } catch {
        notify("login_form.send_email_token.error", {type: "error"});
    }
  }

  const handleClose = () => {
      setSent(false)
      setOpen(false)
  }

  return (
    <Box width="100%">
        <Button
          id="email-login-button"
          fullWidth
          variant="contained"
          color="inverted"
          onClick={ () => setOpen(true) }
        >
          { translate("login_form.consent_with_email") }
        </Button>
        <Dialog open={open} onClose={handleClose} fullWidth maxWidth="sm" slotProps={{ backdrop: {sx: {background: "rgba(0,0,0,0.9)"}}}} PaperProps={ {"variant": "outlined", sx:{ borderWidth: "10px" }} }>
            { !sent ? 
                <DialogContent>
                    <Typography mb="1em" fontSize="1.2em" fontFamily="LeagueSpartanBlack" letterSpacing="-0.03em">
                        { translate("login_form.send_email_token.title") }
                    </Typography>
                    <Form sanitizeEmptyValues onSubmit={onSubmit}>
                      <TextInput fullWidth validate={[required(), email()]} size="large" variant="filled" source="email" label="Email"/>
                      <SaveButton fullWidth id="submit-email-for-login-link" size="large" label={ translate("login_form.send_email_token.submit") } icon={<SendIcon/>} />
                    </Form>
                </DialogContent>
                :
                <Alert  severity="success" icon={false}>
                  <Typography sx={{ color: "inherit" }} fontSize="1.5em" fontFamily="LeagueSpartanBlack" letterSpacing="-0.03em">
                    { translate("login_form.send_email_token.done") }
                  </Typography>
                  <Typography sx={{ color: "inherit" }}>
                    { translate("login_form.send_email_token.done_contact_us") }
                  </Typography>
                </Alert>
            }
        </Dialog>
    </Box>);
}

export default Login;
