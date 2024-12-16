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
import { Link } from 'react-router-dom';
import { BareLayout, DeckCard, ExplorerLayout } from '../layout';
import { Box, Typography } from '@mui/material';
import { BigNumField, AmountField } from '../../components/custom_fields';

export const HandleList = () => {
  let translate = useTranslate();

  const filters = [
    <TextInput source="idEq" alwaysOn />,
    <TextInput source="usernameLike" alwaysOn />,
    <TextInput source="accountIdEq" alwaysOn/>,
  ];

  return (
    <ExplorerLayout>
      <Typography mt="0.5em" variant="h3">{ translate("explorer.handles.title") }</Typography>
      <Typography variant="body">{ translate("explorer.handles.description") }</Typography>
      <List disableAuthentication filters={filters} exporter={false}>
        <Datagrid bulkActionButtons={false} >
          <TextField source="id" />
          <TextField source="username" sortable={false} />
          <BigNumField textAlign="right" source="score" />
          <TextField source="status" sortable={false} />
          <FunctionField textAlign="right" source="totalCollabs" render={ (record) =>
            record.totalCollabs > 0 ?
              <Link to={`/Collab?displayedFilters=%7B%7D&filter=%7B%22handleIdEq%22%3A${record.id}%7D`}>
                <NumberField source="totalCollabs" />
              </Link>
              :
              <NumberField source="totalCollabs" />
          }/>
          <AmountField textAlign="right" source="totalCollabRewards" />
          <FunctionField source="accountId"  sortable={false} render={ (record) =>
            <Link to={`/Account?displayedFilters=%7B%7D&filter=%7B%22idEq%22%3A${record.accountId}%7D`}>
              <TextField source="accountId" />
            </Link>
          }/>
          <TextField source="site" sortable={false} />
        </Datagrid>
      </List>
    </ExplorerLayout>
  );
};
