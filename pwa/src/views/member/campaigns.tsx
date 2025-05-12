/// <reference types="vite-plugin-svgr/client" />
import { Alert, Box, Button, Typography, Card, Stack } from "@mui/material";
import { formatEther } from "ethers";
import { useSafeSetState, useTranslate, Confirm } from "react-admin";
import { TwitterTweetEmbed } from "react-twitter-embed";
import { contentId } from "../../lib/campaign";
import { useEffect, useRef } from "react";

const ConfirmHideCampaign = ({ campaignId, setPreference }) => {
  const translate = useTranslate();
  const [open, setOpen] = useSafeSetState(false);
  const [hide, setHide] = useSafeSetState(false);

  const handleConfirm = () => {
    setPreference(campaignId, true, false);
    setOpen(false);
    setHide(true);
  };

  if (hide) {
    return null;
  }

  return (
    <>
      <Button
        fullWidth
        id={`open-hide-campaign-${campaignId}`}
        size="small"
        color="secondary"
        fullWidth
        onClick={() => setOpen(true)}
      >
        {translate("member_campaigns.hide_campaign.button")}
      </Button>
      <Confirm
        isOpen={open}
        title={translate("member_campaigns.hide_campaign.title")}
        content={translate("member_campaigns.hide_campaign.content")}
        onConfirm={handleConfirm}
        onClose={() => setOpen(false)}
      />
    </>
  );
};

export const XCampaign = ({
  handle,
  campaign,
  prefsContext,
  setPreference,
}) => {
  const translate = useTranslate();
  const repostUrl = `https://twitter.com/intent/retweet?tweet_id=${contentId(campaign)}&related=asami_club`;
  const attemptedOn = prefsContext.data.find(
    (x) => x.campaignId == campaign.id,
  )?.attemptedOn;
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
      id={`campaign-container-${campaign.id}`}
      sx={{
        borderRadius: "13px 13px 4px 4px",
        mb: "1em",
      }}
    >
      <Box overflow="hidden">
        <div ref={containerRef}>
          <TwitterTweetEmbed
            tweetId={contentId(campaign)}
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
        <Stack
          direction="row"
          justifyContent="space-between"
          alignItems="baseline"
        >
          <Typography variant="h6">
            {translate("member_campaigns.header.title", {
              amount: formatEther(campaign.youWouldReceive || 0),
            })}
          </Typography>
          <Typography variant="body2">
            •{translate("member_campaigns.header.subheader")}
          </Typography>
        </Stack>
        {attemptedOn ? (
          <Alert
            id={`alert-repost-attempted-${campaign.id}`}
            variant="outlined"
            color="warning"
            severity="info"
            sx={{ border: "none", p: 0 }}
          >
            {translate("member_campaigns.x.attempted")}
          </Alert>
        ) : (
          <Button
            id={`button-repost-${campaign.id}`}
            sx={{ my: "0.5em" }}
            onClick={() => setPreference(campaign.id, false, true)}
            fullWidth
            variant="contained"
            href={repostUrl}
            target="_blank"
          >
            {translate("member_campaigns.x.main_button")}
          </Button>
        )}
        {!attemptedOn && (
          <ConfirmHideCampaign
            campaignId={campaign.id}
            setPreference={setPreference}
          />
        )}
      </Box>
    </Card>
  );
};
