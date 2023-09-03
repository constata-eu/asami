import { useEffect } from 'react';
import { Box, Button, Typography } from '@mui/material';
import { useCheckAuth, useSafeSetState } from 'react-admin';
import { useNavigate } from 'react-router-dom';
import authProvider from '../auth_provider';
import { BareLayout } from './layout';
import { Head1 } from '../components/theme';

const Login = () => {
  const checkAuth = useCheckAuth();
  const navigate = useNavigate();
  const [error, setError] = useSafeSetState();

  useEffect(() => {
    const check = async () => {
      try {
        await checkAuth();
        navigate("/")
      } catch (e) {
      }
    }
    check();
  }, [checkAuth, navigate]);

  const login = async () => {
    try {
      await authProvider.login();
      navigate("/")
    } catch (e) {
      setError(e.message)
    }
  }

  return (<BareLayout>
    <Head1 variant="h1" sx={{my: 3}}>Welcome to Asami</Head1>
    <Typography my={3}>
      You can login using Metamask or compatible wallets.
    </Typography>
    <Box>
      <Button
        onClick={login}
        sx={{my: 3}}
        variant="contained"
        size="large"
        fullWidth
      >
        Connect as advertiser
      </Button>
      <Button
        onClick={login}
        sx={{my: 3}}
        variant="contained"
        size="large"
        fullWidth
      >
        Connect as creator
      </Button>
    </Box>
  </BareLayout>);
};

export default Login;
