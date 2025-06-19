import {
  useTranslate,
  NumberField,
  ShowBase,
  useRecordContext,
  FunctionField,
} from "react-admin";
import { BareLayout, CardTable, DeckCard, ExplorerLayout } from "../layout";
import { useNavigate } from "react-router-dom";
import {
  Box,
  Button,
  Card,
  CardContent,
  Stack,
  Typography,
} from "@mui/material";
import { AmountField } from "../../components/custom_fields";
import { BigText, Head1, Lead } from "../../components/theme";
import { BigNumField, truncateEther } from "../../components/custom_fields";
import { AttributeTable } from "../../components/attribute_table";

export const StatsShow = () => {
  const t = useTranslate();

  return (
    <ExplorerLayout>
      <ShowBase>
        <Cards />
      </ShowBase>
    </ExplorerLayout>
  );
};

const Cards = () => {
  const record = useRecordContext();
  const t = useTranslate();

  if (!record) {
    return;
  }

  const docFormatter = new Intl.NumberFormat("en-US", {
    minimumFractionDigits: 2,
    maximumFractionDigits: 2,
    useGrouping: false,
  });

  return (
    <>
      <Box p="0.5em">
        <Head1>{t("explorer.stats.title")}</Head1>
      </Box>
      <Stack
        my="1em"
        gap="1em"
        direction="row"
        flexWrap="wrap"
        alignItems="stretch"
      >
        <StatsCard
          i18nScope="accounts"
          bigText={record.totalActiveMembers}
          link="/Account/"
        >
          <NumberField textAlign="right" source="totalSignups" />
          <NumberField textAlign="right" source="totalAdvertisers" />
        </StatsCard>

        <StatsCard
          i18nScope="active_handles"
          bigText={record.totalActiveHandles}
          link="/Handle/"
        >
          <NumberField textAlign="right" source="currentlyActive" />
          <NumberField textAlign="right" source="joinedRecently" />
        </StatsCard>

        <StatsCard
          i18nScope="campaigns"
          bigText={record.totalCampaigns}
          link="/Campaign/"
        >
          <NumberField textAlign="right" source="recentCampaigns" />
          <NumberField textAlign="right" source="thirtyDayAverageCampaigns" />
        </StatsCard>

        <StatsCard
          i18nScope="collabs"
          bigText={record.totalCollabs}
          link="/Collab/"
        >
          <NumberField textAlign="right" source="recentCollabs" />
          <NumberField textAlign="right" source="thirtyDayAverageCollabs" />
        </StatsCard>

        <StatsCard
          i18nScope="rewards_paid"
          bigText={`$ ${record.totalRewardsPaid ? docFormatter.format(record.totalRewardsPaid) : "?"}`}
          link="/Collabs/"
        >
          <FunctionField
            render={(x) => `${docFormatter.format(x.recentRewardsPaid)} DOC`}
            textAlign="right"
            source="recentRewardsPaid"
          />
          <FunctionField
            render={(x) =>
              `${docFormatter.format(x.thirtyDayAverageRewardsPaid)} DOC`
            }
            textAlign="right"
            source="thirtyDayAverageRewardsPaid"
          />
        </StatsCard>
      </Stack>
    </>
  );
};

const StatsCard = ({ i18nScope, bigText, link, children }) => {
  const navigate = useNavigate();
  const t = useTranslate();

  return (
    <Card sx={{ flex: "1 1 270px" }}>
      <CardContent
        sx={{ display: "flex", alignItems: "stretch", height: "100%" }}
      >
        <Box flexGrow={1} display="flex" flexDirection="column">
          <Typography
            mb="0.5em"
            fontSize="1.4em"
            fontFamily="'LeagueSpartanBold'"
            lineHeight="1.1em"
            letterSpacing="-0.05em"
            textAlign="center"
            color="secondary.main"
          >
            {t(`resources.Stats.show_page_cards.${i18nScope}.title`)}
          </Typography>
          <BigText
            sx={{
              fontSize: "3em",
              lineHeight: "1em",
              color: "secondary.main",
              textAlign: "center",
            }}
          >
            {bigText}
          </BigText>
          <Typography
            color="secondary"
            fontSize="1em"
            letterSpacing="0em"
            margin="auto"
            padding="0"
            fontWeight="100"
            fontFamily="'LeagueSpartanLight'"
            textTransform="uppercase"
          >
            {t(`resources.Stats.show_page_cards.${i18nScope}.subtitle`)}
          </Typography>

          <Box my="1.5em">
            <AttributeTable fontSize="1em !important" resource="Stats">
              {children}
            </AttributeTable>
          </Box>
          <Button variant="outlined" fullWidth onClick={() => navigate(link)}>
            {t(`resources.Stats.show_page_cards.${i18nScope}.button`)}
          </Button>
        </Box>
      </CardContent>
    </Card>
  );
};
