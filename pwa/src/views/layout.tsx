import {
  Card,
  Typography,
  Divider,
  Box,
  Button,
  Backdrop,
  Container,
  AppBar,
  Toolbar,
  IconButton,
  useMediaQuery,
} from "@mui/material";
import CssBaseline from "@mui/material/CssBaseline";
import {
  useLogout,
  useTranslate,
  useSafeSetState,
  CoreAdminContext,
  I18nContext,
} from "react-admin";
import { useEffect, useContext } from "react";
import { useNavigate } from "react-router-dom";
import { publicDataProvider } from "../lib/data_provider";
import RootstockLogo from "../assets/rootstock.svg?react";
import ConstataLogo from "../assets/constata.svg?react";
import RootstockLabsLogo from "../assets/rootstock_labs.svg?react";
import RootstockCollectiveLogo from "../assets/rootstock_collective.svg?react";
import FpBlockLogo from "../assets/fpblock.svg?react";
import AsamiLogo from "../assets/logo.svg?react";
import XIcon from "@mui/icons-material/X";
import DashboardIcon from "@mui/icons-material/Dashboard";
import MenuIcon from "@mui/icons-material/Menu";
import CloseIcon from "@mui/icons-material/Close";

export const DeckCard = ({
  id,
  borderColor,
  background,
  color,
  elevation,
  children,
}) => {
  const border = borderColor ? "1px solid" : "none";
  return (
    <Card
      id={id}
      elevation={elevation || 1}
      sx={{
        background,
        color,
        border,
        borderColor,
        marginBottom: "1em",
        breakInside: "avoid",
        flex: "1 1 250px",
      }}
    >
      {children}
    </Card>
  );
};

export const LoggedInNavCard = () => {
  const translate = useTranslate();
  const logout = useLogout();

  return (
    <Box
      sx={{
        breakInside: "avoid",
        p: "1em",
        gap: "0.5em",
        mb: "1em",
        display: "flex",
        flexDirection: "column",
        alignItems: "center",
      }}
    >
      <Box mb="1em">
        <AsamiLogo margin="auto" width="250px" height="100%" />
      </Box>
      <Button
        color="inverted"
        href="/#/?role=member"
        fullWidth
        size="small"
        id="button-post-to-earn"
      >
        {translate("logged_in_nav_card.post_to_earn")}
      </Button>
      <Button
        color="inverted"
        href="/#/?role=advertiser"
        fullWidth
        size="small"
        id="button-pay-to-amplify"
      >
        {translate("logged_in_nav_card.pay_to_amplify")}
      </Button>
      <Button
        href="/#/about"
        size="small"
        color="inverted"
        fullWidth
        id="button-about-us"
      >
        {translate("login.about_asami_button")}
      </Button>
      <Button
        href="/#/Stats/0/show"
        color="inverted"
        fullWidth
        id="button-stats"
      >
        {translate("explorer.menu.stats")}
      </Button>
      <Button
        href={`https://x.com/${translate("login.x_handle")}`}
        target="_blank"
        startIcon={<XIcon />}
        color="inverted"
        size="small"
        fullWidth
        id="button-visit-x"
      >
        {translate("login.x_handle")}
      </Button>
      <Button
        color="inverted"
        onClick={logout}
        fullWidth
        size="small"
        id="button-logout"
      >
        {translate("logged_in_nav_card.logout")}
      </Button>
    </Box>
  );
};

