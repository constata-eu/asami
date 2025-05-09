import { Box, Typography } from "@mui/material";
import { Head2, Head3 } from "../components/theme";
import { messages, browserLocale } from "../i18n";
import whitepaper from "../generated/whitepaper.json";
import whitepaper_es from "../generated/whitepaper_es.json";
import { ResponsiveAppBar } from "./responsive_app_bar";

const About = () => {
  const s = messages[browserLocale].about_us;
  const whitepaper_body =
    browserLocale == "en" ? whitepaper.html : whitepaper_es.html;

  return (
    <>
      <ResponsiveAppBar />
      <Box id="about">
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

        <LongText
          title={s.privacy_statement_title}
          lines={s.privacy_statement}
        />
        <LongText title={s.data_rights_title} lines={s.data_rights} />
      </Box>
    </>
  );
};

export default About;

const LongText = ({ title, lines }) => {
  const html = lines.map((l) => `<p>${l}</p>`).join("");
  return (
    <Box>
      <Head2 sx={{ mt: "1em" }}>{title}</Head2>
      <Typography dangerouslySetInnerHTML={{ __html: html }} />
    </Box>
  );
};
