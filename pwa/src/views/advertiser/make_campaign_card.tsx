import { useSafeSetState, useTranslate } from "react-admin";
import { LinearProgress, Alert, Box, Button, CardContent, Typography } from "@mui/material";
import { Dialog } from '@mui/material';
import { toBeHex, zeroPadValue, parseEther } from "ethers";
import { DeckCard } from '../layout';
import { useContracts } from "../../components/contracts_context";
import { Head2, light } from '../../components/theme';
import { parseCampaignSiteAndContentId, defaultValidUntil } from '../../lib/campaign';
import { formatAddress } from '../../lib/formatters';
import Paper from '@mui/material/Paper';
import { Form, TextInput, SaveButton, useNotify } from 'react-admin';
import ContentCopyIcon from '@mui/icons-material/ContentCopy';
import LaunchIcon from '@mui/icons-material/Launch';
import { Stack } from '@mui/material';
import CampaignIcon from '@mui/icons-material/Campaign';

export const MakeCampaignCard = ({account}) => {
  const translate = useTranslate();
  const [open, setOpen] = useSafeSetState(false);
  const [step, setStep] = useSafeSetState("FORM"); 
  const [approvalTx, setApprovalTx] = useSafeSetState();
  const [creationTx, setCreationTx] = useSafeSetState();
  const [failure, setFailure] = useSafeSetState();
  const { contracts } = useContracts();

  const handleClose = () => {
    setOpen(false);
    setApprovalTx(null);
    setCreationTx(null);
    setFailure(null);
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
      setFailure(e);
      setOpen(true)
      setStep("ERROR");
    }
  }

  const validate = (values) => {
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
        { step == "ERROR" && <Failure failure={failure} handleClose={handleClose} /> }
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

const Failure = ({failure, handleClose}) => {
  const notify = useNotify();
  const translate = useTranslate();

  let msg;
  let info;

  if(failure == "Modal closed by user" || failure.code == "ACTION_REJECTED"){
    msg = translate("make_campaign_card.failure_step.action_rejected");
  } else if (failure.code == "WRONG_SIGNER") {
    msg = translate("make_campaign_card.failure_step.wrong_signer", {expected: formatAddress(failure.expected), actual: formatAddress(failure.actual)});
  } else {
    msg = translate("make_campaign_card.failure_step.unexpected_error");
    info = failure.info && JSON.stringify(failure.info, null, 2);
  }

  const copyText = async () => {
    notify("make_campaign_card.failure_step.info_copied", { anchorOrigin: { vertical: 'top', horizontal: 'center' }});
    await navigator.clipboard.writeText(info);
  }

  return <Alert severity="error" variant="filled" icon={false}>
    <Head2 sx={{ mb: "0.5em" }}>
      { translate("make_campaign_card.failure_step.title") }
    </Head2>
    {msg}
    { info && <Paper sx={{ mt: "1em", p: "0.5em" }}>
      <Typography component="pre" variant="body2" sx={{ whiteSpace:"break-spaces", lineBreak:"anywhere"}}>{info}</Typography>
      <Button sx={{ mt: "1em"}} fullWidth startIcon={<ContentCopyIcon/>} onClick={copyText} color="inverted">
        { translate("make_campaign_card.failure_step.copy_info") }
      </Button>
      </Paper>
    }

    <Button sx={{ mt: "1em"}} fullWidth onClick={handleClose} color="inverted" variant="outlined">
      { translate("make_campaign_card.close") }
    </Button>
  </Alert>
}
