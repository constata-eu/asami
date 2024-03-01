import { useEffect } from "react";
import { useDataProvider, useAuthenticated, useSafeSetState, useTranslate, ReferenceField, useGetAll, useGetOne, useGetList} from "react-admin";
import LoadingButton from '@mui/lab/LoadingButton';
import { viewPostUrl } from '../../lib/campaign';
import { Alert, Box, Button, Card, CardActions, CardContent, Container, FormControl, FormHelperText, InputLabel, MenuItem, Select, Skeleton, Typography, IconButton } from "@mui/material";
import { Avatar, Badge, Dialog, Divider, DialogContent, DialogTitle, DialogActions, Chip, Stack } from '@mui/material';
import { ethers, parseUnits, formatEther, toQuantity, toBeHex, zeroPadValue, parseEther } from "ethers";
import schnorr from "bip-schnorr";
import { Buffer } from "buffer";
import Login from "./views/login";
import { useContracts } from "../../components/contracts_context";
import { Head1, Head2, BulletPoint, CardTitle, green, light } from '../../components/theme';
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
import { DeckCard } from '../layout';
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

import CampaignIcon from '@mui/icons-material/Campaign';
import CloseIcon from '@mui/icons-material/Close';
import { getAuthKeys } from '../../lib/auth_provider';
import ClaimAccountButton from '../claim_account';

import InstagramIcon from '@mui/icons-material/Instagram';

import asamiABI from "../../abi/Asami.json";
import docABI from "../../abi/Doc.json";

const BalanceCard = () => {
  const {data, isLoading, error, refetch} = useGetOne(
    "Account",
    { id: getAuthKeys().session.accountId },
    { refetchInterval: 10000 }
  );

  let content;

  if (isLoading || !data) {
    content = <></>;
  } else if(data.status == "DONE") {
    content = <Done account={data} />;
  } else {
    content = <Unclaimed account={data} />;
  }
  
  return <DeckCard id="balance-card">
    { content }
  </DeckCard>;
}

const Unclaimed = ({account}) => {
  const translate = useTranslate();
  return <>
    <CardContent>
      <Head2>{ translate("balance_card.title") }</Head2>
      <SimpleShowLayout record={account} sx={{ p: 0, my: 1}}>
        <FunctionField label={ translate("balance_card.unclaimed.doc_label") }
          render={ record => `${formatEther(record.unclaimedDocRewards)} DOC` } />
        <FunctionField label={ translate("balance_card.unclaimed.asami_label") }
          render={ record => `${formatEther(record.unclaimedAsamiTokens)} ASAMI` }  />
        <FunctionField
          label={ translate("balance_card.account_id_label")}
          render={ record => `${BigInt(record.id)}` }
        />
      </SimpleShowLayout>
    </CardContent>
    <Divider />
    { account.status == null ? <NotRequested/> : <ProcessingRequest/> }
  </>;
}

const NotRequested = () => {
  const translate = useTranslate();
  return <>
    <CardContent>
      <Typography mb="1em" id="account-summary-claim-none" variant="body2">
        { translate("balance_card.unclaimed.not_requested") }
      </Typography>
      <ClaimAccountButton id="balance-card-claim-account-button" color="inverted" variant="outlined"
        label={ translate("balance_card.unclaimed.not_requested_button") }/>
    </CardContent>
  </>;
}

const ProcessingRequest = () => {
  const translate = useTranslate();
  return <CardContent>
    <Typography id="account-summary-claim-pending" variant="body2">
      { translate("balance_card.unclaimed.pending") }
    </Typography>
  </CardContent>;
}

const Done = ({account}) => {
  const translate = useTranslate();
  return <>
    <CardContent>
      <Head2>{ translate("balance_card.title") }</Head2>
      <SimpleShowLayout record={account} sx={{ p: 0, my: 1}}>
        <FunctionField label={ translate("balance_card.claimed.doc_label") }
          render={ record => `${formatEther(record.docBalance)} DOC` } />
        <FunctionField label={ translate("balance_card.claimed.asami_label") }
          render={ record => `${formatEther(record.asamiBalance)} ASAMI` }  />
        <FunctionField label={ translate("balance_card.claimed.address") }
          render={ record => `${record.addr.substring(0,8)}…${record.addr.substring(36)}`} />
        <FunctionField label={ translate("balance_card.account_id_label")} render={ record => `${BigInt(record.id)}` }  />
      </SimpleShowLayout>
    </CardContent>
    <Divider />
    <CardContent>
      <Typography id="account-summary-claim-done" variant="body2">
        { translate("balance_card.claimed.participate_in_governance") }
      </Typography>
    </CardContent>
  </>;
}

export default BalanceCard;
