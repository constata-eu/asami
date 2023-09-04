import {AppBar, Toolbar, IconButton, Box, Button, Container, styled, Backdrop, Skeleton, useMediaQuery } from '@mui/material';
import CssBaseline from '@mui/material/CssBaseline';
import { useNavigate } from 'react-router-dom';
import { useLogout, useTranslate, useSafeSetState, useGetOne} from 'react-admin';
import DashboardIcon from "@mui/icons-material/Dashboard";
import MenuIcon from '@mui/icons-material/Menu';
import CloseIcon from '@mui/icons-material/Close';
import logo from '../assets/asami.png';

const ResponsiveAppBar = ({loggedIn}) => {
  const [menuOpen, setMenuOpen] = useSafeSetState(false);
  const logout = useLogout();
  const navigate = useNavigate();
  const translate = useTranslate();
  const isSmall = useMediaQuery((theme: any) => theme.breakpoints.down('sm'));

  const MobileMenu = () => <Box sx={{ flexGrow: 1, display: { xs: 'flex', md: 'none' }, justifyContent: "end" }} id="mobile-menu">
    <IconButton
      size="large"
      aria-controls="menu-appbar"
      onClick={() => setMenuOpen(true) }
      color="inherit"
    >
      <MenuIcon />
    </IconButton>
    <Backdrop
      sx={{ color: '#fff', backgroundColor:"rgba(0, 0, 0, 0.9)", zIndex: (theme) => theme.zIndex.drawer + 1 }}
      open={menuOpen}
      onClick={() => setMenuOpen(false) }
    >
      <Box display="flex" flexDirection="column">
        <IconButton sx={{ "svg": { fontSize: "80px !important" }, mb: 2 }} color="inverted" onClick={ () => setMenuOpen(false) } >
          <CloseIcon />
        </IconButton>

        <Button size="large" sx={{ fontSize: 40, mb: 2, textTransform: "uppercase" }} color="inverted" onClick={() => logout() } id="logout-mobile-menu-item">
          Logout
        </Button>
      </Box>
    </Backdrop>
  </Box>

  const ComputerMenu = () => <Box sx={{ flexGrow: 1, display: { xs: 'none', md: 'flex' }, justifyContent:"end" }} id="desktop-menu">
    <Button sx={{ ml: 1, textTransform: "uppercase" }} variant="outlined" color="highlight" onClick={() => logout() } id="logout-menu-item">
      Logout
    </Button>
  </Box>

  return (
    <AppBar position="static" color="inverted" sx={{ py: isSmall ? "14px" : "28px"}}>
      <Container maxWidth="md" style={{ padding: 0}}>
        <Toolbar sx={{ minHeight: "0 !important" }}>
          <Box sx={{ display: "flex"}} >
            <a href="https://constata.eu" style={{lineHeight: 0}} target="_blank" rel="noreferrer">
              <img src={logo} alt={translate("certos.menu.logo")} style={{ height: isSmall ? "20px" : "30px", width: "auto" }}/>
            </a>
          </Box>
          {loggedIn && <>
              <MobileMenu />
              <ComputerMenu />
            </>
          }
        </Toolbar>
      </Container>
    </AppBar>
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
      <Container maxWidth="sm">
        { children }
        <Box textAlign="center" mt={8} mb={4}>
          <img src={logo}  alt={translate("certos.menu.logo")} style={{width: "200px" }} />
        </Box>
      </Container>
    </Box>
  )
}

export const ToolbarLayout = ({children, loggedIn}) => {
  return (
    <Root>
      <CssBaseline/>
      <AppFrame>
        <ResponsiveAppBar loggedIn={loggedIn} />
        <Content>
          {children}
        </Content>
      </AppFrame>
    </Root>
  )
}

export const NoLoggedInLayout = ({ children }) => {
  return <ToolbarLayout loggedIn={false} children={children} />;
};

export const AsamiLayout = ({ children }) => {
  return <ToolbarLayout loggedIn={true} children={children} />;
};

