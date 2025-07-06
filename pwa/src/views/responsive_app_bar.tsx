import {
  Box,
  Button,
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
  Divider,
  ListItemIcon,
} from "@mui/material";
import { useNavigate } from "react-router-dom";

import { useAuthState, useLogout, useStore, useTranslate } from "react-admin";
import { useState } from "react";
import AsamiLogo from "../assets/logo.svg?react";
import MenuIcon from "@mui/icons-material/Menu";
import CloseIcon from "@mui/icons-material/Close";
import HomeIcon from "@mui/icons-material/Home";
import XIcon from "@mui/icons-material/X";
import HelpOutlineIcon from "@mui/icons-material/HelpOutline";
import QueryStatsIcon from "@mui/icons-material/QueryStats";
import LogoutIcon from "@mui/icons-material/Logout";
import LoginIcon from "@mui/icons-material/Login";
import WaterfallChartIcon from "@mui/icons-material/WaterfallChart";
import CampaignIcon from "@mui/icons-material/Campaign";
import HandshakeIcon from "@mui/icons-material/Handshake";
import MenuBookIcon from "@mui/icons-material/MenuBook";
import { useEmbedded } from "../components/embedded_context";
import { backgroundGradientRules, paperBackground } from "../components/theme";

export const ResponsiveAppBar = ({
  expandExplorer = false,
}: {
  expandExplorer?: boolean;
}) => {
  const t = useTranslate();
  const [open, setOpen] = useState(false);
  const isMobile = useMediaQuery((theme: any) => theme.breakpoints.down("md"));
  const navigate = useNavigate();
  const { authenticated } = useAuthState();
  const logout = useLogout();
  const isEmbedded = useEmbedded();

  const defs = {
    home: {
      id: "home",
      onClick: () => navigate("/"),
      icon: HomeIcon,
      label: "app_bar.home",
    },
    x: {
      id: "visit-x",
      href: `https://x.com/@${t("app_bar.x_handle")}`,
      icon: SmallXIcon,
      label: "app_bar.x_handle",
    },
    mobile_x: {
      id: "visit-x",
      href: `https://x.com/${t("app_bar.x_handle")}`,
      icon: XIcon,
      label: "app_bar.x_handle",
    },
    help: {
      id: "about",
      onClick: () => navigate("/about"),
      icon: HelpOutlineIcon,
      label: "app_bar.help",
    },
    explorer: {
      id: "explorer",
      onClick: () => navigate("/Stats/0/show"),
      icon: QueryStatsIcon,
      label: "app_bar.stats",
    },
    token: {
      id: "token",
      onClick: () => navigate("/Token"),
      icon: WaterfallChartIcon,
      label: "app_bar.token",
    },
    login: {
      id: "login",
      onClick: () => navigate("/login"),
      icon: LoginIcon,
      label: "app_bar.login",
    },
    logout: {
      id: "logout",
      onClick: logout,
      icon: LogoutIcon,
      label: "app_bar.logout",
    },
    myCampaigns: {
      id: "my-campaigns",
      onClick: () => {
        localStorage.setItem("asami_user_role", "advertiser");
        navigate("/dashboard");
      },
      icon: CampaignIcon,
      label: "app_bar.my_campaigns",
    },
    myCollabs: {
      id: "my-collabs",
      onClick: () => {
        localStorage.setItem("asami_user_role", "member");
        navigate("/dashboard");
      },
      icon: HandshakeIcon,
      label: "app_bar.my_collabs",
    },
    whitepaper: {
      id: "whitepaper",
      onClick: () => navigate("/whitepaper"),
      icon: MenuBookIcon,
      label: "app_bar.whitepaper",
    },
  };

  return (
    <>
      <AppBar position="static" color="transparent" elevation={0}>
        <Toolbar
          disableGutters={true}
          sx={{
            mt: "1em",
            mb: {
              xs: "0.5em",
              md: "2.5em",
            },
            gap: "1em",
            flexWrap: { xs: "nowrap", sm: "wrap" },
            alignItems: isMobile ? "center" : "flex-end",
          }}
        >
          {!isMobile && <AsamiLogo width="250px" height="100%" />}
          <Box sx={{ flexGrow: 1 }} />

          {isMobile ? (
            <Box
              display="flex"
              flexWrap="nowrap"
              whiteSpace="nowrap"
              alignItems="center"
            >
              {authenticated && (
                <>
                  <IconTextEntry def={defs.myCampaigns} />
                  <IconTextEntry def={defs.myCollabs} />
                </>
              )}
              <IconButton color="primary" onClick={() => setOpen(true)}>
                <MenuIcon sx={{ fontSize: "1.5em" }} />
              </IconButton>
            </Box>
          ) : (
            <Box
              display="flex"
              flexWrap="nowrap"
              whiteSpace="nowrap"
              alignItems="center"
            >
              {!isEmbedded && <IconEntry def={defs.home} />}
              {!isEmbedded && <IconEntry def={defs.x} />}
              <IconEntry def={defs.help} />
              <TextEntry def={defs.whitepaper} />
              <TextEntry def={defs.explorer} />
              <TextEntry def={defs.token} />

              <VerticalDivider />

              {!authenticated && <TextEntry def={defs.login} />}
              {authenticated && (
                <>
                  <TextEntry def={defs.myCampaigns} />
                  <TextEntry def={defs.myCollabs} />
                  {!isEmbedded && <IconEntry def={defs.logout} />}
                </>
              )}
            </Box>
          )}
        </Toolbar>
      </AppBar>

      <Dialog
        fullScreen
        open={open}
        onClose={() => setOpen(false)}
        PaperProps={{
          sx: {
            ...backgroundGradientRules(100),
            color: "primary.main",
          },
        }}
      >
        <DialogContent sx={{ padding: 0 }}>
          <Box display="flex" justifyContent="flex-end">
            <IconButton onClick={() => setOpen(false)}>
              <CloseIcon
                sx={{
                  fontSize: "2em",
                  color: "primary.main",
                }}
              />
            </IconButton>
          </Box>

          <List>
            {authenticated && (
              <>
                <ListItemEntry def={defs.myCampaigns} />
                <ListItemEntry def={defs.myCollabs} />
              </>
            )}
            {!isEmbedded && <ListItemEntry def={defs.home} />}
            {!authenticated && !isEmbedded && (
              <ListItemEntry def={defs.login} />
            )}
            <ListItemEntry def={defs.explorer} />
            <ListItemEntry def={defs.token} />

            {!isEmbedded && <ListItemEntry def={defs.mobile_x} />}
            <ListItemEntry def={defs.help} />
            <ListItemEntry def={defs.whitepaper} />
            {authenticated && !isEmbedded && (
              <ListItemEntry def={defs.logout} />
            )}
          </List>
        </DialogContent>
      </Dialog>
    </>
  );
};

