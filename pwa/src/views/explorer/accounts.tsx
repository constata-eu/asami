import {
  Datagrid,
  List,
  useTranslate,
  TextField,
  FunctionField,
  TextInput,
  DateField,
  NumberField,
  BooleanField,
  SimpleShowLayout,
  ReferenceManyField,
  SingleFieldList,
  Show,
  ShowBase,
  Labeled,
  useShowController,
  useListController,
  RecordContextProvider,
  ReferenceArrayField,
} from "react-admin";
import { Link, useParams } from "react-router-dom";
import { CardTable, DeckCard, ExplorerLayout } from "../layout";
import { CardContent, Chip, Stack, Typography } from "@mui/material";
import { AmountField, BigNumField } from "../../components/custom_fields";
import XIcon from "@mui/icons-material/X";

import { green, Head2, Head3 } from "../../components/theme";

export const AccountList = () => {
  let translate = useTranslate();

  const filters = [
    <TextInput source="idEq" alwaysOn />,
    <TextInput source="addrLike" alwaysOn />,
  ];

  return (
    <ExplorerLayout>
      <Typography mt="0.5em" variant="h3">
        {translate("explorer.accounts.title")}
      </Typography>
      <Typography variant="body">
        {translate("explorer.accounts.description")}
      </Typography>
      <List disableAuthentication filters={filters} exporter={false}>
        <Datagrid bulkActionButtons={false} expand={<ExpandAccount />}>
          <BigNumField source="id" />
          <AmountField textAlign="right" source="unclaimedAsamiBalance" />
          <AmountField textAlign="right" source="unclaimedDocBalance" />

          <FunctionField
            textAlign="right"
            source="totalCollabs"
            render={(record) =>
              record.totalCollabs > 0 ? (
                <Link
                  to={`/Collab?displayedFilters=%7B%7D&filter=%7B%22memberIdEq%22%3A${record.id}%7D`}
                >
                  <NumberField source="totalCollabs" />
                </Link>
              ) : (
                <NumberField source="totalCollabs" />
              )
            }
          />
          <AmountField textAlign="right" source="totalCollabRewards" />
          <FunctionField
            textAlign="right"
            source="totalCampaigns"
            render={(record) =>
              record.totalCampaigns > 0 ? (
                <Link
                  to={`/Campaign?displayedFilters=%7B%7D&filter=%7B%22accountIdEq%22%3A${record.id}%7D`}
                >
                  <NumberField source="totalCampaigns" />
                </Link>
              ) : (
                <NumberField source="totalCampaigns" />
              )
            }
          />
          <FunctionField
            textAlign="right"
            source="totalCollabsReceived"
            render={(record) =>
              record.totalCollabsReceived > 0 ? (
                <Link
                  to={`/Collab?displayedFilters=%7B%7D&filter=%7B%22advertiserIdEq%22%3A%22${record.id}%22%7D`}
                >
                  <NumberField source="totalCollabsReceived" />
                </Link>
              ) : (
                <NumberField source="totalCollabsReceived" />
              )
            }
          />
          <AmountField textAlign="right" source="totalSpent" />
          <DateField source="createdAt" />
        </Datagrid>
      </List>
    </ExplorerLayout>
  );
};

const ExpandAccount = () => (
  <SimpleShowLayout>
    <TextField emptyText="-" source="addr" />
    <ReferenceManyField label="Handle" reference="Handle" target="accountIdEq">
      <SingleFieldList>
        <FunctionField
          source="username"
          render={(record) => (
            <Link
              to={`/Handle?displayedFilters=%7B%7D&filter=%7B%22accountIdEq%22%3A${record.accountId}%7D`}
            >
              <TextField source="username" />
            </Link>
          )}
        />
      </SingleFieldList>
    </ReferenceManyField>
    <AmountField textAlign="right" source="asamiBalance" />
    <AmountField textAlign="right" source="rbtcBalance" />
    <AmountField textAlign="right" source="docBalance" />
    <BooleanField source="allowsGasless" />
  </SimpleShowLayout>
);

