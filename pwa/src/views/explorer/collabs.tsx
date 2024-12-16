import React from 'react';
import { useEffect, useContext } from 'react';
import { SelectInput, SearchInput, Datagrid, List,
  useSafeSetState,
  useTranslate,
  ListBase,
  TextField, FunctionField, Button, useRedirect,
  TextInput,
  NumberInput,
  DateField,
  NumberField,
  SimpleShowLayout,
  BooleanField, ReferenceInput, AutocompleteInput, BooleanInput,
  EditButton, ReferenceField,
} from 'react-admin';
import { Link } from 'react-router-dom';
import { BareLayout, DeckCard, ExplorerLayout } from '../layout';
import { Box, Typography } from '@mui/material';
import { AmountField } from '../../components/custom_fields';

export const CollabList = () => {
  let translate = useTranslate();

  const statusChoices = [
    {id: 'REGISTERED', name: 'REGISTERED'},
    {id: 'CLEARED', name: 'CLEARED'},
    {id: 'FAILED', name: 'FAILED'}
  ];

  const filters = [
    <TextInput source="idEq" alwaysOn />,
    <ReferenceInput source="campaignIdEq" reference="Campaign">
      <NumberInput source="campaignIdEq" size="small" />
    </ReferenceInput>,
    <ReferenceInput source="handleIdEq" reference="Handle">
      <NumberInput source="handleIdEq" size="small" />
    </ReferenceInput>,
    <ReferenceInput source="advertiserIdEq" reference="Account">
      <NumberInput source="advertiserIdEq" size="small" />
    </ReferenceInput>,
    <ReferenceInput source="memberIdEq" reference="Account">
      <NumberInput source="memberIdEq" size="small" />
    </ReferenceInput>,
    <TextInput source="collabTriggerUniqueLike" />,
    <SelectInput source="statusEq" choices={statusChoices} />,
    <SelectInput source="statusNe" choices={statusChoices} />,
  ];

  return (
    <ExplorerLayout>
      <Typography mt="0.5em" variant="h3">{ translate("explorer.collabs.title") }</Typography>
      <Typography variant="body">{ translate("explorer.collabs.description") }</Typography>
      <List disableAuthentication filters={filters} exporter={false}>
        <Datagrid bulkActionButtons={false} expand={<ExpandCollab/>}>
          <TextField source="id" />
          <FunctionField source="campaignId" render={ (record) =>
            <Link to={`/Campaign?displayedFilters=%7B%7D&filter=%7B%22idEq%22%3A${record.campaignId}%7D`}>
              <TextField source="campaignId" />
            </Link>
          }/>
          <ReferenceField source="handleId" reference="Handle" sortable={false}>
            <FunctionField source="handleId" render={ (record) =>
              <Link to={`/Handle?displayedFilters=%7B%7D&filter=%7B%22idEq%22%3A${record.id}%7D`}>
                <TextField source="username" />
              </Link>
            }/>
          </ReferenceField>
          <AmountField textAlign="right" source="reward" />
          <AmountField textAlign="right" source="fee" />
          <TextField source="status" sortable={false} />
        </Datagrid>
      </List>
    </ExplorerLayout>
  );
};

const ExpandCollab = () => <SimpleShowLayout>
  <FunctionField source="advertiserId" render={ (record) =>
    <Link to={`/Account?displayedFilters=%7B%7D&filter=%7B%22idEq%22%3A${record.advertiserId}%7D`}>
      <TextField source="advertiserId" />
    </Link>
  }/>
  <FunctionField source="memberId" render={ (record) =>
    <Link to={`/Account?displayedFilters=%7B%7D&filter=%7B%22idEq%22%3A${record.memberId}%7D`}>
      <TextField source="memberId" />
    </Link>
  }/>
  <TextField source="disputeReason" sortable={false} />
  <TextField source="collabTriggerUniqueId" sortable={false} />
</SimpleShowLayout>
