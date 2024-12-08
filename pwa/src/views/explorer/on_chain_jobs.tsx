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

export const OnChainJobList = () => {
  let translate = useTranslate();

  const filters = [
    <TextInput source="idEq" alwaysOn />,
    //<SelectInput label="Sport" source="sport" choices={sportChoices} alwaysOn />,
    //<ReferenceInput label="Country" source="country" reference="countries" alwaysOn>
    //  <AutocompleteInput size="small" optionText="name" />
    //</ReferenceInput>,
  ];

  return (
    <ExplorerLayout>
      <Typography mt="0.5em" variant="h3">{ translate("explorer.handles.title") }</Typography>
      <Typography variant="body">{ translate("explorer.handles.description") }</Typography>
      <List disableAuthentication filters={filters} exporter={false}>
        <Datagrid rowClick={false} bulkActionButtons={false}>
          <TextField source="id" />
          <TextField source="status" sortable={false} />
          <TextField source="kind" sortable={false} />
          <DateField source="createdAt" showTime />
          <DateField source="sleepUntil" sortable={false} showTime />
          <TextField source="txHash" sortable={false} />
          <BigNumField textAlign="right" source="gasUsed" sortable={false} />
          <BigNumField textAlign="right" source="nonce" sortable={false} />
          <TextField source="block" sortable={false} />
          <TextField source="statusLine" sortable={false} />
        </Datagrid>
      </List>
    </ExplorerLayout>
  );
};
