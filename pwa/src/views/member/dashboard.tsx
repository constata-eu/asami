import { useEffect } from "react";
import { useDataProvider, useGetIdentity, useSafeSetState, useTranslate } from "react-admin";
import LoadingButton from '@mui/lab/LoadingButton';
import { Alert, Box, Button, Card, CardActions, CardContent, Container, FormControl, FormHelperText, InputLabel, MenuItem, Select, Skeleton, Typography, IconButton } from "@mui/material";
import { Dialog, DialogContent, DialogTitle, DialogActions } from '@mui/material';
import { ethers, parseUnits, formatEther, toQuantity, toBeHex, zeroPadValue, parseEther } from "ethers";
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

const Dashboard = () => {
  const [loading, setLoading] = useSafeSetState(true);
  const [needsRefresh, setNeedsRefresh] = useSafeSetState(false);

  const handles = useListController({
    debounce: 500,
    disableSyncWithLocation: true,
    filter: {accountIdIn: getAuthKeys().session.accountIds, siteEq: "X"},
    perPage: 1,
    resource: "Handle",
  });

  useEffect(() => {
    const load = async () => {
      setLoading(false);
    }
    load();
  }, []);

  if(loading) {
    return <Container maxWidth="md">
      <Skeleton animation="wave" />
    </Container>;
  }

  return <Container maxWidth="md" id="advertiser-dashboard">
    <Head1 sx={{my:3}}>Hello!</Head1>
    <Typography my="1em">
      The ASAMI community is a human based amplifier for social network posts. You get paid for reposting content.
    </Typography>
    <ul>
      <li> Once you verify your X account you can start reposting things.</li>
      <li>You can set the price of your reposts, but you can only change it once every 14 days.</li>
      <li>The ASAMI smart contract ensures you get paid for your activity, and keeps the rules clear.</li>
      <li>A third party entity, called the ADMIN, verifies your account, your social reach, and your reposts.</li>
      <li>ASAMI will deduct a fee from your rewards, to keep the protocol working.</li>
      <li>You'll get paid in the stablecoin DoC for your reposts and accumulate the amount until you claim your account using a WEB3 wallet.</li>
      <li>You'll also receive ASAMI tokens for each collaboration, these work like stock in the protocol.</li>
      <li>Fees collected will be distributed among token holders, similar to a company's dividends, every 14 days.</li>
      <li>Tokens also allow voting on changing the fee rate and electing new ADMIN.</li>
    </ul>

    <ConfigureXAccount handles={handles}/>
    <CampaignList handles={handles}/>
  </Container>;
}

const CampaignList = ({handles}) => {
  const listContext = useListController({
    debounce: 500,
    disableSyncWithLocation: true,
    filter: {finishedEq: false},
    perPage: 20,
    queryOptions: {
      refetchInterval: 2000,
    },
    resource: "Campaign",
  });

  if (listContext.total == 0 || handles.isLoading || handles.total == 0 ){
    return <></>;
  }

  return (
    <ListContextProvider value={listContext}>
      <Card id="campaign-list" sx={{my:"3em"}}>
        <CardTitle text="Some campaigns for you!" >
          <Typography>Repost these campaigns and get rewards!</Typography>
        </CardTitle>
        <Datagrid bulkActionButtons={false}>
          <FunctionField label="Post" render={record => <a target="_blank" href={`https://x.com/twitter/status/${record.contentId}`}>See post</a>} />
          <TextField source="site" />
          <TextField source="validUntil" />
          <Button variant="contained" href="#">Repost it!</Button>
        </Datagrid>
        <Pagination rowsPerPageOptions={[]} />
      </Card>
    </ListContextProvider>
  );
}

const ConfigureXAccount = ({handles}) => {
  const [open, setOpen] = useSafeSetState(false);

  const reqs = useListController({
    debounce: 500,
    disableSyncWithLocation: true,
    filter: {siteEq: "X", statusIn: ["UNVERIFIED", "VERIFIED", "APPRAISED", "SUBMITTED"]},
    perPage: 1,
    resource: "HandleRequest",
  });

  let content;

  if (handles.isLoading || reqs.isLoading ){
    content = (<>
      <Skeleton />
      <Skeleton />
    </>);
  } else if (handles.data[0]) {
    let h = handles.data[0];
    content = <Typography>
      You've verified your handle
      { h.username }, with user id { h.userId }.
      The price for each message is { formatEther(h.price) },
      and the admin has given it a score of { toQuantity(h.score) }.
    </Typography>;
  } else {
    if (reqs.data[0]) {
      const req = reqs.data[0];
      if (req.status == "UNVERIFIED") {
        content = <Typography>To fully verify your account, and your willingness to become a member, we need you to post this message on X.</Typography>;
      } else {
        content = <Typography>
          We've verified {req.username}, and we're in process of publishing it to the blockchain.
          Once we're done you can start participating in campaigns.
          This shouldn't take long.
        </Typography>;
      }
    } else {
      content = <CreateHandleRequest />;
    }
  }

  return (<Box>
    <Card id="configure-x-handle-card" sx={{my:"3em"}}>
      <CardTitle text="𝕏 &nbsp; Your account" />
      { content }
    </Card>
  </Box>);
}

const CreateHandleRequest = () => {
  const transformIt = async (values) => {
    return { input: values.handleRequestInput };
  }

  const validate = (values) => {
    let errors = {};
    let input = { accountId: getAuthKeys().session.accountIds[0], site: "X"};

    if ( values.username.match(/^@?(\w){1,15}$/) ) {
      input.username = values.username.replace("@","");
    } else {
      errors.username = "That does not seem to be an X username. It should be something like '@jack'";
    }

    values.handleRequestInput = input;
    return errors;
  }
  return <CreateBase resource="HandleRequest" transform={transformIt} redirect={false}>
    <SimpleForm sanitizeEmptyValues validate={validate} toolbar={false}>
      <TextInput fullWidth required={true} size="large" variant="filled" source="username" label="Tell us your X username." />
      <SaveButton fullWidth id="submit-handle-request-form" size="large" label="Continue" />
    </SimpleForm>
  </CreateBase>;
}

export default Dashboard;

