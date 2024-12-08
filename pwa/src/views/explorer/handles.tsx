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
import { BigNumField } from '../../components/custom_fields';

export const HandleList = () => {
  let translate = useTranslate();

  const filters = [
    <TextInput source="idEq" alwaysOn />,
    <TextInput source="usernameLike" alwaysOn />,
  ];

  return (
    <ExplorerLayout>
      <Typography mt="0.5em" variant="h3">{ translate("explorer.handles.title") }</Typography>
      <Typography variant="body">{ translate("explorer.handles.description") }</Typography>
      <List disableAuthentication filters={filters} exporter={false}>
        <Datagrid rowClick={false} bulkActionButtons={false}>
          <TextField source="id" />
          <TextField source="site" sortable={false} />
          <TextField source="username" sortable={false} />
          <BigNumField textAlign="right" source="score" />
          <TextField source="status" sortable={false} />
          <BigNumField source="accountId" label="Member" sortable={false} />
        </Datagrid>
      </List>
    </ExplorerLayout>
  );
};
