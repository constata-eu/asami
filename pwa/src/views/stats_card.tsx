import { useTranslate, ShowBase, SimpleShowLayout, NumberField, DateField } from "react-admin";
import { DeckCard } from './layout';
import { CardContent, Typography, Button } from "@mui/material";
import { Head2, green, yellow} from '../components/theme';
import { AmountField } from '../components/custom_fields';

export default () => {
  const translate = useTranslate();

  return <DeckCard id="stats-card">
    <CardContent>
      <Head2>{ translate("explorer.stats.title") }</Head2>
      <ShowBase resource="Stats" id="1">
          <SimpleShowLayout>
              <NumberField source="totalActiveHandles" />
              <NumberField source="totalCollabs" />
              <NumberField source="totalCampaigns" />
              <AmountField source="totalRewardsPaid" />
          </SimpleShowLayout>
      </ShowBase>
      <Button href="/#/Stats/0/Show" fullWidth variant="outlined" color="inverted">{ translate("explorer.stats.explore_btn")}</Button>
    </CardContent>
  </DeckCard>;
}
