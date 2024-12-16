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
  SimpleShowLayout,
  BooleanField, ReferenceInput, AutocompleteInput, BooleanInput,
  EditButton, ReferenceField,
} from 'react-admin';
import { BareLayout, DeckCard, ExplorerLayout } from '../layout';
import { Box, Typography } from '@mui/material';
import { BigNumField } from '../../components/custom_fields';

export const OnChainJobList = () => {
  let translate = useTranslate();

  return (
    <ExplorerLayout>
      <Typography mt="0.5em" variant="h3">{ translate("explorer.oracle.title") }</Typography>
      <Typography variant="body">{ translate("explorer.oracle.description") }</Typography>
      <List disableAuthentication exporter={false}>
        <Datagrid bulkActionButtons={false} expand={<ExpandOnChainJob/>}>
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

const ExpandOnChainJob = () => <SimpleShowLayout>
    <TextField source="txHash"/>
    <BigNumField textAlign="right" source="gasUsed" />
    <BigNumField textAlign="right" source="nonce" />
    <TextField source="block" />
    <TextField source="statusLine" />
</SimpleShowLayout>
