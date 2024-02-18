import { useEffect } from 'react';
import {
  Alert, AlertTitle, Badge, Divider,
  Dialog, DialogActions, DialogContent, DialogTitle,
  IconButton, Box, Button, Container, Paper, styled,
  Toolbar, Typography, Skeleton, useMediaQuery
} from '@mui/material';
import { useCheckAuth, useSafeSetState, useStore } from 'react-admin';
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

const About = () => {
  const [role, setRole] = useStore('user.role', 'advertiser');
  const [open, setOpen] = useSafeSetState(false);
  const [error, setError] = useSafeSetState();

  const loginAs = async (role) => {
    try {
      setRole(role);
      setOpen(true);
    } catch (e) {
      setError(e.message)
    }
  }

  return (<BareLayout>
    <Box sx={{columnCount: { xs: 1, md: 2}, columnRule: `1px solid ${green}`, columnGap:"2em"}}>
      <Box maxWidth="250px" mb="2em">
        <AsamiLogo width="100%" height="100%"/>
      </Box>
      <Button sx={{mb: "3em"}} color="primary" href="#/" id="button-back-home">
        Go back
      </Button>

      <Typography mb="0.5em" id="terms_and_policy" fontSize="2em" fontFamily="LeagueSpartanBlack" letterSpacing="-0.03em">
        It really is a club, and you're welcome to join!
      </Typography>

      <Typography mb="0.7em">
        We're a club of people that get paid for reposting content on social media.
      </Typography>
      <Typography mb="0.7em">
        You can participate as an <b>Advertiser</b>, a <b>Member</b>, or even as the club's <b>Admin</b>.
      </Typography>
      <Typography mb="0.7em">
        <b>Advertisers create campaigns</b> by submitting the link to a post, and how much to spend.
      </Typography>
      <Typography mb="0.7em">
        <b>Members</b> review all the available campaigns and decide to <b>collaborate by reposting</b>, then get paid when they do.
      </Typography>
      <Typography mb="0.7em">
        <b>The Admin</b> watches for member reposts and <b>registers them</b> as collaborations.
      </Typography>
      <Typography mb="0.7em">
        <b>The Admin</b> also looks up each member's personal network and <b>rates them with sparkles ✨</b> .
      </Typography>
      <Typography mb="0.7em">
        <b>Members may set their own price</b>, which gives place to their <b>price per sparkle ✨</b>, and update it every 14 days.
      </Typography>
      <Typography mb="0.7em">
        <b>Advertisers can require a given price per sparkle ✨</b> from members before they can collaborate on their campaign.
      </Typography>
      <Typography mb="0.7em">
        The rules governing these 3 roles are <b>enforced by a smart contract called ASAMI</b>, which exists in the RSK public blockchain.
      </Typography>
      <Typography mb="0.7em">
        When a collaboration is registered, a payment is issued to the member, and ASAMI collects a fee from it.
      </Typography>
      <Typography mb="0.7em">
        Also, everytime a fee is collected, an equivalent amount of <b>ASAMI Tokens are issued and distributed</b> to the member, the advertiser, and the admin. They get 10%, 10% and 80% of the new tokens respectively.
      </Typography>
      <Typography mb="0.7em">
        <b>All fees collected are distributed</b> at prorata to all the ASAMI token holders every 14 days.
      </Typography>
      <Typography mb="0.7em">
        ASAMI Token is a <b>standard ERC20</b> token which can be transfered, bought and sold in secondary markets.
      </Typography>
      <Typography mb="0.7em">
        Token holders can also <b>vote to change the ASAMI fee rate</b>, which affects their short term token revenue, but also their dilution.
      </Typography>
      <Typography mb="0.7em">
        <b>A new admin may be voted</b> by token holders too.
      </Typography>

      <Typography mt="2em" mb="0.5em" id="terms_and_policy" fontSize="2em" fontFamily="LeagueSpartanBlack" letterSpacing="-0.03em">
        We have an important privacy statement to make.
      </Typography>
      
      <Typography mb="0.7em">
      As part of our commitment to fostering a transparent, trustworthy, and fair community, we have specific policies regarding member data that you should be aware of before joining.
      </Typography>
      <Typography mb="0.7em">
        1. <b>Access to Participant Information</b> Upon joining, please be informed that all participants will have access to certain details about other participants and their interactions within the club. This includes social media handles, rewards received, campaigns created, and their vote in governance aspects.
      </Typography>
      <Typography mb="0.7em">
        2. <b>Permanence of Data:</b> It is crucial to our community's integrity that we maintain a permanent record of interactions and member details. Therefore, once you provide any information to the club, it will be retained indefinitely in the RSK blockchain.
      </Typography>
      <Typography mb="0.7em">
        3. <b>No Option for Erasure or Amendment:</b> Please understand that once your data is submitted to the club, you will not have the option to erase or amend this information. This is in line with our policy of maintaining a lasting record for transparency and fairness among all members. It is also a technical impediment arising from the usage of a public blockchain network which replicates data worldwide.
      </Typography>
      <Typography mb="0.7em">
        4. <b>Public Availability and Potential Risks:</b> By agreeing to join our club, you acknowledge and consent that all data you provide will be made publicly available worldwide, in perpetuity. We advise you to consider the potential impact this could have on your privacy and reputation.
      </Typography>
      <Typography mb="0.7em">
        5. <b>Informed Consent:</b> Your decision to join the club signifies your acceptance of these terms. We believe this policy is essential to attract new members who value transparency, ensure fair treatment for all, and maintain the high level of trust within our community.
      </Typography>
      <Typography mb="0.7em">
        Please take a moment to consider these points carefully. By joining ASAMI you agree to these terms regarding your data. We are excited to have you as part of our community and are committed to upholding these principles to create a positive and open environment for all members.
      </Typography>

      <Typography mt="2em" mb="0.5em" id="terms_and_policy" fontSize="2em" fontFamily="LeagueSpartanBlack" letterSpacing="-0.03em">
        About this website and your data rights.
      </Typography>

      <Typography mb="0.7em">
        This website serves as an interface for engaging with the ASAMI smart contract and participating in the club. The personal information we collect from you is solely for these purposes.
      </Typography>
      <Typography mb="0.7em">
        Given the worldwide nature of the public blockchain that enforces the ASAMI smart contract between participants, you must consent to permanetly disclose and store your data globally.
        This means you'll be transferring your data to jurisdictions outside the european union that have no data protection guarantees.
      </Typography>

      <Typography mb="0.7em">
        You may not be able to have your data amended, or deleted. Your full history of interactions with other participants such as members, advertisers and votes, and also your collected rewards will be publicly available to anyone, forever.
      </Typography>

      <Typography mb="0.7em">
        This site is developed and run by <b>Constata EU Digital Trust Services S.L.</b>, registered in Spain with C.I.F. nº B02983997, Headquartered in Calle Paseo de la Castellana 40, 8ª planta, Madrid, 28046, Spain, and registered in Registro Mercantil de Madrid, Tomo 41408, folio 174, Hoja M 733896,
      </Typography>

      <Typography mb="0.7em">
        If you have any question about the risks associated with global blockchain disclosure, or wish to exercise your data protection rights in aspects managed by this website, you can contact us at <a target="_blank" href="mailto:dpo@constata.eu">dpo@constata.eu</a>.
      </Typography>
    </Box>
  </BareLayout>);
};

export default About;

