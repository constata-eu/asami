import {
  Form,
  NumberInput,
  SaveButton,
  TextInput,
  useDataProvider,
  useAuthenticated,
  useListController,
  useNotify,
  useTranslate,
} from "react-admin";
import { useState } from "react";
import {
  Box,
  Button,
  Card,
  CardContent,
  Dialog,
  DialogContent,
  DialogTitle,
  Stack,
  Typography,
} from "@mui/material";
import { getAuthKeys } from "../../lib/auth_provider";
import BalanceCard from "../balance_card";
import CollabList from "./collab_list";
import TitleCard from "./title_card";
import { XCampaign } from "./campaigns";
import { HandleSettings } from "./handle_settings";
import { ResponsiveAppBar } from "../responsive_app_bar";
import { CardTable } from "../layout";
import { Head2, Lead } from "../../components/theme";
import { normalizeRootstockJoinedAt } from "../../lib/rootstock_joined_at";

type ScorableAccountRecord = {
  id: string;
  accountId: string;
  name: string;
  reason: string;
  scorableUntil: string;
};

const Dashboard = () => {
  useAuthenticated();

  const campaigns = useListController({
    disableSyncWithLocation: true,
    filter: { availableToAccountId: getAuthKeys().session.accountId },
    perPage: 20,
    queryOptions: {
      refetchInterval: 10000,
    },
    resource: "Campaign",
  });

  const handles = useListController({
    disableSyncWithLocation: true,
    filter: { accountIdEq: getAuthKeys().session.accountId },
    queryOptions: {
      refetchInterval: 20000,
    },
    resource: "Handle",
  });

  return (
    <Box id="member-dashboard">
      <ResponsiveAppBar />
      <CardTable mb="2em">
        <TitleCard handles={handles} campaigns={campaigns} />
        <Box mb="1em">
          <BalanceCard />
        </Box>
        <HandleSettings handles={handles} />
        <ScoringTasksCard />
        <CampaignList handles={handles} />
      </CardTable>
      <CollabList />
    </Box>
  );
};

const ScoringTasksCard = () => {
  const t = useTranslate();
  const notify = useNotify();
  const dataProvider = useDataProvider();
  const listContext = useListController({
    disableSyncWithLocation: true,
    perPage: 50,
    queryOptions: {
      refetchInterval: 30000,
    },
    resource: "ScorableAccount",
  });

  const [selected, setSelected] = useState<ScorableAccountRecord | null>(null);

  if (listContext.isLoading || !listContext.data?.length) {
    return <></>;
  }

  const onSkip = async (record: ScorableAccountRecord) => {
    try {
      await dataProvider.create("CommunityMemberSkip", {
        data: {
          memberId: record.accountId,
          reason: "dont_know_enough",
        },
      });
      notify(t("scoring_tasks.skip_success"), { type: "success" });
      await listContext.refetch();
    } catch (error: any) {
      notify(error?.message || t("scoring_tasks.skip_error"), { type: "error" });
    }
  };

  return (
    <Card sx={{ mb: "1em", minWidth: "320px" }}>
      <CardContent>
        <Head2>{t("scoring_tasks.title")}</Head2>
        <Lead>{t("scoring_tasks.description")}</Lead>
        <Stack gap="0.75em" mt="1em">
          {listContext.data.map((record) => (
            <Card key={record.id} variant="outlined">
              <CardContent>
                <Stack gap="0.5em">
                  <Typography fontFamily="'LeagueSpartanBold'" fontSize="1.2em">
                    {record.name}
                  </Typography>
                  <Typography variant="body2">
                    {t(`scoring_tasks.reasons.${record.reason}`, { _: record.reason })}
                  </Typography>
                  <Typography variant="body2">
                    {t("scoring_tasks.until", { date: new Date(record.scorableUntil).toLocaleString() })}
                  </Typography>
                  <Stack direction="row" gap="0.75em">
                    <Button variant="contained" onClick={() => setSelected(record)}>
                      {t("scoring_tasks.score")}
                    </Button>
                    <Button variant="outlined" onClick={() => onSkip(record)}>
                      {t("scoring_tasks.skip")}
                    </Button>
                  </Stack>
                </Stack>
              </CardContent>
            </Card>
          ))}
        </Stack>
      </CardContent>
      <ScoreDialog
        open={!!selected}
        record={selected}
        onClose={async (refresh) => {
          setSelected(null);
          if (refresh) {
            await listContext.refetch();
          }
        }}
      />
    </Card>
  );
};

