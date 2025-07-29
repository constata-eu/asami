import {
  Datagrid,
  List,
  useTranslate,
  TextField,
  FunctionField,
  TextInput,
  DateField,
  NumberField,
  useShowController,
  useListController,
  RecordContextProvider,
  ShowButton,
  ListBase,
  ReferenceField,
  ListView,
} from "react-admin";
import { viewPostUrl } from "../../lib/campaign";
import { useParams, Link } from "react-router-dom";
import { ExplorerLayout } from "../layout";
import {
  Alert,
  Box,
  Card,
  CardContent,
  styled,
  Typography,
} from "@mui/material";
import { AddressField, AmountField } from "../../components/custom_fields";
import XIcon from "@mui/icons-material/X";
import BadgeIcon from "@mui/icons-material/Badge";
import CampaignIcon from "@mui/icons-material/Campaign";
import FavoriteIcon from "@mui/icons-material/Favorite";
import Diversity1Icon from "@mui/icons-material/Diversity1";
import ForumIcon from "@mui/icons-material/Forum";
import CachedIcon from "@mui/icons-material/Cached";
import AlternateEmailIcon from "@mui/icons-material/AlternateEmail";
import SpeedIcon from "@mui/icons-material/Speed";
import SmartToyIcon from "@mui/icons-material/SmartToy";
import SentimentSatisfiedIcon from "@mui/icons-material/SentimentSatisfied";
import SentimentVerySatisfiedIcon from "@mui/icons-material/SentimentVerySatisfied";
import SentimentNeutralIcon from "@mui/icons-material/SentimentNeutral";
import SentimentVeryDissatisfiedIcon from "@mui/icons-material/SentimentVeryDissatisfied";
import SentimentDissatisfiedIcon from "@mui/icons-material/SentimentDissatisfied";
import VisibilityOffIcon from "@mui/icons-material/VisibilityOff";
import VerifiedIcon from "@mui/icons-material/Verified";
import GroupAddIcon from "@mui/icons-material/GroupAdd";
import DiamondIcon from "@mui/icons-material/Diamond";
import StarHalfIcon from "@mui/icons-material/StarHalf";
import StarIcon from "@mui/icons-material/Star";
import GrassIcon from "@mui/icons-material/Grass";

import { red, Head1, Head2, Lead, BigText } from "../../components/theme";
import { AttributeTable } from "../../components/attribute_table";

