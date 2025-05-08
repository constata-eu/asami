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

import { useAuthState, useTranslate } from "react-admin";
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

export const ResponsiveAppBar = ({
  expandExplorer = false,
}: {
  expandExplorer?: boolean;
}) => {
  const translate = useTranslate();
  const [open, setOpen] = useState(false);
  const isMobile = useMediaQuery((theme: any) => theme.breakpoints.down("md"));
  const navigate = useNavigate();
  const { authenticated: isLoggedIn } = useAuthState();

  const defs = {
    home: {
      id: "home",
      onClick: () => navigate("/"),
      icon: HomeIcon,
    },
    x: {
      id: "visit-x",
      href: `https://x.com/${translate("login.x_handle")}`,
      icon: SmallXIcon,
    },
    help: {
      id: "about",
      onClick: () => navigate("/about"),
      icon: HelpOutlineIcon,
    },
    explorer: {
      id: "explorer",
      onClick: () => navigate("/Stats/0/show"),
      icon: QueryStatsIcon,
      label: "explorer.menu.stats",
    },
    login: {
      id: "login",
      onClick: () => navigate("/login"),
      icon: LoginIcon,
      label: "login.login_button",
    },
    logout: {
      id: "logout",
      onClick: () => logout(),
      icon: LoginIcon,
      label: "logged_in_nav_card.logout",
    },
    myCampaigns: {
      id: "my-campaings",
      onClick: () => navigate("/?role=advertiser"),
      icon: CampaignIcon,
      label: "logged_in_nav_card.pay_to_amplify",
    },
    myCollabs: {
      id: "my-collabs",
      onClick: () => navigate("/?role=member"),
      icon: HandshakeIcon,
      label: "logged_in_nav_card.post_to_earn",
    },
  };

  const explorerItems = [
    {
      id: "accounts-menu-item",
      href: "/#/Account",
      text: "explorer.menu.accounts",
    },
    {
      id: "handles-menu-item",
      href: "/#/Handle",
      text: "explorer.menu.handles",
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
  ];

  return (
    <>
      <AppBar position="static" color="transparent" elevation={0}>
        <Toolbar
          disableGutters={true}
          sx={{ mt: "1em", mb: "3em", gap: "1em", alignItems: "flex-end" }}
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
              gap="0.5em"
            >
              <IconEntry def={defs.home} />
              <IconEntry def={defs.x} />
              <IconEntry def={defs.help} />

              <VerticalDivider />

              {expandExplorer && <IconEntry def={defs.explorer} />}

              {expandExplorer &&
                explorerItems.map((i) => (
                  <Button
                    key={i.id}
                    href={i.href}
                    color="primary"
                    id={i.id}
                    disableRipple
                    sx={{ minWidth: 0 }}
                  >
                    {translate(i.text)}
                  </Button>
                ))}

              {!expandExplorer && (
                <Button
                  onClick={defs.explorer.onClick}
                  startIcon={<defs.explorer.icon />}
                >
                  {translate(defs.explorer.label)}
                </Button>
              )}

              <VerticalDivider />

              {!isLoggedIn && <TextEntry def={defs.login} />}
              {isLoggedIn && <TextEntry def={defs.myCampaigns} />}
              {isLoggedIn && <TextEntry def={defs.myCollabs} />}
              {isLoggedIn && <IconEntry def={defs.logout} />}
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
            <ListItem disablePadding>
              <ListItemButton color="primary" href="/" fullWidth>
                <ListItemIcon>
                  <CampaignIcon
                    color="inverted"
                    sx={{ fontSize: "2em", mr: "0.5em" }}
                  />
                </ListItemIcon>
                <BigListItemText primary="My Campaigns" />
              </ListItemButton>
            </ListItem>
            <ListItem disablePadding>
              <ListItemButton color="primary" href="/" fullWidth>
                <ListItemIcon>
                  <HandshakeIcon
                    color="inverted"
                    sx={{ fontSize: "2em", mr: "0.5em" }}
                  />
                </ListItemIcon>
                <BigListItemText primary="My Collabs" />
              </ListItemButton>
            </ListItem>

            <ListItem disablePadding>
              <ListItemButton color="primary" href="/" fullWidth>
                <ListItemIcon>
                  <HomeIcon
                    color="inverted"
                    sx={{ fontSize: "2em", mr: "0.5em" }}
                  />
                </ListItemIcon>
                <BigListItemText primary="Home" />
              </ListItemButton>
            </ListItem>

            <ListItem disablePadding>
              <ListItemButton color="primary" href="/" fullWidth>
                <ListItemIcon>
                  <LoginIcon
                    color="inverted"
                    sx={{ fontSize: "2em", mr: "0.5em" }}
                  />
                </ListItemIcon>
                <BigListItemText primary="Login" />
              </ListItemButton>
            </ListItem>
            <ListItem disablePadding>
              <ListItemButton color="primary" href="/" fullWidth>
                <ListItemIcon>
                  <QueryStatsIcon
                    color="inverted"
                    sx={{ fontSize: "2em", mr: "0.5em" }}
                  />
                </ListItemIcon>
                <BigListItemText primary="Explorer" />
              </ListItemButton>
            </ListItem>

            {explorerItems.map((i) => (
              <ListItem disablePadding>
                <ListItemButton
                  sx={{ ml: "2em" }}
                  color="primary"
                  href={i.href}
                  fullWidth
                  id={`${i.id}-mobile`}
                >
                  <ListItemIcon sx={{ minWidth: 0 }}>
                    <ArrowRightIcon color="inverted" sx={{ fontSize: "2em" }} />
                  </ListItemIcon>
                  <BigListItemText primary={translate(i.text)} />
                </ListItemButton>
              </ListItem>
            ))}
            <ListItem disablePadding>
              <ListItemButton color="primary" href="/" fullWidth>
                <ListItemIcon>
                  <XIcon
                    color="inverted"
                    sx={{ fontSize: "2em", mr: "0.5em" }}
                  />
                </ListItemIcon>
                <BigListItemText primary="asami_club" />
              </ListItemButton>
            </ListItem>

            <ListItem disablePadding>
              <ListItemButton color="primary" href="/" fullWidth>
                <ListItemIcon>
                  <HelpOutlineIcon
                    color="inverted"
                    sx={{ fontSize: "2em", mr: "0.5em" }}
                  />
                </ListItemIcon>
                <BigListItemText primary="Help" />
              </ListItemButton>
            </ListItem>

            <ListItem disablePadding>
              <ListItemButton color="primary" href="/" fullWidth>
                <ListItemIcon>
                  <LogoutIcon
                    color="inverted"
                    sx={{ fontSize: "2em", mr: "0.5em" }}
                  />
                </ListItemIcon>
                <BigListItemText primary="Logout" />
              </ListItemButton>
            </ListItem>
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
      sx={{ minWidth: 0, padding: "0 5px" }}
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
    }}
  />
);
