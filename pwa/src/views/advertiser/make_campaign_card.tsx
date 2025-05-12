import {
  useSafeSetState,
  useDataProvider,
  useTranslate,
  ReferenceArrayInput,
  AutocompleteArrayInput,
} from "react-admin";

import {
  LinearProgress,
  Alert,
  Box,
  Button,
  CardContent,
  Typography,
} from "@mui/material";
import { Dialog } from "@mui/material";
import { formatAddress } from "../../lib/formatters";
import { toBeHex, zeroPadValue, parseEther } from "ethers";
import { DeckCard } from "../layout";
import { useContracts } from "../../components/contracts_context";
import { Head2, Head3, light } from "../../components/theme";
import { validateCampaignLink, defaultValidUntil } from "../../lib/campaign";
import Paper from "@mui/material/Paper";
import { Form, TextInput, SaveButton, useNotify } from "react-admin";
import ContentCopyIcon from "@mui/icons-material/ContentCopy";
import LaunchIcon from "@mui/icons-material/Launch";
import { Stack } from "@mui/material";
import CampaignIcon from "@mui/icons-material/Campaign";
import ClaimAccountButton from "../claim_account";
import HourglassTopIcon from "@mui/icons-material/HourglassTop";
import RemoveCircleOutlineIcon from "@mui/icons-material/RemoveCircleOutline";

export const MakeCampaignWithDocCard = ({ account, onCreate }) => {
  const t = useTranslate();
  const status = account.status;

  return (
    <DeckCard>
      <CardContent>
        <Head2>{t("make_campaign.with_doc.title")}</Head2>
        {status == "MANAGED" && <ClaimAccountToUseDoc />}
        {status == "CLAIMING" && <ClaimInProgress />}
        {status == "BANNED" && <Banned />}
        {status == "CLAIMED" && (
          <MakeCampaignWithDocDialog account={account} onCreate={onCreate} />
        )}
      </CardContent>
    </DeckCard>
  );
};

const ClaimAccountToUseDoc = () => {
  const t = useTranslate();
  return (
    <>
      <Typography my="1em">
        {t("make_campaign.with_doc.claim_to_use")}
      </Typography>
      <ClaimAccountButton
        id="advertiser-claim-account-button"
        color="secondary"
        variant="outlined"
        label={t("make_campaign.with_doc.claim_to_use_button")}
      />
    </>
  );
};

const ClaimInProgress = () => {
  const t = useTranslate();
  return (
    <Stack mt="1em" gap="1em" alignItems="center">
      <HourglassTopIcon sx={{ fontSize: "7em", color: "secondary.main" }} />
      <Typography>{t("make_campaign.with_doc.claim_in_progress")}</Typography>
    </Stack>
  );
};

const Banned = () => (
  <Stack alignItems="center">
    <RemoveCircleOutlineIcon
      sx={{
        fontSize: "7em",
        my: "1rem",
        color: "error.main",
      }}
    />
  </Stack>
);

