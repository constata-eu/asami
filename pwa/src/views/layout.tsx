import {Paper, Card, CardContent, AppBar, Toolbar, Typography, Divider, IconButton, Box, Button, Container, styled, Backdrop, Skeleton, useMediaQuery } from '@mui/material';
import CssBaseline from '@mui/material/CssBaseline';
import { useNavigate } from 'react-router-dom';
import { useLogout, useTranslate, useSafeSetState, useGetOne} from 'react-admin';
import DashboardIcon from "@mui/icons-material/Dashboard";
import MenuIcon from '@mui/icons-material/Menu';
import CloseIcon from '@mui/icons-material/Close';
import RootstockLogo from '../assets/rootstock.svg?react';
import AsamiLogo from '../assets/logo.svg?react';
import InstagramIcon from '@mui/icons-material/Instagram';
import XIcon from '@mui/icons-material/X';
import FacebookIcon from '@mui/icons-material/Facebook';
import { Head1, Head2, BulletPoint, CardTitle, green, yellow } from '../components/theme';

export const DeckCard = ({id, borderColor, background, color, elevation, children}) => {
  const border = borderColor ? "1px solid" : "none";
  return <Card id={id} elevation={elevation || 1} sx={{ background, color, border, borderColor, marginBottom: "1em", breakInside: "avoid", flex: "1 1 250px" }} >
    { children }
  </Card>;
}

export const LoggedInNavCard = () => {
  const translate = useTranslate();
  const logout = useLogout();

  return (
    <Box sx={{ breakInside: "avoid", p:"1em", gap:"0.5em", mb:"1em", display:"flex", flexDirection:"column", alignItems:"center" }}>
      <Box mb="1em">
        <AsamiLogo margin="auto" width="250px" height="100%"/>
      </Box>
      <Button
        color="inverted"
        href="/#/?role=member"
        fullWidth
        size="small"
        id="button-post-to-earn"
      >
        { translate("logged_in_nav_card.post_to_earn") }
      </Button>
      <Button
        color="inverted"
        href="/#/?role=advertiser"
        fullWidth
        size="small"
        id="button-pay-to-amplify"
      >
        { translate("logged_in_nav_card.pay_to_amplify") }
      </Button>
      <Button href="/#/about" size="small" color="inverted" fullWidth id="button-about-us" >
        { translate("login.about_asami_button") }
      </Button>
      <Button
        href={ `https://x.com/${translate("login.x_handle")}` }
        target="_blank"
        startIcon={ <XIcon /> }
        color="inverted"
        size="small"
        fullWidth
        id="button-visit-x"
      >
        { translate("login.x_handle") }
      </Button>
      <Button
        href={ `https://instagram.com/${translate("login.ig_handle")}` }
        target="_blank"
        startIcon={ <InstagramIcon /> }
        color="inverted"
        size="small"
        fullWidth
        id="button-visit-instagram"
      >
        { translate("login.ig_handle") }
      </Button>
      <Button
        color="inverted"
        onClick={logout}
        fullWidth
        size="small"
        id="button-logout"
      >
        { translate("logged_in_nav_card.logout") }
      </Button>
    </Box>
  );
}

const Root = styled("div")(({ theme }) => ({
  display: "flex",
  flexDirection: "column",
  zIndex: 1,
  minHeight: "100vh",
  backgroundColor: theme.palette.background.default,
  position: "relative",
}));

const AppFrame = styled("div")(() => ({
  display: "flex",
  flex: 1,
  flexDirection: "column",
  overflowX: "auto",
  alignItems: "center",
  marginBottom: "3em",
}));

const Content = styled("div")(({ theme }) => ({
  width: "100%",
  display: "flex",
  flex: 1,
  flexDirection: "column",
  overflowX: "auto",
  paddingTop: "2em",
}));

export const BareLayout = ({children}) => {
  const translate = useTranslate();

  return (<Box sx={{
      minHeight: "100vh",
      display: "flex",
      flexDirection: "column",
    }}>
      <CssBaseline/>
      <Container maxWidth="xl">
        { children }
        <Divider light sx={{ mt: "5em", mb: "2em" }}/>

        <Button href="https://rootstock.io/" target="_blank" sx={{textDecoration: "none", mb: "2em"}}>
          <Box display="flex" flexDirection="column">
            <Typography fontSize="1em" textTransform="uppercase" fontFamily="LeagueSpartanBold">
              { translate("footer.built_with") }
            </Typography>
            <RootstockLogo/>
          </Box>
        </Button>
      </Container>
    </Box>
  )
}

export const ColumnsContainer = ({children}) =>
  <Box sx={{columnCount: { xs: 1, sm: 2, md: 3, lg: 4, xl: 5}, columnGap:"1em"}}>
    { children }
  </Box>
