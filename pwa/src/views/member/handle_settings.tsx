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

export const HandleSettings = ({handles, handleRequests, site, namespace, handleMinLength, handleMaxLength, icon, verificationPost}) => {
  const translate = useTranslate();
  let content;
  const handle = handles.data?.filter((x) => x.site == site)[0];
  const request = handleRequests.data?.filter((x) => x.site == site)[0];

  if (handles.isLoading || handleRequests.isLoading ){
    content = (<>
      <Skeleton />
      <Skeleton />
    </>);
  } else if (handle) {
    content = <HandleStats handle={handle} id={`existing-${namespace}-handle-stats`} />;
  } else {
    if (request) {
      if (request.status == "UNVERIFIED") {
        content = verificationPost;
      } else if (request.status != "DONE" ) {
        content = <HandleSubmissionInProgress req={request} namespace={namespace} />;
      }
    } else {
      content = <CreateHandleRequest
        onSave={handleRequests.refetch}
        namespace={namespace}
        site={site}
        handleMinLength={handleMinLength}
        handleMaxLength={handleMaxLength}
      />;
    }
  }

  return (<Box>
    <DeckCard id={`configure-${namespace}-handle-card`}>
      <CardContent>
        <Stack direction="row" gap="1em" mb="1em">
          { icon }
          <Head2>{ translate("handle_settings.title") }</Head2>
        </Stack>
        { content }
      </CardContent>
    </DeckCard>
  </Box>);
}

export const HandleStats = ({handle, id}) => {
  const translate = useTranslate();
  const cycleDuration = 60 * 60 * 24 * 15 * 1000;
  const thisEpoch = Math.trunc(Date.now() / cycleDuration);
  const nextCycle = new Date((thisEpoch + 1) * cycleDuration);

  return <Box id={id}>
    <SimpleShowLayout record={handle} sx={{ p: 0, mt: 1}}>
      <TextField label={ translate("handle_settings.stats.username")} source="username" />
      <FunctionField label={ translate("handle_settings.stats.sparkles") } render={ record => `${BigInt(handle.score)} ✨` }  />
      <FunctionField label={ translate("handle_settings.stats.price_per_repost") }
        render={ record => `${formatEther(record.price)} DOC` } />
      <FunctionField label={ translate("handle_settings.stats.sparkle_rate") }
        render={ record => formatEther(BigInt(handle.price)/BigInt(handle.score)) } />
      <FunctionField label={ translate("handle_settings.stats.locked_until") }
        render={ record => nextCycle.toLocaleDateString(undefined, { month: 'long', day: 'numeric'}) } />
      <TextField label={ translate("handle_settings.stats.user_id")} source="userId" />
    </SimpleShowLayout>
  </Box>;
}

export const CreateHandleRequest = ({onSave, namespace, site, handleMinLength, handleMaxLength }) => {
  const translate = useTranslate();
  const notify = useNotify();

  const transformIt = async (values) => {
    return { input: values.handleRequestInput };
  }

  const onSuccess = () => {
    notify(`handle_settings.${namespace}.create_request.success`);
    onSave();
  }

  const validate = (values) => {
    let errors = {};
    let input = { site: site};

    if ( values.username.match(new RegExp(`^@?(\\w){${handleMinLength},${handleMaxLength}}$`) )) {
      input.username = values.username.replace("@","");
    } else {
      errors.username = translate(`handle_settings.${namespace}.create_request.username_error`);
    }

    values.handleRequestInput = input;
    return errors;
  }

  return <CreateBase resource="HandleRequest" transform={transformIt} mutationOptions={{ onSuccess }} >
    <SimpleForm id={`${namespace}-handle-request-form`} sx={{ p: "0 !important", m: "0" }} sanitizeEmptyValues validate={validate} toolbar={false}>
      <Typography mb="1em" variant="body2">
        { translate(`handle_settings.${namespace}.create_request.text`) }
      </Typography>
      <TextInput sx={{mb: "1em" }} fullWidth required={true} size="large" variant="filled" source="username"
        label={ translate(`handle_settings.${namespace}.create_request.username_label`) }
        helperText={ translate(`handle_settings.${namespace}.create_request.username_help`) }
      />
      <SaveButton
        fullWidth
        id={`submit-${namespace}-handle-request-form`}
        label={ translate(`handle_settings.${namespace}.create_request.save`) }
        icon={<></>}
      />
    </SimpleForm>
  </CreateBase>;
}

const HandleSubmissionInProgress = ({req, namespace}) => {
  const translate = useTranslate();
  
  return <Box id={`handle-${namespace}-submission-in-progress-message`}>
    <Typography variant="body2">
      { translate(`handle_settings.${namespace}.in_progress.text`, {username: req.username}) }
    </Typography>
  </Box>;
}
