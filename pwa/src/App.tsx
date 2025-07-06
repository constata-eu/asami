import { useContext, useEffect, useState } from "react";
import {
  Admin,
  Resource,
  CustomRoutes,
  useStore,
  Authenticated,
  useAuthenticated,
} from "react-admin";
import { ContractsProvider } from "./components/contracts_context";
import { Settings } from "./settings";
import { GoogleReCaptchaProvider } from "react-google-recaptcha-v3";
import { Route, useSearchParams } from "react-router-dom";
import { authProvider } from "./lib/auth_provider";
import { defaultDataProvider, publicDataProvider } from "./lib/data_provider";
import Landing from "./views/landing";
import Login from "./views/login";
import About from "./views/about";
import asamiTheme from "./components/theme";
import { BareLayout } from "./views/layout";
import AdvertiserDashboard from "./views/advertiser/dashboard";
import MemberDashboard from "./views/member/dashboard";
import { StripeSuccess, StripeCancel } from "./views/stripe";
import { Container, Skeleton } from "@mui/material";
import {
  XLogin,
  XGrantAccess,
  Eip712Login,
  OneTimeTokenLogin,
  SessionMigrationForX,
} from "./views/oauth_redirect";
import polyglotI18nProvider from "ra-i18n-polyglot";
import { messages, browserLocale } from "./i18n";
import { HandleEdit, HandleList } from "./views/explorer/handles";
import { CampaignList, CampaignShow } from "./views/explorer/campaigns";
import { AccountList, AccountShow } from "./views/explorer/accounts";
import { CollabList } from "./views/explorer/collabs";
import { StatsShow } from "./views/explorer/stats";
import Whitepaper from "./views/whitepaper";
import Token from "./views/explorer/token";
import { EmbeddedProvider } from "./components/embedded_context";
import { MergeAccounts, AcceptMergeFromSource } from "./views/merge_accounts";

const Dashboard = () => {
  const [searchParams] = useSearchParams();
  const storedRole = localStorage.getItem("asami_user_role");
  const role = searchParams.get("role") || storedRole || "member";
  const { isPending, isError } = useAuthenticated(); // redirects to login if not authenticated

  if (isPending || isError) {
    return <></>;
  }

  return role == "advertiser" ? <AdvertiserDashboard /> : <MemberDashboard />;
};

const Embedded = () => {
  localStorage.setItem("embedded", "true");
  return <Dashboard />;
};

const UnEmbedded = () => {
  localStorage.removeItem("embedded");
  return <Dashboard />;
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
        <EmbeddedProvider>
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
              edit={HandleEdit}
              recordRepresentation={(record) => record.username}
            />

            <Resource name="Campaign" list={CampaignList} show={CampaignShow} />

            <Resource name="Account" list={AccountList} show={AccountShow} />

            <Resource name="Collab" list={CollabList} />

            <Resource name="Stats" show={StatsShow} />
            <Resource name="TokenStats" />

            <Resource name="Topic" />
            <Resource name="CommunityMember" />

            <CustomRoutes>
              <Route path="/embedded" element={<Embedded />} />
              <Route path="/un-embedded" element={<UnEmbedded />} />
              <Route path="/" element={<Landing />} />
              <Route path="/dashboard" element={<Dashboard />} />
              <Route path="/about" element={<About />} />
              <Route path="/whitepaper" element={<Whitepaper />} />
              <Route path="/token" element={<Token />} />
            </CustomRoutes>

            <CustomRoutes noLayout>
              <Route path="/merge-accounts" element={<MergeAccounts />} />
              <Route path="/stripe-success" element={<StripeSuccess />} />
              <Route path="/stripe-cancel" element={<StripeCancel />} />
              <Route path="/s/:token" element={<SessionMigrationForX />} />
              <Route path="/m/:code" element={<AcceptMergeFromSource />} />
              <Route
                path="/one_time_token_login"
                element={<OneTimeTokenLogin />}
              />
              <Route path="/x_login" element={<XLogin />} />
              <Route path="/eip712_login" element={<Eip712Login />} />
              <Route path="/x_grant_access" element={<XGrantAccess />} />
            </CustomRoutes>
          </Admin>
        </EmbeddedProvider>
      </GoogleReCaptchaProvider>
    </ContractsProvider>
  );
};
