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

const IgSettings = ({handles, handleRequests}) =>
  <HandleSettings
    handles={handles}
    handleRequests={handleRequests}
    icon={<InstagramIcon/>}
    site="INSTAGRAM"
    namespace="ig"
    handleMinLength={1}
    handleMaxLength={30}
    verificationPost={ <MakeIgVerificationPost /> }
  />

const MakeIgVerificationPost = () => {
  const translate = useTranslate();
  const notify = useNotify();
  const [config, setConfig] = useSafeSetState(null);
  let accountId = BigInt(getAuthKeys().session.accountId);

  useEffect(() => {
    const load = async () => {
      setConfig((await (await fetch(`${Settings.apiDomain}/config`)).json()));
    }
    load();
  }, []);

  if (!config) {
    return <>
      <Skeleton />
      <Skeleton />
    </>;
  }

  const caption = `${config.instagram_verification_caption} [${accountId}]`;

  const copyText = async () => {
    notify("handle_settings.ig.make_verification_post.caption_copied");
    await navigator.clipboard.writeText(caption);
  }

  return <Box>
    <Typography mb="0.5em" variant="body2">
      { translate("handle_settings.ig.make_verification_post.verify_username") }
      <strong> { translate("handle_settings.ig.make_verification_post.guidelines") } </strong>
    </Typography>
    <Box display="flex" flexDirection="column" gap="1em">
      <Box display="flex" borderRadius="10px" overflow="hidden">
        <img width="100%" src={config.instagram_verification_image_url} />
      </Box>
      <Paper elevation={5} sx={{p:"1em"}}><Typography>{ caption }</Typography></Paper>
      <Box display="flex" gap="1em">
        <Button fullWidth variant="contained" target="_blank" href={config.instagram_verification_image_url} rel="noopener noreferrer" download>
          { translate("handle_settings.ig.make_verification_post.get_image") }
        </Button> 
        <Button fullWidth onClick={() => copyText() } variant="contained" >
          { translate("handle_settings.ig.make_verification_post.copy_text") }
        </Button> 
      </Box>
    </Box>
  </Box>
}

export default IgSettings;