const IconEntry = ({ def }) => {
  return (
    <Button
      id={`menu-${def.id}`}
      onClick={def.onClick}
      target={def.target}
      href={def.href}
      sx={{ minWidth: 0, padding: "0 7px" }}
    >
      <def.icon />
    </Button>
  );
};

const TextEntry = ({ def }) => {
  const t = useTranslate();
  return (
    <Button
      id={`menu-${def.id}`}
      onClick={def.onClick}
      target={def.target}
      href={def.href}
    >
      {t(def.label)}
    </Button>
  );
};

const IconTextEntry = ({ def }) => {
  const t = useTranslate();
  return (
    <Button
      id={`menu-${def.id}`}
      onClick={def.onClick}
      target={def.target}
      href={def.href}
      startIcon={<def.icon />}
    >
      {t(def.label)}
    </Button>
  );
};

const BigListItemText = ({ primary }) => (
  <ListItemText
    primary={primary}
    primaryTypographyProps={{
      fontFamily: '"LeagueSpartanBlack", serif',
      fontSize: "2em",
      textTransform: "uppercase",
      fontWeight: "bold",
    }}
  />
);

const SmallXIcon = () => <XIcon sx={{ fontSize: "1.4em" }} />;

const VerticalDivider = () => (
  <Divider
    orientation="vertical"
    flexItem={true}
    sx={{
      borderColor: "primary.main",
      mx: "0.5em",
    }}
  />
);

const ListItemEntry = ({ def }) => {
  const t = useTranslate();
  return (
    <ListItem disablePadding>
      <ListItemButton
        id={`mobile-menu-${def.id}`}
        onClick={def.onClick}
        target={def.target}
        href={def.href}
        fullWidth
      >
        <ListItemIcon>
          <def.icon color="primary" sx={{ fontSize: "2em", mr: "0.5em" }} />
        </ListItemIcon>
        <BigListItemText primary={t(def.label)} />
      </ListItemButton>
    </ListItem>
  );
};
