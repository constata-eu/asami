import { useDataProvider, useAuthenticated } from "react-admin";
import { Box, Stack } from "@mui/material";
import { useListController } from "react-admin";
import { getAuthKeys } from "../../lib/auth_provider";
import BalanceCard from "../balance_card";
import CollabList from "./collab_list";
import TitleCard from "./title_card";
import { XCampaign } from "./campaigns";
import { HandleSettings } from "./handle_settings";
import { ResponsiveAppBar } from "../responsive_app_bar";
import { CardTable } from "../layout";

const Dashboard = () => {
  useAuthenticated();

  const campaigns = useListController({
    disableSyncWithLocation: true,
    filter: { availableToAccountId: getAuthKeys().session.accountId },
    perPage: 20,
    queryOptions: {
      refetchInterval: 10000,
    },
    resource: "Campaign",
  });

  const handles = useListController({
    disableSyncWithLocation: true,
    filter: { accountIdEq: getAuthKeys().session.accountId },
    queryOptions: {
      refetchInterval: 20000,
    },
    resource: "Handle",
  });

  return (
    <Box id="member-dashboard">
      <ResponsiveAppBar />
      <CardTable mb="2em">
        <TitleCard handles={handles} campaigns={campaigns} />
        <Box mb="1em">
          <BalanceCard />
        </Box>
        <HandleSettings handles={handles} />
        <CampaignList handles={handles} />
      </CardTable>
      <CollabList />
    </Box>
  );
};

const CampaignList = ({ handles }) => {
  const dataProvider = useDataProvider();
  const listContext = useListController({
    disableSyncWithLocation: true,
    filter: { availableToAccountId: getAuthKeys().session.accountId },
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

  if (
    prefsContext.isLoading ||
    listContext.isLoading ||
    !listContext.total ||
    handles.isLoading ||
    !handles.total
  ) {
    return <></>;
  }

  const setPreference = async (campaignId, notInterested, attempted) => {
    await dataProvider.create("CampaignPreference", {
      data: { input: { campaignId, notInterested, attempted } },
    });
    await listContext.refetch();
    if (attempted) {
      await prefsContext.refetch();
    }
  };

  const xHandle = handles.data?.filter((x) => x.site == "X")[0];

  return (
    <>
      {listContext.data.map((item) => (
        <XCampaign
          key={item.id}
          handle={xHandle}
          campaign={item}
          prefsContext={prefsContext}
          setPreference={setPreference}
          height="300px"
        />
      ))}
    </>
  );
};

export default Dashboard;
