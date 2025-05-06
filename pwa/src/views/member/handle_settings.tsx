import {
  useTranslate,
  ReferenceArrayField,
  SingleFieldList,
  FunctionField,
  SimpleShowLayout,
} from "react-admin";
import { DeckCard } from "../layout";
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
  Link,
  ListItemIcon,
} from "@mui/material";

import CheckCircleIcon from "@mui/icons-material/CheckCircle";
import PollIcon from "@mui/icons-material/Poll";
import CampaignIcon from "@mui/icons-material/Campaign";
import LockResetIcon from "@mui/icons-material/LockReset";
import HelpOutlineIcon from "@mui/icons-material/HelpOutline";

import { Head2 } from "../../components/theme";
import XIcon from "@mui/icons-material/X";
import { makeXAuthorize } from "../../lib/auth_provider";
import { useState } from "react";

export const HandleSettings = ({ handles }) => {
  const translate = useTranslate();

  return (
    <Box>
      <DeckCard id={`configure-x-handle-card`}>
        <CardContent>
          <Stack direction="row" gap="1em" mb="1em">
            <XIcon />
            <Head2>{translate("handle_settings.title")}</Head2>
          </Stack>
          <HandleSettingsContent handles={handles} />
        </CardContent>
      </DeckCard>
    </Box>
  );
};

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

  if (handle.needsRefreshToken) {
    if (handle.status == "UNVERIFIED") {
      return <GrantPermissionsAndMakePost />;
    } else {
      return <GrantPermissionsAgain />;
    }
  }

  if (handle.status == "ACTIVE") {
    return <HandleStats handle={handle} id={`existing-x-handle-stats`} />;
  }

  return <HandleSubmissionInProgress handle={handle} />;
};

export const HandleStats = ({ handle, id }) => {
  const translate = useTranslate();

  return (
    <Box id={id}>
      <SimpleShowLayout record={handle} sx={{ p: 0, mt: 1 }}>
        <FunctionField
          label={translate("handle_settings.stats.username")}
          render={(x) => (
            <>
              {x.username}{" "}
              <Typography
                variant="span"
                sx={{ fontSize: "0.8em", lineHeight: "1em" }}
              >
                [{x.userId}]
              </Typography>
            </>
          )}
        />
        <FunctionField
          label={translate("handle_settings.stats.score")}
          render={(h) => `${BigInt(h.score)} 力`}
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
      </SimpleShowLayout>
    </Box>
  );
};

const HandleInactive = ({ handle }) => {
  const translate = useTranslate();

  return (
    <Box id={`handle-x-inactive`}>
      <Typography variant="body2">
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
        <Typography variant="body2">
          {translate(`handle_settings.x.in_progress.summary`, {
            username: handle.username,
          })}
        </Typography>
      </Box>
      <Dialog open={open} onClose={handleClose} fullWidth maxWidth="xs">
        <DialogTitle>
          {translate("handle_settings.x.in_progress.dialog_title", {
            username: handle.username,
          })}
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
            Got it
          </Button>
        </DialogActions>
      </Dialog>
    </>
  );
};

const GrantPermissionsAndMakePost = () => {
  const [open, setOpen] = useState(true);
  const translate = useTranslate();

  const startXLogin = async () => {
    const { url, verifier } = await makeXAuthorize();
    localStorage.setItem("grantAccessOauthVerifier", verifier);
    document.location.href = url;
  };

  return (
    <>
      <Box>
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
        fullWidth={true}
        onClose={(_event, reason) => {
          if (reason === "backdropClick" || reason === "escapeKeyDown") {
            // Do nothing
            return;
          }
        }}
        slotProps={{ backdrop: { sx: { background: "rgba(0,0,0,0.9)" } } }}
      >
        <DialogTitle>
          {translate(
            "handle_settings.x.grant_permissions_and_make_posts.dialog_title",
          )}
        </DialogTitle>
        <DialogContent>
          <List disablePadding>
            <ListItem>
              <ListItemIcon>
                <CheckCircleIcon />
              </ListItemIcon>
              <ListItemText
                primary={translate(
                  "handle_settings.x.grant_permissions_and_make_posts.we_will_check_your_activity",
                )}
              />
            </ListItem>
            <ListItem>
              <ListItemIcon>
                <PollIcon />
              </ListItemIcon>
              <ListItemText
                primary={translate(
                  "handle_settings.x.grant_permissions_and_make_posts.we_will_post_a_poll",
                )}
              />
            </ListItem>
            <ListItem>
              <ListItemIcon>
                <CampaignIcon />
              </ListItemIcon>
              <ListItemText
                primary={translate(
                  "handle_settings.x.grant_permissions_and_make_posts.you_will_earn_rewards",
                )}
              />
            </ListItem>
            <ListItem>
              <ListItemIcon>
                <LockResetIcon />
              </ListItemIcon>
              <ListItemText
                primary={translate(
                  "handle_settings.x.grant_permissions_and_make_posts.you_can_revoke",
                )}
              />
            </ListItem>
            <ListItem>
              <ListItemIcon>
                <HelpOutlineIcon />
              </ListItemIcon>
              <ListItemText
                primary={
                  <span
                    dangerouslySetInnerHTML={{
                      __html: translate(
                        "handle_settings.x.grant_permissions_and_make_posts.help_text",
                      ),
                    }}
                  />
                }
              />
            </ListItem>
          </List>{" "}
        </DialogContent>
        <DialogActions>
          <Button
            id="button-cancel-grant-permission-and-make-post"
            fullWidth
            onClick={() => setOpen(false)}
          >
            I'll do this later
          </Button>
          <Button
            id="button-grant-permission-and-make-post"
            fullWidth
            variant="contained"
            onClick={startXLogin}
          >
            Let's Go
          </Button>
        </DialogActions>
      </Dialog>
    </>
  );
};

const GrantPermissionsAgain = () => {
  const [open, setOpen] = useState(true);
  const handleClose = () => setOpen(false);
  const translate = useTranslate();

  const startXLogin = async () => {
    const { url, verifier } = await makeXAuthorize();
    localStorage.setItem("grantAccessOauthVerifier", verifier);
    document.location.href = url;
  };

  return (
    <>
      <Box id="grant-x-permission-again">
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

      <Dialog open={open} onClose={handleClose} fullWidth maxWidth="xs">
        <DialogTitle>
          <Box display="flex" alignItems="baseline" gap={1}>
            <Typography variant="h6" component="span">
              {translate(
                "handle_settings.x.grant_permissions_again.dialog_title",
              )}
            </Typography>
          </Box>
        </DialogTitle>
        <DialogContent sx={{ pt: 1 }}>
          <Box display="flex" flexDirection="column" gap={2} p={2}>
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
    </>
  );
};
