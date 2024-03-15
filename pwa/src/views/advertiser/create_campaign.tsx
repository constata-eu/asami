import { useEffect } from "react";
import { useDataProvider, useAuthenticated, useSafeSetState, useTranslate, useGetList} from "react-admin";
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

const CreateCampaign = ({onSave}) => {
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
    currentDate.setTime(currentDate.getTime() + (30 * 24 * 60 * 60 * 1000));
    return currentDate;
  }

  const onSubmit = async (values) => {
    if (data?.[0]?.status == "DONE") {
      const { doc, asami, asamiAddress, signer } = await contracts();
      const input = values.campaignRequestInput;
      const budget = BigInt(input.budget);
      const site = { 'X': 0, 'NOSTR': 1, 'INSTAGRAM': 2 }[input.site];
      const approval = await doc.approve(asamiAddress, budget, signer);
      await approval.wait();
      notify("Campaign budget approved.");
      
      const creation = await asami.makeCampaigns([
        { site,
          budget: BigInt(input.budget),
          contentId: input.contentId,
          priceScoreRatio: BigInt(input.priceScoreRatio),
          topics: input.topicIds,
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
    let input = { accountId: getAuthKeys().session.accountId};

    try {
      const u = new URL(values.contentUrl);
      const path = u.pathname.replace(/\/$/, '').split("/");
      const contentId = path[path.length - 1];

      if ( (u.host.match(/\.?x\.com$/) || u.host.match(/\.?twitter\.com#/)) && contentId.match(/^\d+$/) ) {
        input.site = "X";
      } else if (u.host.match(/\.?instagram.com$/) && contentId.match(/^[\d\w\-_]+$/)) {
        input.site = "INSTAGRAM";
      } else {
        errors.contentUrl = "The URL does not seem to be for an X nor Instagram post.";
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
    input.validUntil = defaultValidUntil().toISOString();
    input.topicIds = [];

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
          <TextInput fullWidth required={true} size="large" variant="filled" source="contentUrl" label="Your Instagram or X post URL" />
          <TextInput fullWidth required={true} size="large" variant="filled" source="budget" label="How much to spend, in DoC (USD)" />
          <Box width="100%" display="flex" gap="1em" justifyContent="space-between">
            <SaveButton id="submit-start-campaign-form" size="large" label="Start Campaign" icon={<CampaignIcon/>} />
            <Button size="large" variant="contained" color="grey" onClick={handleClose}>Cancel</Button>
          </Box>
        </Form>
      </Box>
    </Dialog>
  </Box>);
}

const OnChainCampaign = ({handleClose}) => {
  const [step, setStep] = useSafeSetState("FORM");
  return <>
    { state == "FORM" && <CampaignForm handleClose={handleClose} validate={validate} onSubmit={onSubmit} /> }
    { state == "SENDING" && <Typography>Sending request</Typography> }
    { state == "SENT" && <Typography>Your request for funding a campaign has been received.</Typography> }
  }
}

const CampaignRequest = ({values}) => {

}

const CampaignForm = ({handleClose, validate, onSubmit}) => {
  return <Form sanitizeEmptyValues validate={validate} onSubmit={onSubmit}>
    <TextInput fullWidth required={true} size="large" variant="filled" source="contentUrl" label="Your Instagram or X post URL" />
    <TextInput fullWidth required={true} size="large" variant="filled" source="budget" label="How much to spend, in DoC (USD)" />
    <Box width="100%" display="flex" gap="1em" justifyContent="space-between">
      <SaveButton id="submit-start-campaign-form" size="large" label="Start Campaign" icon={<CampaignIcon/>} />
      <Button size="large" variant="contained" color="grey" onClick={handleClose}>Cancel</Button>
    </Box>
  </Form>;
}

export default CreateCampaign;
