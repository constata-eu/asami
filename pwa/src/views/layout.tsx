import {
  Card,
  Typography,
  Box,
  Button,
  Backdrop,
  Container,
  AppBar,
  Toolbar,
  IconButton,
  List,
  ListItem,
  ListItemButton,
  ListItemText,
  useMediaQuery,
  DialogContent,
  Dialog,
} from "@mui/material";
import CssBaseline from "@mui/material/CssBaseline";
import {
  useLogout,
  useTranslate,
  CoreAdminContext,
  I18nContext,
  useTheme,
} from "react-admin";
import { useEffect, useContext, useState } from "react";
import { useNavigate } from "react-router-dom";
import { publicDataProvider } from "../lib/data_provider";
import { ResponsiveAppBar } from "./responsive_app_bar";
import RootstockLogo from "../assets/rootstock.svg?react";
import ConstataLogo from "../assets/constata.svg?react";
import RootstockLabsLogo from "../assets/rootstock_labs.svg?react";
import RootstockCollectiveLogo from "../assets/rootstock_collective.svg?react";
import FpBlockLogo from "../assets/fpblock.svg?react";
import AsamiLogo from "../assets/logo.svg?react";
import XIcon from "@mui/icons-material/X";
import MenuIcon from "@mui/icons-material/Menu";
import CloseIcon from "@mui/icons-material/Close";

export const DeckCard = ({
  id,
  borderColor,
  background,
  color,
  elevation,
  children,
  width,
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
        width,
        marginBottom: "1em",
        breakInside: "avoid",
        flex: "1 1 250px",
      }}
    >
      {children}
    </Card>
  );
};

export const LoggedOutAppBar = ({ loginAs }) => {
  const translate = useTranslate();

  return (
    <ResponsiveAppBar
      items={[
        {
          id: "button-about-us",
          href: "/#/about",
          text: "login.about_asami_button",
        },
        {
          id: "button-stas",
          href: "/#/Stats/0/show",
          text: "explorer.menu.stats",
        },
        {
          id: "button-visit-x",
          href: `https://x.com/${translate("login.x_handle")}`,
          target: "_blank",
          text: "login.x_handle",
        },
        {
          id: "button-login-as-member",
          href: null,
          variant: "contained",
          onClick: () => loginAs("member"),
          text: "login.login_button",
        },
      ]}
    />
  );
};

export const ExplorerAppBar = () => {
  const translate = useTranslate();
  return (
    <ResponsiveAppBar
      items={[
        {
          id: "stats-menu-item",
          href: "/#/Stats/0/show",
          text: "explorer.menu.stats",
        },
        {
          id: "handles-menu-item",
          href: "/#/Handle",
          text: "explorer.menu.handles",
        },
        {
          id: "accounts-menu-item",
          href: "/#/Account",
          text: "explorer.menu.accounts",
        },
        {
          id: "campaigns-menu-item",
          href: "/#/Campaign",
          text: "explorer.menu.campaigns",
        },
        {
          id: "collabs-menu-item",
          href: "/#/Collab",
          text: "explorer.menu.collabs",
        },
        {
          id: "oracle-menu-item",
          href: "/#/Oracle",
          text: "explorer.menu.oracle",
        },
      ]}
    />
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
      <Container
        maxWidth="xl"
        sx={{ display: "flex", flexDirection: "column", minHeight: "100vh" }}
      >
        {children}
        <Box sx={{ flexGrow: 1 }} />
        <Box display="flex" direction="row" gap="2em" flexWrap="wrap" my="2em">
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

export const ExplorerLayout = ({ children }) => {
  const i18nProvider = useContext(I18nContext);
  const [pubDataProvider, setPubDataProvider] = useState();
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
