import {
  AutocompleteInput,
  BooleanInput,
  Datagrid,
  EditBase,
  EditButton,
  Form,
  List,
  useTranslate,
  TextField,
  FunctionField,
  TextInput,
  DateField,
  NumberField,
  NumberInput,
  useShowController,
  useListController,
  RecordContextProvider,
  ShowButton,
  SimpleShowLayout,
  ListBase,
  ReferenceField,
  ReferenceInput,
  ListView,
  SaveButton,
  SelectInput,
  Toolbar,
  SimpleForm,
  useDataProvider,
  useNotify,
  useRefresh,
  required,
  minValue,
  maxValue,
  useRecordContext,
} from "react-admin";
import { viewPostUrl } from "../../lib/campaign";
import { useParams, Link } from "react-router-dom";
import { ExplorerLayout } from "../layout";
import { ResponsiveAppBar } from "../responsive_app_bar";
import {
  Alert,
  Box,
  Button,
  Card,
  CardContent,
  Stack,
  styled,
  Typography,
} from "@mui/material";
import { AddressField, AmountField } from "../../components/custom_fields";
import BadgeIcon from "@mui/icons-material/Badge";
import CampaignIcon from "@mui/icons-material/Campaign";
import FavoriteIcon from "@mui/icons-material/Favorite";
import Diversity1Icon from "@mui/icons-material/Diversity1";
import ForumIcon from "@mui/icons-material/Forum";
import VerifiedIcon from "@mui/icons-material/Verified";
import GrassIcon from "@mui/icons-material/Grass";
import GroupAddIcon from "@mui/icons-material/GroupAdd";

import { Head1, Head2, Lead, BigText } from "../../components/theme";
import { AttributeTable } from "../../components/attribute_table";
import { normalizeRootstockJoinedAt } from "../../lib/rootstock_joined_at";

const acceptanceStatusChoices = [
  { id: "ACTIVE", name: "Active" },
  { id: "GUESTLIST", name: "Waitlist" },
  { id: "INACTIVE", name: "Inactive" },
];

export const AccountList = () => {
  const t = useTranslate();
  const storedSession = localStorage.getItem("session");
  const session = storedSession ? JSON.parse(storedSession) : null;

  const filters = [
    <TextInput source="nameLike" alwaysOn />,
    <TextInput source="idEq" alwaysOn />,
    <TextInput source="addrLike" alwaysOn />,
    <SelectInput
      source="acceptanceStatusEq"
      alwaysOn
      choices={acceptanceStatusChoices}
    />,
  ];

  return (
    <ExplorerLayout>
      <Head1>{t("explorer.accounts.title")}</Head1>
      <Lead>{t("explorer.accounts.description")}</Lead>

      <List
        disableAuthentication
        filters={filters}
        filterDefaultValues={{ acceptanceStatusEq: "ACTIVE" }}
        sort={{ field: "createdAt", order: "DESC" }}
      >
        <Datagrid bulkActionButtons={false} expand={<ExpandAccount />}>
          <Box source="name">
            <TextField
              source="name"
              defaultValue={t("account_show.title.default_name")}
            />
          </Box>
          <DateField source="createdAt" />
          <FunctionField
            source="acceptanceStatus"
            label={t("explorer.accounts.acceptance_status")}
            render={(record) =>
              acceptanceStatusChoices.find(
                (choice) => choice.id === record.acceptanceStatus,
              )?.name ?? record.acceptanceStatus
            }
          />
          <TextField
            source="invitedByName"
            label={t("explorer.accounts.invited_by")}
          />
          <NumberField textAlign="right" source="totalCollabs" />
          <AmountField
            textAlign="right"
            currency=""
            source="totalCollabRewards"
          />
          <NumberField textAlign="right" source="totalCampaigns" />
          <NumberField textAlign="right" source="totalCollabsReceived" />
          <AmountField textAlign="right" currency="" source="totalSpent" />
          {session && <AddToCommunityButton />}
          <ShowButton textAlign="right" />
          {session?.admin && <EditButton label={false} textAlign="right" />}
        </Datagrid>
      </List>
    </ExplorerLayout>
  );
};

