import { useEffect } from "react";
import { useDataProvider, useAuthenticated, useSafeSetState, useTranslate, useGetList} from "react-admin";
import LoadingButton from '@mui/lab/LoadingButton';
import { Alert, Box, Button, Card, CardActions, CardContent, Container, FormControl, FormHelperText, InputLabel, MenuItem, Select, Skeleton, Typography, IconButton } from "@mui/material";
import { Dialog, DialogContent, DialogTitle, DialogActions } from '@mui/material';
import { ethers, parseUnits, formatEther, toBeHex, zeroPadValue, parseEther } from "ethers";
import { LoggedInNavCard, ColumnsContainer, DeckCard } from '../layout';
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
import { viewPostUrl } from '../../lib/campaign';
import Table from '@mui/material/Table';
import TableBody from '@mui/material/TableBody';
import TableCell from '@mui/material/TableCell';
import TableContainer from '@mui/material/TableContainer';
import TableHead from '@mui/material/TableHead';
import TableRow from '@mui/material/TableRow';
import Paper from '@mui/material/Paper';
import { Toolbar, Create, SimpleForm, CreateBase, Form, TextInput, RichTextInput, SaveButton, useNotify } from 'react-admin';
import { ListBase, Title, ListToolbar, Pagination, Datagrid, TextField, FunctionField} from 'react-admin';
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
import { parseCampaignSiteAndContentId, defaultValidUntil } from '../../lib/campaign';

export const CampaignRequestCard = () => {
  const translate = useTranslate();
  const notify = useNotify();
  const [open, setOpen] = useSafeSetState(false);
  const handleClose = () => setOpen(false);
  const dataProvider = useDataProvider();

  const onSubmit = async (values) => {
    try {
      let result = await dataProvider.create("CampaignRequest", { data: { input: values.campaignRequestInput } });
      notify("campaign_request.success", { anchorOrigin: { vertical: 'top', horizontal: 'center' }});
    } catch (e) {
      notify(e.body.message, { anchorOrigin: { vertical: 'top', horizontal: 'center' }, type: "error"});
    }
    handleClose();
  }

  const validate = (values) => {
    const {error, site, contentId} = parseCampaignSiteAndContentId(values.contentUrl);

    if (error) {
      return { contentUrl: translate(`campaign_request.errors.${error}`) };
    }

    values.campaignRequestInput = {
      contentId,
      site,
      budget: zeroPadValue(toBeHex(parseEther("10")), 32),
      priceScoreRatio: zeroPadValue(toBeHex(parseEther("0.001")), 32),
      validUntil: defaultValidUntil().toISOString(),
      topicIds: []
    };

    return errors;
  }

  return <DeckCard>
    <CardContent>
      <Head2>{ translate("campaign_request.title") }</Head2>
      <Typography my="1em" >{ translate("campaign_request.text") }</Typography>

      <Button fullWidth variant="contained" size="large" id="open-start-campaign-request-dialog" onClick={ () => setOpen(true) }>
        <CampaignIcon sx={{mr:"5px"}}/>
        { translate("campaign_request.send_suggestion") }
      </Button>
      <Dialog id="start-campaign-dialog" open={open} onClose={handleClose} maxWidth="md" fullWidth>
        <Box p="1em">
          <Form sanitizeEmptyValues validate={validate} onSubmit={onSubmit}>
            <TextInput fullWidth required={true} size="large" variant="filled" source="contentUrl"
              label={ translate("campaign_request.post_url_label") }
            />
            <Stack gap="1em">
              <SaveButton fullWidth id="submit-start-campaign-form" size="large" label={ translate("campaign_request.send_suggestion") } icon={<CampaignIcon/>} />
              <Button fullWidth variant="text" color="inverted" onClick={handleClose}>
                { translate("campaign_request.cancel") }
              </Button>
            </Stack>
          </Form>
        </Box>
      </Dialog>
    </CardContent>
  </DeckCard>;
}
