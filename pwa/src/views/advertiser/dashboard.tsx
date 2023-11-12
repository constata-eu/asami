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

import { Stack } from '@mui/material';
import CampaignIcon from '@mui/icons-material/Campaign';
import CloseIcon from '@mui/icons-material/Close';
import { getAuthKeys } from '../../lib/auth_provider';

const Dashboard = () => {
  const [loading, setLoading] = useSafeSetState(true);
  const [campaigns, setCampaigns] = useSafeSetState(true);
  const [socialNetworks, setSocialNetworks] = useSafeSetState([]);
  const dataProvider = useDataProvider();

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

  return <Container maxWidth="md">
    <Head1 sx={{my:3}}>Hello Advertiser!</Head1>
    <Typography my="1em">
      Get the ASAMI community to repost your X content, just enter the URL of your X post and how much you want to spend.
      <br/>
      You'll be paying between 0.001 and 0.005 USD for each <strong>real person</strong> reached.
      <br/>
      Anyone will be able to participate reposting in the next 7 days.
      <br/> 
      Read the <a href="#">Asami community guide</a> to learn more about the process.
      <br/> 
      As you're an early adopter, we may be able to fund your campaign with no charge to you, just <a href="#">Contact us</a>.
    </Typography>

    <CreateCampaignRequest />
    <CampaignList />

  </Container>;
}

const CampaignList = () => {
  return <ListBase resource="CampaignRequest" disableSyncWithLocation>
    <Card sx={{my:"3em"}}>
      <CardTitle text="Requested campaigns" />
      <Datagrid bulkActionButtons={false}>
        <TextField source="status" />
        <TextField source="site" />
        <FunctionField label="Post" render={record => <a target="_blank" href={`https://x.com/twitter/status/${record.contentId}`}>See post</a>} />
        <FunctionField label="budget" render={record => `${formatEther(record.budget)} DOC` } />
        
      </Datagrid>
      <Pagination />
    </Card>
  </ListBase>;
}

const CreateCampaignRequest = () => {
  const [open, setOpen] = useSafeSetState(false);

  const notify = useNotify();
  const handleClose = () => setOpen(false);
  const onSuccess = () => {
    notify("Campaign will be started soon");
    handleClose();
  }

  const transformIt = async (values, a, b, c) => {
    return { input: values.campaignRequestInput };
  }

  const validate = (values) => {
    let errors = {};
    let keys = getAuthKeys();
    let input = { accountId: getAuthKeys().session.accountIds[0], site: "X"};

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

    input.priceScoreRatio = zeroPadValue(toBeHex(parseEther("0.001")), 32);

    let currentDate = new Date();
    currentDate.setTime(currentDate.getTime() + (7 * 24 * 60 * 60 * 1000));
    input.validUntil = currentDate.toISOString();

    values.campaignRequestInput = input;
    return errors;
  }
  
  return (<Box>
    <Button fullWidth variant="contained" size="large" onClick={ () => setOpen(true) }>
      <CampaignIcon sx={{mr:"5px"}}/>
      Start Campaign
    </Button>
    <CreateBase resource="CampaignRequest" transform={transformIt} mutationOptions={{ onSuccess }} >
      <Dialog open={open} onClose={handleClose} maxWidth="md" fullWidth>
        <Box p="0 1em">
          <SimpleForm sanitizeEmptyValues validate={validate} toolbar={false}>
            <TextInput fullWidth required={true} size="large" variant="filled" source="contentUrl" label="Your post URL" />
            <TextInput fullWidth required={true} size="large" variant="filled" source="budget" label="How much you have to spend, in DoC (USD)" />
            <Box width="100%" display="flex" gap="1em" justifyContent="space-between">
              <SaveButton size="large" label="Start Campaign" icon={<CampaignIcon/>} />
              <Button size="large" variant="contained" color="grey" onClick={handleClose}>Cancel</Button>
            </Box>
          </SimpleForm>
        </Box>
      </Dialog>
    </CreateBase>
  </Box>);
}

export default Dashboard;

/*
  useEffect(() => {
    const load = async () => {
    
      const { asami, signer } = await useContracts();
      const all = await asami.getCampaigns();
      setCampaigns(_.filter(all, (x) => x.advertiser == signer.address ));
      setSocialNetworks(await asami.getSocialNetworks());
      setLoading(false);
    }
    load();
  }, []);
*/