const ExpandAccount = () => (
  <SimpleShowLayout>
    <AddressField source="addr" />
    <AmountField textAlign="right" currency="" source="unclaimedAsamiBalance" />
    <AmountField textAlign="right" currency="" source="unclaimedDocBalance" />
  </SimpleShowLayout>
);

export const AccountShow = () => (
  <ExplorerLayout>
    <Box mt="1em">
      <AccountCardTable />
    </Box>
  </ExplorerLayout>
);

const AccountCardTable = () => {
  let t = useTranslate();
  const { id } = useParams(); // id of the campaign from route
  const storedSession = localStorage.getItem("session");
  const session = storedSession ? JSON.parse(storedSession) : null;

  const showAccount = useShowController({ disableAuthentication: true });

  const listHandles = useListController({
    disableSyncWithLocation: true,
    filter: { accountIdEq: id },
    resource: "Handle",
  });

  if (showAccount.isPending || !showAccount.record || listHandles.isPending) {
    return <></>;
  }

  const handle = listHandles.data?.[0];

  return (
    <>
      <RecordContextProvider value={showAccount.record}>
        <Head1 sx={{ color: "primary.main" }}>{showAccount.record.name}</Head1>
        {session?.admin && (
          <Box mt="0.5em">
            <EditButton resource="Account" record={showAccount.record} />
          </Box>
        )}
        {(handle?.status == "DISCONNECTED" ||
          handle?.status == "RECONNECTING") && (
          <Alert
            id="account-disconnected"
            sx={{ mt: "1em" }}
            severity="warning"
          >
            {t("account_show.disconnected")}
          </Alert>
        )}
        <HorizontalCardTable mt="2em">
          {handle && (
            <ReputationCardTable account={showAccount.record} handle={handle} />
          )}
          {showAccount.record.totalCampaigns > 0 && (
            <FixedWidthCard elevation={10}>
              <CardContent>
                <CardTitle>{t("account_show.advertiser_title")}</CardTitle>
                <CampaignIcon sx={{ fontSize: 90, flexGrow: 1 }} />
                <AttributeTable>
                  <NumberField source="totalCampaigns" />
                  <AmountField
                    textAlign="right"
                    currency=""
                    source="totalSpent"
                  />
                  <NumberField source="totalCollabsReceived" />
                </AttributeTable>
              </CardContent>
            </FixedWidthCard>
          )}
          <FixedWidthCard elevation={10}>
            <CardContent>
              <CardTitle>{t("account_show.member_info_title")}</CardTitle>
              <BadgeIcon sx={{ fontSize: 90, flexGrow: 1 }} />
              <AttributeTable>
                <DateField source="createdAt" />
                <FunctionField
                  label={t("explorer.accounts.acceptance_status")}
                  render={(record) =>
                    acceptanceStatusChoices.find(
                      (choice) => choice.id === record.acceptanceStatus,
                    )?.name ?? record.acceptanceStatus
                  }
                />
                <TextField
                  source="invitedByName"
                  label={t("explorer.accounts.invited_by")}
                />
                <AddressField source="addr" />
                <AmountField
                  textAlign="right"
                  source="unclaimedAsamiBalance"
                  currency=""
                />
                <AmountField
                  textAlign="right"
                  source="unclaimedDocBalance"
                  currency=""
                />
              </AttributeTable>
            </CardContent>
          </FixedWidthCard>
        </HorizontalCardTable>
      </RecordContextProvider>
      <AccountCollabs account={showAccount.record} />
      <AccountCampaigns account={showAccount.record} />
    </>
  );
};

