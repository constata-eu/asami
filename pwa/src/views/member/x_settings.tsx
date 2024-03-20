import { useSafeSetState, useTranslate} from "react-admin";
import { Alert, Box, Button, Typography} from "@mui/material";
import XIcon from '@mui/icons-material/X';
import Paper from '@mui/material/Paper';
import { getAuthKeys } from '../../lib/auth_provider';
import { HandleSettings } from "./handle_settings";

const XSettings = ({handles, handleRequests}) =>
  <HandleSettings
    handles={handles}
    handleRequests={handleRequests}
    icon={<XIcon/>}
    site="X"
    namespace="x"
    handleMinLength={4}
    handleMaxLength={15}
    verificationPost={ <MakeXVerificationPost /> }
  />

const MakeXVerificationPost = () => {
  const translate = useTranslate();
  const [clicked, setClicked] = useSafeSetState(false);

  let accountId = BigInt(getAuthKeys().session.accountId);
  let text = translate("handle_settings.x.make_verification_post.post_text", {accountId });
  let intent_url = `https://x.com/intent/tweet?text=${text}`;

  return <Box>
    <Typography variant="body2" mb="1em">
      { translate("handle_settings.x.make_verification_post.text") }
    </Typography>

    <Paper elevation={5} sx={{ my:"1em", p:"1em"}}> {text} </Paper>
    { clicked ?
      <Alert>{ translate("handle_settings.x.make_verification_post.done") }</Alert> :
      <Button fullWidth 
        onClick={() => setClicked(true)}
        variant="contained"
        href={intent_url}
        target="_blank" 
        rel="noopener noreferrer"
      >
        { translate("handle_settings.x.make_verification_post.button") }
      </Button> 
    }
  </Box>
}

export default XSettings;

