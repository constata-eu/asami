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
import { Head1 } from '../components/theme';
import logo from '../assets/asami.png';
import rootstock from '../assets/rootstock.png';
import { useContracts } from "../components/contracts_context";

const Login = () => {
  const [role, setRole] = useStore('user.role', 'advertiser');
  const [oauthVerifier, setOauthVerifier] = useStore('user.oauthChallenge');
  const [open, setOpen] = useSafeSetState(false);
  const { signLoginMessage } = useContracts();

  const checkAuth = useCheckAuth();
  const navigate = useNavigate();
  const [error, setError] = useSafeSetState();

  const startLoginFlow = async (method) => {
    setOpen(false);
    switch (method) {
      case "Eip712":
        const code = await signLoginMessage();
        navigate(`/eip712_login?code=${code}`);
        break;
      case "X":
        const { url, verifier } = await makeXUrl();
        localStorage.setItem("oauthVerifier", verifier);
        document.location.href = url;
        break;
      case "Instagram":
        document.location.href = makeInstagramUrl();
        break;
    }
  };

  const loginAs = async (role) => {
    try {
      setRole(role);
      setOpen(true);
    } catch (e) {
      setError(e.message)
    }
  }

/*
 * Authentic
Social
Amplification &
Media
Interaction
*/
  return (<BareLayout>
    <LoginSelector open={open} onSelect={startLoginFlow} />

    <Box alignItems="center" m="1em 0 2em 0">
      <Typography variant="h1" fontSize="6em" margin="0" lineHeight="0.5em" fontFamily="Sacramento" fontWeight="bold">
        asami
      </Typography>
      <Typography variant="p" fontSize="0.8em">
        Authentic Socially Amplified Marketing Initiative.
      </Typography>
    </Box>

    <Alert severity="warning" sx={{my: "1em" }}>
      <AlertTitle>This is a tech preview, you're seeing our staging environment now.</AlertTitle>
      There's no real rewards and these are not real campaigns. It's just for testing.
      <br/>
      Any data you enter here may be deleted or abandoned in the RSK testnet.
      The last reset of this staging environment was on november 22 2023.
      <br/>
      We have a github repo at <a href="https://github.com/constata-eu/asami">constata-eu/asami</a>
    </Alert>

    
    <Typography my="1em" fontSize="min(4em, 1em + 8vw)" flexBasis="300px" flexGrow="2" lineHeight="1em" fontFamily="LeagueSpartanBlack">
      Get paid for sharing on social media in our club.
    </Typography>

    <Box display="flex" flexWrap="wrap" gap="3em" mb="3em">
      <Box flexGrow="1" flexBasis="400px" display="flex" flexDirection="column">
        <Paper sx={{ p: "1em"}}>
          <Typography fontSize="2em" fontFamily="LeagueSpartanBlack" letterSpacing="-0.03em">
            Want to join?
          </Typography>
          <Typography mb="2em">
            You only need an X account to join. You'll eventually need a WEB3 wallet such as metamask to collect your rewards.
          </Typography>
          <Button
            onClick={() => loginAs("member")}
            sx={{margin: "auto 0 0 0"}}
            variant="contained"
            size="large"
            fullWidth
            id="button-login-as-member"
          >
            Connect as member
          </Button>
        </Paper>
      </Box>

      <Box flexGrow="1" flexBasis="400px" display="flex" flexDirection="column">
        <Paper sx={{ p: "1em"}}>
          <Typography fontSize="2em" fontFamily="LeagueSpartanBlack" letterSpacing="-0.03em">
            Need to advertise?
          </Typography>
          <Typography mb="2em">
            Submit a link to your X post, and we'll have our members repost it.
            You'll get genuine amplification for a good price.
          </Typography>
          <Button
            onClick={() => loginAs("advertiser")}
            sx={{margin: "auto 0 0 0"}}
            variant="contained"
            size="large"
            fullWidth
            id="button-login-as-advertiser"
          >
            Connect as advertiser
          </Button>
        </Paper>
      </Box>
    </Box>

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
      <b>The Admin</b> watches for member reposts and <b>registers them</b> as collaborations. It also <b>gives a score</b> to member collaborations.
    </Typography>
    <Typography mb="0.7em">
      <b>Members may set their own price</b> for each repost and update it every 14 days.
    </Typography>
    <Typography mb="0.7em">
      <b>Advertisers can require a given score/price ratio</b> from members before they can collaborate on their campaign.
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

    <Divider light sx={{ mt: "5em", mb: "2em" }}/>

    <Button href="https://rootstock.io/" target="_blank" sx={{textDecoration: "none", mb: "2em"}}>
      <Box display="flex" flexDirection="column">
        <Typography fontSize="1em" textTransform="uppercase" fontFamily="LeagueSpartanBold">
          Built with
        </Typography>
        <img src={rootstock}  style={{width: "150px" }} />
      </Box>
    </Button>
  </BareLayout>);
};

const LoginSelector = ({open, onSelect}) => {
  return (<Dialog open={open} fullWidth maxWidth="sm">
    <Alert severity="warning" icon={false}>
      <Typography sx={{ color: "inherit" }} fontSize="1.2em" fontFamily="LeagueSpartanBlack" letterSpacing="-0.03em">
        Consent to transfer your data outside the EU
      </Typography>
      Using ASAMI means agreeing to permanent disclosure and storage of your data globally.
      This non-revocable policy is essential for our club's fairness and transparency.
      Before logging in, please assess the potential privacy and reputation risks of disclosing your social media handles and rewards.
      <Button href="https://eur-lex.europa.eu/legal-content/EN/TXT/?uri=CELEX%3A32016R0679" sx={{mt:"1em"}} id="no-login-button" fullWidth color="warning" variant="outlined">
        I do not consent!
      </Button>
    </Alert>
    <DialogContent>
      <Box p="0em 1em 0em 0em" display="flex" gap="1em" flexDirection="column">
        <Box>
          <Typography fontSize="1.2em" fontFamily="LeagueSpartanBlack" letterSpacing="-0.03em">
            Login with X to start right away.
          </Typography>
          <ul>
            <li>Make campaigns with limited budgets as an advertiser.</li>
            <li>Collaborate in campaigns as a member.</li>
            <li>Accumulate your rewards, and setup your WEB3 wallet later to claim them.</li>
          </ul>
          <Button id="x-login-button" fullWidth variant="contained" onClick={() => onSelect("X")}>Accept and login with X</Button>
        </Box>
        <Divider light sx={{ m: "1em 0" }}/>
        <Box>
          <Typography fontSize="1.2em" fontFamily="LeagueSpartanBlack" letterSpacing="-0.03em">
            Login as a full member with WEB3.
          </Typography>
          <ul>
            <li>Requires working and funded WEB3 wallet such as metamask.</li>
            <li>Get paid immediately for your collaborations.</li>
            <li>Get ASAMI tokens, passive income from the club fees, and more benefits.</li>
            <li>Participate on the club governance.</li>
          </ul>
          <Button id="wallet-login-button" fullWidth variant="contained" onClick={() => onSelect("Eip712")}>Accept and login with WEB3</Button>
        </Box>
      </Box>
    </DialogContent>
  </Dialog>);
}

export default Login;

