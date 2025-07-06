import {
  Alert,
  AlertTitle,
  Box,
  Button,
  Card,
  Container,
  Dialog,
  DialogActions,
  DialogContent,
  DialogTitle,
  Paper,
  Stack,
  Typography,
} from "@mui/material";
import { useState } from "react";
import {
  useAuthenticated,
  useDataProvider,
  useGetList,
  useGetOne,
  useLogout,
  useTranslate,
} from "react-admin";
import { useLocation, useNavigate, useParams } from "react-router-dom";
import { getAuthKeys } from "../lib/auth_provider";
import { useContracts } from "../components/contracts_context";
import { useEmbedded } from "../components/embedded_context";
import { Head1, Head1Primary, Head2, Head3, Lead } from "../components/theme";

export const MergeAccounts = () => {
  const { isPending, isError } = useAuthenticated(); // redirects to login if not authenticated

  if (isPending || isError) {
    return <></>;
  }
  return <AccountMergeManager />;
};

const AccountMergeManager = (onClick) => {
  const [step, setStep] = useState("WELCOME");
  const [request, setRequest] = useState(null);
  const dataProvider = useDataProvider();
  const navigate = useNavigate();
  const t = useTranslate();

  const { isLoading } = useGetList(
    "AccountMerge",
    {
      pagination: { page: 1, perPage: 1 },
      sort: {},
      filter: {},
    },
    {
      refetchInterval: 5000,
      onSuccess: (response) => {
        const item = response?.data?.[0];
        if (item) {
          setRequest(item);
          setStep(item.status);
        }
      },
    },
  );

  if (isLoading) {
    return <></>;
  }

  const createRequest = async () => {
    const response = await dataProvider.create("AccountMerge", { data: {} });
    setRequest(response.data);
    setStep("PENDING");
  };

  return (
    <Container maxWidth="md" sx={{ py: "1em" }}>
      {step == "WELCOME" && <Welcome onClick={() => createRequest()} />}
      {step == "PENDING" && <Pending request={request} />}
      {step != "DONE" && (
        <Button
          id="changed-my-mind"
          sx={{ mt: "2em" }}
          fullWidth
          onClick={() => navigate("/")}
        >
          {t("merge_accounts.changed_my_mind")}
        </Button>
      )}
      {step == "DONE" && <Done request={request} />}
    </Container>
  );
};

const Welcome = ({ onClick }) => {
  const t = useTranslate();
  return (
    <Stack gap="2em">
      <Head1Primary>{t("merge_accounts.welcome.title")}</Head1Primary>
      <Box>
        <Head3>{t("merge_accounts.welcome.title_1")}</Head3>
        <Typography>{t("merge_accounts.welcome.text_1")}</Typography>
      </Box>
      <Box>
        <Head3>{t("merge_accounts.welcome.title_2")}</Head3>
        <Typography>{t("merge_accounts.welcome.text_2")}</Typography>
      </Box>
      <Box>
        <Head3>{t("merge_accounts.welcome.title_3")}</Head3>
        <Typography>{t("merge_accounts.welcome.text_3")}</Typography>
      </Box>
      <Button
        id="start-merging-button"
        sx={{ mt: "1em" }}
        variant="contained"
        fullWidth
        onClick={onClick}
      >
        {t("merge_accounts.welcome.button")}
      </Button>
    </Stack>
  );
};

const Pending = ({ request }) => {
  const t = useTranslate();
  const link = `asami.club/m/${request.code}`;

  const handleCopy = async () => {
    try {
      await navigator.clipboard.writeText(link);
    } catch (err) {
      console.error("cannot_copy", err);
    }
  };

  return (
    <Stack gap="1em">
      <Head1Primary>{t("merge_accounts.pending.title")}</Head1Primary>
      <Lead>{t("merge_accounts.pending.lead")}</Lead>
      <Paper sx={{ p: "1em", textAlign: "center" }}>
        <Typography id="merge-link-text" sx={{ fontFamily: "monospace" }}>
          {link}
        </Typography>
      </Paper>
      <Typography>
        {t("merge_accounts.pending.you_will_need_to_accept")}
      </Typography>
      <Typography>{t("merge_accounts.pending.notice")}</Typography>
      <Button
        id="copy-merge-link"
        sx={{ mt: "1em" }}
        variant="contained"
        fullWidth
        onClick={handleCopy}
      >
        {t("merge_accounts.pending.copy_link")}
      </Button>
    </Stack>
  );
};

const Done = ({ request }) => {
  const navigate = useNavigate();
  const t = useTranslate();
  return (
    <Stack id="account-merged-succesfully" gap="1em">
      <Head1Primary>{t("merge_accounts.done.title")}</Head1Primary>
      <Typography>{t("merge_accounts.done.message")}</Typography>
      <Button
        sx={{ mt: "1em" }}
        variant="contained"
        fullWidth
        onClick={() => navigate("/")}
      >
        {t("merge_accounts.done.button")}
      </Button>
    </Stack>
  );
};

