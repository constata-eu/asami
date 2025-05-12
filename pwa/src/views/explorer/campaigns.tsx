import {
  SelectInput,
  Datagrid,
  List,
  useTranslate,
  TextField,
  FunctionField,
  TextInput,
  DateField,
  NumberField,
  NumberInput,
  ReferenceInput,
  SimpleShowLayout,
  ReferenceField,
} from "react-admin";
import { Link } from "react-router-dom";
import { ExplorerLayout } from "../layout";
import { viewPostUrl } from "../../lib/campaign";
import { AmountField, AmountInput } from "../../components/custom_fields";
import { Head1, Lead } from "../../components/theme";

export const CampaignList = () => {
  const t = useTranslate();

  const statusChoices = [
    { id: "DRAFT", name: "DRAFT" },
    { id: "SUBMITTED", name: "SUBMITTED" },
    { id: "PUBLISHED", name: "PUBLISHED" },
  ];

  const filters = [
    <TextInput source="idEq" alwaysOn />,
    <TextInput source="briefingJsonLike" />,
    <TextInput source="briefingHashLike" />,
    <ReferenceInput source="accountIdEq" reference="Account">
      <NumberInput source="accountIdEq" size="small" />
    </ReferenceInput>,
    <SelectInput source="statusEq" choices={statusChoices} />,
    <SelectInput source="statusNe" choices={statusChoices} />,
    <AmountInput source="budgetLt" />,
    <AmountInput source="budgetGt" />,
    <AmountInput source="budgetEq" />,
  ];

  return (
    <ExplorerLayout>
      <Head1>{t("explorer.campaigns.title")}</Head1>
      <Lead>{t("explorer.campaigns.description")}</Lead>
      <List
        disableAuthentication
        filters={filters}
        sort={{ field: "createdAt", order: "DESC" }}
      >
        <Datagrid bulkActionButtons={false} expand={<ExpandCampaign />}>
          <FunctionField
            label={translate("campaign_list.post")}
            render={(record) => (
              <a target="_blank" href={viewPostUrl(record)} rel="noreferrer">
                {translate("campaign_list.see_post")}
              </a>
            )}
          />
          <TextField source="id" />
          <TextField source="status" sortable={false} />
          <FunctionField
            textAlign="right"
            source="totalCollabs"
            render={(record) =>
              record.totalCollabs > 0 ? (
                <Link
                  to={`/Collab?displayedFilters=%7B%7D&filter=%7B%22campaignIdEq%22%3A${record.id}%7D`}
                >
                  <NumberField source="totalCollabs" />
                </Link>
              ) : (
                <NumberField source="totalCollabs" />
              )
            }
          />
          <AmountField textAlign="right" currency="" source="budget" />
          <AmountField textAlign="right" currency="" source="totalSpent" />
          <AmountField textAlign="right" currency="" source="totalBudget" />
          <DateField source="validUntil" showTime />
          <DateField source="createdAt" showTime />
          <ReferenceField source="accountId" resource="Account" />
          <FunctionField
            source="briefingJson"
            sortable={false}
            render={(record) => (
              <a target="_blank" href={viewPostUrl(record)} rel="noreferrer">
                {JSON.parse(record.briefingJson)}
              </a>
            )}
          />
        </Datagrid>
      </List>
    </ExplorerLayout>
  );
};

const ExpandCampaign = () => (
  <SimpleShowLayout>
    <TextField source="briefingHash" />
  </SimpleShowLayout>
);
