import { useDataProvider, useSafeSetState, useTranslate } from "react-admin";
import { Box, Button, CardContent, Typography} from "@mui/material";
import { Dialog } from '@mui/material';
import { toBeHex, zeroPadValue, parseEther } from "ethers";
import { DeckCard } from '../layout';
import { Head2 } from '../../components/theme';
import { Form, TextInput, SaveButton, useNotify } from 'react-admin';
import { Stack } from '@mui/material';
import CampaignIcon from '@mui/icons-material/Campaign';
import { parseCampaignSiteAndContentId, defaultValidUntil } from '../../lib/campaign';

export const CampaignRequestCard = () => {
  const translate = useTranslate();
  const notify = useNotify();
  const [open, setOpen] = useSafeSetState(false);
  const handleClose = () => setOpen(false);
  const dataProvider = useDataProvider();

  const onSubmit = async (values) => {
    try {
      await dataProvider.create("CampaignRequest", { data: { input: values.campaignRequestInput } });
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
