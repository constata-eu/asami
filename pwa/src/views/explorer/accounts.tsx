import React from 'react';
import { useEffect, useContext } from 'react';
import { SelectInput, SearchInput, Datagrid, List,
  useSafeSetState,
  useTranslate,
  ListBase,
  TextField, FunctionField, Button, useRedirect,
  TextInput,
  DateField,
  NumberField,
  BooleanField, ReferenceInput, AutocompleteInput, BooleanInput,
  EditButton, ReferenceField,
} from 'react-admin';
import { BareLayout, DeckCard, ExplorerLayout } from '../layout';
import { Box, Typography } from '@mui/material';
import { AmountField, BigNumField } from '../../components/custom_fields';

export const AccountList = () => {
  let translate = useTranslate();

  const filters = [
    <TextInput source="idEq" alwaysOn />,
    <TextInput source="addrLike" alwaysOn />,
  ];

  return (
    <ExplorerLayout>
      <Typography mt="0.5em" variant="h3">{ translate("explorer.accounts.title") }</Typography>
      <Typography variant="body">{ translate("explorer.accounts.description") }</Typography>
      <List disableAuthentication filters={filters} exporter={false}>
        <Datagrid rowClick={false} bulkActionButtons={false}>
          <BigNumField source="id" />
          <TextField source="status" sortable={false} />
          <TextField source="addr" sortable={false} />
          <AmountField textAlign="right" source="unclaimedAsamiBalance" sortable={false} />
          <AmountField textAlign="right" source="unclaimedDocBalance" sortable={false} />
          <AmountField textAlign="right" source="asamiBalance" sortable={false} />
          <DateField source="createdAt" showTime sortable={false} />
        </Datagrid>
      </List>
    </ExplorerLayout>
  );
};