export const AccountEdit = () => {
  const t = useTranslate();
  const storedSession = localStorage.getItem("session");
  const session = storedSession ? JSON.parse(storedSession) : null;

  if (!session?.admin) {
    return (
      <Box id="asami-explorer">
        <ExplorerLayout>
          <Alert severity="error">{t("explorer.accounts.admin_only")}</Alert>
        </ExplorerLayout>
      </Box>
    );
  }

  return (
    <Box id="asami-explorer">
      <ResponsiveAppBar expandExplorer={true} />
      <EditBase mutationMode="pessimistic">
        <FunctionField
          render={(r) => (
            <Head1>
              {t("explorer.accounts.review_member", { name: r.name })}
            </Head1>
          )}
        />
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
              <SelectInput
                sx={{ m: "0", flex: "1 1 260px" }}
                source="acceptanceStatus"
                choices={acceptanceStatusChoices}
                label={t("explorer.accounts.acceptance_status")}
              />
              <ReferenceInput
                sx={{ m: "0", flex: "2 1 360px" }}
                source="invitedByAccountId"
                reference="Account"
                perPage={25}
                filter={{ acceptanceStatusEq: "ACTIVE" }}
                label={t("explorer.accounts.invited_by")}
              >
                <AutocompleteInput optionText="name" />
              </ReferenceInput>
            </Stack>
            <Stack direction="row" gap="1em" flexWrap="wrap">
              <NumberInput
                sx={{ m: "0", flex: "1 1 260px" }}
                source="invitesPerMonth"
                label={t("explorer.accounts.invites_per_month")}
              />
              <NumberInput
                sx={{ m: "0", flex: "1 1 260px" }}
                source="availableInvites"
                label={t("explorer.accounts.available_invites")}
              />
            </Stack>
          </SimpleForm>
        </Card>
        <AccountCommunityMemberForm />
      </EditBase>
    </Box>
  );
};

const scoreValueValidate = [required(), minValue(1), maxValue(10)];

const AddToCommunityButton = () => {
  const t = useTranslate();
  const record = useRecordContext();
  const dataProvider = useDataProvider();
  const notify = useNotify();
  const refresh = useRefresh();
  const storedSession = localStorage.getItem("session");
  const session = storedSession ? JSON.parse(storedSession) : null;

  if (!session?.accountId || session.accountId == record?.id) {
    return null;
  }

  const addMember = async () => {
    try {
      await dataProvider.create("CommunityMemberMembership", {
        data: { memberId: record.id },
      });
      notify(t("community.member_added"), { type: "success" });
      refresh();
    } catch (error: any) {
      notify(error?.message || t("community.member_add_failed"), {
        type: "error",
      });
    }
  };

  return (
    <Button
      variant="text"
      size="small"
      onClick={addMember}
      startIcon={<GroupAddIcon />}
    >
      {t("community.add_button")}
    </Button>
  );
};

