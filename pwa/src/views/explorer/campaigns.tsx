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
  ListView,
  ListBase,
  useShowController,
  useListController,
  ShowButton,
  ExportButton,
} from "react-admin";
import { Link } from "react-router-dom";
import { ExplorerLayout } from "../layout";
import { contentId, viewPostUrl } from "../../lib/campaign";
import { AmountField, AmountInput } from "../../components/custom_fields";
import { Head1, Head2, Lead } from "../../components/theme";
import { Box, Card, Stack } from "@mui/material";
import { TwitterTweetEmbed } from "react-twitter-embed";
import XIcon from "@mui/icons-material/X";

export const CampaignList = () => {
  const t = useTranslate();

  const filters = [
    <TextInput source="idEq" alwaysOn />,
    <TextInput source="briefingJsonLike" />,
    <TextInput source="briefingHashLike" />,
    <ReferenceInput source="accountIdEq" reference="Account">
      <NumberInput source="accountIdEq" size="small" />
    </ReferenceInput>,
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
        filter={{ isPublishedEq: true }}
        filters={filters}
        sort={{ field: "createdAt", order: "DESC" }}
      >
        <Datagrid bulkActionButtons={false} expand={<ExpandCampaign />}>
          <FunctionField source="briefingJson" render={contentId} />
          <ReferenceField source="accountId" reference="Account" link="show" />
          <NumberField source="totalCollabs" />
          <AmountField textAlign="right" currency="" source="budget" />
          <AmountField textAlign="right" currency="" source="totalSpent" />
          <AmountField textAlign="right" currency="" source="totalBudget" />
          <DateField source="validUntil" />
          <DateField source="createdAt" />
          <TextField source="id" />
          <ShowButton />
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

export const CampaignShow = () => (
  <ExplorerLayout>
    <CampaignShowInner />
  </ExplorerLayout>
);

export const CampaignShowInner = () => {
  const show = useShowController({ disableAuthentication: true });

  if (show.isPending || !show.record) {
    return <></>;
  }

  return (
    <Stack direction="row" gap="1em" mt="1em" flexWrap="wrap">
      <Box flex="0 1 500px">
        <TwitterTweetEmbed
          tweetId={contentId(show.record)}
          options={{
            align: "center",
            width: "100%",
            conversation: "none",
            margin: 0,
          }}
        />
        <Card sx={{ mt: "1em", justifySelf: "left" }} elevation={1}>
          <SimpleShowLayout record={show.record} direction="row">
            <TextField source="id" />
            <TextField source="status" sortable={false} />
            <AmountField textAlign="right" currency="" source="budget" />
            <AmountField textAlign="right" currency="" source="totalSpent" />
            <AmountField textAlign="right" currency="" source="totalBudget" />
            <NumberField source="totalCollabs" />
            <DateField source="validUntil" showTime />
            <DateField source="createdAt" showTime />
            <ReferenceField source="accountId" reference="Account" />
            <FunctionField
              source="briefingJson"
              sortable={false}
              render={(record) => (
                <a target="_blank" href={viewPostUrl(record)} rel="noreferrer">
                  {contentId(record)}
                </a>
              )}
            />
          </SimpleShowLayout>
        </Card>
      </Box>
      <Box flex="1 0 500px">
        <CampaignCollabs campaign={show.record} />
      </Box>
    </Stack>
  );
};

const CampaignCollabs = ({ campaign }) => {
  const t = useTranslate();
  const listContext = useListController({
    resource: "Collab",
    filter: { campaignIdEq: campaign.id },
    sort: { field: "id", order: "DESC" },
    exporter: false,
    perPage: 30,
  });

  if (listContext.isPending || listContext.total == 0) {
    return <></>;
  }

  return (
    <Box my="0.5em">
      <ListBase disableAuthentication disableSyncWithLocation {...listContext}>
        <Stack gap="1em" mb="1em" alignItems="baseline" direction="row">
          <Head2>{t("resources.Campaign.fields.collabs")}</Head2>
          <ExportButton disabled={listContext.total === 0} resource="Collab" />
        </Stack>

        <ListView filters={false} actions={false}>
          <Datagrid bulkActionButtons={false}>
            <ReferenceField
              label={<XIcon sx={{ fontSize: "1em", mb: "-4px" }} />}
              source="handleId"
              reference="Handle"
              sortable={false}
            >
              <FunctionField
                render={(record) => (
                  <Link to={`/Account/${record.accountId}/show`}>
                    <TextField source="username" />
                  </Link>
                )}
              />
            </ReferenceField>
            <AmountField currency="" textAlign="right" source="reward" />
            <AmountField currency="" textAlign="right" source="fee" />
            <TextField source="status" sortable={false} />
          </Datagrid>
        </ListView>
      </ListBase>
    </Box>
  );
};
