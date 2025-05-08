/// <reference types="vite-plugin-svgr/client" />
import { useEffect, useContext, useState } from "react";
import {
  Alert,
  Divider,
  Dialog,
  DialogContent,
  Box,
  Button,
  Typography,
  Stack,
  Container,
} from "@mui/material";

import { makeXLogin } from "../lib/auth_provider";
import { publicDataProvider } from "../lib/data_provider";
import {
  useSafeSetState,
  useStore,
  useTranslate,
  useDataProvider,
  Form,
  TextInput,
  SaveButton,
  email,
  required,
} from "react-admin";

import { useNavigate } from "react-router-dom";
import { BareLayout } from "./layout";
import { useContracts } from "../components/contracts_context";
import XIcon from "@mui/icons-material/X";
import WalletIcon from "@mui/icons-material/Wallet";
import { isMobile } from "react-device-detect";
import AlternateEmailIcon from "@mui/icons-material/AlternateEmail";
import SendIcon from "@mui/icons-material/Send";
import AsamiLogo from "../assets/logo.svg?react";
import { Head1, Head2, Lead } from "../components/theme";
import { Link } from "react-router-dom";
import ThumbDownAltIcon from "@mui/icons-material/ThumbDownAlt";

const Login = () => {
  return (
    <BareLayout sponsors={false}>
      <Box
        display="flex"
        flexDirection="column"
        my="3em"
        gap="3em"
        alignItems="center"
        minHeight="50vh"
        id="login-form"
      >
        <LoginSelector />
        <AsamiLogo width="150px" height="auto" />
      </Box>
    </BareLayout>
  );
};

const LoginSelector = ({ open, setOpen }) => {
  const translate = useTranslate();
  const { signLoginMessage } = useContracts();
  const navigate = useNavigate();

  const startXLogin = async () => {
    setOpen(false);
    const { url, verifier } = await makeXLogin();
    localStorage.setItem("oauthVerifier", verifier);
    document.location.href = url;
  };

  const startEip712Login = async () => {
    setOpen(false);
    const code = await signLoginMessage();
    navigate(`/eip712_login?code=${code}`);
  };

  /*
        <Typography
          sx={{ color: "inherit" }}
          fontSize="1.2em"
          fontFamily="LeagueSpartanBlack"
          letterSpacing="-0.03em"
        >
          {translate("login_form.disclaimer_title")}
        </Typography>
        {translate("login_form.disclaimer_body")}
        <Box display="flex" gap="1em" flexWrap="wrap" mt="1em">
          <Button
            sx={{ flex: "1 1 300px" }}
            href="https://eur-lex.europa.eu/legal-content/EN/TXT/?uri=CELEX%3A32016R0679"
            id="no-login-button"
            color="warning"
            variant="outlined"
          >
            {translate("login_form.reject_button")}
          </Button>
          <Button
            sx={{ flex: "1 1 100px" }}
            href="/#/about"
            id="more-on-privacy"
            color="warning"
          >
            {translate("login_form.read_more")}
          </Button>
        </Box>
      */

  return (
    <Container maxWidth="md">
      <Stack direction="row" gap="2em" alignItems="end">
        <Stack flex="0 0 50%">
          <Head2>Before you log in,</Head2>
          <Lead>
            One of our club innovative aspects is blockchain based fairness and
            transparency to all members, to achieve it we need to operate it in
            a way that is silghtly different from other websites, some of the
            policies and guarantees you're used to from other services are not
            applicable to asami.
          </Lead>
          <Typography color="secondary.main">
            The service is provided as-is, no warranty to be fit for any
            particular purpose
          </Typography>
          <Typography color="secondary.main">
            You are ultimately responsible to know if your usage of asami is in
            compliance with your local laws and third parties terms of service,
            including X.
          </Typography>
          <Typography color="secondary.main">
            We will transfer your data outside the EU, and it may be stored and
            disclosed forever.
          </Typography>
          <Typography color="secondary.main">
            You can learn more about these policies reading our{" "}
            <Link to="/about">Whitepaper</Link>{" "}
          </Typography>
        </Stack>
        <Box>
          <Box display="flex" gap="1em" flexDirection="column">
            {isMobile ? (
              <LoginWithXMobileWarning onClick={startXLogin} />
            ) : (
              <Button
                id="x-login-button"
                variant="outlined"
                onClick={startXLogin}
                startIcon={<XIcon />}
              >
                I agree, with X
              </Button>
            )}
            <LoginWithEmail />
            <Button
              id="wallet-login-button"
              variant="outlined"
              onClick={startEip712Login}
              startIcon={<WalletIcon />}
            >
              I agree, with a WEB3 wallet
            </Button>
            <Button
              id="wallet-login-button"
              color="secondary"
              variant="outlined"
              startIcon={<ThumbDownAltIcon />}
            >
              I don't agree.
            </Button>
          </Box>
        </Box>
      </Stack>
    </Container>
  );
};