export const BareLayout = ({ children }) => {
  const translate = useTranslate();

  return (
    <Box
      sx={{
        minHeight: "100vh",
        display: "flex",
        flexDirection: "column",
      }}
    >
      <CssBaseline />
      <Container maxWidth="xl">
        {children}
        <Divider light sx={{ mt: "5em", mb: "2em" }} />
        <Box display="flex" direction="row" gap="2em" flexWrap="wrap">
          <Button
            href="https://explorer.rootstock.io/address/0x16039ab4e9b0bf3b79f9a221898d152151026034"
            target="_blank"
            sx={{ textDecoration: "none" }}
          >
            <Box display="flex" flexDirection="column">
              <Typography
                fontSize="1em"
                textTransform="uppercase"
                fontFamily="LeagueSpartanBold"
              >
                {translate("footer.built_with")}
              </Typography>
              <RootstockLogo />
            </Box>
          </Button>
          <Button
            href="https://constata.eu"
            target="_blank"
            sx={{ textDecoration: "none" }}
          >
            <Box display="flex" flexDirection="column">
              <Typography
                fontSize="1em"
                textTransform="uppercase"
                fontFamily="LeagueSpartanBold"
              >
                {translate("footer.campaign_manager")}
              </Typography>
              <ConstataLogo />
            </Box>
          </Button>
          <Button
            href="https://rootstockcollective.xyz"
            target="_blank"
            sx={{ textDecoration: "none" }}
          >
            <Box display="flex" flexDirection="column">
              <Typography
                fontSize="1em"
                textTransform="uppercase"
                fontFamily="LeagueSpartanBold"
              >
                {translate("footer.funds_sponsor")}
              </Typography>
              <RootstockCollectiveLogo />
            </Box>
          </Button>
          <Button
            href="https://www.rootstocklabs.com"
            target="_blank"
            sx={{ textDecoration: "none" }}
          >
            <Box display="flex" flexDirection="column">
              <Typography
                fontSize="1em"
                textTransform="uppercase"
                fontFamily="LeagueSpartanBold"
              >
                {translate("footer.with_support_from")}
              </Typography>
              <RootstockLabsLogo />
            </Box>
          </Button>
          <Button
            href="https://fpblock.com"
            target="_blank"
            sx={{ textDecoration: "none" }}
          >
            <Box display="flex" flexDirection="column">
              <Typography
                fontSize="1em"
                textTransform="uppercase"
                fontFamily="LeagueSpartanBold"
              >
                {translate("footer.tech_sponsor")}
              </Typography>
              <FpBlockLogo />
            </Box>
          </Button>
        </Box>
      </Container>
    </Box>
  );
};

export const ColumnsContainer = ({ children }) => (
  <Box
    sx={{
      columnCount: { xs: 1, sm: 2, md: 3, lg: 4, xl: 5 },
      columnGap: "1em",
    }}
  >
    {children}
  </Box>
);

