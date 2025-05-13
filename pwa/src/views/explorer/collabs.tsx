import {
  SelectInput,
  Datagrid,
  List,
  useTranslate,
  TextField,
  FunctionField,
  TextInput,
  NumberInput,
  SimpleShowLayout,
  ReferenceInput,
  ReferenceField,
} from "react-admin";
import { Link } from "react-router-dom";
import { ExplorerLayout } from "../layout";
import { AmountField } from "../../components/custom_fields";
import { Head1, Lead } from "../../components/theme";

export const CollabList = () => {
  const t = useTranslate();

  const statusChoices = [
    { id: "REGISTERED", name: "REGISTERED" },
    { id: "CLEARED", name: "CLEARED" },
    { id: "FAILED", name: "FAILED" },
  ];

  const filters = [
    <TextInput source="idEq" alwaysOn />,
    <ReferenceInput source="campaignIdEq" reference="Campaign">
      <NumberInput source="campaignIdEq" size="small" />
    </ReferenceInput>,
    <ReferenceInput source="handleIdEq" reference="Handle">
      <NumberInput source="handleIdEq" size="small" />
    </ReferenceInput>,
    <ReferenceInput source="advertiserIdEq" reference="Account">
      <NumberInput source="advertiserIdEq" size="small" />
    </ReferenceInput>,
    <ReferenceInput source="memberIdEq" reference="Account">
      <NumberInput source="memberIdEq" size="small" />
    </ReferenceInput>,
    <TextInput source="collabTriggerUniqueLike" />,
    <SelectInput source="statusEq" choices={statusChoices} />,
    <SelectInput source="statusNe" choices={statusChoices} />,
  ];

  return (
    <ExplorerLayout>
      <Head1>{t("explorer.collabs.title")}</Head1>
      <Lead>{t("explorer.collabs.description")}</Lead>
      <List
        disableAuthentication
        filters={filters}
        sort={{ field: "id", order: "DESC" }}
      >
        <Datagrid bulkActionButtons={false} expand={<ExpandCollab />}>
          <TextField source="id" />
          <ReferenceField
            source="campaignId"
            reference="Campaign"
            sortable={false}
            link="show"
          />
          <ReferenceField source="handleId" reference="Handle" sortable={false}>
            <TextField source="username" />
          </ReferenceField>
          <AmountField textAlign="right" source="reward" />
          <AmountField textAlign="right" source="fee" />
          <TextField source="status" sortable={false} />
          <ReferenceField
            source="advertiserId"
            reference="Account"
            sortable={false}
            link="show"
          />
          <ReferenceField
            source="memberId"
            reference="Account"
            sortable={false}
            link="show"
          />
        </Datagrid>
      </List>
    </ExplorerLayout>
  );
};

const ExpandCollab = () => (
  <SimpleShowLayout>
    <TextField source="disputeReason" sortable={false} />
    <TextField source="collabTriggerUniqueId" sortable={false} />
  </SimpleShowLayout>
);