const MakeCampaignWithDocDialog = ({ account, onCreate }) => {
  const translate = useTranslate();
  const [open, setOpen] = useSafeSetState(false);
  const [step, setStep] = useSafeSetState("FORM");
  const [approvalTx, setApprovalTx] = useSafeSetState();
  const [creationTx, setCreationTx] = useSafeSetState();
  const [failure, setFailure] = useSafeSetState();
  const { contracts } = useContracts();
  const dataProvider = useDataProvider();

  const handleClose = () => {
    setOpen(false);
    setApprovalTx(null);
    setCreationTx(null);
    setFailure(null);
    setStep("FORM");
  };

  const onSubmit = async (values) => {
    setOpen(false);
    try {
      const { doc, asami, asamiAddress, signer } = await contracts(
        account.addr,
      );

      setStep("APPROVING");
      setOpen(true);
      const input = values.makeCampaignInput;

      const campaign = await dataProvider.create("CreateCampaignFromLink", {
        data: {
          input: {
            link: input.link,
            pricePerPoint: toBeHex(input.pricePerPoint, 32),
            maxIndividualReward: toBeHex(input.maxIndividualReward, 32),
            minIndividualReward: toBeHex(input.minIndividualReward, 32),
            topicIds: input.topicIds,
          },
        },
      });

      const allowance = await doc.allowance(signer, asamiAddress);
      if (allowance < input.budget) {
        const approval = await doc.approve(asamiAddress, input.budget, signer);
        setApprovalTx(approval);
        await approval.wait();
      }

      setStep("CREATING");

      let time = new Date().getTime() + input.duration * 24 * 60 * 60 * 1000;

      const creation = await asami.makeCampaigns([
        {
          budget: input.budget,
          briefingHash: campaign.data.briefingHash,
          validUntil: BigInt(Math.floor(time / 1000)),
        },
      ]);

      setCreationTx(creation);
      await creation.wait();
      setStep("DONE");
      await dataProvider.update("Campaign", { id: campaign.data.id, data: {} });
      onCreate();
    } catch (e) {
      setFailure(e);
      setOpen(true);
      setStep("ERROR");
    }
  };

  const parseNumber = (value, min_allowed, too_low_msg, nan_message) => {
    try {
      const parsed = parseEther(value);
      if (parsed < parseEther(min_allowed)) {
        return {
          error: translate(`make_campaign.errors.${too_low_message}`),
        };
      }
      return { ok: BigInt(zeroPadValue(toBeHex(parsed), 32)) };
    } catch {
      return { error: translate(`make_campaign.errors.${nan_message}`) };
    }
  };

  const validate = (values) => {
    let input = {
      duration: 15,
    };

    const error = validateCampaignLink(values.contentUrl);
    if (error) {
      return { contentUrl: translate(`make_campaign.errors.${error}`) };
    }
    input.link = values.contentUrl;
    input.topicIds = values.topic_ids;
    input.pricePerPoint = values.pricePerPoint;

    const budget = parseNumber(
      values.budget,
      "1",
      "budget_too_low",
      "budget_not_a_number",
    );
    if (budget.error) {
      return { budget: budget.error };
    }
    input.budget = budget.ok;

    const ppp = parseNumber(
      values.pricePerPoint,
      "0.001",
      "price_per_point_too_low",
      "price_per_point_not_a_number",
    );
    if (ppp.error) {
      return { pricePerPoint: ppp.error };
    }
    input.pricePerPoint = ppp.ok;

    const max = parseNumber(
      values.maxIndividualReward,
      "0",
      "max_too_low",
      "max_not_a_number",
    );
    if (max.error) {
      return { maxIndividualReward: max.error };
    }
    input.maxIndividualReward = max.ok;

    const min = parseNumber(
      values.minIndividualReward,
      "0",
      "min_too_low",
      "min_not_a_number",
    );
    if (min.error) {
      return { minIndividualReward: min.error };
    }
    input.minIndividualReward = min.ok;

    values.makeCampaignInput = input;
  };

  const hasDoc = BigInt(account.docBalance) >= parseEther("1");

  return (
    <Box id="make-campaign-with-doc-dialog-container">
      <Typography my="1em">
        {translate("make_campaign.with_doc.create_text")}
      </Typography>
      {hasDoc ? (
        <Button
          fullWidth
          variant="contained"
          size="large"
          id="open-start-campaign-dialog"
          onClick={() => setOpen(true)}
        >
          <CampaignIcon sx={{ mr: "5px" }} />
          {translate("make_campaign.with_doc.create_button")}
        </Button>
      ) : (
        <>
          <Typography my="1em">
            {translate("make_campaign.with_doc.doc_funded")}
          </Typography>
          <Button
            fullWidth
            target="_blank"
            variant="outlined"
            href="https://wiki.moneyonchain.com/espanol-1/comenzando/que-es-lo-primero-que-debo-saber"
          >
            {translate("make_campaign.with_doc.get_doc")}
          </Button>
        </>
      )}
      <Dialog
        id="start-campaign-with-doc-dialog"
        open={open}
        maxWidth="md"
        fullWidth
      >
        {step == "FORM" && (
          <CampaignForm
            onSubmit={onSubmit}
            validate={validate}
            handleClose={handleClose}
          />
        )}
        {step == "APPROVING" && (
          <TxWaiter
            title="make_campaign.tx_waiter.approving_title"
            text="make_campaign.tx_waiter.approving_text"
            tx={approvalTx}
            id="approval-waiter"
          />
        )}
        {step == "CREATING" && (
          <TxWaiter
            title="make_campaign.tx_waiter.creating_title"
            text="make_campaign.tx_waiter.creating_text"
            tx={creationTx}
            id="creation-waiter"
          />
        )}
        {step == "DONE" && <Done tx={creationTx} handleClose={handleClose} />}
        {step == "ERROR" && (
          <Failure failure={failure} handleClose={handleClose} />
        )}
      </Dialog>
    </Box>
  );
};

