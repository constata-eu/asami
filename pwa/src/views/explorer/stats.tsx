import React from 'react';
import { useEffect, useContext } from 'react';
import { SelectInput, SearchInput, Datagrid, List,
  useSafeSetState,
  useTranslate,
  Show,
  SimpleShowLayout,
  TextField, FunctionField, Button, useRedirect,
  TextInput,
  DateField,
  NumberField,
  BooleanField, ReferenceInput, AutocompleteInput, BooleanInput,
  EditButton, ReferenceField,
} from 'react-admin';
import { BareLayout, DeckCard, ExplorerLayout } from '../layout';
import { Box, Typography } from '@mui/material';

export const StatsShow = () => {
  let translate = useTranslate();

  return (
    <ExplorerLayout>
      <Typography mt="0.5em" variant="h3">{ translate("explorer.stats.title") }</Typography>
      <Typography variant="body">{ translate("explorer.stats.description") }</Typography>
      <Show>
          <SimpleShowLayout>
              <NumberField source="totalActiveHandles" />
              <NumberField source="totalCollabs" />
              <NumberField source="totalCampaigns" />
              <NumberField source="totalRewardsPaid" />
              <DateField source="date" showTime />
          </SimpleShowLayout>
      </Show>
    </ExplorerLayout>
  );
};
