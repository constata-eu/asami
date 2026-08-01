/// <reference types="vite-plugin-svgr/client" />
import { useEffect, useContext, useState, useRef } from "react";
import {
  CardContent,
  Box,
  Button,
  Typography,
  Stack,
  Card,
  Skeleton,
} from "@mui/material";

import { etherToHex } from "../lib/formatters";
import { formatEther } from "ethers";
import { publicDataProvider } from "../lib/data_provider";
import {
  useListController,
  CoreAdminContext,
  useTranslate,
  I18nContext,
  useRedirect,
} from "react-admin";

import { TwitterTweetEmbed } from "react-twitter-embed";
import { CardTable } from "./layout";
import { Head2, Head1, Lead } from "../components/theme";
import { useNavigate } from "react-router-dom";
import CampaignListEmpty from "./campaign_list_empty";
import StatsCard from "./stats_card";
import { ResponsiveAppBar } from "./responsive_app_bar";
import { contentId, isXRepostCampaign } from "../lib/campaign";
import { useEmbedded } from "../components/embedded_context";

export default () => {
  const t = useTranslate();
  const navigate = useNavigate();
  const [pubDataProvider, setPubDataProvider] = useState();
  const i18nProvider = useContext(I18nContext);
  const isEmbedded = useEmbedded();
  const redirect = useRedirect();

  useEffect(() => {
    if (isEmbedded) {
      return redirect("/login");
    }

    async function initApp() {
      const prov = await publicDataProvider();
      setPubDataProvider(prov);
    }
    initApp();
  }, []);

  if (isEmbedded) {
    return <></>;
  }

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
            <LandingMembers />
          </CoreAdminContext>
        )}
      </Box>
    </>
  );
};

const LandingMembers = () => {
  const t = useTranslate();
  const listContext = useListController({
    disableSyncWithLocation: true,
    page: 1,
    perPage: 6,
    resource: "LandingMember",
  });

  if (listContext.isLoading) {
    return (
      <Stack direction="row" gap="1em" flexWrap="wrap">
        {Array.from({ length: 3 }).map((_, i) => (
          <Card key={i} sx={{ flex: "1 1 320px", minHeight: "320px" }}>
            <Skeleton variant="rounded" height={320} />
          </Card>
        ))}
      </Stack>
    );
  }

  if (!listContext.data?.length) {
    return <></>;
  }

  return (
    <Box>
      <CardTable>
        {listContext.data.map((member) => (
          <MemberProfileCard key={member.id} member={member} />
        ))}
      </CardTable>
    </Box>
  );
};

declare global {
  interface Window {
    twttr?: {
      widgets: {
        load: (element?: HTMLElement | null) => void;
      };
    };
  }
}

const TWITTER_WIDGET_SCRIPT_ID = "twitter-wjs";

function ensureTwitterWidgetScript() {
  if (document.getElementById(TWITTER_WIDGET_SCRIPT_ID)) return;

  const script = document.createElement("script");
  script.id = TWITTER_WIDGET_SCRIPT_ID;
  script.src = "https://platform.twitter.com/widgets.js";
  script.async = true;
  script.charset = "utf-8";

  document.body.appendChild(script);
}

export function TwitterTimeline() {
  const containerRef = useRef<HTMLDivElement>(null);
  const loadedRef = useRef(false);

  useEffect(() => {
    if (loadedRef.current) return;
    loadedRef.current = true;

    ensureTwitterWidgetScript();

    // If the script was already present, ask it to scan only this component.
    window.twttr?.widgets.load(containerRef.current);
  }, []);

  return (
    <div ref={containerRef}>
      <a
        className="twitter-timeline"
        href="https://twitter.com/nubis_bruno?ref_src=twsrc%5Etfw"
        data-height="320"
      >
        Tweets by nubis_bruno
      </a>
    </div>
  );
}

const MemberProfileCard = ({ member }) => {
  if (member.xUsername) {
    return (
      <Card
        sx={{
          borderRadius: "13px 13px 4px 4px",
          mb: "1em",
          overflow: "hidden",
        }}
      >
        <TwitterTimeline />
      </Card>
    );
  }

  return (
    <Card
      sx={{
        minHeight: "320px",
        display: "flex",
        alignItems: "center",
        justifyContent: "center",
        p: "2em",
        mb: "1em",
        borderRadius: "13px 13px 4px 4px",
      }}
    >
      <Typography
        textAlign="center"
        color="primary.main"
        fontFamily="'LeagueSpartanBold'"
        fontSize="2rem"
      >
        {member.name}
      </Typography>
    </Card>
  );
};

const PublicCampaignList = ({ loginAs }) => {
  const listContext = useListController({
    debounce: 500,
    disableSyncWithLocation: true,
    filter: {
      budgetGt: etherToHex("1"),
      isPublishedEq: true,
      kindEq: "X_REPOST",
    },
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

  const visibleCampaigns = listContext.data.filter(
    (item) => isXRepostCampaign(item) && !item.isEnded,
  );

  if (visibleCampaigns.length == 0) {
    return <CampaignListEmpty />;
  }

  return (
    <CardTable>
      {visibleCampaigns.map((item, index) => (
        <Box key={item.id}>
          <PublicXCampaign key={item.id} item={item} loginAs={loginAs} />
          {index == 0 && (
            <>
              <AboutToken />
              <StatsCard />
            </>
          )}
        </Box>
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
            {translate("join_now.button")}
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
            onClick={() => navigate("/Token")}
          >
            {t("about_token.learn_more")}
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
        <Typography mb="0.5em">
          {translate("public_card_header.title", {
            author: item.advertiserName,
            amount: formatEther(item.maxIndividualReward),
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
