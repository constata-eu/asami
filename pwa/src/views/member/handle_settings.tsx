import {
  useTranslate,
  ReferenceArrayField,
  SingleFieldList,
  FunctionField,
  SimpleShowLayout,
  TextField,
  ResourceContextProvider,
  NumberField,
  useDataProvider,
  useNotify,
} from "react-admin";
import { DeckCard } from "../layout";
import { useNavigate } from "react-router-dom";
import {
  Button,
  Box,
  Chip,
  CardContent,
  Skeleton,
  Typography,
  Stack,
  Dialog,
  DialogTitle,
  DialogContent,
  DialogActions,
  List,
  ListItem,
  ListItemText,
  ListItemIcon,
  Card,
  Paper,
} from "@mui/material";

import CheckCircleIcon from "@mui/icons-material/CheckCircle";
import PollIcon from "@mui/icons-material/Poll";
import CampaignIcon from "@mui/icons-material/Campaign";
import LockResetIcon from "@mui/icons-material/LockReset";

import { BigText, Head2, Head3 } from "../../components/theme";
import XIcon from "@mui/icons-material/X";
import { makeXAuthorize } from "../../lib/auth_provider";
import { useEffect, useRef, useState } from "react";
import { AttributeTable } from "../../components/attribute_table";
import { AmountField } from "../../components/custom_fields";
import { useEmbedded } from "../../components/embedded_context";

export const HandleSettings = ({ handles }) => {
  const translate = useTranslate();

  return (
    <Card sx={{ mb: "1em", minWidth: "250px" }} id={`configure-x-handle-card`}>
      <CardContent
        sx={{ display: "flex", alignItems: "stretch", height: "100%" }}
      >
        <Stack alignItems="stretch" sx={{ width: "100%" }}>
          <HandleSettingsContent handles={handles} />
        </Stack>
      </CardContent>
    </Card>
  );
};

const HeadX = () => (
  <Head2 sx={{ textAlign: "center" }}>
    <XIcon
      sx={{
        color: "secondary.main",
        fontSize: 50,
      }}
    />
  </Head2>
);

const HandleSettingsContent = ({ handles }) => {
  const handle = handles.data?.[0];

  if (handles.isLoading) {
    return (
      <>
        <Skeleton />
        <Skeleton />
      </>
    );
  }

  if (!handle) {
    return <GrantPermissionsAndMakePost />;
  }

  if (handle.status == "INACTIVE") {
    return <HandleInactive handle={handle} />;
  }

  if (handle.status == "NEVER_CONNECTED") {
    return <GrantPermissionsAndMakePost />;
  }

  if (handle.status == "DISCONNECTED") {
    return <GrantPermissionsAgain />;
  }

  if (handle.status == "ACTIVE") {
    return <HandleStats handle={handle} id={`existing-x-handle-stats`} />;
  }

  if (handle.status == "RECONNECTING") {
    return <ReconnectingHandle handle={handle} />;
  }

  return <HandleSubmissionInProgress handle={handle} />;
};

