import {
  useTranslate,
  ReferenceArrayInput,
  AutocompleteArrayInput,
  BooleanInput,
} from "react-admin";

import { Alert, Box, Button, Typography } from "@mui/material";
import { Head2, Head3, light } from "../../components/theme";
import { validateCampaignLink } from "../../lib/campaign";
import Paper from "@mui/material/Paper";
import { Form, TextInput, SaveButton, useNotify } from "react-admin";
import ContentCopyIcon from "@mui/icons-material/ContentCopy";
import LaunchIcon from "@mui/icons-material/Launch";
import { Stack } from "@mui/material";
import CampaignIcon from "@mui/icons-material/Campaign";
import RemoveCircleOutlineIcon from "@mui/icons-material/RemoveCircleOutline";
import ThumbUpIcon from "@mui/icons-material/ThumbUp";
import { parseNumber } from "../../components/custom_fields";

export const CampaignForm = ({
  helpTitle,
  helpText,
  amountLabel,
  amountHelp,
  onSubmit,
  handleClose,
  minAmount,
  askDuration,
}) => {
  const translate = useTranslate();
  const defaultValues = {
    pricePerPoint: "0.01",
    maxIndividualReward: "5",
    minIndividualReward: "0.1",
    durationDays: 15,
  };

  const validate = (values) => {
    let input = {};

    const error = validateCampaignLink(values.contentUrl);
    if (error) {
      return { contentUrl: translate(`make_campaign.errors.${error}`) };
    }
    input.link = values.contentUrl;
    input.topicIds = values.topic_ids;
    input.pricePerPoint = values.pricePerPoint;
    input.thumbsUpOnly = !!values.thumbsUpOnly;

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

    if (askDuration) {
      try {
        const parsed = parseInt(values.durationDays);
        if (parsed < 2) {
          return {
            durationDays: translate(
              "make_campaign.errors.duration_days_too_low",
            ),
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
    }

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

  return (
    <Box p="1em" id="campaign-form">
      <Form
        defaultValues={defaultValues}
        sanitizeEmptyValues
        validate={validate}
        onSubmit={onSubmit}
      >
        <Stack direction="row" gap="1em" flexWrap="wrap">
          <Stack flex="1 1 450px" gap="0.5em">
            <TextInput
              fullWidth
              required={true}
              size="small"
              variant="filled"
              source="contentUrl"
              label={translate("make_campaign.form_step.content_url")}
              helperText={
                <span
                  dangerouslySetInnerHTML={{
                    __html: translate(
                      "make_campaign.form_step.content_url_help",
                    ),
                  }}
                />
              }
            />
            <Stack flexWrap="wrap" direction="row" gap="1em" mb="0.5em">
              <TextInput
                fullWidth
                required={true}
                size="small"
                variant="filled"
                source="budget"
                label={translate(amountLabel)}
                helperText={amountHelp ? translate(amountHelp) : false}
                sx={{ flex: "1 1 200px" }}
              />
              {askDuration && (
                <TextInput
                  fullWidth
                  required={true}
                  size="small"
                  variant="filled"
                  source="durationDays"
                  helperText={translate(
                    "make_campaign.form_step.duration_days_help",
                  )}
                  sx={{ flex: "1 1 200px" }}
                  label={translate("make_campaign.form_step.duration_days")}
                />
              )}
            </Stack>

            <TextInput
              fullWidth
              required={true}
              value="0.005"
              size="small"
              variant="filled"
              source="pricePerPoint"
              helperText={translate(
                "make_campaign.form_step.price_per_point_help",
              )}
              sx={{ mb: "0.5em" }}
              label={translate("make_campaign.form_step.price_per_point")}
            />
            <Stack flexWrap="wrap" direction="row" gap="1em" mb="0.5em">
              <TextInput
                required={true}
                size="small"
                variant="filled"
                source="maxIndividualReward"
                helperText={translate(
                  "make_campaign.form_step.max_individual_reward_help",
                )}
                label={translate(
                  "make_campaign.form_step.max_individual_reward",
                )}
                sx={{ flex: "1 1 200px" }}
              />
              <TextInput
                required={true}
                size="small"
                variant="filled"
                source="minIndividualReward"
                helperText={translate(
                  "make_campaign.form_step.min_individual_reward_help",
                )}
                label={translate(
                  "make_campaign.form_step.min_individual_reward",
                )}
                sx={{ flex: "1 1 200px" }}
              />
            </Stack>
            <Stack
              flexWrap="wrap"
              direction="row"
              gap="1em"
              mb="0.5em"
              alignItems="flex-end"
            >
              <Box flex="0 1 50%">
                <ReferenceArrayInput
                  size="large"
                  variant="filled"
                  source="topic_ids"
                  reference="Topic"
                >
                  <AutocompleteArrayInput
                    label={translate("make_campaign.form_step.topics")}
                    helperText={translate(
                      "make_campaign.form_step.topics_help",
                    )}
                    size="small"
                    variant="filled"
                    optionText={(x) =>
                      translate(`resources.Topic.names.${x.name}`)
                    }
                  />
                </ReferenceArrayInput>
              </Box>
              <BooleanInput
                source="thumbsUpOnly"
                label={
                  <Stack direction="row" gap="0.5em">
                    {translate("make_campaign.form_step.thumbs_up_only")}
                    <ThumbUpIcon color="primary" fontSize="small" />
                  </Stack>
                }
                helperText={translate(
                  "make_campaign.form_step.thumbs_up_only_help",
                )}
              />
            </Stack>
          </Stack>
          <Stack gap="0.5em" flex="1 1 250px">
            <Head3 sx={{ color: "primary.main", mb: "0.5em" }}>
              {translate(helpTitle)}
            </Head3>
            <Typography
              mb="1em"
              dangerouslySetInnerHTML={{
                __html: translate(helpText),
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
              {translate("make_campaign.i_changed_my_mind")}
            </Button>
          </Stack>
        </Stack>
      </Form>
    </Box>
  );
};

export const Banned = () => (
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

export const Done = ({
  title,
  leadText,
  primaryText,
  primaryLink,
  handleClose,
  alertId,
}) => {
  const translate = useTranslate();

  return (
    <Alert
      id={alertId}
      sx={{
        color: light,
        backgroundColor: (theme) => theme.palette.primary.main,
      }}
      variant="filled"
      icon={false}
    >
      <Head2 sx={{ color: "inverted.main" }}>{translate(title)}</Head2>
      <Typography my="1em">{translate(leadText)}</Typography>
      <Stack direction="row" flexWrap="wrap">
        <Button
          sx={{ flex: "1 1 auto" }}
          id="campaign-done-close"
          variant="text"
          onClick={handleClose}
          color="inverted"
        >
          {translate("make_campaign.close")}
        </Button>
        <Button
          sx={{ flex: "1 1 auto" }}
          color="inverted"
          variant="outlined"
          startIcon={<LaunchIcon />}
          href={primaryLink}
          target="_blank"
        >
          {primaryText}
        </Button>
      </Stack>
    </Alert>
  );
};

export const Failure = ({ msg, info, handleClose }) => {
  const notify = useNotify();
  const translate = useTranslate();

  const copyText = async () => {
    notify("make_campaign.failure_step.info_copied", {
      anchorOrigin: { vertical: "top", horizontal: "center" },
    });
    await navigator.clipboard.writeText(info);
  };

  return (
    <Alert severity="error" variant="filled" icon={false}>
      <Head2 sx={{ mb: "0.5em", color: "inverted.main" }}>
        {translate("make_campaign.failure_step.title")}
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
            {translate("make_campaign.failure_step.copy_info")}
          </Button>
        </Paper>
      )}

      <Button
        sx={{ mt: "1em" }}
        fullWidth
        onClick={handleClose}
        id="campaign-failure-close"
        color="inverted"
        variant="outlined"
      >
        {translate("make_campaign.close")}
      </Button>
    </Alert>
  );
};
