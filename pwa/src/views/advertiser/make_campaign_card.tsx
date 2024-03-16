import { useEffect } from "react";
import { useDataProvider, useAuthenticated, useSafeSetState, useTranslate, useGetList} from "react-admin";
import LoadingButton from '@mui/lab/LoadingButton';
import { Chip, LinearProgress, Alert, Box, Button, Card, CardActions, CardContent, Container, FormControl, FormHelperText, InputLabel, MenuItem, Select, Skeleton, Typography, IconButton } from "@mui/material";
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
import { viewPostUrl, parseCampaignSiteAndContentId, defaultValidUntil } from '../../lib/campaign';
import { getAuthKeys } from '../../lib/auth_provider';
import { formatTxHash } from '../../lib/formatters';
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
import ClaimAccountButton from '../claim_account';

export const MakeCampaignCard = () => {
  const [open, setOpen] = useSafeSetState(false);
  const [step, setStep] = useSafeSetState("FORM"); // APPROVING, CREATING, DONE, ERROR
  const [approvalTx, setApprovalTx] = useSafeSetState();
  const [creationTx, setCreationTx] = useSafeSetState();
  const { contracts } = useContracts();

  const handleClose = () => setOpen(false);
  const setError = (e) => {
    console.log(e);
      /*  e.code == 'ACTION_REJECTED'
 *        e.info.payload
 *        e.info.code
      */
    setStep("ERROR");
  }

  const onSubmit = async (values) => {
    setOpen(false)
    const { doc, asami, asamiAddress, signer } = await contracts();
    setStep("APPROVING");
    setOpen(true)
    const input = values.makeCampaignInput;
    try {
      const approval = await doc.approve(asamiAddress, input.budget, signer);
      console.log(approval); // Approval tx changes the text of the approving msg.
      setApprovalTx(approval);
      await approval.wait();
      setStep("CREATING");
    } catch (e) {
      return setError(e);
    }
    /*

    try {
      const creation = await asami.makeCampaigns([input]);
      console.log(creation); // Creation tx changes the text of the msg.
      // Here we may add the local storage peding tx.
      await creation.wait();
      setStep("DONE");
    } catch (e) {
      return setError(e);
    }
    */
  }

  const validate = (values) => {
    let errors = {};
    let keys = getAuthKeys();
    let input = {
      priceScoreRatio: BigInt(zeroPadValue(toBeHex(parseEther("0.001")), 32)),
      validUntil: BigInt(Math.floor(defaultValidUntil().getTime() / 1000)),
      topicIds: []
    };

    const {err, site, contentId} = parseCampaignSiteAndContentId(values.contentUrl);
    if (err) {
      return { contentUrl: translate(`make_campaign_card.errors.${err}`) };
    }
    input.site = { 'X': 0, 'INSTAGRAM': 1 }[site];
    input.contentId = contentId;

    try {
      const parsed = parseEther(values.budget);
      if( parsed <= parseEther("1") ) {
        return { budget: translate('make_campaign_card.errors.budget_too_low') };
      }
      input.budget = BigInt(zeroPadValue(toBeHex(parsed), 32));
    } catch {
      return { budget: translate('make_campaign_card.errors.budget_not_a_number') };
    }

    values.makeCampaignInput = input;
  }
  
  return (
    <DeckCard>
      <CardContent>
        <Head2>Start a campaign!</Head2>
        <Typography my="1em">
          Pay club members to repost any X post, or to clone any Instagram post.
        </Typography>
        <Typography my="1em">
          Campaigns are funded in DOC, a USD pegged stablecoin.
        </Typography>
        <Button sx={{mb: "1em"}} fullWidth variant="outlined" color="inverted" href="https://moneyonchain.com/about-ipfs/">Get DOC here</Button>
        <Button fullWidth variant="contained" size="large" id="open-start-campaign-dialog" onClick={ () => setOpen(true) }>
          <CampaignIcon sx={{mr:"5px"}}/>
          Start Campaign
        </Button>
      </CardContent>
      <Dialog id="start-campaign-dialog" open={open} onClose={handleClose} maxWidth="md" fullWidth>
        <Box p="1em">
          { step == "FORM" && <CampaignForm onSubmit={onSubmit} validate={validate} handleClose={handleClose} /> }
          { step == "APPROVING" && <Approving tx={approvalTx} /> }
          { step == "CREATING" && <Typography>Sending the tx that creates your campaign.</Typography> }
          { step == "DONE" && <Typography>Your campaign was created, it will show on your dashboard in a few minutes.</Typography> }
          { step == "ERROR" && <Typography>Oops, we messed up.</Typography> }
        </Box>
      </Dialog>
    </DeckCard>);
}

const CampaignForm = ({onSubmit, validate, handleClose}) => {
  return <>
    <Typography mb="1em">
      Just enter the URL of an <strong>X or Instagram</strong> post and how much you want to spend.
      <br/>
      Members will get paid according to the size and quality of their audience, which we measure in "sparkles".  
      <br/>
      Each sparkle rouglhy equates to a real person being ifluenced by their repost. You'll be paying 0.001 DOC per sparkle.
      <br/>
      <strong>On X</strong> members will hit the "repost" button of your post.
      <br/>
      <strong>On Instagram</strong> members will clone your original post image and caption, and post it as their own. Make sure to use hashtags!
      <br/>
      Your campaign will be up for 30 days, any funds left will be returned to you.
    </Typography>
    <Form sanitizeEmptyValues validate={validate} onSubmit={onSubmit}>
      <TextInput fullWidth required={true} size="large" variant="filled" source="contentUrl" label="Your Instagram or X post URL" />
      <TextInput fullWidth required={true} size="large" variant="filled" source="budget" label="How much to spend, in DoC (USD)" />
      <Box width="100%" display="flex" gap="1em" justifyContent="space-between">
        <SaveButton id="submit-start-campaign-form" size="large" label="Start Campaign" icon={<CampaignIcon/>} />
        <Button size="large" variant="contained" color="grey" onClick={handleClose}>Cancel</Button>
      </Box>
    </Form>
  </>;
}

const Approving = ({tx}) => {
  return <>
    <Head2>Approving Campaign budget</Head2>
    <LinearProgress sx={{ my: "1em" }} />
    { !tx ? 
      <Typography>Go to your wallet and approve spending your campaign's budget.</Typography> :
      <>
        <Typography my="1em">
          Waiting for approval transaction to confirm
        </Typography>
        <Chip label={formatTxHash(tx.hash)} />
        <Button fullWidth color="inverted" variant="outlined" href={ `https://explorer.rootstock.io/tx/${tx.hash}`} target="_blank">
          See transaction in block explorer
        </Button>
      </>
    }
  </>
}

const Creating = ({tx}) => {
  return <>
    <Head2>Create Campaign</Head2>
    <LinearProgress sx={{ my: "1em" }} />
    { !tx ? 
      <Typography>Go to your wallet and sign the transaction that will create your campaign in ASAMI.</Typography> :
      <>
        <Typography my="1em">
          Waiting for approval transaction to confirm
        </Typography>
        <Chip label={formatTxHash(tx.hash)} />
        <Button fullWidth color="inverted" variant="outlined" href={ `https://explorer.rootstock.io/tx/${tx.hash}`} target="_blank">
          See transaction in block explorer
        </Button>
      </>
    }
  </>
}