export const AccountList = () => {
  const t = useTranslate();

  const filters = [
    <TextInput source="idEq" alwaysOn />,
    <TextInput source="addrLike" alwaysOn />,
    <TextInput source="nameLike" alwaysOn />,
  ];

  return (
    <ExplorerLayout>
      <Head1>{t("explorer.accounts.title")}</Head1>
      <Lead>{t("explorer.accounts.description")}</Lead>

      <List
        disableAuthentication
        filters={filters}
        sort={{ field: "createdAt", order: "DESC" }}
      >
        <Datagrid bulkActionButtons={false}>
          <Box source="name">
            <TextField
              source="name"
              defaultValue={t("account_show.title.default_name")}
            />
          </Box>
          <DateField source="createdAt" />
          <NumberField textAlign="right" source="totalCollabs" />
          <AmountField
            textAlign="right"
            currency=""
            source="totalCollabRewards"
          />
          <NumberField textAlign="right" source="totalCampaigns" />
          <NumberField textAlign="right" source="totalCollabsReceived" />
          <AmountField textAlign="right" currency="" source="totalSpent" />
          <AmountField
            textAlign="right"
            currency=""
            source="unclaimedAsamiBalance"
          />
          <AmountField
            textAlign="right"
            currency=""
            source="unclaimedDocBalance"
          />
          <AddressField source="addr" />
          <ShowButton textAlign="right" />
        </Datagrid>
      </List>
    </ExplorerLayout>
  );
};

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

  const showAccount = useShowController({ disableAuthentication: true });

  const listHandles = useListController({
    disableSyncWithLocation: true,
    filter: { accountIdEq: id },
    resource: "Handle",
  });

  if (showAccount.isPending || !showAccount.record || listHandles.isPending) {
    return <></>;
  }

  const handle = listHandles.data[0];

  return (
    <>
      <RecordContextProvider value={showAccount.record}>
        <Head1 sx={{ color: "primary.main" }}>{showAccount.record.name}</Head1>
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
          {handle &&
            (handle.currentScoringId ? (
              <ScoringCardTable handle={handle} />
            ) : (
              <IconTextCard
                title={`@${handle.username}`}
                text="scoring_in_progress"
                icon={XIcon}
              />
            ))}
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

const ScoringCardTable = ({ handle }) => {
  const t = useTranslate();

  const showScoring = useShowController({
    disableAuthentication: true,
    resource: "HandleScoring",
    id: handle.currentScoringId,
  });

  if (showScoring.isPending) {
    return <></>;
  }

  const scoring = showScoring.record;

  return (
    <>
      <FixedWidthCard elevation={10}>
        <CardContent>
          <XIcon sx={{ fontSize: 30 }} />
          <Box flexGrow="1" mt="1em">
            <Typography
              my="0.1em"
              fontSize="1.1em"
              fontFamily="'LeagueSpartanBold'"
              lineHeight="1.1em"
              letterSpacing="-0.05em"
            >
              {handle.username}
            </Typography>
            <BigText sx={{ textAlign: "center" }}>
              <Box id="score-value">{BigInt(handle.score)}</Box>
              <Typography
                fontSize="0.4em"
                letterSpacing="0em"
                marign="0"
                padding="0"
                fontWeight="100"
                fontFamily="'LeagueSpartanLight'"
                lineHeight="0.7em"
                textTransform="uppercase"
              >
                {t("account_show.score")}
              </Typography>
            </BigText>
          </Box>
          <AttributeTable record={scoring}>
            <NumberField source="audienceSize" />
            <FunctionField
              source="authority"
              render={(h) => `${BigInt(h.authority)}%`}
            />
          </AttributeTable>
          <AttributeTable record={handle}>
            <NumberField source="totalCollabs" />
            <AmountField
              textAlign="right"
              currency=""
              source="totalCollabRewards"
            />
            <DateField source="lastScoring" emptyText="-" />
          </AttributeTable>
        </CardContent>
      </FixedWidthCard>
      <RecordContextProvider value={scoring}>
        <OnlineEngagementCard scoring={scoring} />
        <OfflineEngagementCard scoring={scoring} />
        <AudiencePollCard scoring={scoring} />
        <OperationalStatusCard scoring={scoring} />

        {(scoring.referrerScore || scoring.referrerScoreOverride) && (
          <IconTextCard
            title="referrer_title"
            text="referrer_text"
            textParam={scoring.referrerScoreOverrideReason}
            icon={GroupAddIcon}
          />
        )}

        {(scoring.holderScore || scoring.holderScoreOverride) && (
          <IconTextCard
            title="holder_title"
            text="holder_text"
            textParam={scoring.holderScoreOverrideReason}
            icon={DiamondIcon}
          />
        )}

        {scoring.ghostAccount && (
          <IconTextCard
            color={red}
            title="ghost_title"
            text="ghost_text"
            icon={SentimentNeutralIcon}
          />
        )}
        {scoring.repostFatigue && (
          <IconTextCard
            color={red}
            title="repost_fatigue_title"
            text="repost_fatigue_text"
            icon={SpeedIcon}
          />
        )}
        {scoring.indeterminateAudience && (
          <IconTextCard
            color={red}
            title="indeterminate_audience_title"
            text="indeterminate_audience_text"
            icon={SmartToyIcon}
          />
        )}
        {scoring.followed && (
          <IconTextCard
            title="followed_title"
            text="followed_text"
            icon={Diversity1Icon}
          />
        )}
        {scoring.liked && (
          <IconTextCard
            title="liked_title"
            text="liked_text"
            icon={FavoriteIcon}
          />
        )}
        {scoring.replied && (
          <IconTextCard
            title="replied_title"
            text="replied_text"
            icon={ForumIcon}
          />
        )}
        {scoring.reposted && (
          <IconTextCard
            title="reposted_title"
            text="reposted_text"
            icon={CachedIcon}
          />
        )}
        {scoring.mentioned && (
          <IconTextCard
            title="mentioned_title"
            text="mentioned_text"
            icon={AlternateEmailIcon}
          />
        )}
      </RecordContextProvider>
    </>
  );
};

const OnlineEngagementCard = ({ scoring }) => {
  const score =
    scoring.onlineEngagementOverride || scoring.onlineEngagementScore;

  return (
    <>
      {score == "AVERAGE" && (
        <IconTextCard
          title="online_engagement.title"
          text="online_engagement.average"
          textParam={scoring.onlineEngagementOverrideReason}
          icon={StarHalfIcon}
        />
      )}
      {score == "HIGH" && (
        <IconTextCard
          title="online_engagement.title"
          text="online_engagement.high"
          textParam={scoring.onlineEngagementOverrideReason}
          icon={StarIcon}
        />
      )}
      {score == "NONE" && (
        <IconTextCard
          color={red}
          title="online_engagement.title"
          text="online_engagement.none"
          textParam={scoring.onlineEngagementOverrideReason}
          icon={SentimentVeryDissatisfiedIcon}
        />
      )}
    </>
  );
};

const OfflineEngagementCard = ({ scoring }) => {
  return (
    <>
      {scoring.offlineEngagementScore == "AVERAGE" && (
        <IconTextCard
          title="offline_engagement_title"
          text={scoring.offlineEngagementDescription || ""}
          icon={GrassIcon}
        />
      )}
      {scoring.offlineEngagementScore == "HIGH" && (
        <IconTextCard
          title="offline_engagement_title"
          text={scoring.offlineEngagementDescription || ""}
          icon={GrassIcon}
        />
      )}
    </>
  );
};

const AudiencePollCard = ({ scoring }) => {
  const score = scoring.pollScoreOverride || scoring.pollScore;

  return (
    <>
      {score == "HIGH" && (
        <IconTextCard
          title="audience_poll.title"
          text="audience_poll.high"
          textParam={scoring.pollOverrideReason}
          icon={SentimentVerySatisfiedIcon}
        />
      )}
      {score == "AVERAGE" && (
        <IconTextCard
          title="audience_poll.title"
          text="audience_poll.average"
          textParam={scoring.pollOverrideReason}
          icon={SentimentSatisfiedIcon}
        />
      )}
      {score == "REVERSE" && (
        <IconTextCard
          color={red}
          title="audience_poll.title"
          text="audience_poll.reverse"
          textParam={scoring.pollOverrideReason}
          icon={SentimentDissatisfiedIcon}
        />
      )}
    </>
  );
};

const OperationalStatusCard = ({ scoring }) => {
  const score =
    scoring.operationalStatusOverride || scoring.operationalStatusScore;

  return (
    <>
      {(score == "BANNED" || score == "SHADOWBANNED") && (
        <IconTextCard
          color={red}
          title="banned_title"
          text="banned_text"
          textParam={scoring.operationalStatusOverrideReason}
          icon={VisibilityOffIcon}
        />
      )}
      {score == "ENHANCED" && (
        <IconTextCard
          title="verified_title"
          text="verified_text"
          textParam={scoring.operationalStatusOverrideReason}
          icon={VerifiedIcon}
        />
      )}
    </>
  );
};

const IconTextCard = ({ color, title, icon: Icon, text, textParam }) => {
  const t = useTranslate();
  return (
    <FixedWidthCard color={color}>
      <CardContent>
        <CardTitle>{t(`account_show.${title}`, { _: title })}</CardTitle>
        <Icon sx={{ fontSize: "100px", margin: "30px" }} />
        <Box sx={{ flexGrow: "1" }} />
        <Typography align="center" fontSize="0.9em" lineHeight="1.2em">
          {t(`account_show.${text}`, { _: text, extra: textParam || "" })}
        </Typography>
      </CardContent>
    </FixedWidthCard>
  );
};

const FixedWidthCard = ({ id, color, children }) => {
  return (
    <Card
      id={id}
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

const HorizontalCardTable = ({ children, ...props }) => (
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
