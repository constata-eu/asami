import { useTranslate, ReferenceField } from "react-admin";
import { viewPostUrl } from '../../lib/campaign';
import { Card, Typography } from "@mui/material";
import { formatEther } from "ethers";
import { CardTitle } from '../../components/theme';
import { Pagination, Datagrid, FunctionField } from 'react-admin';
import { useListController, ListContextProvider } from 'react-admin';
import { getAuthKeys } from '../../lib/auth_provider';

const CollabList = () => {
  const translate = useTranslate();

  const listContext = useListController({
    disableSyncWithLocation: true,
    filter: {memberIdEq: getAuthKeys().session.accountId },
    perPage: 20,
    queryOptions: {
      refetchInterval: 3000,
    },
    resource: "Collab",
  });

  if (listContext.total == 0 || listContext.isLoading) {
    return <></>;
  }

  return (
    <ListContextProvider value={listContext}>
      <Card id="collab-list" sx={{my:"3em"}}>
        <CardTitle text="collab_list.title">
          <Typography>{ translate("collab_list.text") }.</Typography>
        </CardTitle>
        <Datagrid bulkActionButtons={false}>
					<FunctionField source="status" render={record => 
						<>
							<Typography>{translate(`collab_list.statuses.${record.status}.title`)}</Typography>
							<Typography variant="caption">
								{translate(`collab_list.statuses.${record.status}.text`)}
							</Typography>
						</>
					} />
          <ReferenceField source="campaignId" reference="Campaign">
            <FunctionField label={false} render={record =>
              <a target="_blank" href={viewPostUrl(record)} rel="noreferrer">{ translate("collab_list.see_post") }</a>
            }/>
          </ReferenceField>
          <FunctionField label={ translate("collab_list.reward") } render={record => `${formatEther(record.reward)} DOC`} />
          <FunctionField label={ translate("collab_list.fee") } render={record => 
						record.fee ? `${formatEther(record.fee)} DOC` : "-"
					} />
        </Datagrid>
        <Pagination rowsPerPageOptions={[]} />
      </Card>
    </ListContextProvider>
  );
}

export default CollabList;

