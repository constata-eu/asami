import { useEffect } from "react";
import { useDataProvider, useAuthenticated, useSafeSetState, useTranslate, ReferenceField, useGetAll, useGetOne, useGetList} from "react-admin";
import { rLogin } from "../../lib/rLogin";
import LoadingButton from '@mui/lab/LoadingButton';
import { viewPostUrl } from '../../lib/campaign';
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
import { Settings } from '../../settings';

import Table from '@mui/material/Table';
import TableBody from '@mui/material/TableBody';
import TableCell from '@mui/material/TableCell';
import TableContainer from '@mui/material/TableContainer';
import TableHead from '@mui/material/TableHead';
import TableRow from '@mui/material/TableRow';
import Paper from '@mui/material/Paper';
import { Toolbar, Create, Confirm, SimpleForm, CreateBase, Form, TextInput, RichTextInput, SaveButton, useNotify } from 'react-admin';
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

import InstagramIcon from '@mui/icons-material/Instagram';

import asamiABI from "../../abi/Asami.json";
import docABI from "../../abi/Doc.json";

const Dashboard = () => {
  useAuthenticated();

  const [loading, setLoading] = useSafeSetState(true);
  const [needsRefresh, setNeedsRefresh] = useSafeSetState(false);

  const handles = useListController({
    disableSyncWithLocation: true,
    filter: {accountIdEq: getAuthKeys().session.accountId},
    queryOptions: {
      refetchInterval: 2000,
    },
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
    <ConfigureInstagramAccount handles={handles}/>
  </Container>;
}

const AccountState = () => {
  const {data, isLoading} = useGetList(
    "ClaimAccountRequest",
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
    <SimpleShowLayout record={balances} sx={{ ".RaSimpleShowLayout-stack": {gap: "2em", display: "flex", flexDirection: "row"} ,".ra-field": {flex: "1 1"}}}>
      <FunctionField label="ASAMI token balance" render={ record => `${formatEther(record.asami)}` }  />
      <FunctionField label="DOC balance" render={ record => `${formatEther(record.doc)}` }  />
    </SimpleShowLayout>
  </Box>;
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

  if (prefsContext.isLoading || listContext.isLoading || handles.isLoading || handles.total == 0 ){
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

  const setPreference = async (campaignId, notInterested, attempted) => {
    await dataProvider.create('CampaignPreference', { data: { input: {campaignId, notInterested, attempted} } });
    await listContext.refetch();
    if(attempted) {
      await prefsContext.refetch();
    }
  };

  return ( <>
    <Head1 sx={{ my: "1em" }}>Check out these campaigns</Head1>
    <Box id="campaign-list" display="flex" flexWrap="wrap" gap="2em">
      { listContext.data.map((item) => item.site == "X" ?
        <XCampaign key={item.id} item={item} prefsContext={prefsContext} setPreference={setPreference} /> :
        <InstagramCampaign key={item.id} item={item} prefsContext={prefsContext} setPreference={setPreference} />
      )}
    </Box>
    </>
  );
}

const XCampaign = ({item, prefsContext, setPreference}) => {
  const attemptedOn = prefsContext.data.find((x) => x.campaignId == item.id)?.attemptedOn;

  return <Card sx={{ flex: "1 1 400px", p: "1em" }} key={item.id} id={`campaign-container-${item.id}`}>
    <TwitterTweetEmbed tweetId={item.contentId} options={{ align: "center", width: "550px", conversation: "none"}} />
    <RepostXCampaign attemptedOn={attemptedOn} campaignId={item.id} contentId={item.contentId} onAttempt={(id) => setPreference(id, false, true) } />
    <ConfirmHideCampaign campaignId={item.id} onConfirm={(id) => setPreference(id, true, false) } />
  </Card>;
}

const InstagramCampaign = ({item, prefsContext, setPreference}) => {
  const notify = useNotify();
  const {data, isLoading} = useGetList(
    "IgCampaignRule",
    { filter: {campaignIdEq: item.id}, perPage: 1,}
  );

  if (isLoading || !data[0]) {
    return null;
  }

  const dataUri = "data:image/jpeg;base64,"+data[0].image;
  const filename = `campaign_${data[0].campaignId}.jpg`;
  const copyText = async () => {
    notify("Caption copied");
    await navigator.clipboard.writeText(data[0].caption);
  }

  const attemptedOn = prefsContext.data.find((x) => x.campaignId == item.id)?.attemptedOn;

  return <Card sx={{ flex: "1 1 400px"}} key={item.id} id={`campaign-container-${item.id}`}>
    <CardTitle text="Post this to Instagram.">
      And keep it there for at least two weeks.
    </CardTitle>
    <CardContent>
      <Box display="flex" flexDirection="column" alignItems="center">
        <a href={ dataUri } target="_blank" download={filename}>
          <img style={{maxWidth: "100%", maxHeight: "400px"}} src={dataUri} />
        </a>
        { !!data[0].caption && <Paper elevation={5} sx={{p:"1em", my:"1em"}}><Typography>{ data[0].caption }</Typography></Paper> }
      </Box>
      
      <Button sx={{ mb: "1em" }} fullWidth size="large" variant="contained" target="_blank" href={ dataUri } rel="noopener noreferrer" download={filename}>
        Download the image
      </Button> 
      { !!data[0].caption &&
        <Button fullWidth size="large" onClick={() => copyText() } variant="contained" >
          Copy the text
        </Button> 
      }
      <ConfirmHideCampaign campaignId={item.id} onConfirm={(id) => setPreference(id, true, false) } />
    </CardContent>
  </Card>;
}

const RepostXCampaign = ({campaignId, contentId, attemptedOn, onAttempt}) => {
  const repostUrl = `https://twitter.com/intent/retweet?tweet_id=${contentId}&related=asami_club`;

  if (attemptedOn) {
    return <>
      <Alert id={`alert-repost-attempted-${campaignId}`} severity="info" sx={{mb: "0.5em"}}
        action={
          <Button color="info" fullWidth size="small" href={repostUrl} target="_blank" >
            Try again!
          </Button>
        }
      >
        You tried to repost this
      </Alert>
    </>;
  } else {
    return <Button
      id={`button-repost-${campaignId}`}
      sx={{mb: "0.5em" }}
      onClick={() => onAttempt(campaignId)}
      fullWidth
      size="large"
      variant="contained"
      href={repostUrl}
      target="_blank"
    >
      Repost it!
    </Button>;
  }
}

const ConfirmHideCampaign = ({campaignId, onConfirm }) => {
  const [open, setOpen] = useSafeSetState(false);
  const [hide, setHide] = useSafeSetState(false);
  
  const handleConfirm = () => {
    onConfirm(campaignId);
    setOpen(false);
    setHide(true);
  }

  if (hide) { return null; }

  return <>
    <Button fullWidth id={`open-hide-campaign-${campaignId}`} size="small" variant="outlined" onClick={() => setOpen(true) } >
      Not interested
    </Button>
    <Confirm
      isOpen={open}
      title="Hiding this campaign"
      content="We won't show this campaign again, this cannot be undone."
      onConfirm={handleConfirm}
      onClose={() => setOpen(false)}
    />
  </>;
}

const CollabList = () => {
  const listContext = useListController({
    disableSyncWithLocation: true,
    filter: {memberIdEq: getAuthKeys().session.accountId },
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
            <FunctionField label={false} render={record => <a target="_blank" href={viewPostUrl(record)}>See post</a>} />
          </ReferenceField>
          <FunctionField label="Reward" render={record => `${formatEther(record.gross)} DOC`} />
          <FunctionField label="Asami Fee" render={record => `${formatEther(record.fee)} DOC`} />
        </Datagrid>
        <Pagination rowsPerPageOptions={[]} />
      </Card>
    </ListContextProvider>
  );
}

const ConfigureInstagramAccount = ({handles}) => {
  const [needsRefresh, setNeedsRefresh] = useSafeSetState(false);

  const reqs = useListController({
    disableSyncWithLocation: true,
    filter: {siteEq: "INSTAGRAM"},
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
  const handle = handles.data?.filter((x) => x.site == "INSTAGRAM")[0];

  if (handles.isLoading || reqs.isLoading ){
    content = (<>
      <Skeleton />
      <Skeleton />
    </>);
  } else if (handle) {
    content = <InstagramHandleStats handle={handle} />;
  } else {
    const req = reqs.data[0];
    if (req) {
      if (req.status == "UNVERIFIED") {
        content = <MakeInstagramVerificationPost />;
      } else if (req.status != "DONE" ) {
        content = <InstagramHandleSubmissionInProgress req={req} />;
      }
    } else {
      content = <CreateInstagramHandleRequest {...{setNeedsRefresh}} />;
    }
  }

  return (<Box>
    <Card id="configure-instagram-handle-card" sx={{my:"3em"}}>
      <CardTitle text="Your Instagram" />
      { content }
    </Card>
  </Box>);
}

const CreateInstagramHandleRequest = ({setNeedsRefresh}) => {
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
    let input = { site: "INSTAGRAM"};

    if ( values.ig_username.match(/^(\w){1,15}$/) ) {
      input.username = values.ig_username.replace("@","");
    } else {
      errors.ig_username = "That does not seem to be an instagram username. It should be something like 'mark_zuck'";
    }

    values.handleRequestInput = input;
    return errors;
  }

  return <CreateBase resource="HandleRequest" transform={transformIt} mutationOptions={{ onSuccess }} >
    <SimpleForm sanitizeEmptyValues validate={validate} toolbar={false}>
      <Typography mb="1em">Let's set up your Instagram account in ASAMI so you can start getting paid for your reposts.</Typography>
      <TextInput fullWidth required={true} size="large" variant="filled" source="ig_username" label="Tell us your instagram username." />
      <SaveButton fullWidth id="submit-instagram-handle-request-form" size="large" label="Continue" icon={<></>} />
    </SimpleForm>
  </CreateBase>;
}

const MakeInstagramVerificationPost = () => {
  const notify = useNotify();
  const [config, setConfig] = useSafeSetState(null);
  let accountId = BigInt(getAuthKeys().session.accountId);

  useEffect(() => {
    const load = async () => {
      setConfig((await (await fetch(`${Settings.apiDomain}/config`)).json()));
    }
    load();
  }, []);

  if (!config) {
    return <>
      <Skeleton />
      <Skeleton />
    </>;
  }

  const caption = `${config.instagram_verification_caption} [${accountId}]`;

  const copyText = async () => {
    notify("Caption copied");
    await navigator.clipboard.writeText(caption);
  }

  return <Box p="1em">
    <Typography mb="0.5em">
      Now, to verify your username, and your willingness to use it in ASAMI, we need you to post this image, with the given caption.
    </Typography>
    <Typography mb="0.5em">
      Do not apply any filters. You can add more text after the caption if you want, but don't change it.
    </Typography>
    <Typography mb="0.5em">
      Well look for it in your posts, <strong>not in your stories</strong>.
    </Typography>
    <Box>
      <Box display="flex" flexWrap="wrap" gap="1em" mb="1em">
        <Box justifyContent="center" display="flex" >
          <a href={config.instagram_verification_image_url} target="_blank" download>
            <img style={{maxWidth: "100%", maxHeight: "400px"}} src={config.instagram_verification_image_url} />
          </a>
        </Box>
        <Paper elevation={5} sx={{p:"1em", flex:"1 0 300px", alignSelf: "center"}}><Typography>{ caption }</Typography></Paper>
      </Box>
      <Box display="flex" gap="1em" mb="1em">
        <Button fullWidth size="large" variant="contained" target="_blank" href={config.instagram_verification_image_url} rel="noopener noreferrer" download>
          Get image
        </Button> 
        <Button fullWidth size="large" onClick={() => copyText() } variant="contained" >
          Copy text
        </Button> 
      </Box>
    </Box>
  </Box>
}

const InstagramHandleSubmissionInProgress = ({req}) => 
  <Box p="1em" id="handle-instagram-submission-in-progress-message">
    <Typography>
      We've verified {req.username}, and we're in process of publishing it to the blockchain.
      Once we're done you can start participating in campaigns.
      This shouldn't take long.
    </Typography>
  </Box>

const InstagramHandleStats = ({handle}) => 
  <Box pb="1em" id="existing-instagram-handle-stats">
    <RecordContextProvider value={handle}>
      <SimpleShowLayout>
        <TextField label="Username" source="username" />
        <TextField label="User ID" source="userId" />
        <FunctionField label="Price per repost" render={ record => `${formatEther(record.price)} DOC` }  />
        <FunctionField label="Score" render={ record => `${BigInt(handle.score)} ✭` }  />
      </SimpleShowLayout>
    </RecordContextProvider>
  </Box>


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
  const handle = handles.data?.filter((x) => x.site == "X")[0];

  if (handles.isLoading || reqs.isLoading ){
    content = (<>
      <Skeleton />
      <Skeleton />
    </>);
  } else if (handle) {
    content = <XHandleStats handle={handle} />;
  } else {
    const req = reqs.data[0];
    if (req) {
      if (req.status == "UNVERIFIED") {
        content = <MakeXVerificationPost />;
      } else if (req.status != "DONE" ) {
        content = <XHandleSubmissionInProgress req={req} />;
      }
    } else {
      content = <CreateXHandleRequest {...{setNeedsRefresh}} />;
    }
  }

  return (<Box>
    <Card id="configure-x-handle-card" sx={{my:"3em"}}>
      <CardTitle text="Your 𝕏" />
      { content }
    </Card>
  </Box>);
}

const CreateXHandleRequest = ({setNeedsRefresh}) => {
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
    let input = { site: "X"};

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
      <SaveButton fullWidth id="submit-x-handle-request-form" size="large" label="Continue" icon={<></>} />
    </SimpleForm>
  </CreateBase>;
}

const MakeXVerificationPost = () => {
  const [clicked, setClicked] = useSafeSetState(false);

  let accountId = BigInt(getAuthKeys().session.accountId);
  let text = `Starting today, some of my reposts will be paid for, as I joined @asami_club [${accountId}].`;
  let intent_url = `https://x.com/intent/tweet?text=${text}`;

  return <Box p="1em">
    <Typography mb="1em">Now, to verify your username, and your willingness to use it in ASAMI, we need you to post this message on X.</Typography>
    <Paper elevation={5} sx={{ my:"1em", p:"1em"}}> {text} </Paper>
    { clicked ?
      <Alert>Thank you! It will take a moment for us to pick up your post.</Alert> :
      <Button fullWidth onClick={() => setClicked(true)} size="large" variant="contained" href={intent_url} target="_blank" rel="noopener noreferrer">
        Post the message
      </Button> 
    }
  </Box>
}

const XHandleSubmissionInProgress = ({req}) => 
  <Box p="1em" id="handle-submission-in-progress-message">
    <Typography>
      We've verified {req.username}, and we're in process of publishing it to the blockchain.
      Once we're done you can start participating in campaigns.
      This shouldn't take long.
    </Typography>
  </Box>

const XHandleStats = ({handle}) => 
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

