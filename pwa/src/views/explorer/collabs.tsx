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
import { AmountField } from '../../components/custom_fields';

export const CollabList = () => {
  let translate = useTranslate();

  const filters = [
    <TextInput source="idEq" alwaysOn />,
    <TextInput source="campaignIdEq" alwaysOn />,
    <TextInput source="handleIdEq" alwaysOn />,
    //<SelectInput label="Sport" source="sport" choices={sportChoices} alwaysOn />,
    //<ReferenceInput label="Country" source="country" reference="countries" alwaysOn>
    //  <AutocompleteInput size="small" optionText="name" />
    //</ReferenceInput>,
  ];

  return (
    <ExplorerLayout>
      <Typography mt="0.5em" variant="h3">{ translate("explorer.collabs.title") }</Typography>
      <Typography variant="body">{ translate("explorer.collabs.description") }</Typography>
      <List disableAuthentication filters={filters} exporter={false}>
        <Datagrid rowClick={false} bulkActionButtons={false}>
          <TextField source="id" />
          <ReferenceField source="campaignId" reference="Campaign" sortable={false}/>
          <ReferenceField source="handleId" reference="Handle" sortable={false}>
            <>
              #<TextField source="id"/> (<TextField source="username"/>)
            </>
          </ReferenceField>
          <AmountField textAlign="right" source="reward" />
          <AmountField textAlign="right" source="fee" />
          <TextField source="status" sortable={false} />
          <TextField source="disputeReason" sortable={false} />
          <TextField source="collabTriggerUniqueId" sortable={false} />
        </Datagrid>
      </List>
    </ExplorerLayout>
  );
};
