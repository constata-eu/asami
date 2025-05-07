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
} from "@mui/material";

import { useTranslate } from "react-admin";
import { useState } from "react";
import AsamiLogo from "../assets/logo.svg?react";
import MenuIcon from "@mui/icons-material/Menu";
import CloseIcon from "@mui/icons-material/Close";

export const ResponsiveAppBar = ({ items }) => {
  const translate = useTranslate();
  const [open, setOpen] = useState(false);
  const isMobile = useMediaQuery((theme: any) => theme.breakpoints.down("md"));

  return (
    <>
      <AppBar position="static" color="transparent" elevation={0}>
        <Toolbar
          disableGutters={true}
          sx={{ mt: "1em", mb: "3em", gap: "1em", alignItems: "flex-end" }}
        >
          <AsamiLogo margin="auto" width="250px" height="100%" />
          <Box sx={{ flexGrow: 1 }} />

          {isMobile ? (
            <IconButton
              color="inverted"
              edge="end"
              onClick={() => setOpen(true)}
            >
              <MenuIcon sx={{ fontSize: "1.5em" }} />
            </IconButton>
          ) : (
            <Box
              display="flex"
              flexWrap="nowrap"
              whiteSpace="nowrap"
              gap="0.5em"
            >
              {items.map((i) => (
                <Button
                  key={i.id}
                  href={i.href}
                  variant={i.variant}
                  color="primary"
                  onClick={i.onClick}
                  target={i.target}
                  fullWidth
                  id={i.id}
                >
                  {translate(i.text, { _: i.text })}
                </Button>
              ))}
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
        <DialogContent>
          <Box display="flex" justifyContent="flex-end">
            <IconButton onClick={() => setOpen(false)}>
              <CloseIcon
                sx={{
                  fontSize: "3em",
                  color: (theme) => theme.palette.inverted.main,
                }}
              />
            </IconButton>
          </Box>

          <List>
            {items.map((i) => (
              <ListItem disablePadding>
                <ListItemButton
                  color="primary"
                  href={i.href}
                  onClick={i.onClick}
                  fullWidth
                  target={i.target}
                  id={`${i.id}-mobile`}
                >
                  <BigListItemText primary={translate(i.text, { _: i.text })} />
                </ListItemButton>
              </ListItem>
            ))}
          </List>
        </DialogContent>
      </Dialog>
    </>
  );
};

const BigListItemText = ({ primary }) => (
  <ListItemText
    primary={primary}
    primaryTypographyProps={{
      fontFamily: '"LeagueSpartanBlack", serif',
      fontSize: "3em",
      textTransform: "uppercase",
      fontWeight: "bold",
    }}
  />
);
