import {
  Admin,
  Resource,
  ListGuesser,
  EditGuesser,
  ShowGuesser,
  CustomRoutes,
  useSafeSetState,
  useStore,
} from "react-admin";
import { Route } from "react-router-dom";
import { authProvider } from "./auth_provider";
import Login from "./views/login";
import asamiTheme from './components/theme';
import { AsamiLayout } from './views/layout';

import CampaignWizard from "./views/advertiser/campaign_wizard";
import AdvertiserDashboard from "./views/advertiser/dashboard";
import CreatorDashboard from "./views/creator/dashboard";

const Dashboard = () => {
  const [role, setRole] = useStore('user.role', 'advertiser');

  return (role == 'advertiser' ? <AdvertiserDashboard /> : <CreatorDashboard />);
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
      <Route path="/advertiser/campaign_wizard" element={<CampaignWizard/>}/>
    </CustomRoutes>
  </Admin>;
