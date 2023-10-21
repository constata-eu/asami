import { useCallback } from 'react';
import { Alert, Typography, Box, CircularProgress, Button } from '@mui/material';
import { useTranslate, useSafeSetState, useStore } from 'react-admin';
import { useNavigate } from 'react-router-dom';
import { useSearchParams } from "react-router-dom";
import { Head2 } from "../components/theme";
import { BareLayout } from './layout';
import { Settings } from '../settings';
import authProvider from '../lib/auth_provider';
import { NoAccounts, LiveHelp, Replay } from '@mui/icons-material';
import {
  GoogleReCaptchaProvider,
  GoogleReCaptcha
} from 'react-google-recaptcha-v3';

export const Eip712Login = () => {
  const [searchParams,] = useSearchParams();
  const translate = useTranslate();
  const code = searchParams.get("code");

  return (<BareLayout>
    <Box
      display="flex"
      flexDirection="column"
      marginTop="3em"
      alignItems="center"
      minHeight="50vh"
    >
      { code ?
        <RegularLogin authData={code} authMethodKind="Eip712" /> :
        <Errors error={ translate("components.oauth_redirect.invalid_redirect_state") } />
      }
    </Box>
  </BareLayout>);
}

export const InstagramLogin = () => {
  const [searchParams,] = useSearchParams();
  const translate = useTranslate();
  const code = searchParams.get("code");

  return (<BareLayout>
    <Box
      display="flex"
      flexDirection="column"
      marginTop="3em"
      alignItems="center"
      minHeight="50vh"
    >
      { code ?
        <RegularLogin authData={code} authMethodKind="Instagram" /> :
        <Errors error={ translate("components.oauth_redirect.invalid_redirect_state") } />
      }
    </Box>
  </BareLayout>);
}

export const XLogin = () => {
  const [searchParams,] = useSearchParams();
  const translate = useTranslate();
  const code = searchParams.get("code");
  const oauthVerifier = localStorage.getItem("oauthVerifier");
  const authData = JSON.stringify({code, oauthVerifier});

  return (<BareLayout>
    <Box
      display="flex"
      flexDirection="column"
      marginTop="3em"
      alignItems="center"
      minHeight="50vh"
    >
      { authData ?
        <RegularLogin authData={authData} authMethodKind="X" /> :
        <Errors error={ translate("components.oauth_redirect.invalid_redirect_state") } />
      }
    </Box>
  </BareLayout>);
}

const RegularLogin = ({ authData, authMethodKind}) => {
  const navigate = useNavigate();
  const translate = useTranslate();
  const [error, setError] = useSafeSetState();

  const onVerify = useCallback(async (recaptchaToken) => {
    try {
      await authProvider.login(authMethodKind, authData, recaptchaToken);
      navigate("/")
    } catch (e) {
      setError(e.message || translate("certos.error.default"))
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  return (!error ?
    <>
      <CircularProgress sx={{mb: 3}}/>
      <Head2>{ translate("components.login.logging_in") } </Head2>
      <GoogleReCaptchaProvider reCaptchaKey={ Settings.recaptchaSiteKey }>
        <GoogleReCaptcha onVerify={onVerify} />
      </GoogleReCaptchaProvider>
    </>
    :
    <Errors error={error} />
  );
}

const Errors = ({error}) => {
  const translate = useTranslate();
  const navigate = useNavigate();

  return <>
      <NoAccounts sx={{ mb: 3, width: "2em", height: "auto" }}/>
      <Head2 sx={{mb: 3}}>{ translate("components.login.error_title") } </Head2>
      <Typography>{ translate("components.login.error_description") }</Typography>
      <Alert severity="info" sx={{my: "2em" }}>{ translate(`components.login.errors.${error}`, { _: error }) }</Alert>

      <Button
        sx={{mb: 2}}
        variant="contained"
        onClick={() => navigate("/login")}
        startIcon={<Replay />}
      >
        { translate("components.login.try_again") }
      </Button>
      <Button
        variant="outlined"
        href="mailto:werify@werify.eu"
        startIcon={<LiveHelp />}
      >
        { translate("components.login.contact_us") }
      </Button>
  </>;
}

