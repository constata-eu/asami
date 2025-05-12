import { useEffect } from "react";
import {
  useAuthenticated,
  useSafeSetState,
  useGetOne,
  ListView,
  ListBase,
  DateField,
  NumberField,
} from "react-admin";
import {
  Box,
  Button,
  Card,
  CardContent,
  Container,
  Skeleton,
  Typography,
} from "@mui/material";
import { formatEther } from "ethers";
import { ColumnsContainer, DeckCard } from "../layout";
import { CardTitle, Head1, Head2, Lead, green } from "../../components/theme";
import { viewPostUrl } from "../../lib/campaign";
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
    <Box id="campaign-list" sx={{ mt: "1em", mb: "2em" }}>
      <Head2 sx={{ mt: "2em" }}>{translate("campaign_list.title")}.</Head2>
      <ListBase disableAuthentication disableSyncWithLocation {...listContext}>
        <ListView>
          <Datagrid resource="Campaign" bulkActionButtons={false}>
            <FunctionField
              label={translate("campaign_list.post")}
              render={(record) => (
                <a target="_blank" href={viewPostUrl(record)} rel="noreferrer">
                  {translate("campaign_list.see_post")}
                </a>
              )}
            />
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
            <TextField source="id" />
          </Datagrid>
        </ListView>
      </ListBase>
    </Box>
  );
};

export default Dashboard;