export const AcceptMergeFromSource = () => {
  const { isPending, isError } = useAuthenticated(); // redirects to login if not authenticated

  if (isPending || isError) {
    return <></>;
  }
  return <AcceptMergeFromSourceInner />;
};

const AcceptMergeFromSourceInner = () => {
  const logout = useLogout();
  const location = useLocation();
  const t = useTranslate();
  const [open, setOpen] = useState(false);
  const [done, setDone] = useState(false);
  const [error, setError] = useState(false);
  const { code } = useParams<{ code: string }>();
  const { data, isLoading } = useGetOne(
    "Account",
    { id: getAuthKeys().session.accountId },
    { refetchInterval: done ? false : 10000 },
  );
  const dataProvider = useDataProvider();

  const sendAcceptance = async () => {
    try {
      await dataProvider.update("AccountMerge", { id: code });
      setDone(true);
    } catch (e) {
      setError(
        e?.body?.graphQLErrors?.[0]?.extensions?.error?.message || "unknown",
      );
    }
  };

  const reLogin = () => {
    localStorage.setItem("postLoginRedirect", `${location.pathname}`);
    logout();
  };

  const timestamp = new Date(
    getAuthKeys().session.createdAt.replace(/(\.\d{3})\d*Z$/, "$1Z"),
  );
  const now = new Date();
  const mustRelogin = now - timestamp > 10 * 60 * 1000;

  if (isLoading || !data) {
    return <></>;
  }

  if (error) {
    return (
      <Container sx={{ p: "1em", height: "100%" }} maxWidth="sm">
        <Alert
          icon={false}
          id={`alert-error-${error}`}
          severity="error"
          variant="filled"
        >
          <AlertTitle>
            <Head1 sx={{ color: "inverted.main" }}>
              {t("merge_accounts.accept_merge.error.title")}
            </Head1>
          </AlertTitle>
          <Typography>
            {t(`merge_accounts.accept_merge.error.messages.${error}`)}
          </Typography>
        </Alert>
      </Container>
    );
  }

  if (done) {
    return (
      <Container sx={{ p: "1em", height: "100%" }} maxWidth="sm">
        <Alert id="merge-done" severity="primary" variant="filled">
          <AlertTitle>
            <Head1 sx={{ color: "inverted.main" }}>
              {t("merge_accounts.accept_merge.done.title")}
            </Head1>
          </AlertTitle>
          <Typography>
            {t("merge_accounts.accept_merge.done.message")}
          </Typography>
        </Alert>
      </Container>
    );
  }

  const hasPending =
    BigInt(data.unclaimedDocBalance) > BigInt(0) ||
    BigInt(data.unclaimedAsamiBalance) > BigInt(0);

  return (
    <Container maxWidth="md">
      <Stack gap="1em">
        <Head1Primary>{t("merge_accounts.accept_merge.title")}</Head1Primary>

        {!mustRelogin && (
          <Alert severity="warning" variant="filled">
            <AlertTitle>
              {t("merge_accounts.accept_merge.security_alert_title")}
            </AlertTitle>
            <Typography>
              {t("merge_accounts.accept_merge.security_alert_text")}
            </Typography>
          </Alert>
        )}

        <Typography>{t("merge_accounts.accept_merge.description")}</Typography>

        {!!data.addr && (
          <Typography>
            {t("merge_accounts.accept_merge.wallet_loss_warning", {
              addr: data.addr,
            })}
          </Typography>
        )}

        {hasPending && (
          <Typography>
            {t("merge_accounts.accept_merge.pending_rewards_warning")}
          </Typography>
        )}

        {mustRelogin && (
          <Button
            id="login-again"
            sx={{ mt: "1em" }}
            variant="contained"
            fullWidth
            onClick={reLogin}
          >
            Login again to merge
          </Button>
        )}
        {!mustRelogin && (
          <Button
            id="accept-merge-button"
            sx={{ mt: "1em" }}
            variant="contained"
            fullWidth
            onClick={() => setOpen(true)}
          >
            {t("merge_accounts.accept_merge.cta_button")}
          </Button>
        )}
      </Stack>
      <Dialog open={open}>
        <DialogTitle>
          {t("merge_accounts.accept_merge.confirm_dialog.title")}
        </DialogTitle>
        <DialogContent>
          {t("merge_accounts.accept_merge.confirm_dialog.description")}
        </DialogContent>
        <DialogActions>
          <Button
            id="cancel-merge-button"
            sx={{ mt: "1em" }}
            variant="outlined"
            fullWidth
            onClick={() => setOpen(false)}
          >
            {t("merge_accounts.accept_merge.confirm_dialog.cancel_button")}
          </Button>
          <Button
            id="confirm-merge-button"
            sx={{ mt: "1em" }}
            variant="outlined"
            fullWidth
            onClick={sendAcceptance}
          >
            {t("merge_accounts.accept_merge.confirm_dialog.confirm_button")}
          </Button>
        </DialogActions>
      </Dialog>
    </Container>
  );
};
