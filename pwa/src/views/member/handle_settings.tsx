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
} from "@mui/material";

import { Head2 } from "../../components/theme";
import XIcon from "@mui/icons-material/X";
import { makeXAuthorize } from "../../lib/auth_provider";

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
        TODO: Your handle has been deactivated.
        {translate(`handle_settings.x.in_progress.text`, {
          username: handle.username,
        })}
      </Typography>
    </Box>
  );
};

const HandleSubmissionInProgress = ({ handle }) => {
  const translate = useTranslate();

  return (
    <Box id={`handle-x-submission-in-progress-message`}>
      <Typography variant="body2">
        {translate(`handle_settings.x.in_progress.text`, {
          username: handle.username,
        })}
      </Typography>
    </Box>
  );
};

const GrantPermissionsAndMakePost = () => {
  const startXLogin = async () => {
    const { url, verifier } = await makeXAuthorize();
    localStorage.setItem("grantAccessOauthVerifier", verifier);
    document.location.href = url;
  };

  return (
    <Box>
      <Typography>
        Link your account, we will create a poll on your behalf.
      </Typography>
      <Button
        id="button-grant-permission-and-make-post"
        fullWidth
        variant="contained"
        onClick={startXLogin}
      >
        Link account <XIcon sx={{ ml: "5px" }} />
      </Button>
    </Box>
  );
};

const GrantPermissionsAgain = () => {
  const startXLogin = async () => {
    const { url, verifier } = await makeXAuthorize();
    localStorage.setItem("grantAccessOauthVerifier", verifier);
    document.location.href = url;
  };

  return (
    <Box id="grant-x-permission-again">
      <Typography>
        We can't access your X account. We need your permission again, otherwise
        we won't be able to score your influence level and you won't be able to
        participate in campaigns.
      </Typography>
      <Button
        id="button-grant-x-permission-again"
        fullWidth
        variant="contained"
        onClick={startXLogin}
      >
        Link account <XIcon sx={{ ml: "5px" }} />
      </Button>
      ;
    </Box>
  );
};
