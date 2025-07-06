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
  Modal,
  CardActions,
  Container,
} from "@mui/material";

import CheckCircleIcon from "@mui/icons-material/CheckCircle";
import PollIcon from "@mui/icons-material/Poll";
import CampaignIcon from "@mui/icons-material/Campaign";
import LockResetIcon from "@mui/icons-material/LockReset";

import {
  backgroundGradientRules,
  BigText,
  Head2,
  Head3,
  paperBackground,
} from "../../components/theme";
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

const GrantPermissionsAndMakePost = () => {
  const [open, setOpen] = useState(true);
  const [step, setStep] = useState("PLAN");
  const translate = useTranslate();
  const isEmbedded = useEmbedded();
  const navigate = useNavigate();

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
            mergeAccount={() => navigate("/merge-accounts")}
            onCancel={onCancel}
          />
        )}
        {step == "LOGIN_ELSEWHERE" && (
          <GrantDialogLoginElsewhereStep onCancel={onCancel} />
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
          {translate("handle_settings.x.will_do_it_later")}
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
  mergeAccount,
  onCancel,
}) => {
  const t = useTranslate();

  return (
    <WizModal title="handle_settings.x.select_device.dialog_title">
      <DeviceOption
        text="login_here_text"
        button="login_here_button"
        onClick={loginHere}
      />
      <DeviceOption
        text="other_device_text"
        button="other_device_button"
        onClick={loginElsewhere}
      />
      <DeviceOption
        text="merge_account_text"
        button="merge_account_button"
        onClick={mergeAccount}
      />
      <Button fullWidth variant="contained" color="inverted" onClick={onCancel}>
        {t("handle_settings.x.will_do_it_later")}
      </Button>
    </WizModal>
  );
};

const DeviceOption = ({ text, button, onClick }) => {
  const t = useTranslate();
  return (
    <Card>
      <CardContent>
        <Typography>{t(`handle_settings.x.select_device.${text}`)}</Typography>
      </CardContent>
      <CardActions>
        <Button
          id={`device-option-${button}`}
          fullWidth
          variant="contained"
          onClick={onClick}
        >
          {t(`handle_settings.x.select_device.${button}`)}
        </Button>
      </CardActions>
    </Card>
  );
};

const GrantDialogLoginElsewhereStep = ({ onCancel }) => {
  const notify = useNotify();
  const t = useTranslate();
  const dataProvider = useDataProvider();
  const [token, setToken] = useState<String>();
  const isRun = useRef(false);

  const handleCopy = async () => {
    try {
      await navigator.clipboard.writeText(token);
    } catch (err) {
      console.error("cannot_copy", err);
    }
  };

  // useEffect para traerse un one time login.
  useEffect(() => {
    if (isRun.current) {
      return;
    }
    isRun.current = true;

    async function fetchData() {
      try {
        const res = await dataProvider.create("OneTimeToken", { data: {} });
        setToken(`asami.club/#/s/${res.data.value}`);
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
    <WizModal title="handle_settings.x.login_elsewhere.dialog_title">
      <Card id="grant-elsewhere-container">
        <CardContent>
          <Typography mb="1em">
            {t("handle_settings.x.login_elsewhere.description")}
          </Typography>
          <Typography
            fontFamily="monospace"
            sx={{ "word-break": "break-word" }}
          >
            {token}
          </Typography>
        </CardContent>
        <CardActions>
          <Button fullWidth variant="contained" onClick={handleCopy}>
            {t("handle_settings.x.login_elsewhere.copy")}
          </Button>
        </CardActions>
      </Card>
      <Button fullWidth variant="contained" color="inverted" onClick={onCancel}>
        {t("handle_settings.x.will_do_it_later")}
      </Button>
    </WizModal>
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

const WizModal = ({ title, children }) => {
  const t = useTranslate();
  return (
    <Modal
      open={true}
      slotProps={{
        backdrop: {
          style: backgroundGradientRules(100),
        },
      }}
    >
      <Container maxWidth="md" sx={{ p: "1em", height: "100%" }}>
        <Stack gap="1em" height="100%">
          <Head3 sx={{ color: (theme) => theme.palette.primary.main }}>
            {t(title)}
          </Head3>
          {children}
        </Stack>
      </Container>
    </Modal>
  );
};