const LoginWithXMobileWarning = ({ onClick }) => {
  const translate = useTranslate();
  const [open, setOpen] = useSafeSetState(false);
  return (
    <Box width="100%">
      <Button
        id="x-login-button"
        fullWidth
        variant="contained"
        color="inverted"
        onClick={() => setOpen(true)}
      >
        {translate("login_form.consent_with_x")}
      </Button>
      <Dialog
        open={open}
        onClose={() => setOpen(false)}
        fullWidth
        maxWidth="sm"
        slotProps={{ backdrop: { sx: { background: "rgba(0,0,0,0.9)" } } }}
        PaperProps={{ variant: "outlined", sx: { borderWidth: "10px" } }}
      >
        <DialogContent>
          <Typography
            mb="1em"
            fontSize="1.2em"
            fontFamily="LeagueSpartanBlack"
            letterSpacing="-0.03em"
          >
            {translate("login_form.x_mobile_warning.title")}
          </Typography>
          <Typography mb="1em">
            {translate("login_form.x_mobile_warning.text")}
          </Typography>
          <Button
            id="x-login-button"
            fullWidth
            variant="contained"
            color="inverted"
            onClick={onClick}
          >
            {translate("login_form.consent_with_x")}
          </Button>
        </DialogContent>
      </Dialog>
    </Box>
  );
};

const LoginWithEmail = ({ onClick }) => {
  const translate = useTranslate();
  const [open, setOpen] = useSafeSetState(false);
  const [sent, setSent] = useSafeSetState(false);
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
        variant="outlined"
        startIcon={<AlternateEmailIcon />}
        onClick={() => setOpen(true)}
      >
        I agree, with Email
      </Button>
      <Dialog
        open={open}
        onClose={handleClose}
        fullWidth
        maxWidth="sm"
        slotProps={{ backdrop: { sx: { background: "rgba(0,0,0,0.9)" } } }}
        PaperProps={{ variant: "outlined", sx: { borderWidth: "10px" } }}
      >
        {!sent ? (
          <DialogContent>
            <Typography
              mb="1em"
              fontSize="1.2em"
              fontFamily="LeagueSpartanBlack"
              letterSpacing="-0.03em"
            >
              {translate("login_form.send_email_token.title")}
            </Typography>
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
                label={translate("login_form.send_email_token.submit")}
                icon={<SendIcon />}
              />
            </Form>
          </DialogContent>
        ) : (
          <Alert severity="success" icon={false}>
            <Typography
              sx={{ color: "inherit" }}
              fontSize="1.5em"
              fontFamily="LeagueSpartanBlack"
              letterSpacing="-0.03em"
            >
              {translate("login_form.send_email_token.done")}
            </Typography>
            <Typography sx={{ color: "inherit" }}>
              {translate("login_form.send_email_token.done_contact_us")}
            </Typography>
          </Alert>
        )}
      </Dialog>
    </>
  );
};

export default Login;
