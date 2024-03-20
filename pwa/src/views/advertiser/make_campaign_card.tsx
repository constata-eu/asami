import { useEffect } from "react";
import { useDataProvider, useAuthenticated, useSafeSetState, useTranslate, useGetList} from "react-admin";
import LoadingButton from '@mui/lab/LoadingButton';
import { Chip, LinearProgress, Alert, AlertTitle, Box, Button, Card, CardActions, CardContent, Container, FormControl, FormHelperText, InputLabel, MenuItem, Select, Skeleton, Typography, IconButton } from "@mui/material";
import { Dialog, DialogContent, DialogTitle, DialogActions } from '@mui/material';
import { ethers, parseUnits, formatEther, toBeHex, zeroPadValue, parseEther } from "ethers";
import { LoggedInNavCard, ColumnsContainer, DeckCard } from '../layout';
import schnorr from "bip-schnorr";
import { Buffer } from "buffer";
import Login from "./views/login";
import { useContracts } from "../../components/contracts_context";
import { Head1, Head2, BulletPoint, CardTitle, light } from '../../components/theme';
import LoginIcon from '@mui/icons-material/Login';
import _ from 'lodash';
import { LocalizationProvider } from '@mui/x-date-pickers';
import { DateField } from '@mui/x-date-pickers/DateField';
import { AdapterDayjs } from '@mui/x-date-pickers/AdapterDayjs'
import dayjs from 'dayjs';
import { viewPostUrl, parseCampaignSiteAndContentId, defaultValidUntil } from '../../lib/campaign';
import { getAuthKeys } from '../../lib/auth_provider';
import { formatTxHash, formatAddress } from '../../lib/formatters';
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

import ContentCopyIcon from '@mui/icons-material/ContentCopy';
import LaunchIcon from '@mui/icons-material/Launch';
import TaskAltIcon from '@mui/icons-material/TaskAlt';
import { Stack } from '@mui/material';
import CampaignIcon from '@mui/icons-material/Campaign';
import CloseIcon from '@mui/icons-material/Close';
import ClaimAccountButton from '../claim_account';

export const MakeCampaignCard = ({account}) => {
  const translate = useTranslate();
  const [open, setOpen] = useSafeSetState(false);
  const [step, setStep] = useSafeSetState("FORM"); 
  const [approvalTx, setApprovalTx] = useSafeSetState();
  const [creationTx, setCreationTx] = useSafeSetState();
  const [error, setError] = useSafeSetState();
  const { contracts } = useContracts();

  const handleClose = () => {
    setOpen(false);
    setApprovalTx(null);
    setCreationTx(null);
    setError(null);
    setStep("FORM");
  }

  const onSubmit = async (values) => {
    setOpen(false)
    try {
      const { doc, asami, asamiAddress, signer } = await contracts(account.addr);

      setStep("APPROVING");
      setOpen(true)
      const input = values.makeCampaignInput;

      const approval = await doc.approve(asamiAddress, input.budget, signer);
      setApprovalTx(approval);
      await approval.wait();
      setStep("CREATING");

      const creation = await asami.makeCampaigns([input]);
      setCreationTx(creation);
      await creation.wait();
      setStep("DONE");
    } catch (e) {
      setError(e);
      setOpen(true)
      setStep("ERROR");
    }
  }

  const validate = (values) => {
    let errors = {};
    let keys = getAuthKeys();
    let input = {
      priceScoreRatio: BigInt(zeroPadValue(toBeHex(parseEther("0.001")), 32)),
      validUntil: BigInt(Math.floor(defaultValidUntil().getTime() / 1000)),
      topics: []
    };

    const {err, site, contentId} = parseCampaignSiteAndContentId(values.contentUrl);
    if (err) {
      return { contentUrl: translate(`make_campaign_card.errors.${err}`) };
    }
    input.site = { 'X': 0, 'INSTAGRAM': 1 }[site];
    input.contentId = contentId;

    try {
      const parsed = parseEther(values.budget);
      if( parsed <= parseEther("0") ) {
        return { budget: translate('make_campaign_card.errors.budget_too_low') };
      }
      input.budget = BigInt(zeroPadValue(toBeHex(parsed), 32));
    } catch {
      return { budget: translate('make_campaign_card.errors.budget_not_a_number') };
    }
    values.makeCampaignInput = input;
  }
  
  const hasDoc = BigInt(account.docBalance) > 0;

  return (
    <DeckCard>
      <CardContent>
        <Head2>{ translate("make_campaign_card.title") }</Head2>
        <Typography my="1em">{ translate("make_campaign_card.pay_members") }</Typography>
        { hasDoc ?
            <Button fullWidth variant="contained" size="large" id="open-start-campaign-dialog" onClick={ () => setOpen(true) }>
              <CampaignIcon sx={{mr:"5px"}}/>
              { translate("make_campaign_card.start_campaign") }
            </Button>
          :
            <>
              <Typography my="1em">{ translate("make_campaign_card.doc_funded") }</Typography>
              <Button fullWidth target="_blank" variant="outlined" href="https://moneyonchain.com/about-ipfs/">
                { translate("make_campaign_card.get_doc") }
              </Button>
            </>
        }
      </CardContent>
      <Dialog id="start-campaign-dialog" open={open} maxWidth="md" fullWidth>
        { step == "FORM" && <CampaignForm onSubmit={onSubmit} validate={validate} handleClose={handleClose} /> }
        { step == "APPROVING" &&
          <TxWaiter
            title="make_campaign_card.tx_waiter.approving_title"
            text="make_campaign_card.tx_waiter.approving_text"
            tx={approvalTx}
            id="approval-waiter"
          />
        }
        { step == "CREATING" &&
          <TxWaiter
            title="make_campaign_card.tx_waiter.creating_title"
            text="make_campaign_card.tx_waiter.creating_text"
            tx={creationTx}
            id="creation-waiter"
          />
        }
        { step == "DONE" && <Done tx={creationTx} handleClose={handleClose} /> }
        { step == "ERROR" && <Error error={error} handleClose={handleClose} /> }
      </Dialog>
    </DeckCard>);
}

