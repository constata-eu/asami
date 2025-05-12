import { Box, Typography } from "@mui/material";
import { Head2 } from "../components/theme";
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
        <LongText title={s.data_rights_title} lines={s.data_rights} />
        <LongText
          title={s.privacy_statement_title}
          lines={s.privacy_statement}
        />
      </Box>
    </>
  );
};

const LongText = ({ title, lines }) => {
  const html = lines.map((l) => `<p>${l}</p>`).join("");
  return (
    <Box>
      <Head2 sx={{ mt: "1em" }}>{title}</Head2>
      <Typography dangerouslySetInnerHTML={{ __html: html }} />
    </Box>
  );
};

export default About;
