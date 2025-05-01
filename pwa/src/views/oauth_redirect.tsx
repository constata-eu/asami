import { useCallback, useState, useEffect, useRef } from "react";
import {
  Alert,
  Typography,
  Box,
  CircularProgress,
  Button,
} from "@mui/material";
import {
  useTranslate,
  useNotify,
  useDataProvider,
  useRedirect,
} from "react-admin";
import { useNavigate } from "react-router-dom";
import { useSearchParams } from "react-router-dom";
import { Head2 } from "../components/theme";
import { BareLayout } from "./layout";
import authProvider from "../lib/auth_provider";
import { NoAccounts, LiveHelp, Replay } from "@mui/icons-material";
import { GoogleReCaptcha } from "react-google-recaptcha-v3";

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
        navigate("/");
      } catch (e) {
        setError(e.message || translate("oauth_redirect.unexpected_error"));
      }
    },
    [authProvider, authMethodKind, authData, navigate],
  );

  return !error ? (
    <>
      <CircularProgress sx={{ mb: 3 }} />
      <Head2>Logging you in, this won&apos;t take long.</Head2>
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
      <NoAccounts sx={{ mb: 3, width: "2em", height: "auto" }} />
      <Head2 sx={{ mb: 3 }}>
        {translate("oauth_redirect.unexpected_error")}
      </Head2>
      <Typography>
        {translate("oauth_redirect.error_description_title")}
      </Typography>
      <Alert severity="info" sx={{ my: "2em" }}>
        {error}
      </Alert>

      <Button
        sx={{ mb: 2 }}
        variant="contained"
        onClick={() => navigate("/login")}
        startIcon={<Replay />}
      >
        {translate("oauth_redirect.try_again")}
      </Button>
      <Button
        variant="outlined"
        href="https://t.me/asami_club"
        startIcon={<LiveHelp />}
      >
        {translate("oauth_redirect.contact_us")}
      </Button>
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
      redirect("/");
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
        redirect("/");
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
          <Head2>{translate("oauth_redirect.x_waiting_for_access")}</Head2>
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
    <BareLayout>
      <Box
        display="flex"
        flexDirection="column"
        marginTop="3em"
        alignItems="center"
        minHeight="50vh"
      >
        {hasRedirectState ? (
          children
        ) : (
          <Errors error={translate("oauth_redirect.invalid_redirect_state")} />
        )}
      </Box>
    </BareLayout>
  );
};
