import { useEffect } from "react";
import { useDataProvider, useAuthenticated, useSafeSetState, useTranslate, ReferenceField, useGetList} from "react-admin";
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
import { ListBase, Title, ListToolbar, Pagination, Datagrid, TextField, FunctionField, RecordContextProvider, SimpleShowLayout} from 'react-admin';
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
  useAuthenticated();

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

  return <Container maxWidth="md" id="member-dashboard">
    <AccountState />
    <CampaignList handles={handles}/>
    <CollabList />
    <ConfigureXAccount handles={handles}/>
  </Container>;
}

const AccountState = () => {
  const {data, isLoading} = useGetList(
    "ClaimAccountRequest",
    {filter: { accountIdEq: getAuthKeys().session.accountIds[0] }},
    { refetchInterval: (data) => data?.[0]?.status == "DONE" ? false : 5000 }
  );

  let content;

  if (isLoading) {
    content = <></>;
  } else if(!data[0]) {
    content = <Card>
      <CardContent>
        <Typography id="account-summary-claim-none">
          You're not a full member yet, you can participate in campaigns and ASAMI will hold on to your rewards for you.
          Once you claim your account by connectng your WEB3 wallet, you'll receive all your outstanding rewards.
          <ClaimAccountButton id="collabs-claim-account-button"/>
        </Typography>
      </CardContent>
    </Card>;
  } else if(data[0].status == "DONE") {
    content = <ClaimedAccountState />;
  } else {
    content = <Card>
      <CardContent>
        <Typography id="account-summary-claim-pending">
          You've requested to manage your own account, we'll let you know when it's done.
        </Typography>
      </CardContent>
    </Card>;
  }
  
  return <Box id="account-summary" mb="2em">
    { content }
  </Box>;
}

const ClaimedAccountState = () => {
  const [loading, setLoading] = useSafeSetState(false);
  const [balances, setBalances] = useSafeSetState({asami: 0, doc: 0});
  const { contracts } = useContracts();

  useEffect(() => {
    const init = async () => {
      const { doc, asami, signer } = await contracts();
      setBalances({
        asami: (await asami.balanceOf(signer.address)),
        doc: (await doc.balanceOf(signer.address))
      });
      setLoading(false);
    }
    init();
  }, [balances, setBalances]);

  if (loading) {
    return null;
  }

  return <Box id="account-summary-claim-done" display="flex" alignItems="center" flexWrap="wrap" gap="1em" flex="1 1">
    <SimpleShowLayout record={balances} sx={{ ".ra-field": {flex: "1 1"}}}>
      <FunctionField label="ASAMI token balance" render={ record => `${formatEther(record.asami)}` }  />
      <FunctionField label="DOC balance" render={ record => `${formatEther(record.doc)}` }  />
    </SimpleShowLayout>
  </Box>;
}

const CampaignList = ({handles}) => {
  const listContext = useListController({
    disableSyncWithLocation: true,
    filter: {availableToAccountIds: getAuthKeys().session.accountIds },
    perPage: 20,
    queryOptions: {
      refetchInterval: 6000,
    },
    resource: "Campaign",
  });

  if (listContext.isLoading || handles.isLoading || handles.total == 0 ){
    return <></>;
  }

  if (listContext.total == 0 ) {
    return <Card id="campaign-list-empty" sx={{my:"3em"}}>
      <CardContent>
        <Head2>We're all out of campaigns.</Head2>
        <Typography>Check back soon though!</Typography>
      </CardContent>
    </Card>;
  }

  return ( <>
    <Head1 sx={{ my: "1em" }}>Check out these campaigns</Head1>
    <Box id="campaign-list" display="flex" flexWrap="wrap" gap="2em">
      { listContext.data.map((item, index) => {
        const repostUrl = `https://twitter.com/intent/retweet?tweet_id=${item.contentId}&related=asami_club`;
        return <Card sx={{ flex: "1 1 400px", p: "1em" }} key={index}>
          <TwitterTweetEmbed tweetId={item.contentId} options={{ align: "center", width: "550px", conversation: "none"}} />
          <Button fullWidth size="large" variant="contained" href={repostUrl} target="_blank">Repost it!</Button>
        </Card>;
      })}
    </Box>
    </>
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
      <CardTitle text="Your 𝕏" />
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
    <Typography mb="1em">Now, to verify your username, and your willingness to use it in ASAMI, we need you to post this message on X.</Typography>
    <Paper sx={{ my:"1em", p:"1em", background:"#eee"}}> {text} </Paper>
    { clicked ?
      <Alert>Thank you! It will take a moment for us to pick up your post.</Alert> :
      <Button fullWidth onClick={() => setClicked(true)} size="large" variant="contained" href={intent_url} target="_blank" rel="noopener noreferrer">
        Post the message
      </Button> 
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
  <Box pb="1em" id="existing-x-handle-stats">
    <RecordContextProvider value={handle}>
      <SimpleShowLayout>
        <TextField label="Username" source="username" />
        <TextField label="User ID" source="userId" />
        <FunctionField label="Price per repost" render={ record => `${formatEther(record.price)} DOC` }  />
        <FunctionField label="Score" render={ record => `${BigInt(handle.score)} ✭` }  />
      </SimpleShowLayout>
    </RecordContextProvider>
  </Box>

export default Dashboard;