const ScoreDialog = ({
  open,
  record,
  onClose,
}: {
  open: boolean;
  record: ScorableAccountRecord | null;
  onClose: (refresh: boolean) => Promise<void>;
}) => {
  const t = useTranslate();
  const notify = useNotify();
  const dataProvider = useDataProvider();

  const onSubmit = async (values: Record<string, any>) => {
    try {
      await dataProvider.create("CommunityMember", {
        data: {
          memberId: record.accountId,
          priority: false,
          alignment: Number(values.alignment),
          trust: Number(values.trust),
          contribution: Number(values.contribution),
          advocacy: Number(values.advocacy),
          judgment: Number(values.judgment),
          rootstockJoinedAt: normalizeRootstockJoinedAt(values.rootstockJoinedAt),
          comment: values.comment || null,
        },
      });
      notify(t("scoring_tasks.score_success"), { type: "success" });
      await onClose(true);
    } catch (error: any) {
      notify(error?.message || t("scoring_tasks.score_error"), { type: "error" });
    }
  };

  return (
    <Dialog open={open} onClose={() => onClose(false)} maxWidth="md" fullWidth>
      <DialogTitle>
        {record ? t("scoring_tasks.dialog_title", { name: record.name }) : ""}
      </DialogTitle>
      <DialogContent>
        {record && (
          <Form
            sanitizeEmptyValues
            onSubmit={onSubmit}
            defaultValues={{
              alignment: 5,
              trust: 5,
              contribution: 5,
              advocacy: 5,
              judgment: 5,
            }}
          >
            <Stack gap="0.75em" mt="0.5em">
              <NumberInput source="alignment" min={1} max={10} />
              <NumberInput source="trust" min={1} max={10} />
              <NumberInput source="contribution" min={1} max={10} />
              <NumberInput source="advocacy" min={1} max={10} />
              <NumberInput source="judgment" min={1} max={10} />
              <TextInput source="rootstockJoinedAt" type="date" />
              <TextInput source="comment" multiline minRows={3} fullWidth />
              <SaveButton label={t("scoring_tasks.submit")} />
            </Stack>
          </Form>
        )}
      </DialogContent>
    </Dialog>
  );
};

const CampaignList = ({ handles }) => {
  const dataProvider = useDataProvider();
  const listContext = useListController({
    disableSyncWithLocation: true,
    filter: { availableToAccountId: getAuthKeys().session.accountId },
    perPage: 20,
    queryOptions: {
      refetchInterval: 6000,
    },
    resource: "Campaign",
  });

  const prefsContext = useListController({
    disableSyncWithLocation: true,
    perPage: 200,
    resource: "CampaignPreference",
  });

  if (
    prefsContext.isLoading ||
    listContext.isLoading ||
    !listContext.total ||
    handles.isLoading ||
    !handles.total
  ) {
    return <></>;
  }

  const setPreference = async (campaignId, notInterested, attempted) => {
    await dataProvider.create("CampaignPreference", {
      data: { input: { campaignId, notInterested, attempted } },
    });
    await listContext.refetch();
    if (attempted) {
      await prefsContext.refetch();
    }
  };

  const xHandle = handles.data?.filter((x) => x.site == "X")[0];

  return (
    <>
      {listContext.data.map((item) => (
        <XCampaign
          key={item.id}
          handle={xHandle}
          campaign={item}
          prefsContext={prefsContext}
          setPreference={setPreference}
          height="300px"
        />
      ))}
    </>
  );
};

export default Dashboard;
