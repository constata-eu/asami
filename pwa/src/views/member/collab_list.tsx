import {
  useTranslate,
  ReferenceField,
  ListBase,
  ExportButton,
  RecordContextProvider,
} from "react-admin";
import { Box, Card, Chip, Stack, Typography } from "@mui/material";
import { Head2 } from "../../components/theme";
import { useListController } from "react-admin";
import { getAuthKeys } from "../../lib/auth_provider";
import { AmountField } from "../../components/custom_fields";

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
      <ListBase disableAuthentication disableSyncWithLocation {...listContext}>
        <Stack gap="1em" mb="1em" alignItems="baseline" direction="row">
          <Head2>{translate("collab_list.title")}</Head2>
          <ExportButton disabled={listContext.total === 0} resource="Collab" />
        </Stack>
        <Typography variant="body2" sx={{ mb: "1em", opacity: 0.75 }}>
          {translate("collab_list.text")}
        </Typography>
        <Stack gap="1em">
          {(listContext.data ?? []).map((record) => (
            <RecordContextProvider key={record.id} value={record}>
              <Card
                elevation={0}
                sx={{
                  p: "1em",
                  borderRadius: "18px",
                  border: "1px solid",
                  borderColor: "primary.main",
                  background:
                    "linear-gradient(135deg, rgba(255,255,255,0.9), rgba(255,242,246,0.85))",
                }}
              >
                <Stack
                  direction={{ xs: "column", md: "row" }}
                  gap="1em"
                  justifyContent="space-between"
                >
                  <Stack gap="0.5em" sx={{ minWidth: 0 }}>
                    <Chip
                      label={translate(
                        `collab_list.statuses.${record.status}.title`
                      )}
                      color={record.status === "CLEARED" ? "primary" : "default"}
                      sx={{ width: "fit-content" }}
                    />
                    <Typography fontFamily="LeagueSpartanBold" fontSize="1.2em">
                      {translate(`collab_list.statuses.${record.status}.text`)}
                    </Typography>
                    <Typography variant="body2" sx={{ opacity: 0.75 }}>
                      {translate("collab_list.campaign")}:{" "}
                      <ReferenceField
                        source="campaignId"
                        reference="Campaign"
                        link="show"
                      />
                    </Typography>
                    <Typography variant="body2" sx={{ opacity: 0.75 }}>
                      {translate("collab_list.community")}:{" "}
                      <ReferenceField
                        source="advertiserId"
                        reference="Account"
                        sortable={false}
                        link="show"
                      />
                    </Typography>
                  </Stack>

                  <Stack
                    direction={{ xs: "row", md: "column" }}
                    gap="1em"
                    justifyContent="space-between"
                    alignItems={{ xs: "flex-start", md: "flex-end" }}
                  >
                    <Box>
                      <Typography variant="caption" sx={{ opacity: 0.75 }}>
                        {translate("collab_list.reward")}
                      </Typography>
                      <Typography fontFamily="LeagueSpartanBold" fontSize="1.3em">
                        <AmountField source="reward" label="" currency="" />
                      </Typography>
                    </Box>
                    <Box>
                      <Typography variant="caption" sx={{ opacity: 0.75 }}>
                        {translate("collab_list.fee")}
                      </Typography>
                      <Typography fontFamily="LeagueSpartanBold" fontSize="1.1em">
                        <AmountField source="fee" label="" currency="" />
                      </Typography>
                    </Box>
                  </Stack>
                </Stack>
              </Card>
            </RecordContextProvider>
          ))}
        </Stack>
      </ListBase>
    </Box>
  );
};

export default CollabList;
