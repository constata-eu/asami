import {
  useUpdate,
  useNotify,
  useRefresh,
  useAuthenticated,
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
  Container,
  IconButton,
  Skeleton,
  Stack,
  Typography,
} from "@mui/material";
import { ColumnsContainer } from "../layout";
import { Head1, Head2, Lead } from "../../components/theme";
import { contentId } from "../../lib/campaign";
import { Datagrid, TextField, FunctionField } from "react-admin";
import { useListController, useTranslate } from "react-admin";
import { getAuthKeys } from "../../lib/auth_provider";
import { MakeCampaignWithDocCard } from "./make_campaign_card";
import { MakeCampaignStripe } from "./make_campaign_stripe";
import BalanceCard from "../balance_card";
import { ResponsiveAppBar } from "../responsive_app_bar";
import { AmountField } from "../../components/custom_fields";
import ThumbUpIcon from "@mui/icons-material/ThumbUp";
import ThumbDownIcon from "@mui/icons-material/ThumbDown";

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

  if (isLoading || listContext.isLoading || !data) {
    return (
      <Container maxWidth="md">
        <Skeleton animation="wave" />
      </Container>
    );
  }

  return (
    <Box id="advertiser-dashboard">
      <ResponsiveAppBar />
      <ColumnsContainer>
        <AdvertiserHelpCard />
        <MakeCampaignWithDocCard
          account={data}
          onCreate={() => listContext.refetch()}
        />
        <MakeCampaignStripe
          account={data}
          onCreate={() => listContext.refetch()}
        />
        {data.status == "CLAIMED" && <BalanceCard />}
      </ColumnsContainer>

      <CampaignList listContext={listContext} />
      <Community />
    </Box>
  );
};

const AdvertiserHelpCard = () => {
  const t = useTranslate();
  return (
    <Box
      id="advertiser-help-card"
      sx={{ breakInside: "avoid" }}
      mb="1em"
      p="0.5em"
    >
      <Head1 sx={{ mb: "0.5em", color: "primary.main" }}>
        {t("advertiser_help.title")}
      </Head1>
      <Lead>{t("advertiser_help.text")}</Lead>
    </Box>
  );
};

const CampaignList = ({ listContext }) => {
  const t = useTranslate();

  if (listContext.total < 1) {
    return <></>;
  }

  return (
    <Box id="campaign-list" sx={{ mt: "2em", mb: "2em" }}>
      <ListBase disableAuthentication disableSyncWithLocation {...listContext}>
        <Stack gap="1em" mb="1em" alignItems="baseline" direction="row">
          <Head2>{t("campaign_list.title")}.</Head2>
          <ExportButton
            disabled={listContext.total === 0}
            resource="Campaign"
          />
        </Stack>
        <ListView filters={false} actions={false}>
          <Datagrid
            rowClick={false}
            resource="Campaign"
            bulkActionButtons={false}
          >
            <FunctionField source="briefingJson" render={contentId} />
            <FunctionField
              source="status"
              render={(r) =>
                t(`resources.Campaign.statuses.${r.privateFields.status}`)
              }
              sortable={false}
            />
            <NumberField source="totalCollabs" />
            <AmountField textAlign="right" currency="" source="budget" />
            <AmountField textAlign="right" currency="" source="totalSpent" />
            <AmountField textAlign="right" currency="" source="totalBudget" />
            <DateField source="validUntil" />
            <DateField source="createdAt" />
            <TextField source="id" />
            <FunctionField
              render={(r) => (
                <>
                  {r.privateFields.status == "PUBLISHED" && <ShowButton />}
                  {r.privateFields.status == "AWAITING_PAYMENT" && (
                    <FunctionField
                      source="stripeSessionUrl"
                      render={(r) => (
                        <Button
                          id={`btn-go-to-checkout-for-${r.id}`}
                          onClick={false}
                          href={r.privateFields.stripeSessionUrl}
                          target="_blank"
                        >
                          {t("campaign_request_list.go_to_stripe")}
                        </Button>
                      )}
                    />
                  )}
                </>
              )}
            />
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
              <TextField source="id" />
              <TextField source="name" />
            </ReferenceField>
            <AmountField currency="" textAlign="right" source="rewards" />
            <NumberField textAlign="right" source="collabs" />
            <DateField source="firstCollabDate" />
            <DateField source="lastCollabDate" />
            <FunctionField
              source="rating"
              render={() => {
                return (
                  <>
                    <ToggleRatingButton value="GOOD" icon={ThumbUpIcon} />
                    <ToggleRatingButton value="BAD" icon={ThumbDownIcon} />
                  </>
                );
              }}
            />
            <Box label={false}>
              <ReferenceField
                source="memberId"
                reference="Account"
                sortable={false}
              >
                <ShowButton />
              </ReferenceField>
            </Box>
          </Datagrid>
        </ListView>
      </ListBase>
    </Box>
  );
};

const ToggleRatingButton = ({ value, icon: Icon }) => {
  const record = useRecordContext();
  const [update, { isLoading }] = useUpdate();
  const notify = useNotify();
  const refresh = useRefresh();

  const selected = record?.rating == value;
  const id = `rating-${record?.memberId}-${value}`;

  const handleClick = () => {
    const rating = value == record?.rating ? "NORMAL" : value;

    update(
      "CommunityMember",
      { id: record?.id, data: { rating }, previousData: record },
      {
        onSuccess: () => {
          notify("community.rating_success", { type: "info" });
          refresh(); // optionally reload the list
        },
        onError: (error) => {
          notify(`Error: ${error.message}`, { type: "error" });
        },
      },
    );
  };

  return (
    <IconButton id={id} onClick={handleClick} size="small" disabled={isLoading}>
      <Icon fontSize="small" color={selected ? "primary" : "normal"} />
    </IconButton>
  );
};

export default Dashboard;