export const AccountShow = () => (
  <ExplorerLayout>
    <AccountCardTable />
  </ExplorerLayout>
);

const AccountCardTable = () => {
  let translate = useTranslate();
  const { id } = useParams(); // id of the campaign from route

  const showAccount = useShowController({ disableAuthentication: true });

  const listHandles = useListController({
    disableSyncWithLocation: true,
    filter: { accountIdEq: id },
    resource: "Handle",
  });

  if (showAccount.isPending || listHandles.isPending) {
    return <></>;
  }

  const handle = listHandles.data[0];

  return (
    <RecordContextProvider value={showAccount.record}>
      <CardTable mt="2em">
        <DeckCard borderColor={green} background={green} elevation={10}>
          <CardContent>
            <Head2>{showAccount.record.name || "Valued Anon"}</Head2>
            <Head3 sx={{ mb: "0.5em" }}>Member #{showAccount.record.id}</Head3>
            <SimpleShowLayout>
              <DateField source="createdAt" />
              <TextField source="status" />
              {showAccount.record.addr && <TextField source="addr" />}
              <AmountField textAlign="right" source="unclaimedAsamiBalance" />
              <AmountField textAlign="right" source="unclaimedDocBalance" />
              {showAccount.record.totalCampaigns && (
                <>
                  <NumberField source="totalCampaigns" />
                  <AmountField textAlign="right" source="totalSpent" />
                  <NumberField source="totalCollabsReceived" />
                </>
              )}
            </SimpleShowLayout>
          </CardContent>
        </DeckCard>
        {handle && (
          <RecordContextProvider value={handle}>
            <DeckCard elevation={10}>
              <CardContent>
                <Stack direction="row" gap="1em">
                  <XIcon />
                  <Head2>@{handle.username}</Head2>
                </Stack>
                <Head3 sx={{ mb: "0.5em" }}>#{handle.userId}</Head3>
                <SimpleShowLayout>
                  <FunctionField
                    label={translate("handle_settings.stats.score")}
                    render={(h) => `${BigInt(h.score)} 力`}
                  />
                  <TextField source="status" />
                  <NumberField source="totalCollabs" />
                  <AmountField textAlign="right" source="totalCollabRewards" />
                  <ReferenceArrayField
                    label={translate("resources.Handle.fields.topic")}
                    reference="Topic"
                    source="topicIds"
                  >
                    <SingleFieldList empty={<>-</>} linkType={false}>
                      <FunctionField
                        render={(h) => (
                          <Chip
                            size="small"
                            variant="outlined"
                            label={translate(`resources.Topic.names.${h.name}`)}
                          />
                        )}
                      />
                    </SingleFieldList>
                  </ReferenceArrayField>
                </SimpleShowLayout>
              </CardContent>
            </DeckCard>
            {handle.currentScoringId && <ScoringCardTable handle={handle} />}
          </RecordContextProvider>
        )}
      </CardTable>
    </RecordContextProvider>
  );
};

const ScoringCardTable = ({ handle }) => {
  let translate = useTranslate();

  const showScoring = useShowController({
    disableAuthentication: true,
    resource: "HandleScoring",
    id: handle.currentScoringId,
  });

  if (showScoring.isPending) {
    return <></>;
  }

  return (
    <RecordContextProvider value={showScoring.record}>
      <DeckCard elevation={10}>
        <CardContent>
          <Stack direction="row" gap="1em">
            <XIcon />
            <Head2>@{handle.username}</Head2>
          </Stack>
          <Head3 sx={{ mb: "0.5em" }}>#{handle.userId}</Head3>
        </CardContent>
      </DeckCard>
      {handle.currentScoringId && <ScoringCardTable handle={handle} />}
    </RecordContextProvider>
  );
};
