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
import { viewPostUrl } from '../../lib/campaign';
import { AmountField, BigNumField } from '../../components/custom_fields';

export const CampaignList = () => {
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
      <Typography mt="0.5em" variant="h3">{ translate("explorer.campaigns.title") }</Typography>
      <Typography variant="body">{ translate("explorer.campaigns.description") }</Typography>
      <List disableAuthentication filters={filters} exporter={false}>
        <Datagrid rowClick={false} bulkActionButtons={false}>
          <TextField source="id" />
          <ReferenceField source="accountId" reference="Account" sortable={false}>
            <BigNumField label="id" source="id" />
          </ReferenceField>
          <FunctionField label={ translate("campaign_list.post") } render={record =>
            <a target="_blank" href={viewPostUrl(record)} rel="noreferrer">
              { translate("campaign_list.see_post") }
            </a>
          } />
          <TextField source="status" />
          <AmountField textAlign="right" label="budget" source="budget" />
          <TextField source="briefingJson" />
          <DateField source="validUntil" showTime />
          <DateField source="createdAt" showTime />
        </Datagrid>
      </List>
    </ExplorerLayout>
  );
};
