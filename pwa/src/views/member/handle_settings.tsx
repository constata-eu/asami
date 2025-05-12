import {
  useTranslate,
  ReferenceArrayField,
  SingleFieldList,
  FunctionField,
  SimpleShowLayout,
  TextField,
  ResourceContextProvider,
  NumberField,
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
  Link,
  ListItemIcon,
  Card,
} from "@mui/material";

import CheckCircleIcon from "@mui/icons-material/CheckCircle";
import PollIcon from "@mui/icons-material/Poll";
import CampaignIcon from "@mui/icons-material/Campaign";
import LockResetIcon from "@mui/icons-material/LockReset";
import HelpOutlineIcon from "@mui/icons-material/HelpOutline";

import { BigText, Head2, Head3 } from "../../components/theme";
import XIcon from "@mui/icons-material/X";
import { makeXAuthorize } from "../../lib/auth_provider";
import { useState } from "react";
import { AttributeTable } from "../../components/attribute_table";
import { AmountField } from "../../components/custom_fields";

export const HandleSettings = ({ handles }) => {
  const translate = useTranslate();

  return (
    <Card
      sx={{ mb: "1em", minWidth: "250px", breakAfter: "column" }}
      id={`configure-x-handle-card`}
    >
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

const GrantDialogTextLine = ({ primary, icon }) => (
  <ListItem>
    <ListItemIcon sx={{ color: "primary.main" }}>{icon}</ListItemIcon>
    <ListItemText primary={primary} />
  </ListItem>
);

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

      <Dialog open={open} onClose={handleClose} fullWidth maxWidth="xs">
        <DialogTitle>
          <Head3 sx={{ color: (theme) => theme.palette.primary.main }}>
            {translate(
              "handle_settings.x.grant_permissions_again.dialog_title",
            )}
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
    </>
  );
};
