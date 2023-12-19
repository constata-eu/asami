import {useEffect } from "react";
import {
  Admin,
  Resource,
  ListGuesser,
  EditGuesser,
  ShowGuesser,
  CustomRoutes,
  useSafeSetState,
  useStore,
  Authenticated
} from "react-admin";
import { ContractsProvider } from './components/contracts_context';
import { Settings } from './settings';
import {
  GoogleReCaptchaProvider,
} from 'react-google-recaptcha-v3';

import { Route, useSearchParams } from "react-router-dom";
import { authProvider } from "./lib/auth_provider";
import { defaultDataProvider } from "./lib/data_provider";
import Login from "./views/login";
import asamiTheme from './components/theme';
import { AsamiLayout } from './views/layout';

import AdvertiserDashboard from "./views/advertiser/dashboard";
import MemberDashboard from "./views/member/dashboard";

import { Alert, AlertTitle, AppBar, Divider, Toolbar, IconButton, Box, Button, Container, Paper, styled, Backdrop, Typography, Skeleton, useMediaQuery } from '@mui/material';
import { Head1 } from './components/theme';
import logo from './assets/asami.png';
import rootstock from './assets/rootstock.png';
import { XLogin, FacebookLogin, Eip712Login, OneTimeTokenLogin } from './views/oauth_redirect';

const GoogleForm = () => {
  return <>
    <Box alignItems="center" marginTop="2em">
      <Typography variant="h1" fontSize="6em" margin="0" lineHeight="0.5em" fontFamily="Sacramento" fontWeight="bold">
        asami
      </Typography>
      <Typography variant="p" fontSize="0.8em">
        朝 (asa), which means morning; 未 (mi), which can mean "not yet", future.
      </Typography>
    </Box>
    
    <Box display="flex" flexWrap="wrap" justifyContent="space-around"  alignItems="center" gap="1em" mt="5em" mb="1em">
      <Typography fontSize="min(4em, 1em + 8vw)" flexBasis="300px" flexGrow="2" lineHeight="1em" fontFamily="LeagueSpartanBlack">
        Authentic Social Ads Marketplace Infrastructure.
      </Typography>
      <Box textAlign="center">
        <img src={logo}  style={{width: "min(200px, 100px + 15vw)" }} />
      </Box>
    </Box>
    <Box display="flex" gap="2em" mb="3em">
      <Typography my="1em">
        🇺🇸
        Have you ever wondered what your social media reposts and shares are worth?

        We're starting a club of social media users that get paid by brands to share their news and content. There are some groups that do something similar, but not like this club. You'll get paid in Bitcoin or stablecoins anytime, we're going to have transparent governance and provable payouts. Just give us your contact details so that we can contact you when we launch.
      </Typography>
      
      <Typography my="1em">
        🇪🇸
        ¿Te has preguntado cuanto valen tus reposts y republicaciones en redes sociales?
        
        Estamos creando un club de usuarios de redes sociales que cobran a las marcas por compartir su contenido. Hay algunos grupos que hacen algo similar, pero no como este club. Cobrarás en Bitcoin o "stablecoins" cuando quieras, también vamos a tener gobernanza transparente y pagos verificables. Solo déjanos tus datos para que te contactemos cuando lancemos.
      </Typography>
    </Box>

    <iframe src="https://docs.google.com/forms/d/e/1FAIpQLSfTHFS64af5flLp7Z3c8xrqr1tasf8oKL9avqcbli-_dBqQGA/viewform?embedded=true" width="100%" height="900px" frameborder="0" marginheight="0" marginwidth="0">Loading…</iframe>
    <Divider light sx={{ mt: "5em", mb: "2em" }}/>

    <Button href="https://rootstock.io/" target="_blank" sx={{textDecoration: "none", mb: "2em"}}>
      <Box display="flex" flexDirection="column">
        <Typography fontSize="1em" textTransform="uppercase" fontFamily="LeagueSpartanBold">
          Built with
        </Typography>
        <img src={rootstock}  style={{width: "150px" }} />
      </Box>
    </Button>
  </>
}

const Dashboard = () => {
  const [searchParams,] = useSearchParams();
  const [storedRole] = useStore('user.role', 'advertiser');
  const role = searchParams.get("role") || storedRole;
  return <Authenticated requireAuth>{role == 'advertiser' ? <AdvertiserDashboard /> : <MemberDashboard />}</Authenticated>;
}

export const App = () => {
  const [dataProvider, setDataProvider] = useSafeSetState<any>(null);

  useEffect(() => {
    async function initApp() {
      const dataProv = await defaultDataProvider();
      setDataProvider(dataProv);
    }
    initApp();
  }, []);


  if (!dataProvider) {
    return <Container maxWidth="md">
      <Skeleton animation="wave" />
    </Container>;
  }

  return (
  <ContractsProvider>
    <GoogleReCaptchaProvider reCaptchaKey={ Settings.recaptchaSiteKey }>
      <Admin
        dashboard={Dashboard}
        disableTelemetry={true}
        theme={asamiTheme}
        layout={AsamiLayout}
        loginPage={Login}
        authProvider={authProvider}
        dataProvider={dataProvider}
      >
        <CustomRoutes>
          <Route path="/one_time_token_login" element={<OneTimeTokenLogin/>}/>
          <Route path="/x_login" element={<XLogin/>}/>
          <Route path="/facebook_login" element={<FacebookLogin/>}/>
          <Route path="/eip712_login" element={<Eip712Login/>}/>
        </CustomRoutes>
      </Admin>
    </GoogleReCaptchaProvider>
  </ContractsProvider>
);
}
