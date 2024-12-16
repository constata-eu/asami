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
  SimpleShowLayout,
  ReferenceManyField,
  SingleFieldList,
  EditButton, ReferenceField,
} from 'react-admin';
import { Link } from 'react-router-dom';
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
        <Datagrid bulkActionButtons={false} expand={<ExpandAccount/>}>
          <BigNumField source="id" />
          <AmountField textAlign="right" source="unclaimedAsamiBalance" />
          <AmountField textAlign="right" source="unclaimedDocBalance" />

          <FunctionField textAlign="right" source="totalCollabs" render={ (record) =>
            record.totalCollabs > 0 ?
              <Link to={`/Collab?displayedFilters=%7B%7D&filter=%7B%22memberIdEq%22%3A${record.id}%7D`}>
                <NumberField source="totalCollabs" />
              </Link>
              :
              <NumberField source="totalCollabs" />
          }/>
          <AmountField textAlign="right" source="totalCollabRewards" />
          <FunctionField textAlign="right" source="totalCampaigns" render={ (record) =>
            record.totalCampaigns > 0 ?
              <Link to={`/Campaign?displayedFilters=%7B%7D&filter=%7B%22accountIdEq%22%3A${record.id}%7D`}>
                <NumberField source="totalCampaigns" />
              </Link>
              :
              <NumberField source="totalCampaigns" />
          }/>
          <FunctionField textAlign="right" source="totalCollabsReceived" render={ (record) =>
            record.totalCollabsReceived > 0 ?
              <Link to={`/Collab?displayedFilters=%7B%7D&filter=%7B%22advertiserIdEq%22%3A%22${record.id}%22%7D`}>
                <NumberField source="totalCollabsReceived" />
              </Link>
              :
              <NumberField source="totalCollabsReceived" />
          }/>
          <AmountField textAlign="right" source="totalSpent" />
          <DateField source="createdAt" />
        </Datagrid>
      </List>
    </ExplorerLayout>
  );
};

const ExpandAccount = () => <SimpleShowLayout>
  <TextField emptyText="-" source="addr" />
  <ReferenceManyField label="Handle" reference="Handle" target="accountIdEq">
      <SingleFieldList>
        <FunctionField source="username" render={ (record) =>
          <Link to={`/Handle?displayedFilters=%7B%7D&filter=%7B%22accountIdEq%22%3A${record.accountId}%7D`}>
            <TextField source="username" />
          </Link>
        }/>
      </SingleFieldList>
  </ReferenceManyField>
  <AmountField textAlign="right" source="asamiBalance" />
  <AmountField textAlign="right" source="rbtcBalance" />
  <AmountField textAlign="right" source="docBalance" />
  <BooleanField source="allowsGasless" />
</SimpleShowLayout>
