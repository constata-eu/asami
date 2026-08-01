import {
  Datagrid,
  List,
  useTranslate,
  TextField,
  TextInput,
  NumberField,
  EditButton,
  ReferenceField,
  ShowButton,
  EditBase,
  SimpleForm,
  Toolbar,
  SaveButton,
  FunctionField,
  ReferenceArrayInput,
  AutocompleteArrayInput,
} from "react-admin";
import { ExplorerLayout } from "../layout";
import { AmountField } from "../../components/custom_fields";
import { Head1, Lead } from "../../components/theme";
import { Box, Card } from "@mui/material";
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
        sort={{ field: "totalCollabRewards", order: "DESC" }}
      >
        <Datagrid bulkActionButtons={false}>
          <TextField source="username" sortable={false} />
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