export const HandleStats = ({ handle, id }) => {
  const translate = useTranslate();
  const navigate = useNavigate();

  return (
    <Box id={id} flexGrow={1} display="flex" flexDirection="column">
      <Typography
        color="secondary"
        fontSize="1em"
        letterSpacing="0em"
        margin="auto"
        padding="0"
        fontWeight="100"
        fontFamily="'LeagueSpartanLight'"
        lineHeight="0.5em"
        textTransform="uppercase"
      >
        {translate("account_show.score")}
      </Typography>
      <Stack direction="row" justifyContent="center" alignItems="center">
        <XIcon
          sx={{
            color: "secondary.main",
            fontSize: 50,
          }}
        />
        <BigText
          sx={{
            fontSize: "4em",
            lineHeight: "1em",
            color: "secondary.main",
            textAlign: "center",
          }}
        >
          {BigInt(handle.score)}
        </BigText>
      </Stack>
      <Typography
        margin="0 0 1em 0"
        fontSize="1.4em"
        fontFamily="'LeagueSpartanBold'"
        lineHeight="1.1em"
        letterSpacing="-0.05em"
        textAlign="center"
        color="secondary.main"
      >
        {handle.username}
      </Typography>

      <AttributeTable
        record={handle}
        fontSize="0.9em !important"
        resource="Handle"
      >
        <TextField source="userId" />
        <NumberField textAlign="right" source="totalCollabs" />
        <AmountField
          textAlign="right"
          currency=""
          source="totalCollabRewards"
        />
        <ReferenceArrayField
          label={translate("resources.Handle.fields.topic")}
          reference="Topic"
          source="topicIds"
        >
          <SingleFieldList empty={<>-</>} linkType={false}>
            <FunctionField
              render={(h) => (
                <Chip
                  size="small"
                  variant="outlined"
                  label={translate(`resources.Topic.names.${h.name}`)}
                />
              )}
            />
          </SingleFieldList>
        </ReferenceArrayField>
      </AttributeTable>
      <Button
        sx={{ mt: "1em" }}
        variant="outlined"
        fullWidth
        onClick={() => navigate(`/Account/${handle.accountId}/show`)}
      >
        {translate("handle_settings.see_full_profile")}
      </Button>
    </Box>
  );
};

const HandleInactive = ({ handle }) => {
  const translate = useTranslate();

  return (
    <Box id={`handle-x-inactive`}>
      <HeadX />
      <Typography>
        {translate(`handle_settings.x.account_deactivated.summary`, {
          username: handle.username,
        })}
      </Typography>
    </Box>
  );
};

const HandleSubmissionInProgress = ({ handle }) => {
  const [open, setOpen] = useState(true);
  const translate = useTranslate();
  const handleClose = () => setOpen(false);

  return (
    <>
      <Box id={`handle-x-submission-in-progress-message`}>
        <HeadX />
        <Typography>
          {translate(`handle_settings.x.in_progress.summary`, {
            username: handle.username,
          })}
        </Typography>
      </Box>
      <Dialog open={open} onClose={handleClose} fullWidth maxWidth="xs">
        <DialogTitle>
          <Head3 sx={{ color: (theme) => theme.palette.primary.main }}>
            {translate("handle_settings.x.in_progress.dialog_title", {
              username: handle.username,
            })}
          </Head3>
        </DialogTitle>
        <DialogContent sx={{ pt: 1 }}>
          <Typography variant="body1" gutterBottom>
            {translate("handle_settings.x.in_progress.may_take_up_to_an_hour")}
          </Typography>
          <Typography variant="body1">
            {translate("handle_settings.x.in_progress.check_the_poll")}
          </Typography>
        </DialogContent>
        <DialogActions>
          <Button
            variant="contained"
            color="primary"
            fullWidth
            onClick={handleClose}
          >
            {translate("handle_settings.x.in_progress.got_it")}
          </Button>
        </DialogActions>
      </Dialog>
    </>
  );
};

const ReconnectingHandle = ({ handle }) => {
  const [open, setOpen] = useState(true);
  const translate = useTranslate();
  const handleClose = () => setOpen(false);

  return (
    <>
      <Box id={`handle-x-reconnecting-message`}>
        <HeadX />
        <Typography>
          {translate(`handle_settings.x.reconnecting.summary`, {
            username: handle.username,
          })}
        </Typography>
      </Box>
      <Dialog open={open} onClose={handleClose} fullWidth maxWidth="xs">
        <DialogTitle>
          <Head3 sx={{ color: (theme) => theme.palette.primary.main }}>
            {translate("handle_settings.x.reconnecting.dialog_title", {
              username: handle.username,
            })}
          </Head3>
        </DialogTitle>
        <DialogContent sx={{ pt: 1 }}>
          <Typography variant="body1" gutterBottom>
            {translate("handle_settings.x.reconnecting.may_take_up_to_an_hour")}
          </Typography>
        </DialogContent>
        <DialogActions>
          <Button
            variant="contained"
            color="primary"
            fullWidth
            onClick={handleClose}
          >
            {translate("handle_settings.x.in_progress.got_it")}
          </Button>
        </DialogActions>
      </Dialog>
    </>
  );
};

