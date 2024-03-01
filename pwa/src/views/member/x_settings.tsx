import { useEffect } from "react";
import { useDataProvider, useAuthenticated, useSafeSetState, useTranslate, ReferenceField, useGetAll, useGetOne, useGetList} from "react-admin";
import { rLogin } from "../../lib/rLogin";
import LoadingButton from '@mui/lab/LoadingButton';
import { LoggedInNavCard, ColumnsContainer, DeckCard } from '../layout';
import { viewPostUrl } from '../../lib/campaign';
import { Alert, Box, Button, Card, CardActions, CardContent, Container, FormControl, FormHelperText, InputLabel, MenuItem, Select, Skeleton, Typography, IconButton } from "@mui/material";
import { Dialog, DialogContent, DialogTitle, DialogActions } from '@mui/material';
import { ethers, parseUnits, formatEther, toQuantity, toBeHex, zeroPadValue, parseEther } from "ethers";
import schnorr from "bip-schnorr";
import { Buffer } from "buffer";
import Login from "./views/login";
import { useContracts } from "../../components/contracts_context";
import { Head1, Head2, BulletPoint, CardTitle } from '../../components/theme';
import LoginIcon from '@mui/icons-material/Login';
import _ from 'lodash';
import { LocalizationProvider } from '@mui/x-date-pickers';
import { DateField } from '@mui/x-date-pickers/DateField';
import { AdapterDayjs } from '@mui/x-date-pickers/AdapterDayjs'
import dayjs from 'dayjs';
import { nip19 } from 'nostr-tools';
import { TwitterTweetEmbed } from 'react-twitter-embed';
import { Settings } from '../../settings';
import AsamiLogo from '../../assets/logo.svg?react';
import XIcon from '@mui/icons-material/X';
import Paper from '@mui/material/Paper';
import { Toolbar, Create, Confirm, SimpleForm, CreateBase, Form, TextInput, RichTextInput, SaveButton, useNotify }
from 'react-admin';
import { ListBase, Title, ListToolbar, Pagination, Datagrid, TextField, FunctionField, RecordContextProvider, SimpleShowLayout} from 'react-admin';
import {  
    useListController,
    defaultExporter,
    ListContextProvider
} from 'react-admin';

import { Stack } from '@mui/material';
import CampaignIcon from '@mui/icons-material/Campaign';
import CloseIcon from '@mui/icons-material/Close';
import { getAuthKeys } from '../../lib/auth_provider';
import ClaimAccountButton from '../claim_account';

import InstagramIcon from '@mui/icons-material/Instagram';

import asamiABI from "../../abi/Asami.json";
import docABI from "../../abi/Doc.json";
import BalanceCard from "./balance_card";
import { HandleSettings, HandleStats, CreateHandleRequest } from "./handle_settings";
import CloudSyncIcon from '@mui/icons-material/CloudSync';

const XSettings = ({handles, handleRequests}) =>
  <HandleSettings
    handles={handles}
    handleRequests={handleRequests}
    icon={<XIcon/>}
    site="X"
    namespace="x"
    handleMinLength={4}
    handleMaxLength={15}
    verificationPost={ <MakeXVerificationPost /> }
  />

const MakeXVerificationPost = () => {
  const translate = useTranslate();
  const [clicked, setClicked] = useSafeSetState(false);

  let accountId = BigInt(getAuthKeys().session.accountId);
  let text = translate("handle_settings.x.make_verification_post.post_text", {accountId });
  let intent_url = `https://x.com/intent/tweet?text=${text}`;

  return <Box>
    <Typography variant="body2" mb="1em">
      { translate("handle_settings.x.make_verification_post.text") }
    </Typography>

    <Paper elevation={5} sx={{ my:"1em", p:"1em"}}> {text} </Paper>
    { clicked ?
      <Alert>{ translate("handle_settings.x.make_verification_post.done") }</Alert> :
      <Button fullWidth 
        onClick={() => setClicked(true)}
        variant="contained"
        href={intent_url}
        target="_blank" 
        rel="noopener noreferrer"
      >
        { translate("handle_settings.x.make_verification_post.button") }
      </Button> 
    }
  </Box>
}

export default XSettings;

