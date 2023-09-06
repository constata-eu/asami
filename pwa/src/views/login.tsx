import { useEffect } from 'react';
import { Alert, AlertTitle, AppBar, Divider, Toolbar, IconButton, Box, Button, Container, Paper, styled, Backdrop, Typography, Skeleton, useMediaQuery } from '@mui/material';
import { useCheckAuth, useSafeSetState, useStore } from 'react-admin';
import { useNavigate } from 'react-router-dom';
import authProvider from '../auth_provider';
import { BareLayout } from './layout';
import { Head1 } from '../components/theme';
import logo from '../assets/asami.png';
import rootstock from '../assets/rootstock.png';

const Login = () => {
  const [role, setRole] = useStore('user.role', 'advertiser');

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

  const loginAs = async (role) => {
    try {
      setRole(role);
      await authProvider.login();
      navigate("/")
    } catch (e) {
      setError(e.message)
    }
  }

  return (<BareLayout>

    <Alert severity="error" sx={{my: "1em" }}>
      <AlertTitle>This is a tech preview</AlertTitle>
      Asami is not ready for production yet, but feel free to look around while we build in public.
      <br/>
      We have a github repo at <a href="https://github.com/constata-eu/asami">constata-eu/asami</a>
    </Alert>

    <Alert severity="info" sx={{my: "1em" }}>
      🎉 We've got 2<sup>nd</sup> place on the <a href="https://rootstock.hackerearth.com/de/">Rootstock Bitcoin Scaling Hackathon</a>
    </Alert>

    <Box alignItems="center" marginTop="2em">
      <Typography variant="h1" fontSize="6em" margin="0" lineHeight="0.5em" fontFamily="Sacramento" fontWeight="bold">
        asami
      </Typography>
      <Typography variant="p" fontSize="0.8em">
        朝 (asa), which means morning; 未 (mi), which can mean "not yet", future.
      </Typography>
    </Box>
    
    <Box display="flex" flexWrap="wrap" justifyContent="space-around"  alignItems="center" gap="1em" my="5em">
      <Typography fontSize="min(4em, 1em + 8vw)" flexBasis="300px" flexGrow="2" lineHeight="1em" fontFamily="LeagueSpartanBlack">
        Authentic Social Ads Marketplace Infrastructure.
      </Typography>
      <Box textAlign="center">
        <img src={logo}  style={{width: "min(200px, 100px + 15vw)" }} />
      </Box>
    </Box>
    <Typography my="3em" fontSize="1.2em" lineHeight="1.2em" fontFamily="LeagueSpartanLight">
      Connect your brand with vetted micro-influencers to achieve unparalleled ROI.
      <br/>
      <br/>
      Our platform empowers content creators with a transparent pathway to monetization, while offering audiences a voice through incentivized feedback.
      <br/>
      <br/>
      Experience the new paradigm in advertising—designed for credibility, cost-effectiveness, and mutual trust.
    </Typography>
    <Box display="flex" flexWrap="wrap" gap="2em" mb="3em">
      <Box flexGrow="1" flexBasis="400px" display="flex" flexDirection="column">
        <Typography fontSize="2em" fontFamily="LeagueSpartanBlack" letterSpacing="-0.03em">Are you an advertiser?</Typography>
        <Typography mb="2em">
          Kickstart your next ad campaign!
          Partner with Asami's micro-influencers for authentic reach and ROI.
          Connect with your RSK account in MetaMask to get started today.
        </Typography>
        <Button
          onClick={() => loginAs("advertiser")}
          sx={{margin: "auto 0 0 0"}}
          variant="contained"
          size="large"
          fullWidth
        >
          Connect as advertiser
        </Button>
      </Box>
      <Box flexGrow="1" flexBasis="400px" display="flex" flexDirection="column">
        <Typography fontSize="2em" fontFamily="LeagueSpartanBlack" letterSpacing="-0.03em">Are you a trusted voice?</Typography>
        <Typography mb="2em">
          Unlock a new revenue stream without compromising your authenticity by joining ASAMI's curated network.
          Connect with your MetaMask RSK wallet to get started on your next brand collaboration.
        </Typography>
        <Button
          onClick={() => loginAs("creator")}
          sx={{margin: "auto 0 0 0"}}
          variant="contained"
          size="large"
          fullWidth
        >
          Connect as collaborator
        </Button>
      </Box>
    </Box>
    <Divider light sx={{ mt: "5em", mb: "2em" }}/>

    <Button href="https://rootstock.io/" target="_blank" sx={{textDecoration: "none", mb: "2em"}}>
      <Box display="flex" flexDirection="column">
        <Typography fontSize="1em" textTransform="uppercase" fontFamily="LeagueSpartanBold">
          Built with
        </Typography>
        <img src={rootstock}  style={{width: "150px" }} />
      </Box>
    </Button>
  </BareLayout>);
};

export default Login;