/* 
  Si es embedded, se muestra una segunda página preguntando:
    * Continuar en Beexo, puede que necesites ingresar tu usuario y contraseña de X.

    * Utilizar otro dispositivo donde ya tengo sesión de X iniciada.
        * Necesitamos una landing diferente: Recibe un one time login, hace el login,
        y directamente redirige al authorization de X. 

    * Ya tenía una cuenta de asami, pero quiero usarla con esta wallet.

      * Creamos el pedido de vinculación. Visita este link para vincular tu cuenta
        "akakakak" a la wallet "abcdededl".
        Necesitarás iniciar sesióñ con tu vieja cuenta para aprobar la vinculación,
        recomendamos que lo abras en un ordenador.
        Abrirlo aquí.
        Abrirlo en otro dispositivo. (copiar, QR).

*/
const GrantPermissionsAndMakePost = () => {
  const [open, setOpen] = useState(true);
  const [step, setStep] = useState("PLAN");
  const translate = useTranslate();
  const isEmbedded = useEmbedded();

  const onCancel = () => {
    setStep("PLAN");
    setOpen(false);
  };

  const startXLogin = async () => {
    const { url, verifier } = await makeXAuthorize();
    localStorage.setItem("grantAccessOauthVerifier", verifier);
    document.location.href = url;
  };

  return (
    <>
      <Box>
        <HeadX />
        <Typography mb="1em">
          {translate(
            "handle_settings.x.grant_permissions_and_make_posts.summary",
          )}
        </Typography>
        <Button
          id="open-grant-permission-and-make-post-dialog"
          fullWidth
          variant="contained"
          onClick={() => setOpen(true)}
        >
          {translate(
            "handle_settings.x.grant_permissions_and_make_posts.get_started",
          )}
        </Button>
      </Box>
      <Dialog
        open={open}
        disableEscapeKeyDown
        fullWidth
        onClose={(_event, reason) => {
          if (reason === "backdropClick" || reason === "escapeKeyDown") {
            // Do nothing
            return;
          }
        }}
        maxWidth="sm"
      >
        {step == "PLAN" && (
          <GrantDialogPlanStep
            onOk={() => (isEmbedded ? setStep("SELECT_DEVICE") : startXLogin())}
            onCancel={onCancel}
          />
        )}
        {step == "SELECT_DEVICE" && (
          <GrantDialogSelectDeviceStep
            loginHere={startXLogin}
            loginElsewhere={() => setStep("LOGIN_ELSEWHERE")}
            maybeMergeAccount={() => setStep("MAYBE_MERGE_ACCOUNT")}
            onCancel={onCancel}
          />
        )}
        {step == "LOGIN_ELSEWHERE" && (
          <GrantDialogLoginElsewhereStep onCancel={onCancel} />
        )}
        {step == "MAYBE_MERGE_ACCOUNT" && (
          <MaybeMergeAccount onCancel={onCancel} />
        )}
      </Dialog>
    </>
  );
};

