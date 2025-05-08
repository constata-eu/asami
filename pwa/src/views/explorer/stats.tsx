import React from "react";
import { useEffect, useContext } from "react";
import {
  SelectInput,
  SearchInput,
  Datagrid,
  List,
  useSafeSetState,
  useTranslate,
  Show,
  SimpleShowLayout,
  TextField,
  FunctionField,
  Button,
  useRedirect,
  TextInput,
  DateField,
  NumberField,
  BooleanField,
  ReferenceInput,
  AutocompleteInput,
  BooleanInput,
  EditButton,
  ReferenceField,
  ShowBase,
} from "react-admin";
import { BareLayout, DeckCard, ExplorerLayout } from "../layout";
import { Box, Card, Divider, Stack, Typography } from "@mui/material";
import { AmountField } from "../../components/custom_fields";
import { Head1, Lead } from "../../components/theme";
import { BigNumField } from "../../components/custom_fields";

export const StatsShow = () => {
  const t = useTranslate();

  return (
    <ExplorerLayout>
      <Stack direction="row" gap="2em" alignItems="center" flexWrap="wrap">
        <Box flex="1 1 300px">
          <Head1>{t("explorer.stats.title")}</Head1>
          <Lead>{t("explorer.stats.description")}</Lead>
        </Box>
        <ShowBase>
          <Card sx={{ mt: "1em", justifySelf: "left" }} elevation={1}>
            <SimpleShowLayout direction="row">
              <NumberField source="totalActiveHandles" />
              <NumberField source="totalCollabs" />
              <NumberField source="totalCampaigns" />
              <AmountField source="totalRewardsPaid" />
              <DateField source="date" showTime />
            </SimpleShowLayout>
          </Card>
        </ShowBase>
      </Stack>

      <Head1 sx={{ mt: "1em" }}>{t("explorer.oracle.title")}</Head1>
      <Lead>{t("explorer.oracle.description")}</Lead>
      <List
        resource="OnChainJob"
        disableSyncWithLocation
        disableAuthentication
        exporter={false}
      >
        <Datagrid bulkActionButtons={false} expand={<ExpandOnChainJob />}>
          <TextField source="id" />
          <TextField source="status" sortable={false} />
          <TextField source="kind" sortable={false} />
          <DateField source="sleepUntil" showTime />
          <DateField source="createdAt" showTime />
        </Datagrid>
      </List>
    </ExplorerLayout>
  );
};

const ExpandOnChainJob = () => (
  <SimpleShowLayout>
    <TextField source="txHash" />
    <BigNumField textAlign="right" source="gasUsed" />
    <BigNumField textAlign="right" source="nonce" />
    <TextField source="block" />
    <TextField source="statusLine" />
  </SimpleShowLayout>
);
