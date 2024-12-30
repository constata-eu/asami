import { useTranslate, ReferenceArrayField, SingleFieldList } from "react-admin";
import { DeckCard } from '../layout';
import { Box, Chip, CardContent, Skeleton, Typography } from "@mui/material";
import { formatEther } from "ethers";
import { Head2 } from '../../components/theme';
import { SimpleForm, CreateBase, TextInput, SaveButton, useNotify } from 'react-admin';
import { FunctionField, SimpleShowLayout} from 'react-admin';
import { Stack } from '@mui/material';

export const HandleSettings = ({handles, site, namespace, handleMinLength, handleMaxLength, icon, verificationPost}) => {
  const translate = useTranslate();
  let content;
  const handle = handles.data?.filter((x) => x.site == site)[0];

  if (handles.isLoading){
    content = (<>
      <Skeleton />
      <Skeleton />
    </>);
  } else if (handle?.status == "ACTIVE") {
    content = <HandleStats handle={handle} id={`existing-${namespace}-handle-stats`} />;
  } else if (handle?.status == "UNVERIFIED") {
    content = verificationPost;
  } else if (handle?.status == "VERIFIED") {
    content = <HandleSubmissionInProgress handle={handle} namespace={namespace} />;
  } else {
		content = <CreateHandle
			onSave={handles.refetch}
			namespace={namespace}
			site={site}
			handleMinLength={handleMinLength}
			handleMaxLength={handleMaxLength}
		/>;
  }

  return (<Box>
    <DeckCard id={`configure-${namespace}-handle-card`}>
      <CardContent>
        <Stack direction="row" gap="1em" mb="1em">
          { icon }
          <Head2>{ translate("handle_settings.title") }</Head2>
        </Stack>
        { content }
      </CardContent>
    </DeckCard>
  </Box>);
}

export const HandleStats = ({handle, id}) => {
  const translate = useTranslate();

  return <Box id={id}>
    <SimpleShowLayout record={handle} sx={{ p: 0, mt: 1}}>
      <FunctionField label={ translate("handle_settings.stats.username")}
        render={ (x) => <>{x.username} <Typography variant="span" sx={{fontSize: "0.8em", lineHeight: "1em" }}>[{x.userId}]</Typography></> }
      />
      <FunctionField label={ translate("handle_settings.stats.score") } render={ h => `${BigInt(h.score)} 力` }  />
      <ReferenceArrayField label={ translate("resources.Handle.fields.topic")} reference="Topic" source="topicIds">
        <SingleFieldList empty={<>-</>} linkType={false}>
            <FunctionField render={ h => <Chip size="small" variant="outlined" label={translate(`resources.Topic.names.${h.name}`)} /> } />
        </SingleFieldList>
      </ReferenceArrayField>
    </SimpleShowLayout>
  </Box>;
}

export const CreateHandle = ({onSave, namespace, site, handleMinLength, handleMaxLength }) => {
  const translate = useTranslate();
  const notify = useNotify();

  const transformIt = async (values) => {
    return { input: values.handleRequestInput };
  }

  const onSuccess = () => {
    notify(`handle_settings.${namespace}.create_request.success`);
    onSave();
  }

  const validate = (values) => {
    let errors = {};
    let input = { site: site};

    if ( values.username.match(new RegExp(`^@?(\\w){${handleMinLength},${handleMaxLength}}$`) )) {
      input.username = values.username.replace("@","");
    } else {
      errors.username = translate(`handle_settings.${namespace}.create_request.username_error`);
    }

    values.handleRequestInput = input;
    return errors;
  }

  return <CreateBase resource="Handle" transform={transformIt} mutationOptions={{ onSuccess }} >
    <SimpleForm id={`${namespace}-handle-request-form`} sx={{ p: "0 !important", m: "0" }} sanitizeEmptyValues validate={validate} toolbar={false}>
      <Typography mb="1em" variant="body2">
        { translate(`handle_settings.${namespace}.create_request.text`) }
      </Typography>
      <TextInput sx={{mb: "1em" }} fullWidth required={true} size="large" variant="filled" source="username"
        id="username"
        label={ translate(`handle_settings.${namespace}.create_request.username_label`) }
        helperText={ translate(`handle_settings.${namespace}.create_request.username_help`) }
      />
      <SaveButton
        fullWidth
        id={`submit-${namespace}-handle-request-form`}
        label={ translate(`handle_settings.${namespace}.create_request.save`) }
        icon={<></>}
      />
    </SimpleForm>
  </CreateBase>;
}

const HandleSubmissionInProgress = ({handle, namespace}) => {
  const translate = useTranslate();
  
  return <Box id={`handle-${namespace}-submission-in-progress-message`}>
    <Typography variant="body2">
      { translate(`handle_settings.${namespace}.in_progress.text`, {username: handle.username}) }
    </Typography>
  </Box>;
}
