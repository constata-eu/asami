/// <reference types="vite-plugin-svgr/client" />
import {
  Alert,
  Dialog,
  DialogContent,
  Box,
  Button,
  Typography,
  Stack,
  Container,
  Card,
  CardContent,
  DialogTitle,
} from "@mui/material";

import { makeXLogin } from "../lib/auth_provider";
import {
  useTranslate,
  useDataProvider,
  useAuthProvider,
  useRedirect,
  Form,
  TextInput,
  SaveButton,
  email,
  required,
  useNotify,
  CoreAdminContext,
  I18nContext,
} from "react-admin";

import { useNavigate } from "react-router-dom";
import { BareLayout } from "./layout";
import { useContracts } from "../components/contracts_context";
import XIcon from "@mui/icons-material/X";
import { isMobile } from "react-device-detect";
import SendIcon from "@mui/icons-material/Send";
import AsamiLogo from "../assets/logo.svg?react";
import { Head1Primary, Head2, Head3 } from "../components/theme";
import { useContext, useEffect, useState } from "react";
import { publicDataProvider } from "../lib/data_provider";
import { useEmbedded } from "../components/embedded_context";

const Login = () => {
  const [pubDataProvider, setPubDataProvider] = useState();
  const i18nProvider = useContext(I18nContext);

  const authProvider = useAuthProvider();
  const redirect = useRedirect();
  const isEmbedded = useEmbedded();

  useEffect(() => {
    authProvider
      ?.checkAuth({})
      .then(() => {
        // If authenticated, redirect to dashboard
        redirect("/dashboard");
      })
      .catch(() => {
        // Not authenticated, stay on login
      });

    async function initApp() {
      const prov = await publicDataProvider();
      setPubDataProvider(prov);
    }
    initApp();
  }, [authProvider, redirect]);

  if (!pubDataProvider) {
    return <></>;
  }

  return (
    <BareLayout sponsors={false}>
      <Box
        display="flex"
        flexDirection="column"
        my="3em"
        gap="5em"
        alignItems="center"
        minHeight="50vh"
        id="login-form"
      >
        <CoreAdminContext
          i18nProvider={i18nProvider}
          dataProvider={pubDataProvider}
        >
          {isEmbedded ? <WalletConnectTrigger /> : <LoginSelector />}
        </CoreAdminContext>
        <AsamiLogo width="250px" height="auto" />
      </Box>
    </BareLayout>
  );
};

const WalletConnectTrigger = () => {
  const { signLoginMessage } = useContracts();
  const navigate = useNavigate();
  const t = useTranslate();

  useEffect(() => {
    async function init() {
      const code = await signLoginMessage(true);
      navigate(`/eip712_login?code=${code}`);
    }
    init();
  }, []);

  return <Head1Primary>{t("login_form.wallet_connect")}</Head1Primary>;
};

const LoginSelector = ({ open, setOpen }) => {
  const t = useTranslate();
  const { signLoginMessage } = useContracts();
  const navigate = useNavigate();

  const startEip712Login = async () => {
    const code = await signLoginMessage();
    navigate(`/eip712_login?code=${code}`);
  };

  return (
    <Container maxWidth="md">
      <Stack
        direction="row"
        gap="3em"
        alignItems="flex-start"
        flexWrap={{ xs: "wrap", md: "nowrap" }}
      >
        <Card elevation={1} sx={{ flex: "1 0 300px" }}>
          <CardContent>
            <Head2 sx={{ color: "primary.main" }}>
              {t("login_form.title")}
            </Head2>
            <Typography my="1em" color="primary.main">
              {t("login_form.read_our_terms")}
            </Typography>
            <Box display="flex" gap="1em" flexDirection="column">
              <Button
                id="read-terms-first"
                variant="outlined"
                onClick={() => navigate("/about")}
                sx={{ mb: "1em" }}
              >
                {t("login_form.read_terms_button")}
              </Button>

              <LoginWithX />
              <LoginWithEmail />
              <Button
                id="wallet-login-button"
                variant="contained"
                fullWidth
                onClick={startEip712Login}
              >
                {t("login_form.rsk_login_button")}
              </Button>
            </Box>
          </CardContent>
        </Card>
        <Stack sx={{ flex: "0 1 auto", mt: "1em" }}>
          <LoginDisclaimer />
        </Stack>
      </Stack>
    </Container>
  );
};

