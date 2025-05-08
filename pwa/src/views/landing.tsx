/// <reference types="vite-plugin-svgr/client" />
import { useEffect, useContext, useState } from "react";
import {
  Avatar,
  CardContent,
  CardHeader,
  Box,
  Button,
  Typography,
} from "@mui/material";

import { etherToHex } from "../lib/formatters";
import { formatEther } from "ethers";
import { publicDataProvider } from "../lib/data_provider";
import {
  useListController,
  CoreAdminContext,
  useTranslate,
  I18nContext,
} from "react-admin";

import { TwitterTweetEmbed } from "react-twitter-embed";
import { DeckCard } from "./layout";
import { Head2, Head3, light, green } from "../components/theme";
import XIcon from "@mui/icons-material/X";
import CampaignIcon from "@mui/icons-material/Campaign";
import chunk from "lodash/chunk";
import flatten from "lodash/flatten";
import CampaignListEmpty from "./campaign_list_empty";
import StatsCard from "./stats_card";
import { ResponsiveAppBar } from "./responsive_app_bar";

/*

  const [, setRole] = useStore("user.role", "advertiser");

  const loginAs = async (newRole) => {
    setRole(newRole);
    setOpen(true);
  };

  */

export default () => {
  const [pubDataProvider, setPubDataProvider] = useState();
  const i18nProvider = useContext(I18nContext);

  useEffect(() => {
    async function initApp() {
      const prov = await publicDataProvider();
      setPubDataProvider(prov);
    }
    initApp();
  }, []);

  if (!pubDataProvider) {
    return <></>;
  }

  return (
    <>
      <ResponsiveAppBar />
      <Box
        sx={{
          columnCount: { xs: 1, sm: 2, md: 3, lg: 4, xl: 5 },
          columnGap: "1em",
        }}
      >
        <JoinNow key="join-now" />
        {pubDataProvider && (
          <CoreAdminContext
            i18nProvider={i18nProvider}
            dataProvider={pubDataProvider}
          >
            <PublicCampaignList />
            <StatsCard />
          </CoreAdminContext>
        )}
      </Box>
    </>
  );
};

const PublicCampaignList = ({ loginAs }) => {
  const listContext = useListController({
    debounce: 500,
    disableSyncWithLocation: true,
    filter: { budgetGt: etherToHex("1") },
    sort: { field: "createdAt", order: "DESC" },
    perPage: 20,
    queryOptions: {
      refetchInterval: 100000,
    },
    resource: "Campaign",
  });

  if (listContext.isLoading) {
    return <></>;
  }

  if (listContext.total == 0) {
    return <CampaignListEmpty />;
  }

  const items = flatten(
    chunk(listContext.data, 4).map((i) => [...i, { yourPostHere: true }]),
  );

  return (
    <>
      {items.map((item, index) => {
        if (item.yourPostHere) {
          return (
            <YourPostHere key={`your-post-here-${index}`} loginAs={loginAs} />
          );
        } else {
          return (
            <PublicXCampaign key={item.id} item={item} loginAs={loginAs} />
          );
        }
      })}
    </>
  );
};
const YourPostHere = ({ loginAs }) => {
  const translate = useTranslate();

  return (
    <DeckCard elevation={10}>
      <CardContent>
        <Head2>{translate("your_post_here.title")}</Head2>
        <Typography>{translate("your_post_here.message")}</Typography>
        <Box mt="1em">
          <Button
            onClick={() => loginAs("advertiser")}
            className="submit-your-post"
            color="inverted"
            fullWidth
            size="large"
            variant="outlined"
          >
            <CampaignIcon sx={{ mr: "5px" }} />
            {translate("your_post_here.button")}
          </Button>
        </Box>
      </CardContent>
    </DeckCard>
  );
};
const JoinNow = () => {
  const translate = useTranslate();

  return (
    <DeckCard borderColor={green} elevation={10}>
      <CardContent>
        <Head2 sx={{ mb: "0.5em" }}>{translate("join_now.title")}</Head2>
        <Typography>{translate("join_now.message")}</Typography>
        <Head3 sx={{ mt: "1em", mb: "0.5em" }}>
          {translate("join_now.no_crypto_no_problem")}
        </Head3>
        <Typography>{translate("join_now.learn_later")}</Typography>
        <Box mt="1em">
          <Button
            className="get-your-sparkles"
            fullWidth
            size="large"
            variant="contained"
          >
            {translate("join_now.button")}
          </Button>
        </Box>
      </CardContent>
    </DeckCard>
  );
};

const PublicCardHeader = ({ loginAs, item, buttonLabel, icon }) => {
  const translate = useTranslate();

  return (
    <CardHeader
      sx={{ mb: "0", pb: "0.5em" }}
      avatar={<Avatar sx={{ bgcolor: light }}>{icon}</Avatar>}
      title={translate("public_card_header.title", {
        amount: formatEther(item.budget),
      })}
      subheader={
        <Button
          sx={{ mt: "0.2em" }}
          onClick={() => loginAs("member")}
          fullWidth
          color="inverted"
          size="small"
          variant="outlined"
        >
          {buttonLabel}
        </Button>
      }
    />
  );
};

const PublicXCampaign = ({ loginAs, item }) => {
  const translate = useTranslate();
  return (
    <DeckCard id={`campaign-container-${item.id}`}>
      <PublicCardHeader
        icon={<XIcon />}
        item={item}
        loginAs={loginAs}
        buttonLabel={translate("public_x_campaign.main_button")}
      />
      <CardContent>
        <TwitterTweetEmbed
          tweetId={JSON.parse(item.briefingJson)}
          options={{
            theme: "dark",
            align: "center",
            width: "250px",
            conversation: "none",
          }}
        />
      </CardContent>
    </DeckCard>
  );
};
