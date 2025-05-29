/// <reference types="vite-plugin-svgr/client" />
import { useEffect, useContext, useState, useRef } from "react";
import {
  Avatar,
  CardContent,
  CardHeader,
  Box,
  Button,
  Typography,
  Stack,
  Card,
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
import { CardTable, DeckCard } from "./layout";
import { Head2, Head3, light, green, Head1, Lead } from "../components/theme";
import { useNavigate } from "react-router-dom";
import CampaignIcon from "@mui/icons-material/Campaign";
import chunk from "lodash/chunk";
import flatten from "lodash/flatten";
import CampaignListEmpty from "./campaign_list_empty";
import StatsCard from "./stats_card";
import { ResponsiveAppBar } from "./responsive_app_bar";
import { contentId } from "../lib/campaign";

export default () => {
  const t = useTranslate();
  const [pubDataProvider, setPubDataProvider] = useState();
  const i18nProvider = useContext(I18nContext);
  const navigate = useNavigate();

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

  const loginAs = async (role) => {
    localStorage.setItem("asami_user_role", role);
    navigate("/login");
  };

  return (
    <>
      <ResponsiveAppBar />
      <CardTable>
        <Stack gap="0.5em" mb="1em">
          <Head1>{t("landing.title")}</Head1>
          <Lead>{t("landing.asami_invites")}</Lead>
          <Lead>{t("landing.boost")}</Lead>
          <Lead>{t("landing.built_on_fairness")}</Lead>
        </Stack>
        <JoinNow key="join-now" loginAs={loginAs} />
        <YourPostHere loginAs={loginAs} />
        {pubDataProvider && (
          <CoreAdminContext
            i18nProvider={i18nProvider}
            dataProvider={pubDataProvider}
          >
            <StatsCard />
          </CoreAdminContext>
        )}
        <AboutToken />
      </CardTable>
      <Box mt="1em">
        <CardTable>
          {pubDataProvider && (
            <CoreAdminContext
              i18nProvider={i18nProvider}
              dataProvider={pubDataProvider}
            >
              <PublicCampaignList loginAs={loginAs} />
            </CoreAdminContext>
          )}
        </CardTable>
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
    chunk(listContext.data, 4).map((i) =>
      i.length == 4 ? [...i, { yourPostHere: true }] : i,
    ),
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
    <DeckCard color="inverted.main" background={(t) => t.palette.primary.main}>
      <CardContent>
        <Lead sx={{ mb: "0.2em", color: "inverted.main" }}>
          {translate("your_post_here.lead")}
        </Lead>
        <Head2 sx={{ color: "inverted.main" }}>
          {translate("your_post_here.title")}
        </Head2>
        <Lead sx={{ my: "1em", color: "inverted.main" }}>
          {translate("your_post_here.message")}
        </Lead>
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

const JoinNow = ({ loginAs }) => {
  const translate = useTranslate();

  return (
    <DeckCard color="primary.main">
      <CardContent>
        <Lead sx={{ color: "primary.main", mb: "0.2em" }}>
          {translate("join_now.lead")}
        </Lead>
        <Head2 sx={{ color: "primary.main" }}>
          {translate("join_now.title")}
        </Head2>
        <Lead sx={{ color: "primary.main", my: "1em" }}>
          {translate("join_now.you_will_be_invited")}
        </Lead>
        <Lead sx={{ color: "primary.main", my: "1em" }}>
          {translate("join_now.join_from_anywhere")}
        </Lead>
        <Button
          fullWidth
          size="large"
          color="primary"
          variant="contained"
          onClick={() => loginAs("member")}
        >
          {translate("join_now.button")}
        </Button>
      </CardContent>
    </DeckCard>
  );
};

/*
Landing ideas:
- Add other stats to club stats:
    * Total reach : 100000 PEOPLE
    * Largest campaign: 
    * User with most rewards: 

    * Highest 
*/

const AboutToken = () => {
  const translate = useTranslate();

  return (
    <DeckCard color="primary.main">
      <CardContent>
        <Lead sx={{ color: "primary.main", mb: "0.2em" }}>
          Invest in Asami’s growth
        </Lead>
        <Head2 sx={{ color: "primary.main" }}>Hold ASAMI Tokens</Head2>
        <Lead sx={{ color: "primary.main", my: "1em" }}>
          Support the project and earn passively—no need to run campaigns or
          collaborate. With a fixed supply of 21 million tokens and no premine,
          ASAMI holders receive a share of club fees every 15 days.
        </Lead>
        <Stack gap="1em">
          <Button
            fullWidth
            size="large"
            color="primary"
            variant="outlined"
            onClick={() => loginAs("member")}
          >
            Learn more
          </Button>
          <Button
            fullWidth
            size="large"
            color="primary"
            variant="contained"
            onClick={() => loginAs("member")}
          >
            Trade ASAMI on OKU
          </Button>
        </Stack>
      </CardContent>
    </DeckCard>
  );
};

const PublicXCampaign = ({ loginAs, item }) => {
  const translate = useTranslate();
  const containerRef = useRef(null);

  useEffect(() => {
    const cleanupMargins = () => {
      const tweetBlockquote =
        containerRef.current?.querySelector(".twitter-tweet");
      if (tweetBlockquote) {
        tweetBlockquote.style.margin = "0";
      }
    };

    const interval = setInterval(cleanupMargins, 200);

    setTimeout(() => clearInterval(interval), 3000);
  }, []);

  return (
    <Card
      id={`campaign-container-${item.id}`}
      sx={{
        borderRadius: "13px 13px 4px 4px",
        mb: "1em",
      }}
    >
      <Box overflow="hidden">
        <div ref={containerRef}>
          <TwitterTweetEmbed
            tweetId={contentId(item)}
            options={{
              align: "center",
              width: "100%",
              conversation: "none",
              margin: 0,
            }}
          />
        </div>
      </Box>

      <Box
        height="50px"
        mt="-50px"
        position="relative"
        sx={{
          backgroundImage:
            "linear-gradient(to bottom, rgba(245, 235, 231, 0) 0%, #f5ebe7 100%)",
        }}
      ></Box>
      <Box p="0.5em 1em 1em 1em">
        <Typography mb="0.5em" variant="h6">
          {translate("public_card_header.title", {
            amount: formatEther(item.budget),
          })}
        </Typography>

        <Button
          sx={{ mt: "0.2em" }}
          onClick={() => loginAs("member")}
          fullWidth
          color="primary"
          variant="outlined"
        >
          {translate("public_x_campaign.main_button")}
        </Button>
      </Box>
    </Card>
  );
};