const LoginWithX = () => {
  const startXLogin = async () => {
    const { url, verifier } = await makeXLogin();
    localStorage.setItem("oauthVerifier", verifier);
    document.location.href = url;
  };

  const t = useTranslate();
  const [open, setOpen] = useState(false);

  return (
    <>
      <Button
        id="x-login-button"
        variant="contained"
        onClick={() => (isMobile ? setOpen(true) : startXLogin())}
        startIcon={<XIcon />}
        fullWidth
      >
        {t("login_form.x_login_button")}
      </Button>
      <Dialog
        open={open}
        onClose={() => setOpen(false)}
        fullWidth
        maxWidth="sm"
        PaperProps={{
          sx: {
            background: (theme) => theme.palette.warning.main,
            border: "1px solid",
            borderColor: (theme) => theme.palette.warning.main,
          },
        }}
      >
        <DialogTitle>
          <Head2
            sx={{
              color: (theme) => theme.palette.inverted.main,
            }}
          >
            {t("login_form.x_mobile_warning.title")}
          </Head2>
        </DialogTitle>
        <DialogContent>
          <Typography color="inverted.main">
            {t("login_form.x_mobile_warning.text")}
          </Typography>
          <Button
            sx={{ mt: "2em" }}
            id="x-login-button"
            fullWidth
            variant="contained"
            color="inverted"
            startIcon={<XIcon />}
            onClick={startXLogin}
          >
            {t("login_form.x_login_button")}
          </Button>
        </DialogContent>
      </Dialog>
    </>
  );
};

const LoginWithEmail = ({ onClick }) => {
  const t = useTranslate();
  const notify = useNotify();
  const [open, setOpen] = useState(false);
  const [sent, setSent] = useState(false);
  const dataProvider = useDataProvider();

  const onSubmit = async (values) => {
    try {
      await dataProvider.create("EmailLoginLink", {
        data: { email: values.email },
      });
      setSent(true);
    } catch {
      notify("login_form.send_email_token.error", { type: "error" });
    }
  };

  const handleClose = () => {
    setSent(false);
    setOpen(false);
  };

  return (
    <>
      <Button
        id="email-login-button"
        variant="contained"
        onClick={() => setOpen(true)}
      >
        {t("login_form.email_login_button")}
      </Button>
      <Dialog open={open} onClose={handleClose} fullWidth maxWidth="sm">
        {!sent ? (
          <>
            <DialogTitle>
              <Head3 sx={{ color: (t) => t.palette.primary.main }}>
                {t("login_form.send_email_token.title")}
              </Head3>
            </DialogTitle>
            <DialogContent>
              <Form sanitizeEmptyValues onSubmit={onSubmit}>
                <TextInput
                  fullWidth
                  validate={[required(), email()]}
                  size="large"
                  variant="filled"
                  source="email"
                  label="Email"
                />
                <SaveButton
                  fullWidth
                  id="submit-email-for-login-link"
                  size="large"
                  label={t("login_form.send_email_token.submit")}
                  icon={<SendIcon />}
                />
              </Form>
            </DialogContent>
          </>
        ) : (
          <Alert variant="filled" severity="success" icon={false}>
            <Head3 sx={{ color: "inverted.main", mb: "0.5em" }}>
              {t("login_form.send_email_token.done")}
            </Head3>
            <Typography sx={{ color: "inherit" }}>
              {t("login_form.send_email_token.done_contact_us")}
            </Typography>
          </Alert>
        )}
      </Dialog>
    </>
  );
};

export default Login;

export const LoginDisclaimer = () => {
  const t = useTranslate();
  return (
    <>
      <Head2>{t("login_form.login_disclaimer.title")}</Head2>
      <Stack spacing="0.5em" sx={{ mt: "1em" }}>
        <Typography fontSize="1.1rem">
          <strong>{t("login_form.login_disclaimer.heads_up")}</strong>
          {t("login_form.login_disclaimer.asami_operates_differently")}
        </Typography>

        <Typography fontSize="1.1rem">
          {t("login_form.login_disclaimer.data_outside_the_eu")}
        </Typography>

        <Typography fontSize="1.1rem">
          {t("login_form.login_disclaimer.no_guarantees")}
        </Typography>

        <Typography fontSize="1.1rem">
          {t("login_form.login_disclaimer.local_laws")}
        </Typography>
      </Stack>
    </>
  );
};
