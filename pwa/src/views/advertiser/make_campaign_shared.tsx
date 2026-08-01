import {
  useTranslate,
  ReferenceArrayInput,
  AutocompleteArrayInput,
  BooleanInput,
  FormDataConsumer,
  SelectInput,
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
}) => {
  const translate = useTranslate();
  const defaultValues = {
    kind: "X_REPOST",
    maxIndividualReward: "5",
    allMembers: true,
  };

  const validate = (values) => {
    let input = {};

    input.kind = values.kind;
    input.thumbsUpOnly = !!values.thumbsUpOnly;
    input.allMembers = !!values.allMembers;
    input.topicIds = values.topic_ids || [];
    input.memberIds = values.member_ids || [];

    if (values.kind === "X_REPOST") {
      const error = validateCampaignLink(values.contentUrl);
      if (error) {
        return { contentUrl: translate(`make_campaign.errors.${error}`) };
      }
      input.link = values.contentUrl;
    } else {
      input.link = null;
      input.topicIds = [];
      input.thumbsUpOnly = false;
      if (!input.allMembers && input.memberIds.length < 1) {
        return {
          member_ids: translate("make_campaign.errors.select_members"),
        };
      }
    }

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
    input.durationDays = 20;

    const max = parseNumber(
      values.kind === "X_REPOST" ? values.maxIndividualReward : values.budget,
      "0",
      "max_too_low",
      "max_not_a_number",
    );
    if (max.error) {
      return { maxIndividualReward: max.error };
    }
    input.maxIndividualReward = max.ok;

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
            <SelectInput
              fullWidth
              size="small"
              variant="filled"
              source="kind"
              choices={[
                {
                  id: "X_REPOST",
                  name: translate("resources.Campaign.kinds.X_REPOST"),
                },
                {
                  id: "COMMUNITY_GIVEAWAY",
                  name: translate("resources.Campaign.kinds.COMMUNITY_GIVEAWAY"),
                },
              ]}
            />
            <FormDataConsumer>
              {({ formData }) =>
                formData.kind === "X_REPOST" ? (
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
                ) : null
              }
            </FormDataConsumer>
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
              <TextInput
                fullWidth
                disabled
                size="small"
                variant="filled"
                source="durationDaysFixed"
                helperText={translate(
                  "make_campaign.form_step.fixed_duration_help",
                )}
                sx={{ flex: "1 1 200px" }}
                defaultValue="20"
                label={translate("make_campaign.form_step.fixed_duration")}
              />
            </Stack>
            <FormDataConsumer>
              {({ formData }) =>
                formData.kind === "X_REPOST" ? (
                  <>
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
                      label={translate(
                        "make_campaign.form_step.max_individual_reward",
                      )}
                    />
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
                  </>
                ) : (
                  <Stack gap="0.5em">
                    <BooleanInput
                      source="allMembers"
                      label={translate("make_campaign.form_step.all_members")}
                      helperText={translate(
                        "make_campaign.form_step.all_members_help",
                      )}
                    />
                    {!formData.allMembers && (
                      <ReferenceArrayInput
                        source="member_ids"
                        reference="Account"
                        filter={{ acceptanceStatusEq: "ACTIVE" }}
                      >
                        <AutocompleteArrayInput
                          label={translate(
                            "make_campaign.form_step.selected_members",
                          )}
                          helperText={translate(
                            "make_campaign.form_step.selected_members_help",
                          )}
                          optionText="name"
                          size="small"
                          variant="filled"
                        />
                      </ReferenceArrayInput>
                    )}
                  </Stack>
                )
              }
            </FormDataConsumer>
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