const AccountCommunityMemberForm = () => {
  const t = useTranslate();
  const notify = useNotify();
  const refresh = useRefresh();
  const dataProvider = useDataProvider();
  const { id } = useParams();
  const showAccount = useShowController({ disableAuthentication: true, id });

  if (showAccount.isPending || !showAccount.record) {
    return <></>;
  }

  const onSubmit = async (values) => {
    try {
      await dataProvider.create("CommunityMember", {
        data: {
          memberId: showAccount.record.id,
          priority: !!values.priority,
          alignment: Number(values.alignment),
          trust: Number(values.trust),
          contribution: Number(values.contribution),
          advocacy: Number(values.advocacy),
          judgment: Number(values.judgment),
          rootstockJoinedAt: normalizeRootstockJoinedAt(values.rootstockJoinedAt),
          comment: values.comment || null,
        },
      });
      notify(t("explorer.accounts.score_created"), { type: "success" });
      refresh();
    } catch (error: any) {
      notify(error?.message || t("explorer.accounts.score_failed"), {
        type: "error",
      });
    }
  };

  return (
    <Card sx={{ mt: "1em" }}>
      <CardContent>
        <Head2>{t("explorer.accounts.score_member")}</Head2>
        <Lead>{t("explorer.accounts.score_member_description")}</Lead>
        <Form
          sanitizeEmptyValues
          onSubmit={onSubmit}
          defaultValues={{
            priority: false,
            alignment: 5,
            trust: 5,
            contribution: 5,
            advocacy: 5,
            judgment: 5,
          }}
        >
          <BooleanInput
            source="priority"
            label={t("explorer.accounts.priority_override")}
          />
          <Stack direction="row" gap="1em" flexWrap="wrap">
            <NumberInput
              sx={{ m: "0", flex: "1 1 180px" }}
              source="alignment"
              validate={scoreValueValidate}
            />
            <NumberInput
              sx={{ m: "0", flex: "1 1 180px" }}
              source="trust"
              validate={scoreValueValidate}
            />
            <NumberInput
              sx={{ m: "0", flex: "1 1 180px" }}
              source="contribution"
              validate={scoreValueValidate}
            />
            <NumberInput
              sx={{ m: "0", flex: "1 1 180px" }}
              source="advocacy"
              validate={scoreValueValidate}
            />
            <NumberInput
              sx={{ m: "0", flex: "1 1 180px" }}
              source="judgment"
              validate={scoreValueValidate}
            />
          </Stack>
          <Stack direction="row" gap="1em" flexWrap="wrap">
            <TextInput
              sx={{ m: "0", flex: "1 1 260px" }}
              source="rootstockJoinedAt"
              type="date"
              validate={required()}
              label={t("account_show.longevity_card_title")}
            />
            <TextInput
              sx={{ m: "0", flex: "2 1 360px" }}
              fullWidth
              multiline
              minRows={3}
              source="comment"
              label={t("explorer.accounts.score_comment")}
            />
          </Stack>
          <Toolbar sx={{ px: 0 }}>
            <SaveButton label={t("explorer.accounts.submit_score")} />
          </Toolbar>
        </Form>
      </CardContent>
    </Card>
  );
};

const ReputationCardTable = ({ account, handle }: any) => {
  const t = useTranslate();
  const scoreCardsReceived = account.scoresReceivedCount ?? 0;
  const scoreCardsIssued = account.scoresIssuedCount ?? 0;
  const formatted = new Intl.NumberFormat("en-US", {
    minimumFractionDigits: 1,
    maximumFractionDigits: 1,
  });

  const cards = [
    {
      title: "alignment_card_title",
      icon: FavoriteIcon,
      value: account.alignmentScore ?? account.baseAlignmentScore,
    },
    {
      title: "trust_card_title",
      icon: VerifiedIcon,
      value: account.trustScore ?? account.baseTrustScore,
    },
    {
      title: "contribution_card_title",
      icon: ForumIcon,
      value: account.contributionScore ?? account.baseContributionScore,
    },
    {
      title: "advocacy_card_title",
      icon: CampaignIcon,
      value: account.advocacyScore ?? account.baseAdvocacyScore,
    },
    {
      title: "judgment_card_title",
      icon: Diversity1Icon,
      value: account.judgmentScore ?? account.baseJudgmentScore,
    },
    {
      title: "longevity_card_title",
      icon: GrassIcon,
      value:
        formatted.format(account.rootstockLongevityScore) ??
        formatted.format(account.rootstockSeniorityScore),
    },
  ];

  return (
    <>
      {cards.map(({ title, icon: Icon, value }) => (
        <FixedWidthCard key={title}>
          <CardContent>
            <CardTitle>{t(`account_show.${title}`)}</CardTitle>
            <Icon sx={{ fontSize: 82, margin: "26px" }} />
            <BigText sx={{ textAlign: "center" }}>
              {value ?? t("account_show.not_scored_yet")}
            </BigText>
          </CardContent>
        </FixedWidthCard>
      ))}
      <FixedWidthCard>
        <CardContent>
          <CardTitle>{t("account_show.scores_title")}</CardTitle>
          <BadgeIcon sx={{ fontSize: 82, margin: "26px" }} />
          <Box sx={{ width: "100%" }}>
            <AttributeTable
              record={{
                scoresReceived: scoreCardsReceived,
                scoresIssued: scoreCardsIssued,
                score: account.reputationScore,
              }}
            >
              <NumberField
                source="scoresReceived"
                label={t("account_show.scores_received")}
              />
              <NumberField
                source="scoresIssued"
                label={t("account_show.scores_issued")}
              />
              <FunctionField
                label={t("account_show.score")}
                render={(record) =>
                  record.score ?? t("account_show.not_scored_yet")
                }
              />
            </AttributeTable>
          </Box>
        </CardContent>
      </FixedWidthCard>
    </>
  );
};

