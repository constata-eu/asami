import React from "react";
import { useEffect, useContext } from "react";
import {
  SelectInput,
  SearchInput,
  Datagrid,
  List,
  useSafeSetState,
  useTranslate,
  ListBase,
  TextField,
  FunctionField,
  Button,
  useRedirect,
  TextInput,
  DateField,
  NumberField,
  SimpleShowLayout,
  BooleanField,
  ReferenceInput,
  AutocompleteInput,
  BooleanInput,
  EditButton,
  ReferenceField,
  ReferenceArrayField,
  SingleFieldList,
  ShowButton,
} from "react-admin";
import { Link } from "react-router-dom";
import { BareLayout, DeckCard, ExplorerLayout } from "../layout";
import { Box, Typography, Chip } from "@mui/material";
import { BigNumField, AmountField } from "../../components/custom_fields";
import { Head1, Lead } from "../../components/theme";

export const HandleList = () => {
  let translate = useTranslate();

  const filters = [
    <TextInput source="idEq" alwaysOn />,
    <TextInput source="usernameLike" alwaysOn />,
  ];

  return (
    <ExplorerLayout>
      <Head1>{translate("explorer.handles.title")}</Head1>
      <Lead>{translate("explorer.handles.description")}</Lead>
      <List disableAuthentication filters={filters} exporter={false}>
        <Datagrid bulkActionButtons={false}>
          <TextField source="id" />
          <TextField source="username" sortable={false} />
          <BigNumField textAlign="right" source="score" />
          <TextField source="status" sortable={false} />
          <FunctionField
            textAlign="right"
            source="totalCollabs"
            render={(record) =>
              record.totalCollabs > 0 ? (
                <Link
                  to={`/Collab?displayedFilters=%7B%7D&filter=%7B%22handleIdEq%22%3A${record.id}%7D`}
                >
                  <NumberField source="totalCollabs" />
                </Link>
              ) : (
                <NumberField source="totalCollabs" />
              )
            }
          />
          <AmountField textAlign="right" source="totalCollabRewards" />
          <TextField source="accountId" sortable={false} />
          <ReferenceField
            textAlign="right"
            source="accountId"
            reference="Account"
            sortable={false}
            label=""
          >
            <ShowButton />
          </ReferenceField>
        </Datagrid>
      </List>
    </ExplorerLayout>
  );
};
