import { useEffect, useState } from "react";
import {
  Admin,
  Resource,
  CustomRoutes,
  useStore,
  Authenticated,
} from "react-admin";
import { ContractsProvider } from "./components/contracts_context";
import { Settings } from "./settings";
import { GoogleReCaptchaProvider } from "react-google-recaptcha-v3";
import { Route, useSearchParams } from "react-router-dom";
import { authProvider } from "./lib/auth_provider";
import { defaultDataProvider } from "./lib/data_provider";
import Login from "./views/login";
import About from "./views/about";
import asamiTheme from "./components/theme";
import { BareLayout } from "./views/layout";
import AdvertiserDashboard from "./views/advertiser/dashboard";
import MemberDashboard from "./views/member/dashboard";
import { Container, Skeleton } from "@mui/material";
import {
  XLogin,
  XGrantAccess,
  Eip712Login,
  OneTimeTokenLogin,
} from "./views/oauth_redirect";
import polyglotI18nProvider from "ra-i18n-polyglot";
import { messages, browserLocale } from "./i18n";
import { HandleList } from "./views/explorer/handles";
import { CampaignList } from "./views/explorer/campaigns";
import { AccountList, AccountShow } from "./views/explorer/accounts";
import { CollabList } from "./views/explorer/collabs";
import { OnChainJobList } from "./views/explorer/on_chain_jobs";
import { StatsShow } from "./views/explorer/stats";

const Dashboard = () => {
  const [searchParams] = useSearchParams();
  const [storedRole] = useStore("user.role", "advertiser");
  const role = searchParams.get("role") || storedRole;
  return (
    <Authenticated requireAuth>
      {role == "advertiser" ? <AdvertiserDashboard /> : <MemberDashboard />}
    </Authenticated>
  );
};

export const App = () => {
  const [dataProvider, setDataProvider] = useState<any>(null);

  useEffect(() => {
    async function initApp() {
      const dataProv = await defaultDataProvider();
      setDataProvider(dataProv);
    }
    initApp();
  }, []);

  const i18nProvider = polyglotI18nProvider(
    (locale) => messages[locale],
    browserLocale,
  );

  if (!dataProvider || !i18nProvider) {
    return (
      <Container maxWidth="md">
        <Skeleton animation="wave" />
      </Container>
    );
  }

  return (
    <ContractsProvider>
      <GoogleReCaptchaProvider reCaptchaKey={Settings.recaptchaSiteKey}>
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
          <Resource
            name="Handle"
            list={HandleList}
            recordRepresentation={(record) => record.username}
          />

          <Resource name="Campaign" list={CampaignList} />

          <Resource name="Account" list={AccountList} show={AccountShow} />

          <Resource name="Collab" list={CollabList} />

          <Resource name="OnChainJob" list={OnChainJobList} />

          <Resource name="Stats" show={StatsShow} />

          <Resource name="Topic" />

          <CustomRoutes>
            <Route path="/about" element={<About />} />
            <Route
              path="/one_time_token_login"
              element={<OneTimeTokenLogin />}
            />
            <Route path="/x_login" element={<XLogin />} />
            <Route path="/eip712_login" element={<Eip712Login />} />
            <Route path="/x_grant_access" element={<XGrantAccess />} />
          </CustomRoutes>
        </Admin>
      </GoogleReCaptchaProvider>
    </ContractsProvider>
  );
};
