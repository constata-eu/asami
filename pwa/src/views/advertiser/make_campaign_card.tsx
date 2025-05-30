import { useSafeSetState, useDataProvider, useTranslate } from "react-admin";

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
import { useContracts } from "../../components/contracts_context";
import { Head2 } from "../../components/theme";
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
          <MakeCampaignWithDocDialog account={account} onCreate={onCreate} />
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
            thumbsUpOnly: input.thumbsUpOnly,
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

  const hasDoc = BigInt(account.docBalance) >= parseEther("1");

  return (
    <>
      <Typography my="1em" id="make-campaign-with-doc-dialog-container">
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
            helpTitle="make_campaign.with_doc.form_title"
            helpText="make_campaign.with_doc.service_summary"
            amountLabel="make_campaign.form_step.budget"
            amountHelp="make_campaign.form_step.budget_help"
            minAmount="1"
            onSubmit={onSubmit}
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
        {step == "DONE" && (
          <Done
            title={"make_campaign.with_doc.done.title"}
            leadText={"make_campaign.with_doc.done.text"}
            primaryLink={`https://explorer.rootstock.io/tx/${creationTx?.hash}`}
            primaryText={translate(
              "make_campaign.with_doc.done.see_in_explorer",
            )}
            handleClose={handleClose}
          />
        )}
        {step == "ERROR" && (
          <OnChainFailure failure={failure} handleClose={handleClose} />
        )}
      </Dialog>
    </>
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
