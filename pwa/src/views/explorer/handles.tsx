import {
  SelectInput,
  Datagrid,
  List,
  useSafeSetState,
  useTranslate,
  TextField,
  useRedirect,
  TextInput,
  NumberField,
  BooleanInput,
  EditButton,
  ReferenceField,
  ShowButton,
  Edit,
  SimpleForm,
  NumberInput,
  DateTimeInput,
  EditBase,
  Toolbar,
  SaveButton,
  FunctionField,
  ReferenceArrayInput,
  AutocompleteArrayInput,
} from "react-admin";
import { ExplorerLayout } from "../layout";
import { BigNumField, AmountField } from "../../components/custom_fields";
import { Head1, Lead } from "../../components/theme";
import { Box, Card, Stack, Typography } from "@mui/material";
import { ResponsiveAppBar } from "../responsive_app_bar";

export const HandleList = () => {
  const translate = useTranslate();
  const storedSession = localStorage.getItem("session");
  const session = storedSession ? JSON.parse(storedSession) : null;

  const filters = [
    <TextInput source="idEq" alwaysOn />,
    <TextInput source="usernameLike" alwaysOn />,
  ];

  return (
    <ExplorerLayout>
      <Head1>{translate("explorer.handles.title")}</Head1>
      <Lead>{translate("explorer.handles.description")}</Lead>
      <List
        disableAuthentication
        filters={filters}
        sort={{ field: "score", order: "DESC" }}
      >
        <Datagrid bulkActionButtons={false}>
          <TextField source="username" sortable={false} />
          <BigNumField textAlign="right" source="score" />
          <TextField source="status" sortable={false} />
          <NumberField textAlign="right" source="totalCollabs" />
          <AmountField textAlign="right" source="totalCollabRewards" />
          <TextField source="id" />
          <ReferenceField
            textAlign="right"
            source="accountId"
            reference="Account"
            sortable={false}
            label=""
          >
            <ShowButton />
          </ReferenceField>
          {session?.admin && <EditButton />}
        </Datagrid>
      </List>
    </ExplorerLayout>
  );
};

const engagementScoreChoices = [
  { id: "NONE", name: "None" },
  { id: "AVERAGE", name: "Average" },
  { id: "HIGH", name: "High" },
];

const pollScoreChoices = [
  { id: "NONE", name: "None" },
  { id: "AVERAGE", name: "Average" },
  { id: "HIGH", name: "High" },
  { id: "REVERSE", name: "Reverse" },
];

const operationalStatusChoices = [
  { id: "BANNED", name: "Banned" },
  { id: "SHADOWBANNED", name: "ShadowBanned" },
  { id: "NORMAL", name: "Normal" },
  { id: "ENHANCED", name: "Enhanced" },
];

export const HandleEdit = () => {
  const t = useTranslate();

  return (
    <Box id="asami-explorer">
      <ResponsiveAppBar expandExplorer={true} />
      <EditBase mutationMode="pessimistic">
        <FunctionField render={(r) => <Head1>Edit {r.username}</Head1>} />
        <Card sx={{ mt: "1em" }}>
          <SimpleForm
            alignItems="stretch"
            toolbar={
              <Toolbar>
                <SaveButton />
              </Toolbar>
            }
          >
            <Stack direction="row" gap="1em" flexWrap="wrap">
              <NumberInput
                size="medium"
                sx={{ m: "0", flex: "1 1 300px" }}
                source="audienceSizeOverride"
              />
              <TextInput
                sx={{ flex: "100 0 300px" }}
                source="audienceSizeOverrideReason"
              />
            </Stack>

            <Stack direction="row" gap="1em" flexWrap="wrap">
              <SelectInput
                source="onlineEngagementOverride"
                choices={engagementScoreChoices}
                sx={{ m: "0", flex: "1 1 300px" }}
              />
              <TextInput
                sx={{ flex: "100 0 300px" }}
                source="onlineEngagementOverrideReason"
              />
            </Stack>

            <Stack direction="row" gap="1em" flexWrap="wrap">
              <SelectInput
                source="offlineEngagementScore"
                choices={engagementScoreChoices}
                sx={{ m: "0", flex: "1 1 300px" }}
              />
              <TextInput
                source="offlineEngagementDescription"
                sx={{ flex: "100 0 300px" }}
              />
            </Stack>

            <Stack direction="row" gap="1em" flexWrap="wrap">
              <SelectInput
                source="pollOverride"
                choices={pollScoreChoices}
                sx={{ m: "0", flex: "1 1 300px" }}
              />
              <TextInput
                source="pollOverrideReason"
                sx={{ flex: "100 0 300px" }}
              />
            </Stack>

            <Stack direction="row" gap="1em" flexWrap="wrap">
              <SelectInput
                source="operationalStatusOverride"
                choices={operationalStatusChoices}
                sx={{ m: "0", flex: "1 1 300px" }}
              />
              <TextInput
                source="operationalStatusOverrideReason"
                sx={{ flex: "100 0 300px" }}
              />
            </Stack>

            <Stack direction="row" gap="1em" flexWrap="wrap">
              <BooleanInput
                sx={{ m: "0", flex: "1 1 300px" }}
                source="referrerScoreOverride"
              />
              <TextInput
                sx={{ flex: "100 0 300px" }}
                source="referrerScoreOverrideReason"
              />
            </Stack>

            <Stack direction="row" gap="1em" flexWrap="wrap">
              <BooleanInput
                sx={{ m: "0", flex: "1 1 300px" }}
                source="holderScoreOverride"
              />
              <TextInput
                sx={{ flex: "100 0 300px" }}
                source="holderScoreOverrideReason"
              />
            </Stack>
            <ReferenceArrayInput
              fullWidth
              size="large"
              variant="outlined"
              source="topicIds"
              reference="Topic"
            >
              <AutocompleteArrayInput
                sx={{ mb: "0.5em" }}
                size="small"
                variant="outlined"
                optionText={(x) => t(`resources.Topic.names.${x.name}`)}
              />
            </ReferenceArrayInput>
          </SimpleForm>
        </Card>
      </EditBase>
    </Box>
  );
};