export const ExplorerAppBar = () => {
  const [menuOpen, setMenuOpen] = useSafeSetState(false);
  const navigate = useNavigate();
  const translate = useTranslate();
  const isSmall = useMediaQuery((theme: any) => theme.breakpoints.down("sm"));

  const MobileMenu = () => (
    <Box
      sx={{
        flexGrow: 1,
        display: { xs: "flex", md: "none" },
        justifyContent: "end",
      }}
      id="mobile-menu"
    >
      <IconButton
        size="large"
        aria-controls="menu-appbar"
        onClick={() => setMenuOpen(true)}
        color="inherit"
      >
        <MenuIcon />
      </IconButton>
      <Backdrop
        sx={{
          color: "#fff",
          backgroundColor: "rgba(0, 0, 0, 0.9)",
          zIndex: (theme) => theme.zIndex.drawer + 1,
        }}
        open={menuOpen}
        onClick={() => setMenuOpen(false)}
      >
        <Box display="flex" flexDirection="column">
          <IconButton
            sx={{ svg: { fontSize: "80px !important" }, mb: 2 }}
            color="inverted"
            onClick={() => setMenuOpen(false)}
          >
            <CloseIcon />
          </IconButton>

          <Button
            size="large"
            sx={{
              svg: { fontSize: "1em !important" },
              fontSize: 40,
              mb: 2,
              textTransform: "uppercase",
            }}
            color="inverted"
            onClick={() => navigate("/Stats/0/show")}
            id="stats-mobile-menu-item"
          >
            {translate("explorer.menu.stats")}
          </Button>

          <Button
            size="large"
            sx={{ fontSize: 40, mb: 2, textTransform: "uppercase" }}
            color="inverted"
            onClick={() => navigate("/Handle")}
            id="handles-mobile-menu-item"
          >
            {translate("explorer.menu.handles")}
          </Button>
          <Button
            size="large"
            sx={{ fontSize: 40, mb: 2, textTransform: "uppercase" }}
            color="inverted"
            onClick={() => navigate("/Account")}
            id="accounts-mobile-menu-item"
          >
            {translate("explorer.menu.accounts")}
          </Button>
          <Button
            size="large"
            sx={{ fontSize: 40, mb: 2, textTransform: "uppercase" }}
            color="inverted"
            onClick={() => navigate("/Campaign")}
            id="campaigns-mobile-menu-item"
          >
            {translate("explorer.menu.campaigns")}
          </Button>
          <Button
            size="large"
            sx={{ fontSize: 40, mb: 2, textTransform: "uppercase" }}
            color="inverted"
            onClick={() => navigate("/Collab")}
            id="collabs-mobile-menu-item"
          >
            {translate("explorer.menu.collabs")}
          </Button>
          <Button
            size="large"
            sx={{ fontSize: 40, mb: 2, textTransform: "uppercase" }}
            color="inverted"
            onClick={() => navigate("/Oracle")}
            id="oracle-mobile-menu-item"
          >
            {translate("explorer.menu.oracle")}
          </Button>
        </Box>
      </Backdrop>
    </Box>
  );

  const ComputerMenu = () => (
    <Box
      sx={{
        flexGrow: 1,
        display: { xs: "none", md: "flex" },
        justifyContent: "end",
      }}
      id="desktop-menu"
    >
      <Button
        sx={{ ml: 1, textTransform: "uppercase" }}
        onClick={() => navigate("/Stats/0/show")}
        id="stats-menu-item"
      >
        {translate("explorer.menu.stats")}
      </Button>
      <Button
        sx={{ ml: 1, textTransform: "uppercase" }}
        onClick={() => navigate("/Account")}
        id="accounts-menu-item"
      >
        {translate("explorer.menu.accounts")}
      </Button>
      <Button
        sx={{ ml: 1, textTransform: "uppercase" }}
        onClick={() => navigate("/Campaign")}
        id="campaigns-menu-item"
      >
        {translate("explorer.menu.campaigns")}
      </Button>
      <Button
        sx={{ ml: 1, textTransform: "uppercase" }}
        onClick={() => navigate("/Handle")}
        id="handles-menu-item"
      >
        {translate("explorer.menu.handles")}
      </Button>
      <Button
        sx={{ ml: 1, textTransform: "uppercase" }}
        onClick={() => navigate("/Collab")}
        id="collabs-menu-item"
      >
        {translate("explorer.menu.collabs")}
      </Button>
      <Button
        sx={{ ml: 1, textTransform: "uppercase" }}
        onClick={() => navigate("/OnChainJob")}
        id="oracle-menu-item"
      >
        {translate("explorer.menu.oracle")}
      </Button>
    </Box>
  );

  return (
    <AppBar
      position="static"
      color="transparent"
      elevation={0}
      sx={{ py: "14px" }}
    >
      <Toolbar sx={{ minHeight: "0 !important" }}>
        <Box sx={{ display: "flex" }}>
          <a
            href="https://asami.club"
            style={{ lineHeight: 0 }}
            target="_blank"
            rel="noreferrer"
          >
            <AsamiLogo
              margin="auto"
              width={isSmall ? "30px" : "60px"}
              height="100%"
            />
          </a>
        </Box>
        <MobileMenu />
        <ComputerMenu />
      </Toolbar>
    </AppBar>
  );
};

export const ExplorerLayout = ({ children }) => {
  const i18nProvider = useContext(I18nContext);
  const [pubDataProvider, setPubDataProvider] = useSafeSetState();
  useEffect(() => {
    async function initApp() {
      const prov = await publicDataProvider();
      setPubDataProvider(prov);
    }
    initApp();
  }, []);

  if (!pubDataProvider) {
    return <></>;
  }

  return (
    <CoreAdminContext
      i18nProvider={i18nProvider}
      dataProvider={pubDataProvider}
      authProvider={null}
    >
      <Box p="1em" id="asami-explorer">
        <ExplorerAppBar />
        {children}
      </Box>
    </CoreAdminContext>
  );
};

export const CardTable = ({ children, ...props }) => (
  <Box
    sx={{
      columnCount: { xs: 1, sm: 2, md: 3, lg: 4, xl: 5 },
      columnGap: "1em",
    }}
    {...props}
    children={children}
  />
);
