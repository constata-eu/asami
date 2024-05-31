import { useTranslate } from "react-admin";
import { DeckCard } from './layout';
import { CardContent, Typography } from "@mui/material";
import { Head2, green, yellow} from '../components/theme';

const VersionTwoCard = () => {
  const translate = useTranslate();

  return <DeckCard id="version-two-card" borderColor={yellow}>
    <CardContent>
      <Head2 sx={{ color: yellow }} >{ translate("version_two.title") }</Head2>
      <Typography mt="1em">{translate("version_two.text")}</Typography>
    </CardContent>
  </DeckCard>;
}

export default VersionTwoCard;

