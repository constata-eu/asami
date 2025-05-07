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
import { Box, Card, Divider, Typography } from "@mui/material";
import { AmountField } from "../../components/custom_fields";
import { Head1, Lead } from "../../components/theme";

export const StatsShow = () => {
  let translate = useTranslate();

  return (
    <ExplorerLayout>
      <Head1 sx={{ textAlign: "center" }}>
        {translate("explorer.stats.title")}
      </Head1>
      <Lead sx={{ textAlign: "center" }}>
        {translate("explorer.stats.description")}
      </Lead>
      <ShowBase>
        <Card sx={{ mt: "1em", justifySelf: "center" }} elevation={1}>
          <SimpleShowLayout direction="row">
            <NumberField source="totalActiveHandles" />
            <NumberField source="totalCollabs" />
            <NumberField source="totalCampaigns" />
            <AmountField source="totalRewardsPaid" />
            <DateField source="date" showTime />
          </SimpleShowLayout>
        </Card>
      </ShowBase>
    </ExplorerLayout>
  );
};
