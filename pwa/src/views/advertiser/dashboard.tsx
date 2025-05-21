import { useEffect } from "react";
import {
  useUpdate,
  useNotify,
  useRefresh,
  useAuthenticated,
  useSafeSetState,
  useGetOne,
  ListView,
  ListBase,
  DateField,
  NumberField,
  ShowButton,
  ExportButton,
  ReferenceField,
  useRecordContext,
} from "react-admin";
import {
  Box,
  Button,
  Card,
  CardContent,
  Container,
  Skeleton,
  Stack,
  Typography,
} from "@mui/material";
import { formatEther } from "ethers";
import { ColumnsContainer, DeckCard } from "../layout";
import { CardTitle, Head1, Head2, Lead, green } from "../../components/theme";
import { contentId, viewPostUrl } from "../../lib/campaign";
import { Pagination, Datagrid, TextField, FunctionField } from "react-admin";
import { Link } from "react-router-dom";
import {
  useListController,
  ListContextProvider,
  useTranslate,
} from "react-admin";
import { getAuthKeys } from "../../lib/auth_provider";
import { MakeCampaignWithDocCard } from "./make_campaign_card";
import BalanceCard from "../balance_card";
import { ResponsiveAppBar } from "../responsive_app_bar";
import { AmountField } from "../../components/custom_fields";

const Dashboard = () => {
  useAuthenticated();

  const { data, isLoading } = useGetOne(
    "Account",
    { id: getAuthKeys().session.accountId },
    { refetchInterval: (d) => (d?.status == "CLAIMED" ? false : 5000) },
  );

  const listContext = useListController({
    debounce: 500,
    disableSyncWithLocation: true,
    filter: {
      accountIdEq: getAuthKeys().session.accountId,
      statusNe: "DRAFT",
    },
    sort: { field: "createdAt", order: "DESC" },
    perPage: 20,
    queryOptions: {
      refetchInterval: 10000,
    },
    resource: "Campaign",
  });

  if (isLoading || !data) {
    return (
      <Container maxWidth="md">
        <Skeleton animation="wave" />
      </Container>
    );
  }

  return (
    <Box p="1em" id="advertiser-dashboard">
      <ResponsiveAppBar />
      <ColumnsContainer>
        <AdvertiserHelpCard account={data} />
        <MakeCampaignWithDocCard
          account={data}
          onCreate={() => listContext.refetch()}
        />
        <MakeCampaignWithStripeCard />
        {data.status == "CLAIMED" && <BalanceCard />}
      </ColumnsContainer>

      <CampaignList listContext={listContext} />
      <Community />
    </Box>
  );
};

const AdvertiserHelpCard = ({ account }) => {
  const translate = useTranslate();
  return (
    <Box
      id="advertiser-help-card"
      sx={{ breakInside: "avoid" }}
      mb="1em"
      p="0.5em"
    >
      <Head1 sx={{ mb: "0.5em" }}>{translate("advertiser_help.title")}</Head1>
      <Lead>{translate("advertiser_help.text")}</Lead>
    </Box>
  );
};

export const MakeCampaignWithStripeCard = () => {
  const t = useTranslate();

  return (
    <DeckCard>
      <CardContent>
        <Head2>{t("make_campaign.with_stripe.title")}</Head2>
        <Typography my="1em">{t("make_campaign.with_stripe.text")}</Typography>
        <Button fullWidth color="primary" disabled variant="outlined">
          {t("make_campaign.with_stripe.coming_soon")}
        </Button>
      </CardContent>
    </DeckCard>
  );
};

const CampaignList = ({ listContext }) => {
  const translate = useTranslate();

  if (listContext.total < 1) {
    return <></>;
  }

  return (
    <Box id="campaign-list" sx={{ mt: "2em", mb: "2em" }}>
      <ListBase disableAuthentication disableSyncWithLocation {...listContext}>
        <Stack gap="1em" mb="1em" alignItems="baseline" direction="row">
          <Head2>{translate("campaign_list.title")}.</Head2>
          <ExportButton
            disabled={listContext.total === 0}
            resource="Campaign"
          />
        </Stack>
        <ListView filters={false} actions={false}>
          <Datagrid resource="Campaign" bulkActionButtons={false}>
            <FunctionField source="briefingJson" render={contentId} />
            <TextField source="status" sortable={false} />
            <NumberField source="totalCollabs" />
            <AmountField textAlign="right" currency="" source="budget" />
            <AmountField textAlign="right" currency="" source="totalSpent" />
            <AmountField textAlign="right" currency="" source="totalBudget" />
            <DateField source="validUntil" />
            <DateField source="createdAt" />
            <TextField source="id" />
            <ShowButton />
          </Datagrid>
        </ListView>
      </ListBase>
    </Box>
  );
};

const Community = () => {
  const t = useTranslate();
  const listContext = useListController({
    resource: "CommunityMember",
    filter: {
      accountIdEq: getAuthKeys().session.accountId,
    },
    sort: { field: "id", order: "DESC" },
    exporter: false,
    perPage: 30,
  });

  if (listContext.isPending || listContext.total == 0) {
    return <></>;
  }

  const toggleRating = (newRating) => {};

  return (
    <Box my="0.5em">
      <ListBase disableAuthentication disableSyncWithLocation {...listContext}>
        <Stack gap="1em" mb="1em" alignItems="baseline" direction="row">
          <Head2>{t("community.title")}</Head2>
          <ExportButton
            disabled={listContext.total === 0}
            resource="CommunityMember"
          />
        </Stack>
        <Typography my="1em">{t("community.description")}</Typography>

        <ListView filters={false} actions={false}>
          <Datagrid bulkActionButtons={false}>
            <ReferenceField
              source="memberId"
              reference="Account"
              sortable={false}
            >
              <FunctionField
                render={(record) => (
                  <Link to={`/Account/${record.accountId}/show`}>
                    <TextField source="id" />
                  </Link>
                )}
              />
            </ReferenceField>
            <AmountField currency="" textAlign="right" source="rewards" />
            <NumberField textAlign="right" source="collabs" />
            <TextField source="rating" />
            <DateField source="firstCollabDate" />
            <DateField source="lastCollabDate" />
            <ToggleRatingButton value="GOOD" />
            <ToggleRatingButton value="BAD" />
          </Datagrid>
        </ListView>
      </ListBase>
    </Box>
  );
};

const ToggleRatingButton = ({ value }) => {
  const record = useRecordContext();
  const [update, { isLoading }] = useUpdate();
  const notify = useNotify();
  const refresh = useRefresh();

  const handleClick = () => {
    const rating = value == record?.rating ? "NORMAL" : value;

    update(
      "CommunityMember",
      { id: record?.id, data: { rating }, previousData: record },
      {
        onSuccess: () => {
          notify(`Rating set to ${notify}`, { type: "info" });
          refresh(); // optionally reload the list
        },
        onError: (error) => {
          notify(`Error: ${error.message}`, { type: "warning" });
        },
      },
    );
  };

  return (
    <Button onClick={handleClick} disabled={isLoading}>
      {value}
    </Button>
  );
};

export default Dashboard;
