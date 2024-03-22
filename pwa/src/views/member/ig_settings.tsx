import { useEffect } from "react";
import { useSafeSetState, useTranslate} from "react-admin";
import { Box, Button, Skeleton, Typography} from "@mui/material";
import { Settings } from '../../settings';
import Paper from '@mui/material/Paper';
import { useNotify } from 'react-admin';
import { getAuthKeys } from '../../lib/auth_provider';
import InstagramIcon from '@mui/icons-material/Instagram';
import { HandleSettings } from "./handle_settings";

const IgSettings = ({handles, handleRequests}) =>
  <HandleSettings
    handles={handles}
    handleRequests={handleRequests}
    icon={<InstagramIcon/>}
    site="INSTAGRAM"
    namespace="ig"
    handleMinLength={1}
    handleMaxLength={30}
    verificationPost={ <MakeIgVerificationPost /> }
  />

const MakeIgVerificationPost = () => {
  const translate = useTranslate();
  const notify = useNotify();
  const [config, setConfig] = useSafeSetState(null);
  let accountId = BigInt(getAuthKeys().session.accountId);

  useEffect(() => {
    const load = async () => {
      setConfig((await (await fetch(`${Settings.apiDomain}/config`)).json()));
    }
    load();
  }, []);

  if (!config) {
    return <>
      <Skeleton />
      <Skeleton />
    </>;
  }

  const caption = `${config.instagram_verification_caption} [${accountId}]`;

  const copyText = async () => {
    notify("handle_settings.ig.make_verification_post.caption_copied");
    await navigator.clipboard.writeText(caption);
  }

  return <Box>
    <Typography mb="0.5em" variant="body2">
      { translate("handle_settings.ig.make_verification_post.verify_username") }
      <strong> { translate("handle_settings.ig.make_verification_post.guidelines") } </strong>
    </Typography>
    <Box display="flex" flexDirection="column" gap="1em">
      <Box display="flex" borderRadius="10px" overflow="hidden">
        <img width="100%" src={config.instagram_verification_image_url} />
      </Box>
      <Paper elevation={5} sx={{p:"1em"}}><Typography>{ caption }</Typography></Paper>
      <Box display="flex" gap="1em">
        <Button fullWidth variant="contained" target="_blank" href={config.instagram_verification_image_url} rel="noopener noreferrer" download>
          { translate("handle_settings.ig.make_verification_post.get_image") }
        </Button> 
        <Button fullWidth onClick={() => copyText() } variant="contained" >
          { translate("handle_settings.ig.make_verification_post.copy_text") }
        </Button> 
      </Box>
    </Box>
  </Box>
}

export default IgSettings;

