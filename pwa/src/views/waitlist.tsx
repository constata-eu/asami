import { useContext, useEffect, useState } from "react";
import {
  CoreAdminContext,
  Form,
  I18nContext,
  Notification,
  SaveButton,
  TextInput,
  useDataProvider,
  useNotify,
  useRedirect,
  useTranslate,
  required,
} from "react-admin";
import { publicDataProvider } from "../lib/data_provider";
import { BareLayout } from "./layout";
import {
  Box,
  Card,
  CardContent,
  Container,
  Skeleton,
  Stack,
  Typography,
} from "@mui/material";
import { Head1, Lead } from "../components/theme";
import { ResponsiveAppBar } from "./responsive_app_bar";

const WaitlistFormCard = () => {
  const t = useTranslate();
  const notify = useNotify();
  const redirect = useRedirect();
  const dataProvider = useDataProvider();

  const onSubmit = async (values: Record<string, any>) => {
    try {
      await dataProvider.create("WaitlistApplication", {
        data: {
          name: values.name,
          email: values.email,
          link1: values.link1,
          link2: values.link2,
          link3: values.link3 || null,
          note: values.note,
        },
      });
      notify(t("waitlist.success"), { type: "success" });
      redirect("/login");
    } catch (error: any) {
      notify(error?.message || t("waitlist.error"), { type: "error" });
    }
  };

  return (
    <Container maxWidth="md">
      <Stack gap="1em" my="3em">
        <Head1>{t("waitlist.title")}</Head1>
        <Lead>{t("waitlist.text")}</Lead>
        <Card>
          <CardContent>
            <Form sanitizeEmptyValues onSubmit={onSubmit}>
              <Stack gap="0.75em">
                <TextInput
                  source="name"
                  label={t("waitlist.fields.name")}
                  validate={required()}
                  fullWidth
                />
                <TextInput
                  source="email"
                  label={t("waitlist.fields.email")}
                  type="email"
                  validate={required()}
                  fullWidth
                />
                <TextInput
                  source="link1"
                  label={t("waitlist.fields.link1")}
                  validate={required()}
                  fullWidth
                />
                <TextInput
                  source="link2"
                  label={t("waitlist.fields.link2")}
                  validate={required()}
                  fullWidth
                />
                <TextInput
                  source="link3"
                  label={t("waitlist.fields.link3")}
                  fullWidth
                />
                <TextInput
                  source="note"
                  label={t("waitlist.fields.note")}
                  validate={required()}
                  multiline
                  minRows={4}
                  fullWidth
                />
                <Box>
                  <SaveButton label={t("waitlist.submit")} />
                </Box>
              </Stack>
            </Form>
          </CardContent>
        </Card>
        <Typography variant="body2">{t("waitlist.review_note")}</Typography>
      </Stack>
    </Container>
  );
};

export default function WaitlistPage() {
  const [pubDataProvider, setPubDataProvider] = useState<any>(null);
  const i18nProvider = useContext(I18nContext);

  useEffect(() => {
    async function initApp() {
      const prov = await publicDataProvider();
      setPubDataProvider(prov);
    }
    initApp();
  }, []);

  if (!pubDataProvider) {
    return (
      <BareLayout sponsors={false}>
        <Container maxWidth="md">
          <Skeleton animation="wave" />
        </Container>
      </BareLayout>
    );
  }

  return (
    <BareLayout sponsors={false}>
      <ResponsiveAppBar />
      <CoreAdminContext
        i18nProvider={i18nProvider}
        dataProvider={pubDataProvider}
      >
        <Notification />
        <WaitlistFormCard />
      </CoreAdminContext>
    </BareLayout>
  );
}
