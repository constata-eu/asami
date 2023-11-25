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

export const OneTimeTokenLogin = () => {
  const [searchParams,] = useSearchParams();
  const [role, setRole] = useStore('user.role', 'advertiser');
  const translate = useTranslate();
  const token = searchParams.get("token");

  return (<BareLayout>
    <Box
      display="flex"
      flexDirection="column"
      marginTop="3em"
      alignItems="center"
      minHeight="50vh"
    >
      { token ?
        <RegularLogin authData={token} authMethodKind="OneTimeToken" /> :
        <Errors error={ translate("components.oauth_redirect.invalid_redirect_state") } />
      }
    </Box>
  </BareLayout>);
}

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
      setError(e.message || "An unexpected error ocurred logging you in")
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  return (!error ?
    <>
      <CircularProgress sx={{mb: 3}}/>
      <Head2>Logging you in, this won't take long.</Head2>
      <GoogleReCaptcha onVerify={onVerify} />
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
      <Head2 sx={{mb: 3}}>An unexpected error ocurred logging you in.</Head2>
      <Typography>We have a description code for the error: </Typography>
      <Alert severity="info" sx={{my: "2em" }}>{ error }</Alert>

      <Button
        sx={{mb: 2}}
        variant="contained"
        onClick={() => navigate("/login")}
        startIcon={<Replay />}
      >
        Try again
      </Button>
      <Button
        variant="outlined"
        href="mailto:hola@constata.eu"
        startIcon={<LiveHelp />}
      >
        Contact us
      </Button>
  </>;
}

