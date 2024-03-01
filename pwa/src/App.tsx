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
import About from "./views/about";
import asamiTheme from './components/theme';
import { BareLayout } from './views/layout';
import AdvertiserDashboard from "./views/advertiser/dashboard";
import MemberDashboard from "./views/member/dashboard";
import { Alert, AlertTitle, AppBar, Divider, Toolbar, IconButton, Box, Button, Container, Paper, styled, Backdrop, Typography, Skeleton, useMediaQuery } from '@mui/material';
import { Head1 } from './components/theme';
import logo from './assets/asami.png';
import rootstock from './assets/rootstock.png';
import { XLogin, FacebookLogin, Eip712Login, OneTimeTokenLogin } from './views/oauth_redirect';
import polyglotI18nProvider from 'ra-i18n-polyglot';
import { messages, browserLocale, LOCALES } from "./i18n";

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

  const i18nProvider =  polyglotI18nProvider(locale => messages[locale], browserLocale);

  if (!dataProvider || !i18nProvider) {
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
        layout={BareLayout}
        loginPage={Login}
        authProvider={authProvider}
        dataProvider={dataProvider}
        i18nProvider={i18nProvider}
      >
        <CustomRoutes>
          <Route path="/about" element={<About/>}/>
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
