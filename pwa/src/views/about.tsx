import { Box, Button, Typography } from '@mui/material';
import { useTranslate } from 'react-admin';
import { BareLayout } from './layout';
import { green } from '../components/theme';
import AsamiLogo from '../assets/logo.svg?react';
import { messages, browserLocale } from '../i18n';

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
  
  return (
    <Box sx={{ py:"2em", columnCount: { xs: 1, md: 2}, columnRule: `1px solid ${green}`, columnGap:"2em"}}>
      <Box maxWidth="250px" mb="2em">
        <AsamiLogo width="100%" height="100%"/>
      </Box>
      <Button sx={{mb: "3em"}} color="primary" href="#/" id="button-back-home">
        { translate("about_us.go_back_button") }
      </Button>

      <LongText title={s.club_title} lines={s.club_description} />
      <LongText title={s.privacy_statement_title} lines={s.privacy_statement} />
      <LongText title={s.data_rights_title} lines={s.data_rights} />
    </Box>
  );
};

export default About;

