import {
  Form,
  NumberInput,
  SaveButton,
  TextInput as RaTextInput,
  useDataProvider,
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
import { useState } from "react";
import {
  Card,
  CardContent,
  Box,
  Button,
  Container,
  Divider,
  IconButton,
  Skeleton,
  Stack,
  TextField as MuiTextField,
  Typography,
} from "@mui/material";
import GroupAddIcon from "@mui/icons-material/GroupAdd";
import PersonAddAlt1Icon from "@mui/icons-material/PersonAddAlt1";
import { ColumnsContainer } from "../layout";
import { Head1, Head2, Lead } from "../../components/theme";
import { campaignLabel } from "../../lib/campaign";
import { Datagrid, TextField, FunctionField } from "react-admin";
import { useListController, useTranslate } from "react-admin";
import { getAuthKeys } from "../../lib/auth_provider";
import {
  ContinueCampaignButton,
  MakeCampaignWithDocCard,
} from "./make_campaign_card";
import BalanceCard from "../balance_card";
import { ResponsiveAppBar } from "../responsive_app_bar";
import { AmountField } from "../../components/custom_fields";
import ThumbUpIcon from "@mui/icons-material/ThumbUp";
import ThumbDownIcon from "@mui/icons-material/ThumbDown";
import { GrantPermissionsAgainDialog } from "../member/handle_settings";
import { useNavigate } from "react-router-dom";
import { normalizeRootstockJoinedAt } from "../../lib/rootstock_joined_at";

const Dashboard = () => {
  useAuthenticated();

  const { data, isLoading } = useGetOne(
    "Account",
    { id: getAuthKeys().session.accountId },
    { refetchInterval: (d) => (d?.status == "CLAIMED" ? false : 5000) },
  );

  const handles = useListController({
    disableSyncWithLocation: true,
    filter: { accountIdEq: getAuthKeys().session.accountId },
    queryOptions: {
      refetchInterval: 20000,
    },
    resource: "Handle",
  });

  const listContext = useListController({
    debounce: 500,
    disableSyncWithLocation: true,
    filter: {
      accountIdEq: getAuthKeys().session.accountId,
    },
    sort: { field: "createdAt", order: "DESC" },
    perPage: 20,
    queryOptions: {
      refetchInterval: 10000,
    },
    resource: "Campaign",
  });

  if (isLoading || listContext.isLoading || handles.isLoading || !data) {
    return (
      <Container maxWidth="md">
        <Skeleton animation="wave" />
      </Container>
    );
  }

  const handle = handles.data?.[0];

  return (
    <Box id="advertiser-dashboard">
      <ResponsiveAppBar />
      {handle?.status == "DISCONNECTED" && <GrantPermissionsAgainDialog />}
      <Stack
        mb="3em"
        gap="2em"
        direction="row"
        flexWrap="wrap"
        alignItems="stretch"
      >
        <Box flex="1 1 350px" display="flex">
          <AdvertiserHelpCard />
        </Box>
        <Box
          flex="1 1 800px"
          alignItems="stretch"
          display="flex"
          flexWrap="wrap"
          gap="1em"
        >
          <Box display="flex" flex="1 1 270px">
            <MakeCampaignWithDocCard
              account={data}
              onCreate={() => listContext.refetch()}
            />
          </Box>
          <Box display="flex" flex="1 1 270px">
            <CommunityTools account={data} />
          </Box>
          {data.status == "CLAIMED" && (
            <Box display="flex" flex="1 1 270px">
              <BalanceCard />
            </Box>
          )}
        </Box>
      </Stack>

      <CampaignList listContext={listContext} account={data} />
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

const CampaignList = ({ listContext, account }) => {
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
            <FunctionField source="briefingJson" render={campaignLabel} />
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
                <Stack>
                  {r.privateFields.status == "PUBLISHED" && <ShowButton />}
                  {r.privateFields.status == "DRAFT" && (
                    <FunctionField
                      source="id"
                      render={(r) => (
                        <ContinueCampaignButton
                          account={account}
                          campaign={r}
                          onCreate={() => listContext.refetch()}
                          id={`btn-resume-doc-payment-for-${r.id}`}
                        />
                      )}
                    />
                  )}
                </Stack>
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

  if (listContext.isPending) {
    return <></>;
  }

  return (
    <Box my="0.5em">
      {listContext.total === 0 ? (
        <Stack gap="1em" mb="1em" alignItems="baseline" direction="row">
          <Head2>{t("community.title")}</Head2>
          <Card>
            <CardContent>
              <Typography fontFamily="'LeagueSpartanBold'" fontSize="1.2em">
                {t("community.empty")}
              </Typography>
            </CardContent>
          </Card>
        </Stack>
      ) : (
        <ListBase
          disableAuthentication
          disableSyncWithLocation
          {...listContext}
        >
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
      )}
    </Box>
  );
};

const CommunityTools = ({ account }) => {
  const t = useTranslate();
  const navigate = useNavigate();

  return (
    <Card sx={{ maxWidth: "720px" }}>
      <CardContent
        sx={{
          height: "100%",
          display: "flex",
          flexDirection: "column",
          justifyContent: "space-between",
        }}
      >
        <Head2>{t("community.manage_title")}</Head2>
        <Typography my="0.75em">{t("community.manage_text")}</Typography>
        <Button
          variant="outlined"
          startIcon={<GroupAddIcon />}
          onClick={() => navigate("/Account")}
        >
          {t("community.add_existing_button")}
        </Button>
        <Divider sx={{ my: "1em" }} />
        <Button
          variant="contained"
          startIcon={<PersonAddAlt1Icon />}
          disabled={account.availableInvites <= 0}
          onClick={() => navigate("/community/invite")}
        >
          {t("community.invite_button")}
        </Button>
        <Typography variant="body2">
          {account.availableInvites > 0
            ? t("community.available_invites", {
                count: account.availableInvites,
              })
            : t("community.no_invites")}
        </Typography>
      </CardContent>
    </Card>
  );
};

export const InviteRootstockerPage = () => {
  useAuthenticated();
  const t = useTranslate();
  const notify = useNotify();
  const dataProvider = useDataProvider();
  const navigate = useNavigate();
  const [inviteUrl, setInviteUrl] = useState<string | null>(null);
  const search = new URLSearchParams(window.location.hash.split("?")[1] || "");
  const { data, isLoading } = useGetOne(
    "Account",
    { id: getAuthKeys().session.accountId },
    { refetchInterval: 10000 },
  );

  if (isLoading || !data) {
    return (
      <Container maxWidth="md">
        <Skeleton animation="wave" />
      </Container>
    );
  }

  const onSubmit = async (values: Record<string, any>) => {
    try {
      const response = await dataProvider.create("Invitation", {
        data: {
          accountName: values.accountName,
          alignment: Number(values.alignment),
          trust: Number(values.trust),
          contribution: Number(values.contribution),
          advocacy: Number(values.advocacy),
          judgment: Number(values.judgment),
          rootstockJoinedAt: normalizeRootstockJoinedAt(values.rootstockJoinedAt),
          comment: values.comment || null,
          priority: false,
        },
      });
      setInviteUrl(response.data.inviteUrl);
      notify(t("community.invitation_created"), { type: "success" });
    } catch (error: any) {
      notify(error?.message, { type: "error" });
    }
  };

  const copy = async () => {
    if (!inviteUrl) return;
    await navigator.clipboard.writeText(inviteUrl);
    notify(t("community.invitation_copied"), { type: "success" });
  };

  return (
    <Box id="advertiser-dashboard">
      <ResponsiveAppBar />
      <Stack gap="1em" maxWidth="820px">
        <Head1>{t("community.invite_page_title")}</Head1>
        <Lead>{t("community.invite_page_text")}</Lead>
        <Typography variant="body2" sx={{ opacity: 0.75 }}>
          {data.availableInvites > 0
            ? t("community.available_invites", { count: data.availableInvites })
            : t("community.no_invites")}
        </Typography>
        <Card>
          <CardContent>
            <Form
              sanitizeEmptyValues
              onSubmit={onSubmit}
              defaultValues={{
                accountName: search.get("name") || "",
                alignment: 5,
                trust: 5,
                contribution: 5,
                advocacy: 5,
                judgment: 5,
                comment: search.get("comment") || "",
              }}
            >
              <Stack gap="0.75em">
                <RaTextInput source="accountName" label={t("community.invite_fields.name")} fullWidth />
                <NumberInput source="alignment" label={t("community.invite_fields.alignment")} helperText={t("community.invite_fields.alignment_help")} />
                <NumberInput source="trust" label={t("community.invite_fields.trust")} helperText={t("community.invite_fields.trust_help")} />
                <NumberInput source="contribution" label={t("community.invite_fields.contribution")} helperText={t("community.invite_fields.contribution_help")} />
                <NumberInput source="advocacy" label={t("community.invite_fields.advocacy")} helperText={t("community.invite_fields.advocacy_help")} />
                <NumberInput source="judgment" label={t("community.invite_fields.judgment")} helperText={t("community.invite_fields.judgment_help")} />
                <RaTextInput
                  source="rootstockJoinedAt"
                  type="date"
                  label={t("community.invite_fields.rootstock_joined_at")}
                  helperText={t("community.invite_fields.rootstock_joined_at_help")}
                  fullWidth
                />
                <RaTextInput
                  source="comment"
                  label={t("community.invite_fields.comment")}
                  helperText={t("community.invite_fields.comment_help")}
                  multiline
                  minRows={3}
                  fullWidth
                />
                <Stack direction={{ xs: "column", md: "row" }} gap="0.75em">
                  <SaveButton
                    disabled={data.availableInvites <= 0}
                    label={t("community.create_invite_button")}
                  />
                  <Button
                    variant="outlined"
                    onClick={() => navigate("/dashboard?role=advertiser")}
                  >
                    {t("make_campaign.cancel")}
                  </Button>
                </Stack>
              </Stack>
            </Form>
            {inviteUrl && (
              <Stack mt="1em" gap="0.5em">
                <Typography sx={{ wordBreak: "break-all" }}>
                  {inviteUrl}
                </Typography>
                <Button onClick={copy}>
                  {t("community.copy_invite_button")}
                </Button>
              </Stack>
            )}
          </CardContent>
        </Card>
      </Stack>
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