const GrantDialogPlanStep = ({ onOk, onCancel }) => {
  const translate = useTranslate();

  return (
    <>
      <DialogTitle>
        <Head3 sx={{ color: (theme) => theme.palette.primary.main }}>
          {translate(
            "handle_settings.x.grant_permissions_and_make_posts.dialog_title",
          )}
        </Head3>
      </DialogTitle>
      <DialogContent>
        <List disablePadding>
          <GrantDialogTextLine
            icon={<CheckCircleIcon />}
            primary={translate(
              "handle_settings.x.grant_permissions_and_make_posts.we_will_check_your_activity",
            )}
          />
          <GrantDialogTextLine
            icon={<PollIcon />}
            primary={translate(
              "handle_settings.x.grant_permissions_and_make_posts.we_will_post_a_poll",
            )}
          />
          <GrantDialogTextLine
            icon={<CampaignIcon />}
            primary={translate(
              "handle_settings.x.grant_permissions_and_make_posts.you_will_earn_rewards",
            )}
          />
          <GrantDialogTextLine
            icon={<LockResetIcon />}
            primary={translate(
              "handle_settings.x.grant_permissions_and_make_posts.you_can_revoke",
            )}
          />
        </List>{" "}
      </DialogContent>
      <DialogActions>
        <Button
          id="button-cancel-grant-permission-and-make-post"
          fullWidth
          onClick={onCancel}
        >
          {translate(
            "handle_settings.x.grant_permissions_and_make_posts.will_do_it_later",
          )}
        </Button>
        <Button
          id="button-grant-permission-and-make-post"
          fullWidth
          variant="contained"
          onClick={onOk}
        >
          {translate(
            "handle_settings.x.grant_permissions_and_make_posts.lets_go",
          )}
        </Button>
      </DialogActions>
    </>
  );
};

const GrantDialogSelectDeviceStep = ({
  loginHere,
  loginElsewhere,
  maybeMergeAccount,
  onCancel,
}) => {
  const translate = useTranslate();

  return (
    <>
      <DialogTitle>
        <Head3 sx={{ color: (theme) => theme.palette.primary.main }}>
          {translate("handle_settings.x.select_device.dialog_title")}
        </Head3>
      </DialogTitle>
      <DialogContent>
        <Stack gap="1em">
          <Stack gap="0.5em">
            <Typography variant="body2">
              You can login to X in Beexo, it should be OK unless you don't
              remember your password.
            </Typography>
            <Button fullWidth variant="contained" onClick={loginHere}>
              I'll login to X here
            </Button>
          </Stack>
          <Stack gap="0.5em">
            <Typography variant="body2">
              You can use an existing session on the X mobile app or on a
              different device. Also use this option if you use 2FA on X and
              doesn't work with Beexo.
            </Typography>
            <Button fullWidth variant="contained" onClick={loginElsewhere}>
              I'll use an X session elsewhere
            </Button>
          </Stack>

          <Stack gap="0.5em">
            <Typography variant="body2">
              Did you already have an asami account with your X account linked
              to it? You can move your existing ASAMI account into Beexo.
            </Typography>
            <Button fullWidth variant="contained" onClick={maybeMergeAccount}>
              Move my Asami account into Beexo
            </Button>
          </Stack>

          <Button fullWidth variant="outlined" onClick={onCancel}>
            I'll do this later
          </Button>
        </Stack>
      </DialogContent>
    </>
  );
};

const GrantDialogLoginElsewhereStep = ({ onCancel }) => {
  const notify = useNotify();
  const translate = useTranslate();
  const dataProvider = useDataProvider();
  const [token, setToken] = useState();
  const isRun = useRef(false);

  // useEffect para traerse un one time login.
  useEffect(() => {
    if (isRun.current) {
      return;
    }
    isRun.current = true;

    async function fetchData() {
      try {
        const res = await dataProvider.create("OneTimeToken", { data: {} });
        setToken(res.data.value);
      } catch (err) {
        notify("Could not create your token", {
          type: "error",
        });
      }
    }

    fetchData();
  }, []);

  if (!token) {
    return <></>;
  }

  return (
    <>
      <DialogTitle>
        <Head3 sx={{ color: (theme) => theme.palette.primary.main }}>
          {translate("handle_settings.x.login_elsewhere.dialog_title")}
        </Head3>
      </DialogTitle>
      <DialogContent>
        <Stack gap="1em">
          <Typography>
            {translate("handle_settings.x.login_elsewhere.description")}
          </Typography>
          <Typography
            fontFamily="monospace"
            sx={{ "word-break": "break-word" }}
          >
            {token}
          </Typography>
          <Button fullWidth variant="contained" onClick={onCancel}>
            Copy link
          </Button>

          <Button fullWidth variant="outlined" onClick={onCancel}>
            I changed my mind
          </Button>
        </Stack>
      </DialogContent>
    </>
  );
};

