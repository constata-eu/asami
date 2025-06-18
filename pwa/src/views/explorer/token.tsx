import {
  Box,
  Button,
  Card,
  CardContent,
  Stack,
  Typography,
} from "@mui/material";
import { ResponsiveAppBar } from "../responsive_app_bar";
import AutoAwesomeIcon from "@mui/icons-material/AutoAwesome";
import {
  BooleanField,
  Datagrid,
  List,
  TextField,
  TextInput,
  useShowController,
  useTranslate,
} from "react-admin";
import { ExplorerLayout } from "../layout";
import { BigText, Head1, Lead } from "../../components/theme";
import {
  AddressField,
  AmountField,
  truncateEther,
} from "../../components/custom_fields";

const Token = () => (
  <ExplorerLayout>
    <TokenInner />
  </ExplorerLayout>
);

const TokenInner = () => {
  const stats = useShowController({ resource: "TokenStats", id: 1 });
  const t = useTranslate();

  if (stats.isPending || !stats.record) {
    return <></>;
  }

  return (
    <Stack>
      <Head1>{t("token.title")}</Head1>
      <Lead>{t("token.text")}</Lead>
      <Stack
        mt="1em"
        mb="2em"
        gap="1em"
        direction="row"
        flexWrap="wrap"
        alignItems="stretch"
      >
        <Section
          bigText={`$ ${stats.record.price ? truncateEther(stats.record.price, 2) : "?"}`}
          i18nScope="price"
          hasButton
        />
        <Section
          bigText={`$ ${stats.record.tokenYield ? truncateEther(stats.record.tokenYield) : "?"}`}
          i18nScope="token_yield"
        />
        <Section
          bigText={`${stats.record.payback ? BigInt(stats.record.payback) : "?"}`}
          i18nScope="payback"
        />
        <Section
          bigText={
            <Stack>
              <Box fontSize="0.9em">
                {stats.record.supply
                  ? truncateEther(BigInt(stats.record.supply) / 1000000n, 2)
                  : "?"}
                M / 21M
              </Box>
            </Stack>
          }
          i18nScope="supply"
          hasButton
        />
        <Section
          bigText={`${stats.record.unclaimed ? truncateEther(stats.record.unclaimed, 0) : "?"}`}
          i18nScope="unclaimed"
        />
        <Section
          bigText={`${stats.record.issuanceRate ? truncateEther(stats.record.issuanceRate, 0) : "?"}`}
          i18nScope="issuance_rate"
        />
        <Section
          bigText={`$ ${stats.record.feePool ? truncateEther(stats.record.feePool, 2) : "?"}`}
          i18nScope="fee_pool"
        />
        <Section
          bigText={
            <Box fontSize="0.9em">
              {formatEpoch(stats.record.cycleStart)}/
              {formatEpoch(stats.record.cycleEnd)}
            </Box>
          }
          i18nScope="current_cycle"
        />
      </Stack>

      <Box>
        <Head1>{t("token.holders.title")}</Head1>
        <Lead>{t("token.holders.description")}</Lead>
        <List
          resource="Holder"
          disableSyncWithLocation
          disableAuthentication
          actions={false}
          filters={[<TextInput source="addressIlike" alwaysOn />]}
          sort={{ field: "balance", order: "DESC" }}
          exporter={false}
        >
          <Datagrid bulkActionButtons={false}>
            <AddressField source="address" sortable={false} />
            <AmountField currency="" source="balance" />
            <AmountField currency="" source="estimatedTotalDocClaimed" />
            <BooleanField source="isContract" sortable={false} />
          </Datagrid>
        </List>
      </Box>
    </Stack>
  );
};

export default Token;

const Section = ({ i18nScope, bigText, hasButton }) => {
  const t = useTranslate();

  return (
    <Card sx={{ flex: "1 1 320px" }}>
      <CardContent
        sx={{ display: "flex", alignItems: "stretch", height: "100%" }}
      >
        <Stack flexGrow={1} gap="1em">
          <Typography
            fontSize="1.4em"
            fontFamily="'LeagueSpartanBold'"
            lineHeight="1.1em"
            letterSpacing="-0.05em"
            textAlign="center"
            color="secondary.main"
            textTransform="uppercase"
          >
            {t(`token.sections.${i18nScope}.title`)}
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
          >
            {t(`token.sections.${i18nScope}.description`)}
          </Typography>
          {hasButton && (
            <Button
              target="_blank"
              href={t(`token.sections.${i18nScope}.button_href`)}
              variant="outlined"
              fullWidth
            >
              {t(`token.sections.${i18nScope}.button_label`)}
            </Button>
          )}
        </Stack>
      </CardContent>
    </Card>
  );
};

const formatEpoch = (e) => {
  const date = new Date(e * 1000);
  const formatter = new Intl.DateTimeFormat("en-US", {
    month: "short",
    day: "numeric",
  });

  return formatter.format(date);
};
