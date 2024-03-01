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
import XSettings from "./x_settings";
import IgSettings from "./ig_settings";
import { useContracts } from "../../components/contracts_context";
import { Head1, Head2, BulletPoint, CardTitle, yellow, red, green, dark } from '../../components/theme';
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
import FacebookIcon from '@mui/icons-material/Facebook';
import CampaignListEmpty from '../campaign_list_empty';

import Table from '@mui/material/Table';
import TableBody from '@mui/material/TableBody';
import TableCell from '@mui/material/TableCell';
import TableContainer from '@mui/material/TableContainer';
import TableHead from '@mui/material/TableHead';
import TableRow from '@mui/material/TableRow';
import Paper from '@mui/material/Paper';
import { Toolbar, Create, Confirm, SimpleForm, CreateBase, Form, TextInput, RichTextInput, SaveButton, useNotify } from 'react-admin';
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

const HelpCard = ({handles, campaigns}) => {
  if (handles.isLoading || campaigns.isLoading ){
    return <></>;
  }

  const translate = useTranslate();
  let text;
  let id;
  if (handles.data.length == 0) {
    text = "help_card.verify_handles";
    id = "help-card-verify-a-handle";
  } else if (campaigns.data.length == 0 ) {
    text = "help_card.wait_for_campaigns";
    id = "help-card-no-campaigns";
  } else {
    text = "help_card.collaborate";
    id = "help-card-go-collab";
  }

  return <DeckCard id="help-card" borderColor={green}>
    <CardContent>
      <Head2 sx={{ color: green}} >{ translate("help_card.title") }</Head2>
      <Typography id={id} mt="1em">{translate(text)}</Typography>
    </CardContent>
  </DeckCard>;
}

export default HelpCard;