const CampaignForm = ({ onSubmit, validate, handleClose }) => {
  const translate = useTranslate();
  const defaultValues = {
    pricePerPoint: "0.003",
    maxIndividualReward: "10",
    minIndividualReward: "0.2",
  };
  return (
    <Box p="1em" id="campaign-form">
      <Form
        defaultValues={defaultValues}
        sanitizeEmptyValues
        validate={validate}
        onSubmit={onSubmit}
      >
        <Stack direction="row" gap="1em" flexWrap="wrap">
          <Box flex="1 1 450px">
            <TextInput
              fullWidth
              required={true}
              size="small"
              variant="filled"
              source="contentUrl"
              label={translate("make_campaign.form_step.content_url")}
            />
            <TextInput
              fullWidth
              required={true}
              size="small"
              variant="filled"
              source="budget"
              label={translate("make_campaign.form_step.budget")}
            />

            <ReferenceArrayInput
              fullWidth
              size="large"
              variant="filled"
              source="topic_ids"
              reference="Topic"
            >
              <AutocompleteArrayInput
                label={translate("make_campaign.form_step.topics")}
                helperText={translate("make_campaign.form_step.topics_help")}
                sx={{ mb: "0.5em" }}
                size="small"
                variant="filled"
                optionText={(x) => translate(`resources.Topic.names.${x.name}`)}
              />
            </ReferenceArrayInput>

            <TextInput
              fullWidth
              required={true}
              value="0.002"
              size="small"
              variant="filled"
              source="pricePerPoint"
              helperText={translate(
                "make_campaign.form_step.price_per_point_help",
              )}
              sx={{ mb: "0.5em" }}
              label={translate("make_campaign.form_step.price_per_point")}
            />
            <TextInput
              fullWidth
              required={true}
              size="small"
              variant="filled"
              source="maxIndividualReward"
              helperText={translate(
                "make_campaign.form_step.max_individual_reward_help",
              )}
              sx={{ mb: "0.5em" }}
              label={translate("make_campaign.form_step.max_individual_reward")}
            />
            <TextInput
              fullWidth
              required={true}
              size="small"
              variant="filled"
              source="minIndividualReward"
              helperText={translate(
                "make_campaign.form_step.min_individual_reward_help",
              )}
              sx={{ mb: "0.5em" }}
              label={translate("make_campaign.form_step.min_individual_reward")}
            />
          </Box>
          <Stack gap="0.5em" flex="1 1 250px">
            <Head3 sx={{ color: "primary.main", mb: "0.5em" }}>
              {translate("make_campaign.with_doc.form_title")}
            </Head3>
            <Typography
              mb="1em"
              dangerouslySetInnerHTML={{
                __html: translate("make_campaign.form_step.service_summary"),
              }}
            />
            <Box flexGrow={1} />
            <SaveButton
              fullWidth
              id="submit-start-campaign-form"
              size="large"
              label={translate("make_campaign.form_step.start_campaign")}
              icon={<CampaignIcon />}
            />
            <Button fullWidth color="secondary" onClick={handleClose}>
              {translate("make_campaign.close")}
            </Button>
          </Stack>
        </Stack>
      </Form>
    </Box>
  );
};

