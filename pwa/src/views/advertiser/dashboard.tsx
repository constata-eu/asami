import { useEffect } from "react";
import { useAuthenticated, useSafeSetState, useGetOne} from "react-admin";
import { Box, Card, CardContent, Container, Skeleton, Typography } from "@mui/material";
import { formatEther } from "ethers";
import { LoggedInNavCard, ColumnsContainer, DeckCard } from '../layout';
import { CardTitle } from '../../components/theme';
import { viewPostUrl } from '../../lib/campaign';
import { Pagination, Datagrid, TextField, FunctionField} from 'react-admin';
import { useListController, ListContextProvider, useTranslate } from 'react-admin';
import { getAuthKeys } from '../../lib/auth_provider';
import ClaimAccountButton from '../claim_account';
import { CampaignRequestCard } from './campaign_request_card';
import { MakeCampaignCard } from './make_campaign_card';
import BalanceCard from "../balance_card";

const Dashboard = () => {
  useAuthenticated();
  const translate = useTranslate();

  const {data, isLoading} = useGetOne(
    "Account",
    { id: getAuthKeys().session.accountId },
    { refetchInterval: (d) => d?.status == "DONE" ? false : 5000 }
  );

  const [needsRefresh, setNeedsRefresh] = useSafeSetState(false);

  if(isLoading) {
    return <Container maxWidth="md">
      <Skeleton animation="wave" />
    </Container>;
  }

  const hasClaim = !!data.status;
  const hasPendingClaim = data.status == "RECEIVED" || data.status == "SUBMITTED";
  const isFullMember = data.status == "DONE";

  return (<Box p="1em" id="advertiser-dashboard">
    <ColumnsContainer>
      <LoggedInNavCard />
      <BalanceCard />
      { !hasClaim && <CampaignRequestCard /> }
      { isFullMember && <MakeCampaignCard account={data} /> }

      { hasPendingClaim && <DeckCard id="advertiser-claim-account-pending">
        <CardContent>{ translate("claim_account.claim_pending") }</CardContent>
        </DeckCard>
      }

      { !hasClaim && <DeckCard id="advertiser-claim-account-none">
          <CardContent>
            { translate("claim_account.claim_available") }
            <ClaimAccountButton variant="outlined"
              label={ translate("claim_account.claim_button") } id="advertiser-claim-account-button"/>
          </CardContent>
        </DeckCard>  
      }
    </ColumnsContainer>

    <CampaignRequestList {...{needsRefresh, setNeedsRefresh}} />
    <CampaignList/>
  </Box>);
}

const CampaignRequestList = ({needsRefresh, setNeedsRefresh}) => {
  const translate = useTranslate();
  const listContext = useListController({
    debounce: 500,
    disableSyncWithLocation: true,
    filter: {statusIn: ["RECEIVED", "PAID", "SUBMITTED"]},
    queryOptions: {
      refetchInterval: 2000,
    },
    perPage: 20,
    resource: "CampaignRequest",
  });

  useEffect(() => {
    if(needsRefresh) {
      listContext.refetch();
      setNeedsRefresh(false);
    }
  }, [needsRefresh, setNeedsRefresh]);

  if (listContext.total < 1 ){
    return <></>;
  }

  return (
    <ListContextProvider value={listContext}>
      <Card id="campaign-request-list" sx={{my:"3em"}}>
        <CardTitle text="campaign_request_list.title" >
          <Typography mt="1em">{ translate("campaign_request_list.admin_will_review") }</Typography>
          <Typography>{ translate("campaign_request_list.claim_to_create") }</Typography>
        </CardTitle>
        <Datagrid bulkActionButtons={false}>
          <FunctionField label={ translate("campaign_request_list.post") } render={record =>
            <a target="_blank" href={viewPostUrl(record)} rel="noreferrer">{ translate("campaign_request_list.see_post") }</a>
          } />
          <TextField source="status" label={ translate("campaign_request_list.status") } />
          <TextField source="site" label={ translate("campaign_request_list.site") }/>
          <FunctionField label={ translate("campaign_request_list.budget") }
            render={record => `${formatEther(record.budget)} DOC` } />
        </Datagrid>
        <Pagination rowsPerPageOptions={[]} />
      </Card>
    </ListContextProvider>
  );
}

const CampaignList = () => {
  const translate = useTranslate();
  const listContext = useListController({
    debounce: 500,
    disableSyncWithLocation: true,
    filter: {accountIdEq: getAuthKeys().session.accountId },
    perPage: 20,
    queryOptions: {
      refetchInterval: 3000,
    },
    resource: "Campaign",
  });

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
            <a target="_blank" href={`https://x.com/twitter/status/${record.contentId}`} rel="noreferrer">
              { translate("campaign_list.see_post") }
            </a>
          } />
          <TextField source="site" label={ translate("campaign_list.site") } />
          <FunctionField label={ translate("campaign_list.budget") } render={record => `${formatEther(record.budget)} DOC` } />
          <FunctionField render={record => `${formatEther(record.remaining)} DOC` } />
          <TextField source="validUntil" label={ translate("campaign_list.ends_on") } />
        </Datagrid>
        <Pagination rowsPerPageOptions={[]} />
      </Card>
    </ListContextProvider>
  );
}

export default Dashboard;
