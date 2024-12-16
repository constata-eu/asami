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
  NumberInput,
  BooleanField, ReferenceInput, AutocompleteInput, BooleanInput,
  SimpleShowLayout,
  EditButton, ReferenceField,
} from 'react-admin';
import { Link } from 'react-router-dom';
import { BareLayout, DeckCard, ExplorerLayout } from '../layout';
import { Box, Typography } from '@mui/material';
import { viewPostUrl } from '../../lib/campaign';
import { AmountField, BigNumField, AmountInput } from '../../components/custom_fields';
import { formatEther, formatUnits, parseEther, toBeHex } from "ethers";

export const CampaignList = () => {
  let translate = useTranslate();

  const statusChoices = [
    {id: 'DRAFT', name: 'DRAFT'},
    {id: 'SUBMITTED', name: 'SUBMITTED'},
    {id: 'PUBLISHED', name: 'PUBLISHED'}
  ];

  const filters = [
    <TextInput source="idEq" alwaysOn />,
    <TextInput source="briefingJsonLike" />,
    <TextInput source="briefingHashLike" />,
    <ReferenceInput source="accountIdEq" reference="Account">
      <NumberInput source="accountIdEq" size="small" />
    </ReferenceInput>,
    <SelectInput source="statusEq" choices={statusChoices} />,
    <SelectInput source="statusNe" choices={statusChoices} />,
    <AmountInput source="budgetLt" />,
    <AmountInput source="budgetGt" />,
    <AmountInput source="budgetEq" />,
  ];

  return (
    <ExplorerLayout>
      <Typography mt="0.5em" variant="h3">{ translate("explorer.campaigns.title") }</Typography>
      <Typography variant="body">{ translate("explorer.campaigns.description") }</Typography>
      <List disableAuthentication filters={filters} exporter={false}>
        <Datagrid bulkActionButtons={false} expand={<ExpandCampaign/>}>
          <TextField source="id" />
          <FunctionField source="briefingJson" sortable={false} render={record =>
            <a target="_blank" href={viewPostUrl(record)} rel="noreferrer">
              { JSON.parse(record.briefingJson) }
            </a>
          } />
          <TextField source="status" sortable={false}/>
          <FunctionField textAlign="right" source="totalCollabs" render={ (record) =>
            record.totalCollabs > 0 ?
              <Link to={`/Collab?displayedFilters=%7B%7D&filter=%7B%22campaignIdEq%22%3A${record.id}%7D`}>
                <NumberField source="totalCollabs" />
              </Link>
              :
              <NumberField source="totalCollabs" />
          }/>
          <AmountField textAlign="right" source="budget" />
          <AmountField textAlign="right" source="totalSpent" />
          <AmountField textAlign="right" source="totalBudget" />
          <DateField source="validUntil" showTime />
          <DateField source="createdAt" showTime />
        </Datagrid>
      </List>
    </ExplorerLayout>
  );
};

const ExpandCampaign = () => <SimpleShowLayout>
  <FunctionField source="accountId" render={ (record) =>
    <Link to={`/Account?displayedFilters=%7B%7D&filter=%7B%22idEq%22%3A${record.accountId}%7D`}>
      <TextField source="accountId" />
    </Link>
  }/>
  <TextField source="briefingHash" />
</SimpleShowLayout>
