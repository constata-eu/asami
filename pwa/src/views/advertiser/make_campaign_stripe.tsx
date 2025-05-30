import { useSafeSetState, useDataProvider, useTranslate } from "react-admin";

import { Box, Button, Card, CardContent, Typography } from "@mui/material";
import { Dialog } from "@mui/material";
import { toBeHex } from "ethers";
import { DeckCard } from "../layout";
import { Head2 } from "../../components/theme";
import CampaignIcon from "@mui/icons-material/Campaign";
import { CampaignForm, Banned, Done, Failure } from "./make_campaign_shared";

export const MakeCampaignStripe = ({ account, onCreate }) => {
  const t = useTranslate();
  const status = account?.status;

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
        <Head2>{t("make_campaign.with_stripe.title")}</Head2>
        {status == "BANNED" && <Banned />}
        {status != "BANNED" && (
          <MakeCampaignStripeDialog account={account} onCreate={onCreate} />
        )}
      </CardContent>
    </Card>
  );
};

const MakeCampaignStripeDialog = ({ onCreate }) => {
  const t = useTranslate();
  const [open, setOpen] = useSafeSetState(false);
  const [step, setStep] = useSafeSetState("FORM");
  const [campaign, setCampaign] = useSafeSetState();
  const [failure, setFailure] = useSafeSetState();
  const dataProvider = useDataProvider();

  const handleClose = () => {
    setOpen(false);
    setFailure(null);
    setStep("FORM");
  };

  const onSubmit = async (values) => {
    try {
      const input = values.makeCampaignInput;

      const response = await dataProvider.create("CreateCampaignFromLink", {
        data: {
          input: {
            link: input.link,
            managedUnitAmount: Number(input.budget / 10n ** 16n),
            pricePerPoint: toBeHex(input.pricePerPoint, 32),
            maxIndividualReward: toBeHex(input.maxIndividualReward, 32),
            minIndividualReward: toBeHex(input.minIndividualReward, 32),
            topicIds: input.topicIds,
            thumbsUpOnly: input.thumbsUpOnly,
          },
        },
      });
      setCampaign(response.data);
      setStep("DONE");
      onCreate();
    } catch (e) {
      setFailure(e);
      setOpen(true);
      setStep("ERROR");
    }
  };

  return (
    <>
      <Typography id="make-campaign-stripe-dialog-container" my="1em">
        {t("make_campaign.with_stripe.text")}
      </Typography>
      <Button
        fullWidth
        variant="contained"
        size="large"
        id="open-start-campaign-stripe-dialog"
        onClick={() => setOpen(true)}
      >
        <CampaignIcon sx={{ mr: "5px" }} />
        {t("make_campaign.with_stripe.create_button")}
      </Button>
      <Dialog
        id="start-campaign-with-doc-dialog"
        open={open}
        maxWidth="md"
        fullWidth
      >
        {step == "FORM" && (
          <CampaignForm
            helpTitle="make_campaign.with_stripe.form_title"
            helpText="make_campaign.with_stripe.service_summary"
            amountLabel="make_campaign.form_step.managed_amount"
            amountHelp="make_campaign.form_step.managed_amount_help"
            minAmount="10"
            onSubmit={onSubmit}
            handleClose={handleClose}
          />
        )}
        {step == "DONE" && (
          <Done
            title={"make_campaign.with_stripe.done.title"}
            leadText={"make_campaign.with_stripe.done.text"}
            primaryLink={campaign?.privateFields.stripeSessionUrl}
            primaryText={t("make_campaign.with_stripe.done.go_to_stripe")}
            handleClose={handleClose}
          />
        )}
        {step == "ERROR" && (
          <Failure
            handleClose={handleClose}
            msg={t("make_campaign.failure_step.unexpected_error")}
            info={
              failure.info
                ? JSON.stringify(failure.info, null, 2)
                : failure.toString()
            }
          />
        )}
      </Dialog>
    </>
  );
};
