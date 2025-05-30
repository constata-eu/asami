import { Box, Button, Stack } from "@mui/material";
import { browserLocale } from "../i18n";
import whitepaper from "../generated/whitepaper.json";
import whitepaper_es from "../generated/whitepaper_es.json";
import { ResponsiveAppBar } from "./responsive_app_bar";
import AutoAwesomeIcon from "@mui/icons-material/AutoAwesome";
import { useTranslate } from "react-admin";

const Whitepaper = () => {
  const t = useTranslate();
  const whitepaper_body =
    browserLocale == "en" ? whitepaper.html : whitepaper_es.html;

  return (
    <>
      <ResponsiveAppBar />
      <Stack id="whitepaper" alignItems="flex-end">
        <Button
          size="large"
          variant="contained"
          href={`https://${t("whitepaper.robot_url")}`}
          startIcon={<AutoAwesomeIcon />}
        >
          Have ChatGPT explain it to you
        </Button>
        <Box
          sx={{
            "& hr": {
              display: "none",
            },
            "& p.lead": {
              fontFamily: "'LeagueSpartanLight'",
              color: (theme) => theme.palette.secondary.main,
              lineHeight: 1,
              fontSize: {
                xs: "15px",
                md: "20px",
              },
            },
            "& h1, & h2, & h3": {
              fontFamily: "LeagueSpartanBold",
              letterSpacing: "-0.05em",
              color: (theme) => theme.palette.secondary.main,
            },
            "& h1": {
              fontSize: "50px",
            },
            "& h2": {
              fontSize: "35px",
            },
            "& h3": {
              fontSize: "20px",
            },
            "& .section": {
              marginBottom: "3em",
            },
            "& .section:not(.no-cols) .paragraphs": {
              columnCount: { xs: 1, md: 2, lg: 3 },
              columnGap: "3em",
              columnRule: "2px solid #ffffff18",
            },
          }}
        >
          <div
            className="prose max-w-none"
            dangerouslySetInnerHTML={{ __html: whitepaper_body }}
          />
        </Box>
      </Stack>
    </>
  );
};

export default Whitepaper;
