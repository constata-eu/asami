import { useEffect } from "react";
import { useDataProvider, useAuthenticated, useSafeSetState, useTranslate, useGetList} from "react-admin";
import LoadingButton from '@mui/lab/LoadingButton';
import { Alert, Box, Button, Card, CardActions, CardContent, Container, FormControl, FormHelperText, InputLabel, MenuItem, Select, Skeleton, Typography, IconButton } from "@mui/material";
import { Dialog, DialogContent, DialogTitle, DialogActions } from '@mui/material';
import { ethers, parseUnits, formatEther, toBeHex, zeroPadValue, parseEther } from "ethers";
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
import { nip19 } from 'nostr-tools'

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

const Dashboard = () => {
  useAuthenticated();

  const [needsRefresh, setNeedsRefresh] = useSafeSetState(false);

  const {data, isLoading} = useGetList(
    "ClaimAccountRequest",
    { refetchInterval: (data) => data?.[0]?.status == "DONE" ? false : 5000 }
  );

  const hasClaim = !!data?.[0];
  const hasPendingClaim = hasClaim && data?.[0].status != "DONE";
  const isFullMember = hasClaim && data?.[0].status == "DONE";

  if(isLoading) {
    return <Container maxWidth="md">
      <Skeleton animation="wave" />
    </Container>;
  }

  return <Container maxWidth="md" id="advertiser-dashboard">
    <Head1 sx={{my:3}}>Hello Advertiser!</Head1>
    <Typography my="1em">
      Just enter the URL of your X post and how much you want to spend.
      <br/>
      You'll be paying between 0.001 and 0.005 USD for each <strong>real person</strong> reached.
      <br/>
      Anyone will be able to participate reposting in the next 7 days.
    </Typography>

    { !hasPendingClaim && <CreateCampaignRequest onSave={() => setNeedsRefresh(true) } /> }

    { hasPendingClaim && <Alert id="advertiser-claim-account-pending" sx={{ mt: "1em" }}>You'll be able to start new campaigns again once your WEB3 account setup is done.</Alert> }
    { !hasClaim && <Alert id="advertiser-claim-account-none" sx={{ mt: "1em" }}>
        Since you haven't connected your WEB3 wallet, your campaigns will be funded privately by the club's admin,
        subject to how much funds are available.
        Connect your wallet to claim your account and fund campaigns with your own budget.
        <ClaimAccountButton id="advertiser-claim-account-button"/>
      </Alert>  
    }
    <CampaignRequestList {...{needsRefresh, setNeedsRefresh}} />
    <CampaignList/>
  </Container>;
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
          <Typography mt="1em">Since you're still using WEB2 login, we'll take care of publishing these for you.</Typography>
          <Typography>Claim your account using your WEB3 wallet to save money and time on your next campaign.</Typography>
        </CardTitle>
        <Datagrid bulkActionButtons={false}>
          <FunctionField label="Post" render={record => <a target="_blank" href={`https://x.com/twitter/status/${record.contentId}`}>See post</a>} />
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
          <Typography>Claim your WEB3 account to get reimbursed if the budget is not spent after the campaign due date.</Typography>
        </CardTitle>
        <Datagrid bulkActionButtons={false}>
          <FunctionField label="Post" render={record => <a target="_blank" href={`https://x.com/twitter/status/${record.contentId}`}>See post</a>} />
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

const CreateCampaignRequest = ({onSave}) => {
  const notify = useNotify();
  const dataProvider = useDataProvider();
  const { contracts } = useContracts();
  const [open, setOpen] = useSafeSetState(false);
  const {data, isLoading} = useGetList(
    "ClaimAccountRequest",
    { refetchInterval: (data) => data?.[0]?.status == "DONE" ? false : 5000 }
  );
  const handleClose = () => setOpen(false);

  const defaultValidUntil = () => {
    let currentDate = new Date();
    currentDate.setTime(currentDate.getTime() + (7 * 24 * 60 * 60 * 1000));
    return currentDate;
  }

  const onSubmit = async (values) => {
    if (data?.[0]?.status == "DONE") {
      const { doc, asami, asamiAddress, signer } = await contracts();
      const input = values.campaignRequestInput;
      const budget = BigInt(input.budget);
      const site = { 'X': 0, 'Nostr': 1, 'Instagram': 2 }[input.site];
      const approval = await doc.approve(asamiAddress, budget, signer);
      await approval.wait();
      notify("Campaign budget approved.");
      
      const creation = await asami.makeCampaigns([
        { site,
          budget: BigInt(input.budget),
          contentId: input.contentId,
          priceScoreRatio: BigInt(input.priceScoreRatio),
          topics: [],
          validUntil: BigInt(Math.floor(defaultValidUntil().getTime() / 1000))
        }
      ]);
      await creation.wait();
    } else {
      await dataProvider.create("CampaignRequest", { data: { input: values.campaignRequestInput } });
    }

    onSave();
    notify("Campaign will be started soon");
    handleClose();
  }

  const validate = (values) => {
    let errors = {};
    let keys = getAuthKeys();
    let input = { accountId: getAuthKeys().session.accountId, site: "X"};

    try {
      const u = new URL(values.contentUrl);
      const path = u.pathname.split("/");
      const contentId = path[path.length - 1];
      if ( (u.host != "x.com" && u.host != "twitter.com") || !contentId.match(/^\d*$/) ) {
        errors.contentUrl = "The URL does not seem to be an X post that can be reposted";
      }
      input.contentId = contentId;
    } catch {
      errors.contentUrl = "Invalid URL";
    }

    try {
      const parsed = parseEther(values.budget);
      if( parsed <= parseEther("1") ) {
        errors.budget = "Budget is too low, must be at least 1 DoC (USD)";
      } else {
        input.budget = zeroPadValue(toBeHex(parsed), 32);
      }
    } catch {
      errors.budget = "Budget must be a number";
    }

    input.priceScoreRatio = zeroPadValue(toBeHex(parseEther("0.02")), 32);

    input.validUntil = defaultValidUntil().toISOString();

    values.campaignRequestInput = input;
    return errors;
  }

  if ( isLoading ) {
    return null;
  }
  
  return (<Box>
    <Button fullWidth variant="contained" size="large" id="open-start-campaign-dialog" onClick={ () => setOpen(true) }>
      <CampaignIcon sx={{mr:"5px"}}/>
      Start Campaign
    </Button>
    <Dialog id="start-campaign-dialog" open={open} onClose={handleClose} maxWidth="md" fullWidth>
      <Box p="1em">
        <Form sanitizeEmptyValues validate={validate} onSubmit={onSubmit}>
          <TextInput fullWidth required={true} size="large" variant="filled" source="contentUrl" label="Your post URL" />
          <TextInput fullWidth required={true} size="large" variant="filled" source="budget" label="How much you have to spend, in DoC (USD)" />
          <Box width="100%" display="flex" gap="1em" justifyContent="space-between">
            <SaveButton id="submit-start-campaign-form" size="large" label="Start Campaign" icon={<CampaignIcon/>} />
            <Button size="large" variant="contained" color="grey" onClick={handleClose}>Cancel</Button>
          </Box>
        </Form>
      </Box>
    </Dialog>
  </Box>);
}

export default Dashboard;
