import {
  Admin,
  Resource,
  ListGuesser,
  EditGuesser,
  ShowGuesser,
  CustomRoutes,
  useSafeSetState,
} from "react-admin";
import { Route } from "react-router-dom";
import { authProvider } from "./auth_provider";
import Login from "./views/login";
import asamiTheme from './components/theme';
import { AsamiLayout } from './views/layout';

import CampaignWizard from "./views/advertiser/campaign_wizard";

const Dashboard = () => {
  return <CampaignWizard />;
}

export const App = () => <Admin
    authProvider={authProvider}
    dashboard={Dashboard}
    disableTelemetry={true}
    loginPage={Login}
    theme={asamiTheme}
    layout={AsamiLayout}
  >
    <CustomRoutes>
      <Route path="/help" element={<Dashboard/>}/>
    </CustomRoutes>
  </Admin>;