const MaybeMergeAccount = ({ onCancel }) => {
  const translate = useTranslate();
  const navigate = useNavigate();

  return (
    <>
      <DialogTitle>
        <Head3 sx={{ color: (theme) => theme.palette.primary.main }}>
          Was your old account connected to a wallet?
        </Head3>
      </DialogTitle>
      <DialogContent>
        <Stack gap="1em">
          <Typography variant="h5">
            No, I was ONLY using X or email to log-in.
          </Typography>
          <Typography>
            Just connect your wallet then. Go to your browser, login to Asami,
            and look for the option to 'Connect your wallet' in your dashboard.
            Select the 'Wallet connect' option and search for Beexo, it may be
            the only option available. "Wallet Connect", supported by Beexo,
            means you can use your beexo wallet in a variety of devices, you can
            use Asami *from within beexo* or in your web browser seamlessly.
          </Typography>

          <Typography variant="h5">
            Yes, I had a wallet connected already.
          </Typography>
          <Typography>
            If you already had an account with your wallet and X handle in it,
            we can merge it here, you'll lose access with your old wallet, and
            will need to approve the merge from your old account as well.
          </Typography>
          <Button
            fullWidth
            variant="contained"
            onClick={() => navigate("/merge-accounts")}
          >
            Merge with my old account
          </Button>

          <Button fullWidth variant="outlined" onClick={onCancel}>
            I changed my mind
          </Button>
        </Stack>
      </DialogContent>
    </>
  );
};

const GrantDialogTextLine = ({ primary, icon }) => (
  <ListItem>
    <ListItemIcon sx={{ color: "primary.main" }}>{icon}</ListItemIcon>
    <ListItemText primary={primary} />
  </ListItem>
);

const startXLogin = async () => {
  const { url, verifier } = await makeXAuthorize();
  localStorage.setItem("grantAccessOauthVerifier", verifier);
  document.location.href = url;
};

const GrantPermissionsAgain = () => {
  const translate = useTranslate();
  return (
    <>
      <Box id="grant-x-permission-again">
        <HeadX />
        <Typography mb="0.5em">
          {translate("handle_settings.x.grant_permissions_again.summary")}
        </Typography>
        <Button
          id="button-grant-x-permission-again-in-summary"
          fullWidth
          variant="contained"
          onClick={startXLogin}
        >
          {translate("handle_settings.x.grant_permissions_again.reconnect")}
        </Button>
      </Box>
      <GrantPermissionsAgainDialog />
    </>
  );
};

export const GrantPermissionsAgainDialog = () => {
  const [open, setOpen] = useState(true);
  const handleClose = () => setOpen(false);
  const translate = useTranslate();

  return (
    <Dialog
      id="grant-permission-again-dialog"
      open={open}
      onClose={handleClose}
      fullWidth
      maxWidth="xs"
    >
      <DialogTitle>
        <Head3 sx={{ color: (theme) => theme.palette.primary.main }}>
          {translate("handle_settings.x.grant_permissions_again.dialog_title")}
        </Head3>
      </DialogTitle>
      <DialogContent>
        <Box display="flex" flexDirection="column" gap={2}>
          <Typography variant="body1">
            {translate(
              "handle_settings.x.grant_permissions_again.cant_measure",
            )}
          </Typography>
          <Typography variant="body1">
            {translate(
              "handle_settings.x.grant_permissions_again.please_reconnect",
            )}
          </Typography>
        </Box>
      </DialogContent>
      <DialogActions>
        <Button
          id="button-cancel-grant-permissions-again"
          fullWidth
          onClick={handleClose}
        >
          {translate("handle_settings.x.grant_permissions_again.later")}
        </Button>
        <Button
          id="button-grant-x-permission-again"
          variant="contained"
          color="primary"
          fullWidth
          onClick={startXLogin}
        >
          {translate("handle_settings.x.grant_permissions_again.reconnect")}
        </Button>
      </DialogActions>
    </Dialog>
  );
};
