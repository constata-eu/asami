import { useEffect } from "react";
import { useDataProvider, useAuthenticated, useSafeSetState, useTranslate, useGetList, useGetOne} from "react-admin";
import LoadingButton from '@mui/lab/LoadingButton';
import { Alert, Box, Button, Card, CardActions, CardContent, Container, FormControl, FormHelperText, InputLabel, MenuItem, Select, Skeleton, Typography, IconButton } from "@mui/material";
import { Dialog, DialogContent, DialogTitle, DialogActions } from '@mui/material';
import { ethers, parseUnits, formatEther, toBeHex, zeroPadValue, parseEther } from "ethers";
import { LoggedInNavCard, ColumnsContainer, DeckCard } from '../layout';
import schnorr from "bip-schnorr";
import { Buffer } from "buffer";
import Login from "./views/login";
import { useContracts } from "../../components/contracts_context";
import { Head1, Head2, BulletPoint, CardTitle } from '../../components/theme';
import LoginIcon from '@mui/icons-material/Login';
import _ from 'lodash';
import { LocalizationProvider } from '@mui/x-date-pickers';
import { DateField } from '@mui/x-date-pickers/DateField';
import { AdapterDayjs } from '@mui/x-date-pickers/AdapterDayjs'
import dayjs from 'dayjs';
import { viewPostUrl } from '../../lib/campaign';
import Table from '@mui/material/Table';
import TableBody from '@mui/material/TableBody';
import TableCell from '@mui/material/TableCell';
import TableContainer from '@mui/material/TableContainer';
import TableHead from '@mui/material/TableHead';
import TableRow from '@mui/material/TableRow';
import Paper from '@mui/material/Paper';
import { Toolbar, Create, SimpleForm, CreateBase, Form, TextInput, RichTextInput, SaveButton, useNotify } from 'react-admin';
import { ListBase, Title, ListToolbar, Pagination, Datagrid, TextField, FunctionField} from 'react-admin';
import {  
    useListController,
    defaultExporter,
    ListContextProvider
} from 'react-admin';

import { Stack } from '@mui/material';
import CampaignIcon from '@mui/icons-material/Campaign';
import CloseIcon from '@mui/icons-material/Close';
import { getAuthKeys } from '../../lib/auth_provider';
import ClaimAccountButton from '../claim_account';
import { CampaignRequestCard } from './campaign_request_card';
import { MakeCampaignCard } from './make_campaign_card';

const Dashboard = () => {
  useAuthenticated();

  const {data, isLoading, error, refetch} = useGetOne(
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
      { !hasClaim && <CampaignRequestCard /> }
      { isFullMember && <MakeCampaignCard account={data} /> }


      { hasPendingClaim && <DeckCard id="advertiser-claim-account-pending">
          <CardContent>
            You'll be able to start new campaigns again once your WEB3 account setup is done.
          </CardContent>
        </DeckCard>
      }

      { !hasClaim && <DeckCard id="advertiser-claim-account-none">
          <CardContent>
            Since you haven't connected your WEB3 wallet, your campaigns will be funded privately by the club's admin,
            subject to how much funds are available.
            Connect your wallet to claim your account and fund campaigns with your own budget.
            <ClaimAccountButton variant="outlined" label="Connect wallet" id="advertiser-claim-account-button"/>
          </CardContent>
        </DeckCard>  
      }
    </ColumnsContainer>

    <CampaignRequestList {...{needsRefresh, setNeedsRefresh}} />
    <CampaignList/>
  </Box>);
}

const CampaignRequestList = ({needsRefresh, setNeedsRefresh}) => {
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
        <CardTitle text="Requested campaigns" >
          <Typography mt="1em">The club's admin will review and decide whether to fund your campaigns.</Typography>
          <Typography>Claim your account using a WEB3 wallet to create campaigns yourself!</Typography>
        </CardTitle>
        <Datagrid bulkActionButtons={false}>
          <FunctionField label="Post" render={record => <a target="_blank" href={viewPostUrl(record)} rel="noreferrer">See post</a>} />
          <TextField source="status" />
          <TextField source="site" />
          <FunctionField label="Budget" render={record => `${formatEther(record.budget)} DOC` } />
        </Datagrid>
        <Pagination rowsPerPageOptions={[]} />
      </Card>
    </ListContextProvider>
  );
}

const CampaignList = ({needsRefresh, setNeedsRefresh}) => {
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
        <CardTitle text="Active campaigns" >
          <Typography mt="1em">The ASAMI club members will be reposting your content until the budget is fully spent.</Typography>
        </CardTitle>
        <Datagrid bulkActionButtons={false}>
          <FunctionField label="Post" render={record => <a target="_blank" href={`https://x.com/twitter/status/${record.contentId}`} rel="noreferrer">See post</a>} />
          <TextField source="site" />
          <FunctionField label="Budget" render={record => `${formatEther(record.budget)} DOC` } />
          <FunctionField label="Remaining" render={record => `${formatEther(record.remaining)} DOC` } />
          <TextField source="validUntil" />
        </Datagrid>
        <Pagination rowsPerPageOptions={[]} />
      </Card>
    </ListContextProvider>
  );
}

export default Dashboard;
