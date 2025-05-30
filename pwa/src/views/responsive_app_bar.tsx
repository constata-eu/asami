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
import ArrowRightIcon from "@mui/icons-material/ArrowRight";
import CampaignIcon from "@mui/icons-material/Campaign";
import HandshakeIcon from "@mui/icons-material/Handshake";
import MenuBookIcon from "@mui/icons-material/MenuBook";

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
      id: "my-campaings",
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

  const explorerItems = [
    {
      id: "accounts-menu-item",
      href: "/Account",
      text: "app_bar.accounts",
    },
    {
      id: "handles-menu-item",
      href: "/Handle",
      text: "app_bar.handles",
    },
    {
      id: "campaigns-menu-item",
      href: "/Campaign",
      text: "app_bar.campaigns",
    },
    {
      id: "collabs-menu-item",
      href: "/Collab",
      text: "app_bar.collabs",
    },
  ];

  return (
    <>
      <AppBar position="static" color="transparent" elevation={0}>
        <Toolbar
          disableGutters={true}
          sx={{
            mt: "1em",
            mb: {
              xs: "1.5em",
              md: "2.5em",
            },
            gap: "1em",
            flexWrap: { xs: "nowrap", sm: "wrap" },
            alignItems: "flex-end",
          }}
        >
          <AsamiLogo width="250px" height="100%" />
          <Box sx={{ flexGrow: 1 }} />

          {isMobile ? (
            <IconButton color="inverted" onClick={() => setOpen(true)}>
              <MenuIcon sx={{ fontSize: "1.5em" }} />
            </IconButton>
          ) : (
            <Box
              display="flex"
              flexWrap="nowrap"
              whiteSpace="nowrap"
              alignItems="center"
            >
              <IconEntry def={defs.home} />
              <IconEntry def={defs.x} />
              <IconEntry def={defs.help} />
              {authenticated && <IconEntry def={defs.whitepaper} />}
              {!authenticated && (
                <Button
                  onClick={defs.whitepaper.onClick}
                  startIcon={<defs.whitepaper.icon />}
                >
                  {t(defs.whitepaper.label)}
                </Button>
              )}

              <VerticalDivider />

              {expandExplorer && <IconEntry def={defs.explorer} />}

              {expandExplorer &&
                explorerItems.map((i) => (
                  <Button
                    key={i.id}
                    onClick={() => navigate(i.href)}
                    color="primary"
                    id={i.id}
                    disableRipple
                    sx={{ minWidth: 0 }}
                  >
                    {t(i.text)}
                  </Button>
                ))}

              {!expandExplorer && (
                <Button
                  onClick={defs.explorer.onClick}
                  startIcon={<defs.explorer.icon />}
                >
                  {t(defs.explorer.label)}
                </Button>
              )}

              <VerticalDivider />

              {!authenticated && <TextEntry def={defs.login} />}
              {authenticated && (
                <>
                  {!expandExplorer && <TextEntry def={defs.myCampaigns} />}
                  {expandExplorer && <IconEntry def={defs.myCampaigns} />}
                  {!expandExplorer && <TextEntry def={defs.myCollabs} />}
                  {expandExplorer && <IconEntry def={defs.myCollabs} />}
                  <IconEntry def={defs.logout} />
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
            color: (theme) => theme.palette.inverted.main,
            backgroundSize: "100% 100%",
            backgroundPosition: "0px 0px,0px 0px,0px 0px,0px 0px,0px 0px",
            backgroundImage: `
              radial-gradient(49% 81% at 45% 47%, #214255 0%, #0d314600 100%),
              radial-gradient(113% 91% at 17% -2%, #4d5962 1%, #0000ff00 99%),
              radial-gradient(142% 91% at 83% 7%, #147bb6b3 1%, #19303d 99%),
              radial-gradient(142% 91% at -6% 74%, #4B6574 1%, #081b2763 99%),
              radial-gradient(142% 91% at 111% 84%, #282426a8 0%, #533f4921 100%)
            `,
          },
        }}
      >
        <DialogContent sx={{ padding: 0 }}>
          <Box display="flex" justifyContent="flex-end">
            <IconButton onClick={() => setOpen(false)}>
              <CloseIcon
                sx={{
                  fontSize: "2em",
                  color: (theme) => theme.palette.inverted.main,
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
            <ListItemEntry def={defs.home} />
            {!authenticated && <ListItemEntry def={defs.login} />}
            <ListItemEntry def={defs.explorer} />

            {explorerItems.map((i) => (
              <ListItem disablePadding>
                <ListItemButton
                  sx={{ ml: "2em" }}
                  onClick={() => navigate(i.href)}
                  fullWidth
                  id={`mobile-menu-${i.id}`}
                >
                  <ListItemIcon sx={{ minWidth: 0 }}>
                    <ArrowRightIcon color="inverted" sx={{ fontSize: "2em" }} />
                  </ListItemIcon>
                  <BigListItemText primary={t(i.text)} />
                </ListItemButton>
              </ListItem>
            ))}
            <ListItemEntry def={defs.mobile_x} />
            <ListItemEntry def={defs.help} />
            <ListItemEntry def={defs.whitepaper} />
            {authenticated && <ListItemEntry def={defs.logout} />}
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

const TextEntry = ({ def, variant }) => {
  const t = useTranslate();
  return (
    <Button
      id={`menu-${def.id}`}
      variant={variant}
      onClick={def.onClick}
      target={def.target}
      href={def.href}
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
          <def.icon color="inverted" sx={{ fontSize: "2em", mr: "0.5em" }} />
        </ListItemIcon>
        <BigListItemText primary={t(def.label)} />
      </ListItemButton>
    </ListItem>
  );
};
