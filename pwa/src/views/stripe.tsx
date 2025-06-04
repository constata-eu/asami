import { Box, Button, Container } from "@mui/material";
import { useTranslate } from "react-admin";
import { Head2, Head3, Lead } from "../components/theme";
import { BareLayout } from "./layout";
import AsamiLogo from "../assets/logo.svg?react";

export const StripeSuccess = () => <Base textScope="success" />;

export const StripeCancel = () => <Base textScope="cancel" />;

const Base = ({ textScope }) => {
  const t = useTranslate();

  return (
    <BareLayout sponsors={false}>
      <Container maxWidth="sm">
        <Box display="flex" flexDirection="column" my="3em" minHeight="50vh">
          <Head2 sx={{ mb: "1em", color: "primary.main" }}>
            {t(`stripe_landings.${textScope}.title`)}
          </Head2>
          <Lead sx={{ mb: "1em" }}>
            {t(`stripe_landings.${textScope}.text`)}
          </Lead>
          <Lead sx={{ mb: "2em" }}>{t(`stripe_landings.you_can_close`)}</Lead>
          <AsamiLogo width="150px" height="auto" />
        </Box>
      </Container>
    </BareLayout>
  );
};
