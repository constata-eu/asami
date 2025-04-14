import { Box, Button, Typography } from '@mui/material';
import { useTranslate } from 'react-admin';
import { green } from '../components/theme';
import AsamiLogo from '../assets/logo.svg?react';
import { messages, browserLocale } from '../i18n';
import whitepaper from '../generated/whitepaper.json';
import whitepaper_es from '../generated/whitepaper_es.json';

const LongText = ({title, lines}) => {
  const html = lines.map((l) => `<p>${l}</p>`).join("");
  return (<Box sx={{ breakInside: "avoid" }}>
    <Typography mb="0.5em" id="terms_and_policy" fontSize="2em" fontFamily="LeagueSpartanBlack" letterSpacing="-0.03em">
      { title }
    </Typography>
    <Typography mb="0.7em" dangerouslySetInnerHTML={{ __html: html }}/>
  </Box>);
}

const About = () => {
  const translate = useTranslate();
  const s = messages[browserLocale].about_us;
  const whitepaper_body = browserLocale == 'en' ? whitepaper.html : whitepaper_es.html;
  
  return (
    <Box sx={{ py:"2em", columnCount: { xs: 1, md: 2}, columnRule: `1px solid ${green}`, columnGap:"2em"}}>
      <Box maxWidth="250px" mb="2em">
        <AsamiLogo width="100%" height="100%"/>
      </Box>
      <Button sx={{mb: "1em"}} color="primary" href="#/" id="button-back-home">
        { translate("about_us.go_back_button") }
      </Button>

      <Box
        sx={{
          '& h1, & h2': {
            fontFamily: "LeagueSpartanBlack",
            letterSpacing: "-0.03em",
          },
        }}
      >
        <div className="prose max-w-none" dangerouslySetInnerHTML={{ __html: whitepaper_body }}/>
      </Box>

      <br/>
      <br/>

      <LongText title={s.privacy_statement_title} lines={s.privacy_statement} />
      <LongText title={s.data_rights_title} lines={s.data_rights} />
    </Box>
  );
};

export default About;

