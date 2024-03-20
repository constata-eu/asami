import { useTranslate } from "react-admin";
import { DeckCard } from '../layout';
import { CardContent, Typography } from "@mui/material";
import { Head2, green} from '../../components/theme';

const HelpCard = ({handles, campaigns}) => {
  const translate = useTranslate();

  if (handles.isLoading || campaigns.isLoading ){
    return <></>;
  }

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