const TxWaiter = ({ title, text, tx, id }) => {
  const translate = useTranslate();
  return (
    <Box p="1em" id={id}>
      <Head2>{translate(title)}</Head2>
      <LinearProgress sx={{ my: "1em" }} />
      {!tx ? (
        <Typography>{translate(text)}</Typography>
      ) : (
        <>
          <Typography my="1em">
            {translate("make_campaign.tx_waiter.waiting_for_tx")}
          </Typography>
          <Button
            fullWidth
            color="inverted"
            variant="outlined"
            startIcon={<LaunchIcon />}
            href={`https://explorer.rootstock.io/tx/${tx.hash}`}
            target="_blank"
          >
            {translate("make_campaign.tx_waiter.see_in_explorer")}
          </Button>
        </>
      )}
    </Box>
  );
};

const Done = ({ tx, handleClose }) => {
  const translate = useTranslate();

  return (
    <Alert
      id="campaign-done"
      severity="success"
      sx={{ color: light }}
      variant="filled"
      icon={false}
    >
      <Head2>{translate("make_campaign_card.done.title")}</Head2>
      <Typography my="1em">
        {translate("make_campaign_card.done.text")}
      </Typography>
      <Button
        fullWidth
        color="inverted"
        variant="text"
        startIcon={<LaunchIcon />}
        href={`https://explorer.rootstock.io/tx/${tx.hash}`}
        target="_blank"
      >
        {translate("make_campaign_card.done.see_in_explorer")}
      </Button>
      <Button
        id="campaign-done-close"
        sx={{ mt: "1em" }}
        fullWidth
        onClick={handleClose}
        color="inverted"
        variant="outlined"
      >
        {translate("make_campaign_card.close")}
      </Button>
    </Alert>
  );
};

const Failure = ({ failure, handleClose }) => {
  const notify = useNotify();
  const translate = useTranslate();

  let msg;
  let info;

  if (failure == "Modal closed by user" || failure.code == "ACTION_REJECTED") {
    msg = translate("make_campaign_card.failure_step.action_rejected");
  } else if (failure.code == "WRONG_SIGNER") {
    msg = translate("make_campaign_card.failure_step.wrong_signer", {
      expected: formatAddress(failure.expected),
      actual: formatAddress(failure.actual),
    });
  } else {
    msg = translate("make_campaign_card.failure_step.unexpected_error");
    info = failure.info
      ? JSON.stringify(failure.info, null, 2)
      : failure.toString();
  }

  const copyText = async () => {
    notify("make_campaign_card.failure_step.info_copied", {
      anchorOrigin: { vertical: "top", horizontal: "center" },
    });
    await navigator.clipboard.writeText(info);
  };

  return (
    <Alert severity="error" variant="filled" icon={false}>
      <Head2 sx={{ mb: "0.5em" }}>
        {translate("make_campaign_card.failure_step.title")}
      </Head2>
      {msg}
      {info && (
        <Paper sx={{ mt: "1em", p: "0.5em" }}>
          <Typography
            component="pre"
            variant="body2"
            sx={{ whiteSpace: "break-spaces", lineBreak: "anywhere" }}
          >
            {info}
          </Typography>
          <Button
            sx={{ mt: "1em" }}
            fullWidth
            startIcon={<ContentCopyIcon />}
            onClick={copyText}
            color="inverted"
          >
            {translate("make_campaign_card.failure_step.copy_info")}
          </Button>
        </Paper>
      )}

      <Button
        sx={{ mt: "1em" }}
        fullWidth
        onClick={handleClose}
        color="inverted"
        variant="outlined"
      >
        {translate("make_campaign_card.close")}
      </Button>
    </Alert>
  );
};
