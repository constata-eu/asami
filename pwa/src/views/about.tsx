import { useEffect } from 'react';
import {
  Alert, AlertTitle, Badge, Divider,
  Dialog, DialogActions, DialogContent, DialogTitle,
  IconButton, Box, Button, Container, Paper, styled,
  Toolbar, Typography, Skeleton, useMediaQuery
} from '@mui/material';
import { useCheckAuth, useSafeSetState, useStore, useTranslate } from 'react-admin';
import { useNavigate } from 'react-router-dom';
import authProvider, { makeXUrl, makeInstagramUrl} from '../lib/auth_provider';
import { BareLayout } from './layout';
import { Head1, green } from '../components/theme';
import logo from '../assets/asami.png';
import rootstock from '../assets/rootstock.png';
import { useContracts } from "../components/contracts_context";
import FacebookLogin from '@greatsumini/react-facebook-login';
import { Settings } from '../settings';
import AsamiLogo from '../assets/logo.svg?react';
import { messages, browserLocale } from '../i18n';

const LongForm = ({title, lines}) => {
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
  
  return (<BareLayout>
    <Box sx={{columnCount: { xs: 1, md: 2}, columnRule: `1px solid ${green}`, columnGap:"2em"}}>
      <Box maxWidth="250px" mb="2em">
        <AsamiLogo width="100%" height="100%"/>
      </Box>
      <Button sx={{mb: "3em"}} color="primary" href="#/" id="button-back-home">
        { translate("about_us.go_back_button") }
      </Button>

      <LongForm title={s.club_title} lines={s.club_description} />
      <LongForm title={s.privacy_statement_title} lines={s.privacy_statement} />
      <LongForm title={s.data_rights_title} lines={s.data_rights} />
    </Box>
  </BareLayout>);
};

export default About;

