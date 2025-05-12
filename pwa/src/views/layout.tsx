import { Card, Typography, Box, Button, Container } from "@mui/material";
import CssBaseline from "@mui/material/CssBaseline";
import { useTranslate, CoreAdminContext, I18nContext } from "react-admin";
import { useEffect, useContext, useState } from "react";
import { publicDataProvider } from "../lib/data_provider";
import { ResponsiveAppBar } from "./responsive_app_bar";
import RootstockLogo from "../assets/rootstock.svg?react";
import ConstataLogo from "../assets/constata.svg?react";
import RootstockLabsLogo from "../assets/rootstock_labs.svg?react";
import RootstockCollectiveLogo from "../assets/rootstock_collective.svg?react";
import FpBlockLogo from "../assets/fpblock.svg?react";

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
      }}
    >
      {children}
    </Card>
  );
};

export const BareLayout = ({ sponsors = true, children }) => {
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
        {sponsors && (
          <Box
            display="flex"
            direction="row"
            gap="2em"
            flexWrap="wrap"
            my="2em"
          >
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
        )}
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
    <Box id="asami-explorer">
      <ResponsiveAppBar expandExplorer={true} />
      <CoreAdminContext
        i18nProvider={i18nProvider}
        dataProvider={pubDataProvider}
        authProvider={null}
      >
        {children}
      </CoreAdminContext>
    </Box>
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
