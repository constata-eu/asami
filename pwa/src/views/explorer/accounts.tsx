import {
  Datagrid,
  List,
  useTranslate,
  TextField,
  FunctionField,
  TextInput,
  DateField,
  NumberField,
  BooleanField,
  SimpleShowLayout,
  ReferenceManyField,
  SingleFieldList,
  Show,
  ShowBase,
  Labeled,
  useShowController,
  useListController,
  RecordContextProvider,
  ReferenceArrayField,
  ShowButton,
} from "react-admin";
import { Link, useParams } from "react-router-dom";
import { CardTable, DeckCard, ExplorerLayout } from "../layout";
import {
  Box,
  Card,
  CardContent,
  Chip,
  Stack,
  styled,
  Typography,
} from "@mui/material";
import { AmountField, BigNumField } from "../../components/custom_fields";
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

import { red, Head1, yellow, green, dark, Lead } from "../../components/theme";

export const AccountList = () => {
  const t = useTranslate();

  const filters = [
    <TextInput source="idEq" alwaysOn />,
    <TextInput source="addrLike" alwaysOn />,
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
            />{" "}
            # <BigNumField source="id" />
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
          <TextField source="addr" />
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

  if (showAccount.isPending || listHandles.isPending) {
    return <></>;
  }

  const handle = listHandles.data[0];

  return (
    <RecordContextProvider value={showAccount.record}>
      <Head1>
        <strong>
          {showAccount.record.name || t("account_show.title.default_name")}
        </strong>
        <Box component="span" fontSize="0.6em">
          {" "}
          # {showAccount.record.id}
        </Box>
      </Head1>
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
        {!showAccount.record.totalCampaigns && (
          <FixedWidthCard elevation={10}>
            <CardContent>
              <CardTitle>{t("account_show.advertiser_title")}</CardTitle>
              <CampaignIcon sx={{ fontSize: 90, flexGrow: 1 }} />
              <SimpleShowLayout>
                <NumberField source="totalCampaigns" />
                <AmountField textAlign="right" source="totalSpent" />
                <NumberField source="totalCollabsReceived" />
              </SimpleShowLayout>
            </CardContent>
          </FixedWidthCard>
        )}
        <FixedWidthCard elevation={10}>
          <CardContent>
            <CardTitle>{t("account_show.member_info_title")}</CardTitle>
            <BadgeIcon sx={{ fontSize: 90, flexGrow: 1 }} />
            <SimpleShowLayout>
              <DateField source="createdAt" />
              <TextField source="addr" />
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
            </SimpleShowLayout>
          </CardContent>
        </FixedWidthCard>
      </HorizontalCardTable>
    </RecordContextProvider>
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
            <BigText sx={{ textAlign: "center", mt: "0.2em" }}>
              {BigInt(handle.score)}
              <Typography
                fontSize="0.4em"
                letterSpacing="0em"
                marign="0"
                padding="0"
                fontWeight="100"
                fontFamily="'LeagueSpartanLight'"
                lineHeight="0.5em"
                textTransform="uppercase"
              >
                {t("account_show.score")}
              </Typography>
            </BigText>
          </Box>
          <RecordContextProvider value={scoring}>
            <SimpleShowLayout>
              <NumberField source="audienceSize" />
              <FunctionField
                source="authority"
                render={(h) => `${BigInt(h.authority)}%`}
              />
            </SimpleShowLayout>
          </RecordContextProvider>
          <RecordContextProvider value={handle}>
            <SimpleShowLayout>
              <NumberField source="totalCollabs" />
              <AmountField textAlign="right" source="totalCollabRewards" />
            </SimpleShowLayout>
          </RecordContextProvider>
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
        <Icon sx={{ fontSize: "100px", mt: "30px" }} />
        <Box sx={{ flexGrow: "1" }} />
        <Typography align="center" fontSize="0.8em">
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

export const HorizontalCardTable = ({ children, ...props }) => (
  <Box
    sx={{
      display: "flex",
      gap: "1em",
      flexWrap: "wrap",
      justifyContent: "center",
      "& .MuiCardContent-root": {
        alignItems: "center",
        display: "flex !important",
        flexDirection: "column",
        height: "100%",
        paddingBottom: "16px !important",
      },
      "& .RaSimpleShowLayout-stack": {
        margin: "0 !important",
        gap: "0.2em !important",
        paddingBottom: "0 !important",
      },
      "& .RaSimpleShowLayout-row.ra-field": {
        display: "flex",
        flexDirection: "row",
        justifyContent: "space-between",
        width: "100%",
        alignItems: "center",
        padding: 0,
        margin: 0,
        background: "none",
      },
      "& .ra-field > p": {
        marginRight: 1,
        fontWeight: 500,
        display: "flex",
      },
      "& .ra-field > span": {
        margin: 0,
      },
    }}
    {...props}
    children={children}
  />
);

export const BigText = styled("h2")(({ theme }) => ({
  fontFamily: "'LeagueSpartanBold'",
  fontSize: "40px",
  lineHeight: "1.1em",
  letterSpacing: "-0.05em",
  margin: 0,
}));

export const CardTitle = styled("h3")(({ theme }) => ({
  fontFamily: "'LeagueSpartanBold'",
  fontSize: "20px",
  lineHeight: "1em",
  letterSpacing: "-0.05em",
  margin: 0,
}));
