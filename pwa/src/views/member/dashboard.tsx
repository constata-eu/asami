import { useEffect } from "react";
import { useDataProvider, useGetIdentity, useSafeSetState, useTranslate, ReferenceField, useShowController} from "react-admin";
import { rLogin } from "../../lib/rLogin";
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
import { nip19 } from 'nostr-tools';
import { TwitterTweetEmbed } from 'react-twitter-embed';

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

import asamiABI from "../../abi/Asami.json";
import docABI from "../../abi/Doc.json";

const Dashboard = () => {
  const [loading, setLoading] = useSafeSetState(true);
  const [needsRefresh, setNeedsRefresh] = useSafeSetState(false);

  const handles = useListController({
    disableSyncWithLocation: true,
    filter: {accountIdIn: getAuthKeys().session.accountIds, siteEq: "X"},
    queryOptions: {
      refetchInterval: 2000,
    },
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
    <ClaimAccountRequestPendingMessage />
    <CampaignList handles={handles}/>
    <CollabList />
    <ConfigureXAccount handles={handles}/>

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
  </Container>;
}

const ClaimAccountRequestPendingMessage = () => {
  const {isLoading, data } = useListController({
    disableSyncWithLocation: true,
    filter: { accountIdEq: getAuthKeys().session.accountIds[0] },
    perPage: 1,
    queryOptions: {
      refetchInterval: 3000,
    },
    resource: "ClaimAccountRequest",
  });

  let content;

  if (isLoading) {
    content = <></>;
  } else if(!data[0]) {
    content = <Typography id="account-summary-claim-none">
      Your rewards will be paid using the DOC token, which is pegged to the united states dollar, in the RSK network.
      To get your rewards, you'll have to claim your account using a WEB3 wallet such as metamask.
      You've claimed your account.
    </Typography>;
  } else if(data[0].status == "DONE") {
    content = <ClaimedAccountState />;
  } else {
    content = <Typography id="account-summary-claim-pending">
      You've requested to manage your own account, we'll let you know when it's done.
    </Typography>;
    
    content = <ClaimedAccountState />;
  }
  
  return <Card id="account-summary" sx={{my:"3em"}}>
    <CardTitle text="Your account" />
    { content }
  </Card>;
}

const ClaimedAccountState = () => {
  const [loading, setLoading] = useSafeSetState(true);


  useEffect(() => {
    const load = async () => {
    }

    if (!isLoading && !isFetching) {
      load();
    }
  }, [isLoading, isFetching]);

  if (loading) {
    return <></>;
  }

  return <Typography id="account-summary-claim-done">
    You've claimed your account.
  </Typography>;
}

const CampaignList = ({handles}) => {
  const listContext = useListController({
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
          <FunctionField label={false} render={record =>  {
            const repostUrl = `https://twitter.com/intent/retweet?tweet_id=${record.contentId}&related=asami_club`;
            return <Box>
              <TwitterTweetEmbed tweetId={record.contentId} options={{ align: "center", width: "550px", conversation: "none"}} />
              <Button fullWidth variant="contained" href={repostUrl} target="_blank">Repost it!</Button>
            </Box>
          }} />
        </Datagrid>
        <Pagination rowsPerPageOptions={[]} />
      </Card>
    </ListContextProvider>
  );
}

const CollabList = () => {
  const listContext = useListController({
    disableSyncWithLocation: true,
    filter: {memberIdIn: getAuthKeys().session.accountIds },
    perPage: 20,
    queryOptions: {
      refetchInterval: 3000,
    },
    resource: "Collab",
  });

  if (listContext.total == 0 || listContext.isLoading) {
    return <></>;
  }

  return (
    <ListContextProvider value={listContext}>
      <Card id="collab-list" sx={{my:"3em"}}>
        <CardTitle text="Your collaboration history" >
          <Typography>These are your collaborations so far.</Typography>
          <Typography>You got paid 2000 DOC in total since you became a member.</Typography>
          <Alert severity="info">
            You're still a WEB2 user, so we're keeping your money safe.
            You need to claim your account with a WEB3 wallet so we can send you the funds.
            <ClaimAccountButton id="collabs-claim-account-button"/>
          </Alert>
        </CardTitle>
        <Datagrid bulkActionButtons={false}>
          <ReferenceField source="campaignId" reference="Campaign">
            <FunctionField label={false} render={record => <a target="_blank" href={`https://x.com/twitter/status/${record.contentId}`}>See post</a>} />
          </ReferenceField>
          <FunctionField label="Reward" render={record => `${formatEther(record.gross)} DOC`} />
          <FunctionField label="Asami Fee" render={record => `${formatEther(record.fee)} DOC`} />
        </Datagrid>
        <Pagination rowsPerPageOptions={[]} />
      </Card>
    </ListContextProvider>
  );
}

const ConfigureXAccount = ({handles}) => {
  const [needsRefresh, setNeedsRefresh] = useSafeSetState(false);

  const reqs = useListController({
    disableSyncWithLocation: true,
    filter: {siteEq: "X"},
    queryOptions: {
      refetchInterval: 10000,
    },
    perPage: 1,
    resource: "HandleRequest",
  });

  useEffect(() => {
    const refetchAll = async () => {
      await handles.refetch();
      await reqs.refetch();
      setNeedsRefresh(false);
    };
    
    if(needsRefresh) {
      refetchAll()
    }
  }, [needsRefresh, setNeedsRefresh, handles, reqs]);

  let content;

  if (handles.isLoading || reqs.isLoading ){
    content = (<>
      <Skeleton />
      <Skeleton />
    </>);
  } else if (handles.data[0]) {
    content = <HandleStats handle={handles.data[0]} />;
  } else {
    const req = reqs.data[0];
    if (req) {
      if (req.status == "UNVERIFIED") {
        content = <MakeVerificationPost />;
      } else if (req.status != "DONE" ) {
        content = <HandleSubmissionInProgress req={req} />;
      }
    } else {
      content = <CreateHandleRequest {...{setNeedsRefresh}} />;
    }
  }

  return (<Box>
    <Card id="configure-x-handle-card" sx={{my:"3em"}}>
      <CardTitle text="𝕏 &nbsp; Your account" />
      { content }
    </Card>
  </Box>);
}

const CreateHandleRequest = ({setNeedsRefresh}) => {
  const notify = useNotify();

  const transformIt = async (values) => {
    return { input: values.handleRequestInput };
  }

  const onSuccess = () => {
    notify("We've got your handle!");
    setNeedsRefresh(true);
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

  return <CreateBase resource="HandleRequest" transform={transformIt} mutationOptions={{ onSuccess }} >
    <SimpleForm sanitizeEmptyValues validate={validate} toolbar={false}>
      <Typography mb="1em">Let's set up your X account in ASAMI so you can start getting paid for your reposts.</Typography>
      <TextInput fullWidth required={true} size="large" variant="filled" source="username" label="Tell us your X username." />
      <SaveButton fullWidth id="submit-handle-request-form" size="large" label="Continue" icon={<></>} />
    </SimpleForm>
  </CreateBase>;
}

const MakeVerificationPost = () => {
  const [clicked, setClicked] = useSafeSetState(false);

  let accountId = BigInt(getAuthKeys().session.accountIds[0]);
  let text = `Starting today, some of my reposts will be paid for, as I joined @asami_club [${accountId}].`;
  let intent_url = `https://x.com/intent/tweet?text=${text}`;

  return <Box p="1em">
    <Typography mb="1em">Now, to verify your account, and your willingness to become a member, we need you to post this message on X.</Typography>
    <Paper sx={{ my:"1em", p:"1em", background:"#eee"}}> {text} </Paper>
    { clicked ?
      <Alert>Thank you! It will take a moment for us to pick up your post.</Alert> :
      <Button fullWidth onClick={() => setClicked(true)} size="large" variant="contained" href={intent_url} target="_blank" rel="noopener noreferrer">Post the message</Button> 
    }
  </Box>
}

const HandleSubmissionInProgress = ({req}) => 
  <Box p="1em" id="handle-submission-in-progress-message">
    <Typography>
      We've verified {req.username}, and we're in process of publishing it to the blockchain.
      Once we're done you can start participating in campaigns.
      This shouldn't take long.
    </Typography>
  </Box>

const HandleStats = ({handle}) => 
  <Box p="1em" id="existing-x-handle-stats">
    <Typography>
      You've verified your handle { handle.username }, with user id { handle.userId }.
      The reward for each message is { formatEther(handle.price) } DOC,
      and the admin has given it a score of { BigInt(handle.score).toString() }.
    </Typography>
  </Box>

export default Dashboard;

