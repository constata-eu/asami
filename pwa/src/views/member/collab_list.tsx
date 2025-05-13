import {
  useTranslate,
  ReferenceField,
  TextField,
  ListBase,
  ListView,
} from "react-admin";
import { viewPostUrl } from "../../lib/campaign";
import { Box, Card, Typography } from "@mui/material";
import { formatEther } from "ethers";
import { CardTitle, Head1, Head2, Lead } from "../../components/theme";
import { Pagination, Datagrid, FunctionField } from "react-admin";
import { useListController, ListContextProvider } from "react-admin";
import { getAuthKeys } from "../../lib/auth_provider";
import { AmountField } from "../../components/custom_fields";
import { Link } from "react-router-dom";

const CollabList = () => {
  const translate = useTranslate();

  const listContext = useListController({
    disableSyncWithLocation: true,
    filter: { memberIdEq: getAuthKeys().session.accountId },
    sort: { field: "id", order: "DESC" },
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
    <Box id="collab-list" sx={{ mt: "1em", mb: "2em" }}>
      <Head2 sx={{ mt: "2em" }}>{translate("collab_list.title")}.</Head2>
      <ListBase disableAuthentication disableSyncWithLocation {...listContext}>
        <ListView>
          <Datagrid bulkActionButtons={false} resource="Collab">
            <FunctionField
              source="status"
              render={(record) => (
                <>
                  <Typography>
                    {translate(`collab_list.statuses.${record.status}.title`)}
                  </Typography>
                  <Typography variant="caption">
                    {translate(`collab_list.statuses.${record.status}.text`)}
                  </Typography>
                </>
              )}
            />
            <ReferenceField
              source="campaignId"
              reference="Campaign"
              link="show"
            />
            <AmountField textAlign="right" source="reward" />
            <AmountField textAlign="right" source="fee" />
            <ReferenceField
              source="advertiserId"
              reference="Account"
              sortable={false}
              link="show"
            />
          </Datagrid>
        </ListView>
      </ListBase>
    </Box>
  );
};

export default CollabList;
