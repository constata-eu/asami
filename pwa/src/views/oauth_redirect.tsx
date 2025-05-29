import { useCallback, useState, useEffect, useRef } from "react";
import {
  Alert,
  Typography,
  Box,
  CircularProgress,
  Button,
  Stack,
} from "@mui/material";
import {
  useTranslate,
  useNotify,
  useDataProvider,
  useRedirect,
} from "react-admin";
import { useNavigate } from "react-router-dom";
import { useSearchParams } from "react-router-dom";
import { Head2, Head3, Lead } from "../components/theme";
import { BareLayout } from "./layout";
import authProvider from "../lib/auth_provider";
import { NoAccounts, LiveHelp, Replay } from "@mui/icons-material";
import { GoogleReCaptcha } from "react-google-recaptcha-v3";
import AsamiLogo from "../assets/logo.svg?react";

export const OneTimeTokenLogin = () => {
  const [searchParams] = useSearchParams();
  const token = searchParams.get("token");

  return (
    <Layout hasRedirectState={token}>
      <RegularLogin authData={token} authMethodKind="OneTimeToken" />
    </Layout>
  );
};

export const Eip712Login = () => {
  const [searchParams] = useSearchParams();
  const code = searchParams.get("code");

  return (
    <Layout hasRedirectState={code}>
      <RegularLogin authData={code} authMethodKind="Eip712" />
    </Layout>
  );
};

export const XLogin = () => {
  const [searchParams] = useSearchParams();
  const code = searchParams.get("code");
  const oauthVerifier = localStorage.getItem("oauthVerifier");
  const authData = JSON.stringify({ code, oauthVerifier });

  return (
    <Layout hasRedirectState={authData}>
      <RegularLogin authData={authData} authMethodKind="X" />
    </Layout>
  );
};

const RegularLogin = ({ authData, authMethodKind }) => {
  const navigate = useNavigate();
  const translate = useTranslate();
  const [error, setError] = useState();

  const onVerify = useCallback(
    async (recaptchaToken) => {
      try {
        await authProvider.login(authMethodKind, authData, recaptchaToken);
        navigate("/dashboard");
      } catch (e) {
        setError(e.message || translate("oauth_redirect.unexpected_error"));
      }
    },
    [authProvider, authMethodKind, authData, navigate],
  );

  return !error ? (
    <>
      <CircularProgress sx={{ mb: 3 }} />
      <Head2 sx={{ color: "primary.main" }}>
        {translate("oauth_redirect.logging_you_in")}
      </Head2>
      <GoogleReCaptcha onVerify={onVerify} />
    </>
  ) : (
    <Errors error={error} />
  );
};

const Errors = ({ error }) => {
  const translate = useTranslate();
  const navigate = useNavigate();

  return (
    <>
      <Alert
        severity="error"
        variant="filled"
        sx={{ fontSize: "1em", mt: "3em" }}
      >
        <Head3 sx={{ color: "inverted.main" }}>
          {translate("oauth_redirect.unexpected_error")}
        </Head3>
        {error}
        <Stack
          mt="2em"
          direction="row"
          gap="1em"
          flexWrap="wrap"
          justifyContent="stretch"
        >
          <Button
            color="inverted"
            variant="contained"
            onClick={() => navigate("/login")}
            startIcon={<Replay />}
            sx={{ flexGrow: 1 }}
          >
            {translate("oauth_redirect.try_again")}
          </Button>
          <Button
            variant="outlined"
            color="inverted"
            href="https://t.me/asami_club"
            startIcon={<LiveHelp />}
          >
            {translate("oauth_redirect.contact_us")}
          </Button>
        </Stack>
      </Alert>
    </>
  );
};

export const XGrantAccess = () => {
  const [searchParams] = useSearchParams();
  const notify = useNotify();
  const translate = useTranslate();
  const code = searchParams.get("code");
  const verifier = localStorage.getItem("grantAccessOauthVerifier");
  const [error, setError] = useState<null | string>(null);
  const redirect = useRedirect();
  const dataProvider = useDataProvider();
  const hasRun = useRef(false);

  useEffect(() => {
    if (hasRun.current) {
      return redirect("/dashboard");
    }
    hasRun.current = true;

    async function fetchData() {
      try {
        await dataProvider.create("XRefreshToken", {
          data: { token: code, verifier },
        });

        notify("oauth_redirect.x_grant_access_success", {
          type: "success",
        });
        redirect("/dashboard");
      } catch (err) {
        setError(translate("oauth_redirect.x_grant_access_error"));
      }
    }

    fetchData();
  }, []);

  return (
    <Layout hasRedirectState={code}>
      {!error ? (
        <>
          <CircularProgress sx={{ mb: 3 }} />
          <Head2 sx={{ color: "primary.main" }}>
            {translate("oauth_redirect.x_waiting_for_access")}
          </Head2>
        </>
      ) : (
        <Errors error={error} />
      )}
    </Layout>
  );
};

const Layout = ({ hasRedirectState, children }) => {
  const translate = useTranslate();

  return (
    <BareLayout sponsors={false}>
      <Box
        display="flex"
        flexDirection="column"
        my="3em"
        gap="3em"
        alignItems="center"
        minHeight="50vh"
      >
        {hasRedirectState ? (
          children
        ) : (
          <Errors error={translate("oauth_redirect.invalid_redirect_state")} />
        )}
        <AsamiLogo width="150px" height="auto" />
      </Box>
    </BareLayout>
  );
};
