import {
  Form,
  RecordContextProvider,
  SaveButton,
  TextInput,
  useDataProvider,
  useTranslate,
} from "react-admin";
import { useState } from "react";

import {
  LinearProgress,
  Box,
  Button,
  CardContent,
  Typography,
  Card,
} from "@mui/material";
import { Dialog } from "@mui/material";
import { formatAddress } from "../../lib/formatters";
import { toBeHex, parseEther } from "ethers";
import { DeckCard } from "../layout";
import { parseNumber } from "../../components/custom_fields";
import { useContracts } from "../../components/contracts_context";
import { Head2, Head3 } from "../../components/theme";
import { useNotify } from "react-admin";
import LaunchIcon from "@mui/icons-material/Launch";
import { Stack } from "@mui/material";
import CampaignIcon from "@mui/icons-material/Campaign";
import ClaimAccountButton from "../claim_account";
import HourglassTopIcon from "@mui/icons-material/HourglassTop";
import { CampaignForm, Banned, Done, Failure } from "./make_campaign_shared";

export const MakeCampaignWithDocCard = ({ account, onCreate }) => {
  const t = useTranslate();
  const status = account.status;

  return (
    <Card>
      <CardContent
        sx={{
          height: "100%",
          display: "flex",
          flexDirection: "column",
          justifyContent: "space-between",
        }}
      >
        <Head2>{t("make_campaign.with_doc.title")}</Head2>
        {status == "MANAGED" && <ClaimAccountToUseDoc />}
        {status == "CLAIMING" && <ClaimInProgress />}
        {status == "BANNED" && <Banned />}
        {status == "CLAIMED" && (
          <CampaignFormOrGetDoc account={account} onCreate={onCreate} />
        )}
      </CardContent>
    </Card>
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

const CampaignFormOrGetDoc = ({ account, onCreate }) => {
  const translate = useTranslate();
  const [open, setOpen] = useState(false);
  const hasDoc = BigInt(account.docBalance) >= parseEther("1");

  const onClose = () => setOpen(false);

  return (
    <>
      <Typography my="1em" id="make-campaign-with-doc-dialog-container">
        {translate("make_campaign.with_doc.create_text")}
      </Typography>
      {hasDoc ? (
        <>
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
          <MakeCampaignWithDocDialog
            open={open}
            setOpen={setOpen}
            account={account}
            onCreate={onCreate}
          />
        </>
      ) : (
        <>
          <Typography my="1em">
            {translate("make_campaign.with_doc.doc_funded")}
          </Typography>
          <Button
            fullWidth
            target="_blank"
            variant="outlined"
            href={translate("make_campaign.with_doc.get_doc_link")}
          >
            {translate("make_campaign.with_doc.get_doc")}
          </Button>
        </>
      )}
    </>
  );
};

export const ContinueCampaignButton = ({ account, onCreate, campaign }) => {
  const translate = useTranslate();
  const [open, setOpen] = useState(false);

  return (
    <>
      <Button
        fullWidth
        variant="contained"
        size="small"
        id={`open-continue-creating-campaign-dialog-${campaign.id}`}
        onClick={() => setOpen(true)}
      >
        <CampaignIcon sx={{ mr: "5px" }} />
        {translate("make_campaign.with_doc.continue")}
      </Button>
      <MakeCampaignWithDocDialog
        open={open}
        setOpen={setOpen}
        account={account}
        onCreate={onCreate}
        continueCampaign={campaign}
      />
    </>
  );
};

const MakeCampaignWithDocDialog = ({
  open,
  setOpen,
  account,
  onCreate,
  continueCampaign = null,
}) => {
  const translate = useTranslate();
  const [step, setStep] = useState("FORM");
  const [approvalTx, setApprovalTx] = useState();
  const [creationTx, setCreationTx] = useState();
  const [failure, setFailure] = useState();
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
    try {
      setOpen(false);
      const { doc, asami, asamiAddress, signer } = await contracts(
        account.addr,
      );

      setStep("APPROVING");
      setOpen(true);
      const input = values.makeCampaignInput;

      const campaign =
        continueCampaign ||
        (
          await dataProvider.create("CreateCampaignFromLink", {
            data: {
              input: {
                link: input.link,
                pricePerPoint: toBeHex(input.pricePerPoint, 32),
                maxIndividualReward: toBeHex(input.maxIndividualReward, 32),
                minIndividualReward: toBeHex(input.minIndividualReward, 32),
                topicIds: input.topicIds,
                thumbsUpOnly: input.thumbsUpOnly,
              },
            },
          })
        ).data;

      const allowance = await doc.allowance(signer, asamiAddress);
      if (allowance < input.budget) {
        const approval = await doc.approve(asamiAddress, input.budget, signer);
        setApprovalTx(approval);
        await approval.wait();
      }

      setStep("CREATING");

      let time =
        new Date().getTime() + input.durationDays * 24 * 60 * 60 * 1000;

      const creation = await asami.makeCampaigns([
        {
          budget: input.budget,
          briefingHash: campaign.briefingHash,
          validUntil: BigInt(Math.floor(time / 1000)),
        },
      ]);

      setCreationTx(creation);
      await creation.wait();
      setStep("DONE");
      await dataProvider.update("Campaign", { id: campaign.id, data: {} });
      onCreate();
    } catch (e) {
      setFailure(e);
      setOpen(true);
      setStep("ERROR");
    }
  };

  return (
    <Dialog
      id="start-campaign-with-doc-dialog"
      open={open}
      maxWidth="md"
      fullWidth
    >
      {step == "FORM" &&
        (continueCampaign ? (
          <ContinueCampaignForm
            campaign={continueCampaign}
            minAmount="1"
            onSubmit={onSubmit}
            handleClose={handleClose}
          />
        ) : (
          <CampaignForm
            helpTitle="make_campaign.with_doc.form_title"
            helpText="make_campaign.with_doc.service_summary"
            amountLabel="make_campaign.form_step.budget"
            amountHelp="make_campaign.form_step.budget_help"
            minAmount="1"
            onSubmit={onSubmit}
            handleClose={handleClose}
            askDuration={true}
          />
        ))}
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
      {step == "DONE" && (
        <Done
          title={"make_campaign.with_doc.done.title"}
          leadText={"make_campaign.with_doc.done.text"}
          primaryLink={`https://explorer.rootstock.io/tx/${creationTx?.hash}`}
          primaryText={translate("make_campaign.with_doc.done.see_in_explorer")}
          handleClose={handleClose}
        />
      )}
      {step == "ERROR" && (
        <OnChainFailure failure={failure} handleClose={handleClose} />
      )}
    </Dialog>
  );
};

export const ContinueCampaignForm = ({
  campaign,
  onSubmit,
  minAmount,
  handleClose,
}) => {
  const translate = useTranslate();

  const validate = (values) => {
    let input = {};

    const budget = parseNumber(
      values.budget,
      minAmount,
      "budget_too_low",
      "budget_not_a_number",
    );
    if (budget.error) {
      return { budget: budget.error };
    }
    input.budget = budget.ok;

    try {
      const parsed = parseInt(values.durationDays);
      if (parsed < 2) {
        return {
          durationDays: translate("make_campaign.errors.duration_days_too_low"),
        };
      }
      input.durationDays = parsed;
    } catch {
      return {
        durationDays: translate(
          "make_campaign.errors.duration_days_not_a_number",
        ),
      };
    }
    values.makeCampaignInput = input;
  };

  return (
    <Box p="1em" id="campaign-form">
      <RecordContextProvider value={{ budget: null, durationDays: 15 }}>
        <Form sanitizeEmptyValues validate={validate} onSubmit={onSubmit}>
          <Stack gap="0.5em">
            <Head3 sx={{ color: "primary.main", mb: "0.5em" }}>
              {translate("make_campaign.with_doc.continue_title")}
            </Head3>
            <Typography
              mb="1em"
              dangerouslySetInnerHTML={{
                __html: translate("make_campaign.with_doc.continue_warning"),
              }}
            />
            <TextInput
              fullWidth
              required={true}
              size="small"
              variant="filled"
              source="budget"
              label={translate("make_campaign.form_step.budget")}
              helperText={translate("make_campaign.form_step.budget_help")}
            />
            <TextInput
              fullWidth
              required={true}
              size="small"
              variant="filled"
              source="durationDays"
              helperText={translate(
                "make_campaign.form_step.duration_days_help",
              )}
              label={translate("make_campaign.form_step.duration_days")}
            />
            <SaveButton
              fullWidth
              id="submit-start-campaign-form"
              size="large"
              label={translate("make_campaign.form_step.start_campaign")}
              icon={<CampaignIcon />}
            />
            <Button fullWidth color="secondary" onClick={handleClose}>
              {translate("make_campaign.i_changed_my_mind")}
            </Button>
          </Stack>
        </Form>
      </RecordContextProvider>
    </Box>
  );
};

const TxWaiter = ({ title, text, tx, id }) => {
  const translate = useTranslate();
  return (
    <Box p="1em" id={id}>
      <Head2 sx={{ color: "primary.main" }}>{translate(title)}</Head2>
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
            color="primary"
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

const OnChainFailure = ({ failure, handleClose }) => {
  const notify = useNotify();
  const translate = useTranslate();

  let msg;
  let info;

  if (failure == "Modal closed by user" || failure.code == "ACTION_REJECTED") {
    msg = translate("make_campaign.failure_step.action_rejected");
  } else if (failure.code == "WRONG_SIGNER") {
    msg = translate("make_campaign.failure_step.wrong_signer", {
      expected: formatAddress(failure.expected),
      actual: formatAddress(failure.actual),
    });
  } else {
    msg = translate("make_campaign.failure_step.unexpected_error");
    info = failure.info
      ? JSON.stringify(failure.info, null, 2)
      : failure.toString();
  }

  return <Failure msg={msg} info={info} handleClose={handleClose} />;
};
