import { CardContent, Typography } from '@mui/material';
import { useTranslate } from 'react-admin';
import { DeckCard } from './layout';
import { Head2 } from '../components/theme';

const CampaignListEmpty = () => {
  const translate = useTranslate();
  return <DeckCard id="campaign-list-empty">
    <CardContent>
      <Head2>{ translate("out_of_campaigns.title") }</Head2>
      <Typography>{ translate("out_of_campaigns.message") }</Typography>
    </CardContent>
  </DeckCard>;
}
export default CampaignListEmpty;
