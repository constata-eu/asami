import {
  useTranslate,
  ShowBase,
  SimpleShowLayout,
  NumberField,
  DateField,
} from "react-admin";
import { DeckCard } from "./layout";
import { CardContent, Typography, Button } from "@mui/material";
import { Head2, green, yellow } from "../components/theme";
import { AmountField } from "../components/custom_fields";
import { AttributeTable } from "../components/attribute_table";

export default () => {
  const translate = useTranslate();

  return (
    <DeckCard id="stats-card">
      <CardContent>
        <Head2 sx={{ mb: "0.3em" }}>{translate("explorer.stats.title")}</Head2>
        <ShowBase resource="Stats" id="1">
          <AttributeTable fontSize="1em !important">
            <NumberField source="totalActiveHandles" />
            <NumberField source="totalCollabs" />
            <NumberField source="totalCampaigns" />
            <AmountField source="totalRewardsPaid" />
          </AttributeTable>
        </ShowBase>
        <Button
          sx={{ mt: "1em" }}
          href="/#/Stats/0/Show"
          fullWidth
          variant="outlined"
          color="secondary"
        >
          {translate("explorer.stats.explore_btn")}
        </Button>
      </CardContent>
    </DeckCard>
  );
};
