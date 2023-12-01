import { useEffect } from 'react';
import {
  Alert, AlertTitle, AppBar, Backdrop, Badge, Divider,
  Dialog, DialogActions, DialogContent, DialogTitle,
  IconButton, Box, Button, Container, Paper, styled,
  Toolbar, Typography, Skeleton, useMediaQuery
} from '@mui/material';
import { useCheckAuth, useSafeSetState, useStore, useDataProvider, useNotify } from 'react-admin';
import { useNavigate } from 'react-router-dom';
import authProvider, { makeXUrl, makeInstagramUrl} from '../lib/auth_provider';
import { BareLayout } from './layout';
import { Head1 } from '../components/theme';
import { useContracts } from "../components/contracts_context";
import logo from '../assets/asami.png';
import { getAuthKeys } from '../lib/auth_provider';

const ClaimAccountButton = ({id}) => {
  const notify = useNotify();
  const dataProvider = useDataProvider();
  const { signLoginMessage } = useContracts();

  const createClaimRequest = async () => {
    try {
      const signature = await signLoginMessage();
      let result = await dataProvider.create("ClaimAccountRequest", { data: { input: { signature }}});
      notify("The admin will hand over control of your account to you soon.", { type: "success" })
    } catch(e) {
      notify("Ooops, we could not receive your request. Please try again or contact the admin.", { type: "error"})
    }
  }

  return (<Button sx={{ mt: "1em"}} fullWidth id={id} variant="contained" color="inherit" onClick={createClaimRequest}>Claim your account</Button>);
};

export default ClaimAccountButton;