const CampaignForm = ({onSubmit, validate, handleClose}) => {
  const translate = useTranslate();
  return <Box p="1em" id="campaign-form">
    <Typography>{ translate("make_campaign_card.form_step.service_summary") }</Typography>
    <Typography>{ translate("make_campaign_card.form_step.x_description") }</Typography>
    <Typography mb="1em">{ translate("make_campaign_card.form_step.ig_description") }</Typography>
    <Form sanitizeEmptyValues validate={validate} onSubmit={onSubmit}>
      <TextInput fullWidth required={true} size="large" variant="filled" source="contentUrl"
        label={ translate("make_campaign_card.form_step.content_url") } />
      <TextInput fullWidth required={true} size="large" variant="filled" source="budget"
        label={ translate("make_campaign_card.form_step.budget") } />
      <Stack gap="1em">
        <SaveButton fullWidth id="submit-start-campaign-form" size="large"
          label={ translate("make_campaign_card.form_step.start_campaign") } icon={<CampaignIcon/>} />
        <Button fullWidth size="large" color="inverted" onClick={handleClose}>
          { translate("make_campaign_card.close") }
        </Button>
      </Stack>
    </Form>
  </Box>;
}

const TxWaiter = ({title, text, tx, id}) => {
  const translate = useTranslate();
  return <Box p="1em" id={id}>
    <Head2>{ translate(title) }</Head2>
    <LinearProgress sx={{ my: "1em" }} />
    { !tx ? 
      <Typography>{ translate(text) }</Typography> :
      <>
        <Typography my="1em">{ translate("make_campaign_card.tx_waiter.waiting_for_tx") }</Typography>
        <Button fullWidth color="inverted" variant="outlined" startIcon={<LaunchIcon/>} href={ `https://explorer.rootstock.io/tx/${tx.hash}`} target="_blank">
          { translate("make_campaign_card.tx_waiter.see_in_explorer") }
        </Button>
      </>
    }
  </Box>
}

const Done = ({tx, handleClose}) => {
  const translate = useTranslate();

  return <Alert id="campaign-done" severity="success" sx={{ color: light }} variant="filled" icon={false}>
    <Head2>{ translate("make_campaign_card.done.title") }</Head2>
    <Typography my="1em">{ translate("make_campaign_card.done.text") }</Typography>
    <Button fullWidth color="inverted" variant="text" startIcon={<LaunchIcon/>} href={ `https://explorer.rootstock.io/tx/${tx.hash}`} target="_blank">
      { translate("make_campaign_card.done.see_in_explorer") }
    </Button>
    <Button id="campaign-done-close" sx={{ mt: "1em"}} fullWidth onClick={handleClose} color="inverted" variant="outlined">
      { translate("make_campaign_card.close") }
    </Button>
  </Alert>
}

const Error = ({error, handleClose}) => {
  const notify = useNotify();
  const translate = useTranslate();

  let msg;
  let info;

  if(error == "Modal closed by user" || error.code == "ACTION_REJECTED"){
    msg = translate("make_campaign_card.error_step.action_rejected");
  } else if (error.code == "WRONG_SIGNER") {
    msg = translate("make_campaign_card.error_step.wrong_signer", {expected: formatAddress(error.expected), actual: formatAddress(error.actual)});
  } else {
    msg = translate("make_campaign_card.error_step.unexpected_error");
    info = error.info && JSON.stringify(error.info, null, 2);
  }

  const copyText = async () => {
    notify("make_campaign_card.error_step.info_copied", { anchorOrigin: { vertical: 'top', horizontal: 'center' }});
    await navigator.clipboard.writeText(info);
  }

  return <Alert severity="error" variant="filled" icon={false}>
    <Head2 sx={{ mb: "0.5em" }}>
      { translate("make_campaign_card.error_step.title") }
    </Head2>
    {msg}
    { info && <Paper sx={{ mt: "1em", p: "0.5em" }}>
      <Typography component="pre" variant="body2" sx={{ whiteSpace:"break-spaces", lineBreak:"anywhere"}}>{info}</Typography>
      <Button sx={{ mt: "1em"}} fullWidth startIcon={<ContentCopyIcon/>} onClick={copyText} color="inverted">
        { translate("make_campaign_card.error_step.copy_info") }
      </Button>
      </Paper>
    }

    <Button sx={{ mt: "1em"}} fullWidth onClick={handleClose} color="inverted" variant="outlined">
      { translate("make_campaign_card.close") }
    </Button>
  </Alert>
}
