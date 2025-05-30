import { useTranslate } from "react-admin";
import { DeckCard } from "../layout";
import { Box, CardContent, Typography } from "@mui/material";
import { Head1, Head2, Lead } from "../../components/theme";

const TitleCard = ({ handles, campaigns }) => {
  const translate = useTranslate();

  if (handles.isLoading || campaigns.isLoading) {
    return <></>;
  }

  let text;
  let id;
  if (handles?.data?.length == 0) {
    text = "title_card.verify_handles";
    id = "title-card-verify-a-handle";
  } else if (campaigns?.data?.length == 0) {
    text = "title_card.wait_for_campaigns";
    id = "title-card-no-campaigns";
  } else {
    text = "title_card.collaborate";
    id = "title-card-go-collab";
  }

  return (
    <Box mb="1em" p="0.5em">
      <Head1 sx={{ mb: "0.5em", color: "primary.main" }}>
        {translate("title_card.title")}
      </Head1>
      <Lead id={id}>{translate(text)}</Lead>
    </Box>
  );
};

export default TitleCard;
