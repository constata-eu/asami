import { useDataProvider, useAuthenticated } from "react-admin";
import { LoggedInNavCard, ColumnsContainer } from '../layout';
import { Box } from "@mui/material";
import XSettings from "./x_settings";
import { useListController } from 'react-admin';
import { getAuthKeys } from '../../lib/auth_provider';
import BalanceCard from "../balance_card";
import CollabList from "./collab_list";
import HelpCard from "./help_card";
import VersionTwoCard from "../version_2";
import { XCampaign, IgCampaign} from "./campaigns";

const Dashboard = () => {
  useAuthenticated();

  const campaigns = useListController({
    disableSyncWithLocation: true,
    filter: {availableToAccountId: getAuthKeys().session.accountId },
    perPage: 20,
    queryOptions: {
      refetchInterval: 10000,
    },
    resource: "Campaign",
  });

  const handles = useListController({
    disableSyncWithLocation: true,
    filter: {accountIdEq: getAuthKeys().session.accountId},
    queryOptions: {
      refetchInterval: 20000,
    },
    resource: "Handle",
  });

  return (<Box p="1em" id="member-dashboard">
    <ColumnsContainer>
      <LoggedInNavCard />
      <HelpCard handles={handles} campaigns={campaigns} />
      <BalanceCard />
      <XSettings handles={handles} />
      <CampaignList handles={handles}/>
    </ColumnsContainer>
    <CollabList />
  </Box>);
}

const CampaignList = ({handles}) => {
  const dataProvider = useDataProvider();
  const listContext = useListController({
    disableSyncWithLocation: true,
    filter: {availableToAccountId: getAuthKeys().session.accountId },
    perPage: 20,
    queryOptions: {
      refetchInterval: 6000,
    },
    resource: "Campaign",
  });

  const prefsContext = useListController({
    disableSyncWithLocation: true,
    perPage: 200,
    resource: "CampaignPreference",
  });

  if (prefsContext.isLoading || listContext.isLoading || handles.isLoading || handles.total == 0 || listContext.total == 0 ){
    return <></>;
  }

  const setPreference = async (campaignId, notInterested, attempted) => {
    await dataProvider.create('CampaignPreference', { data: { input: {campaignId, notInterested, attempted} } });
    await listContext.refetch();
    if(attempted) {
      await prefsContext.refetch();
    }
  };

  const xHandle = handles.data?.filter((x) => x.site == 'X')[0];
  const igHandle = handles.data?.filter((x) => x.site == 'INSTAGRAM')[0];

  return ( <>
    <Box id="campaign-list" display="flex" flexWrap="wrap">
      { listContext.data.map((item) => item.campaignKind == "XREPOST" ?
        <XCampaign key={item.id} handle={xHandle} campaign={item} prefsContext={prefsContext} setPreference={setPreference} /> :
        <IgCampaign key={item.id} handle={igHandle} campaign={item} prefsContext={prefsContext} setPreference={setPreference} />
      )}
    </Box>
    </>
  );
}

export default Dashboard;

