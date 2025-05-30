/// <reference types="vite-plugin-svgr/client" />
import { useEffect, useContext, useState, useRef } from "react";
import {
  CardContent,
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
import CampaignListEmpty from "./campaign_list_empty";
import StatsCard from "./stats_card";
import { ResponsiveAppBar } from "./responsive_app_bar";
import { contentId } from "../lib/campaign";

export default () => {
  const t = useTranslate();
  const navigate = useNavigate();
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

  const loginAs = async (role) => {
    localStorage.setItem("asami_user_role", role);
    navigate("/login");
  };

  return (
    <>
      <ResponsiveAppBar />
      <Stack
        my={{ sm: "2em", md: "3em" }}
        gap="2em"
        direction="row"
        flexWrap="wrap"
        alignItems="stretch"
      >
        <Stack flex="1 1 270px" gap="0.5em">
          <Head1 sx={{ color: "secondary.main" }}>{t("landing.title")}</Head1>
          <Lead>{t("landing.asami_invites")}</Lead>
          <Lead>{t("landing.boost")}</Lead>
          <Lead>{t("landing.together")}</Lead>
        </Stack>
        <Box flex="1 1 300px" display="flex">
          <YourPostHere loginAs={loginAs} />
        </Box>
        <Box flex="1 1 300px" display="flex">
          <JoinNow key="join-now" loginAs={loginAs} />
        </Box>
      </Stack>

      <Box my="3em">
        {pubDataProvider && (
          <CoreAdminContext
            i18nProvider={i18nProvider}
            dataProvider={pubDataProvider}
          >
            <PublicCampaignList loginAs={loginAs} />
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

  useEffect(() => {
    async function initApp() {
      const prov = await publicDataProvider();
      setPubDataProvider(prov);
    }
    initApp();
  }, []);

  if (listContext.isLoading) {
    return <></>;
  }

  if (listContext.total == 0) {
    return <CampaignListEmpty />;
  }

  return (
    <CardTable>
      {listContext.data.map((item, index) => (
        <>
          <PublicXCampaign key={item.id} item={item} loginAs={loginAs} />
          {index == 0 && (
            <>
              <AboutToken />
              <StatsCard />
            </>
          )}
        </>
      ))}
    </CardTable>
  );
};
const YourPostHere = ({ loginAs }) => {
  const translate = useTranslate();

  return (
    <Card
      sx={{
        color: "inverted.main",
        background: (theme) => theme.palette.primary.main,
      }}
    >
      <CardContent
        sx={{
          height: "100%",
          display: "flex",
          flexDirection: "column",
          justifyContent: "space-between",
        }}
      >
        <Box>
          <Lead sx={{ mb: "0.2em", color: "inverted.main" }}>
            {translate("your_post_here.lead")}
          </Lead>
          <Head2 sx={{ color: "inverted.main" }}>
            {translate("your_post_here.title")}
          </Head2>
        </Box>
        <Lead sx={{ my: "0.5em", color: "inverted.main" }}>
          {translate("your_post_here.one")}
        </Lead>
        <Lead sx={{ mt: "0.5em", mb: "1em", color: "inverted.main" }}>
          {translate("your_post_here.two")}
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
    </Card>
  );
};

const JoinNow = ({ loginAs }) => {
  const translate = useTranslate();

  return (
    <Card sx={{ color: "primary.main" }}>
      <CardContent
        sx={{
          height: "100%",
          display: "flex",
          flexDirection: "column",
          justifyContent: "space-between",
        }}
      >
        <Box>
          <Lead sx={{ color: "primary.main", mb: "0.2em" }}>
            {translate("join_now.lead")}
          </Lead>
          <Head2 sx={{ color: "primary.main" }}>
            {translate("join_now.title")}
          </Head2>
        </Box>
        <Lead sx={{ color: "primary.main", my: "0.5em" }}>
          {translate("join_now.one")}
        </Lead>
        <Lead sx={{ color: "primary.main", my: "0.5em" }}>
          {translate("join_now.two")}
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
    </Card>
  );
};

const AboutToken = () => {
  const navigate = useNavigate();
  const t = useTranslate();

  return (
    <Card sx={{ mb: "1em", color: "primary.main" }}>
      <CardContent
        sx={{
          height: "100%",
          display: "flex",
          flexDirection: "column",
          justifyContent: "space-between",
        }}
      >
        <Box>
          <Lead sx={{ color: "primary.main", mb: "0.2em" }}>
            {t("about_token.lead")}
          </Lead>
          <Head2 sx={{ color: "primary.main" }}>{t("about_token.title")}</Head2>
        </Box>
        <Lead sx={{ color: "primary.main", my: "1em" }}>
          {t("about_token.text")}
        </Lead>
        <Stack
          gap="1em"
          direction="row"
          flexWrap="wrap"
          justifyContent="flex-end"
        >
          <Button
            size="large"
            color="primary"
            variant="outlined"
            onClick={() => navigate("/whitepaper")}
          >
            {t("about_token.learn_more")}
          </Button>
          <Button
            size="large"
            color="primary"
            variant="contained"
            href="https://oku.trade/app/rootstock/pool/0x90508e187c7defe5ca1768cea45e4a1ea818594b?isFlipped=false"
            target="_blank"
          >
            {t("about_token.trade")}
          </Button>
        </Stack>
      </CardContent>
    </Card>
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