const FixedWidthCard = ({ id, color, children, ...props }: any) => {
  return (
    <Card
      id={id}
      {...props}
      elevation={5}
      sx={{
        color: color,
        height: "270px",
        width: "190px",
        marginBottom: "1em",
        breakInside: "avoid",
      }}
    >
      {children}
    </Card>
  );
};

const HorizontalCardTable = ({ children, ...props }: any) => (
  <Box
    sx={{
      display: "flex",
      gap: "1em",
      flexWrap: "wrap",
      "& .MuiCardContent-root": {
        alignItems: "center",
        display: "flex !important",
        flexDirection: "column",
        height: "100%",
        paddingBottom: "16px !important",
      },
    }}
    {...props}
    children={children}
  />
);

const CardTitle = styled("h3")(({ theme }) => ({
  fontFamily: "'LeagueSpartanBold'",
  fontSize: "20px",
  lineHeight: "1em",
  letterSpacing: "-0.05em",
  margin: 0,
}));

const AccountCollabs = ({ account }) => {
  const t = useTranslate();
  const listContext = useListController({
    resource: "Collab",
    filter: { memberIdEq: account.id },
    sort: { field: "createdAt", order: "DESC" },
    perPage: 5,
  });

  if (listContext.isPending || listContext.total == 0) {
    return <></>;
  }

  return (
    <>
      <Head2 sx={{ mt: "1em" }}>{t("account_show.collabs_title")}</Head2>
      <ListBase disableAuthentication disableSyncWithLocation {...listContext}>
        <ListView>
          <Datagrid bulkActionButtons={false}>
            <ReferenceField
              source="advertiserId"
              reference="Account"
              sortable={false}
              link="show"
            />
            <FunctionField
              source="campaignId"
              render={(record) => (
                <Link
                  to={`/Campaign?displayedFilters=%7B%7D&filter=%7B%22idEq%22%3A${record.campaignId}%7D`}
                >
                  <TextField source="campaignId" />
                </Link>
              )}
            />
            <TextField source="createdAt" />
            <AmountField textAlign="right" source="reward" />
            <AmountField textAlign="right" source="fee" />
            <TextField source="status" sortable={false} />
            <TextField source="id" />
          </Datagrid>
        </ListView>
      </ListBase>
    </>
  );
};

const AccountCampaigns = ({ account }) => {
  const t = useTranslate();
  const listContext = useListController({
    resource: "Campaign",
    filter: { accountIdEq: account.id },
    sort: { field: "createdAt", order: "DESC" },
    perPage: 5,
  });

  if (listContext.isPending || listContext.total == 0) {
    return <></>;
  }

  return (
    <>
      <Head2 sx={{ mt: "1em" }}>{t("account_show.campaigns_title")}</Head2>
      <ListBase disableAuthentication disableSyncWithLocation {...listContext}>
        <ListView>
          <Datagrid bulkActionButtons={false}>
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
        </ListView>
      </ListBase>
    </>
  );
};
