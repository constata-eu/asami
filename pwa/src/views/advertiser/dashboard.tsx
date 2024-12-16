import { useEffect } from "react";
import { useAuthenticated, useSafeSetState, useGetOne} from "react-admin";
import { Box, Card, CardContent, Container, Skeleton, Typography } from "@mui/material";
import { formatEther } from "ethers";
import { LoggedInNavCard, ColumnsContainer, DeckCard } from '../layout';
import { CardTitle, Head2, green } from '../../components/theme';
import { viewPostUrl } from '../../lib/campaign';
import { Pagination, Datagrid, TextField, FunctionField} from 'react-admin';
import { useListController, ListContextProvider, useTranslate } from 'react-admin';
import { getAuthKeys } from '../../lib/auth_provider';
import { MakeCampaignCard } from './make_campaign_card';
import BalanceCard from "../balance_card";
import StatsCard from "../stats_card";

const Dashboard = () => {
  useAuthenticated();

  const {data, isLoading} = useGetOne(
    "Account",
    { id: getAuthKeys().session.accountId },
    { refetchInterval: (d) => d?.status == "CLAIMED" ? false : 5000 }
  );

  const listContext = useListController({
    debounce: 500,
    disableSyncWithLocation: true,
    filter: {
			accountIdEq: getAuthKeys().session.accountId,
			statusNe: "DRAFT"
		},
    perPage: 20,
    queryOptions: {
      refetchInterval: 10000,
    },
    resource: "Campaign",
  });

  if(isLoading || !data) {
    return <Container maxWidth="md">
      <Skeleton animation="wave" />
    </Container>;
  }

  return (<Box p="1em" id="advertiser-dashboard">
    <ColumnsContainer>
      <LoggedInNavCard />
			<AdvertiserHelpCard account={data} />
      <StatsCard />
      <BalanceCard />
      { data.status == "CLAIMED" && <MakeCampaignCard account={data} onCreate={() => listContext.refetch() } /> }

    </ColumnsContainer>

    <CampaignList listContext={listContext}/>
  </Box>);
}

const AdvertiserHelpCard = ({account}) => {
  const translate = useTranslate();
  const id = {
    "MANAGED": "advertiser-claim-account-none",
    "CLAIMING": "advertiser-claim-account-pending",
    "CLAIMED": "advertiser-claim-account-done",
    "BANNED": "advertiser-banned",
  }[account.status];

  return <DeckCard id="advertiser-help-card" borderColor={green}>
    <CardContent>
      <Head2 sx={{ color: green}} >{ translate('advertiser_help.title') }</Head2>
      <Typography id={id} mt="1em">{ translate(`advertiser_help.${account.status}`) }</Typography>
    </CardContent>
  </DeckCard>;
}

const CampaignList = ({listContext}) => {
  const translate = useTranslate();

  if (listContext.total < 1 ){
    return <></>;
  }

  return (
    <ListContextProvider value={listContext}>
      <Card id="campaign-list" sx={{my:"3em"}}>
        <CardTitle text="campaign_list.title" >
          <Typography mt="1em">{ translate("campaign_list.text") } </Typography>
        </CardTitle>
        <Datagrid bulkActionButtons={false}>
          <FunctionField label={ translate("campaign_list.post") } render={record =>
            <a target="_blank" href={viewPostUrl(record)} rel="noreferrer">
              { translate("campaign_list.see_post") }
            </a>
          } />
          <FunctionField label={ translate("campaign_list.kind") } render={record =>
						translate(`campaign_kinds.${record.campaignKind}`)
          } />
          <FunctionField source="status" label={ translate("campaign_list.status") } render={record => {
						if (record.status == "SUBMITTED") {
							return translate("campaign_list.statuses.publishing");
						} else if (record.budget > BigInt(0)) {
							return translate("campaign_list.statuses.running", {budget: formatEther(record.budget), validUntil: new Date(record.validUntil).toDateString()});
						} else {
							return translate("campaign_list.statuses.stopped")
						}

					}} />
        </Datagrid>
        <Pagination rowsPerPageOptions={[]} />
      </Card>
    </ListContextProvider>
  );
}

export default Dashboard;
