import { useEffect } from "react";
import { useDataProvider, useAuthenticated, useSafeSetState, useTranslate, ReferenceField, useGetAll, useGetOne, useGetList} from "react-admin";
import { rLogin } from "../../lib/rLogin";
import LoadingButton from '@mui/lab/LoadingButton';
import { LoggedInNavCard, ColumnsContainer } from '../layout';
import { viewPostUrl } from '../../lib/campaign';
import { Alert, Box, Button, Card, CardActions, CardContent, Container, FormControl, FormHelperText, InputLabel, MenuItem, Select, Skeleton, Typography, IconButton } from "@mui/material";
import { Dialog, DialogContent, DialogTitle, DialogActions } from '@mui/material';
import { ethers, parseUnits, formatEther, toQuantity, toBeHex, zeroPadValue, parseEther } from "ethers";
import schnorr from "bip-schnorr";
import { Buffer } from "buffer";
import XSettings from "./x_settings";
import IgSettings from "./ig_settings";
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
import ClaimAccountButton from '../claim_account';

import InstagramIcon from '@mui/icons-material/Instagram';

import asamiABI from "../../abi/Asami.json";
import docABI from "../../abi/Doc.json";
import BalanceCard from "./balance_card";
import HelpCard from "./help_card";
import { XCampaign, IgCampaign} from "./campaigns";

const Dashboard = () => {
  useAuthenticated();
  const translate = useTranslate();

  const campaigns = useListController({
    disableSyncWithLocation: true,
    filter: {availableToAccountId: getAuthKeys().session.accountId },
    perPage: 20,
    queryOptions: {
      refetchInterval: 10000,
    },
    resource: "Campaign",
  });

  const handles = useListController({
    disableSyncWithLocation: true,
    filter: {accountIdEq: getAuthKeys().session.accountId},
    queryOptions: {
      refetchInterval: 20000,
    },
    resource: "Handle",
  });

  const handleRequests = useListController({
    disableSyncWithLocation: true,
    queryOptions: {
      refetchInterval: 20000,
    },
    resource: "HandleRequest",
  });

  return (<Box p="1em" id="member-dashboard">
    <ColumnsContainer>
      <LoggedInNavCard />
      <HelpCard handles={handles} campaigns={campaigns} />
      <BalanceCard />
      <CampaignList handles={handles}/>
      <XSettings handles={handles} handleRequests={handleRequests} />
      <IgSettings handles={handles} handleRequests={handleRequests} />
    </ColumnsContainer>
    <CollabList />
  </Box>);
}

const CampaignList = ({handles}) => {
  const dataProvider = useDataProvider();
  const listContext = useListController({
    disableSyncWithLocation: true,
    filter: {availableToAccountId: getAuthKeys().session.accountId },
    perPage: 20,
    queryOptions: {
      refetchInterval: 6000,
    },
    resource: "Campaign",
  });

  const prefsContext = useListController({
    disableSyncWithLocation: true,
    perPage: 200,
    resource: "CampaignPreference",
  });

  if (prefsContext.isLoading || listContext.isLoading || handles.isLoading || handles.total == 0 || listContext.total == 0 ){
    return <></>;
  }

  const setPreference = async (campaignId, notInterested, attempted) => {
    await dataProvider.create('CampaignPreference', { data: { input: {campaignId, notInterested, attempted} } });
    await listContext.refetch();
    if(attempted) {
      await prefsContext.refetch();
    }
  };

  return ( <>
    <Box id="campaign-list" display="flex" flexWrap="wrap">
      { listContext.data.map((item) => item.site == "X" ?
        <XCampaign key={item.id} campaign={item} prefsContext={prefsContext} setPreference={setPreference} /> :
        <IgCampaign key={item.id} campaign={item} prefsContext={prefsContext} setPreference={setPreference} />
      )}
    </Box>
    </>
  );
}

const CollabList = () => {
  const listContext = useListController({
    disableSyncWithLocation: true,
    filter: {memberIdEq: getAuthKeys().session.accountId },
    perPage: 20,
    queryOptions: {
      refetchInterval: 3000,
    },
    resource: "Collab",
  });

  if (listContext.total == 0 || listContext.isLoading) {
    return <></>;
  }

  return (
    <ListContextProvider value={listContext}>
      <Card id="collab-list" sx={{my:"3em"}}>
        <CardTitle text="Your collaboration history" >
          <Typography>These are your collaborations so far.</Typography>
        </CardTitle>
        <Datagrid bulkActionButtons={false}>
          <ReferenceField source="campaignId" reference="Campaign">
            <FunctionField label={false} render={record => <a target="_blank" href={viewPostUrl(record)} rel="noreferrer">See post</a>} />
          </ReferenceField>
          <FunctionField label="Reward" render={record => `${formatEther(record.gross)} DOC`} />
          <FunctionField label="Asami Fee" render={record => `${formatEther(record.fee)} DOC`} />
        </Datagrid>
        <Pagination rowsPerPageOptions={[]} />
      </Card>
    </ListContextProvider>
  );
}

export default Dashboard;

