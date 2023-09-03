import { useSafeSetState, useTranslate } from "react-admin";
import LoadingButton from '@mui/lab/LoadingButton';
import { Alert, Box, Button, Card, CardActions, CardContent, Container, FormControl, FormHelperText, InputLabel, MenuItem, Select, TextField, Typography } from "@mui/material";
import { ethers, parseUnits, formatEther } from "ethers";
import schnorr from "bip-schnorr";
import { Buffer } from "buffer";
import Login from "./views/login";
import { useContracts } from "../../components/contracts_context";
import { Head1, Head2, BulletPoint, CardTitle } from '../../components/theme';
import LoginIcon from '@mui/icons-material/Login';
import _ from 'lodash';
import { LocalizationProvider } from '@mui/x-date-pickers';
import { DateField } from '@mui/x-date-pickers/DateField';
import { AdapterDayjs } from '@mui/x-date-pickers/AdapterDayjs'
import dayjs from 'dayjs';
import { nip19 } from 'nostr-tools'

const CampaignWizard = () => {
  /*
  const [state, setState] = useSafeSetState({
    step: "CampaignType",
    classic: true,
    nostrTerms: null,
    classicTerms: null,
  });
  */

  const [state, setState] = useSafeSetState({
    step: "SummaryAndPay",
    classic: false,
    startDate: 1693717674,
    nostrTerms: {
      requestedContent: "Post something nice",
      offers: [
        {
          rewardAmount: 10,
          rewardAddress: "0xF78A30A396738Ce1a7ea520F707477F8430b9D51",
          nostrNpub: "npub1sg6plzptd64u62a878hep2kev88swjh3tw00gjsfl8f237lmu63q0uf63m",
        }
      ]
    }
  });

  /*
  const [state, setState] = useSafeSetState({
    step: "SummaryAndPay",
    classic: true,
    startDate: 1693717674,
    classicTerms: {
      socialNetwork: 1,
      rulesText: "Post something nice",
      oracleFee: 5,
      oracleAddress: "0xF78A30A396738Ce1a7ea520F707477F8430b9D51",
      offers: [
        {
          rewardAmount: 10,
          rewardAddress: "0xF78A30A396738Ce1a7ea520F707477F8430b9D51",
          username: "nubis_bruno",
        }
      ]
    }
  });
  */

  const steps = {
    "CampaignType": 0,
    "Base": 1,
    "AddCollaborator": 2,
    "SummaryAndPay": 3,
    "AllDone": 4,
  };


  const { step, classic, nostrTerms, classicTerms } = state;
  const params = { state, setState };

  return <Container maxWidth="md">
    <Head1 sx={{my:3}}>Run a campaign!</Head1>

    { steps[step] > steps["CampaignType"] && <CampaignTypeDone {...params} /> }
    { steps[step] > steps["Base"] && (classic ? <ClassicBaseDone {...params} /> : <NostrBaseDone {...params} />) }
    { steps[step] >= steps["AddCollaborator"] && ( 
      nostrTerms?.offers.map((o, i) => <NostrAddCollaboratorDone {...params} position={i} key={`nostr-collaborator-done-${i}`} /> ) ||
      classicTerms?.offers.map((o, i) => <ClassicAddCollaboratorDone {...params} position={i} key={`classic-collaborator-done-${i}`} /> )
    )}
    { steps[step] > steps["SummaryAndPay"] && <SummaryAndPayDone /> }

    { step === "CampaignType" && <CampaignType {...params} /> }
    { step === "Base" && ( classic ? <ClassicBase {...params} /> : <NostrBase {...params} /> )}
    { step === "AddCollaborator" && (classic ? <ClassicAddCollaborator {...params} /> : <NostrAddCollaborator {...params} /> )}
    { step === "SummaryAndPay" && <SummaryAndPay {...params} /> }
    { step === "AllDone" && <AllDone {...params} /> }

  </Container>;
}

const CampaignType = ({ setState }) => {
  const next = (classic) => { 
    setState( (s) => ({...s, step: "Base", classic}) )
  }

  return <Box>
    <BulletPoint label="You'll pay collaborators to post something for at least 14 days." />
    <BulletPoint label="You'll lock up the funds upfront in Asami's smart contract." />
    <BulletPoint label="Asami makes sure the job gets done, or you get refunded." />
    <BulletPoint label={<>We have 2 kinds of campains, <strong>Classic</strong> and <strong>Nostr</strong>.</>} />
    <BulletPoint label="Classic campaigns run on any social network using an impartial third party as a witness." />
    <BulletPoint label="Nostr-only campaigns are fully decentralized and automated, but have a small audience." />
    <Alert severity="warning" sx={{my: "2em"}}>
      This is a tech preview, so there are some things you need to do before you start the campaign:
      <ol>
        <li>Contact the collaborators and make sure they want to participate and are OK with the reward.</li>
        <li>Get their social network handles (or nostr pubkeys).</li>
        <li>Get their RSK addresses so they can get paid.</li>
        <li>Buy "Dollar on Chain" DoC to pay collaborators.</li>
      </ol>
    </Alert>
    <Box display="flex" gap="1em" flexWrap="wrap">
      <Button onClick={() => next(false) } sx={{flexGrow: 1}} size="large" variant="contained">Run Nostr Campaign</Button>
      <Button onClick={() => next(true) } sx={{flexGrow: 1}} size="large" variant="contained">Run Classic Campaign</Button>
    </Box>
  </Box>
}

const CampaignTypeDone = ({ state, setState }) => {
  const onClick = () => setState( (s) => ({...s, step: "CampaignType" }) );
  return <Done>
    <BulletPoint noGutter label={ state.classic ? "Classic social network campaign." : "Nostr self sovereign campaign." }>
      { state.step != "AllDone" && <Button size="small" sx={{ ml: 1 }} variant="outlined" onClick={onClick} >Change</Button> }
    </BulletPoint>
  </Done>;
}

const NostrBase = ({ state, setState }) => {
  const [values, setValues] = useSafeSetState(() => ({
    startDate: state.startDate,
    requestedContent: state.nostrTerms?.requestedContent,
    isValid: true,
    isDirty: false,
  }));

  const updateValue = (field, value) => {
    const updated = {...values, [field]: value};
    setValues(validate(updated));
  }
    
  const validate = (vals) => {
    vals.requestedContentError = null;
    vals.startDateError = null;

    if (_.isEmpty(_.trim(vals.requestedContent))) {
      vals.requestedContentError = "You must specify the content to be posted on nostr.";
    }

    if (!vals.startDate || vals.startDate < dayjs().unix()) {
      vals.startDateError = "You must provide a future date as start time.";
    }

    vals.isValid = !vals.startDateError && !vals.requestedContentError;

    return {...vals};
  }

  const validateAndNext = () => {
    let validated = validate(values);
    if(!validated.isValid) {
      return setValues({...validated, isDirty: true});
    }

    state.nostrTerms = (state.nostrTerms || { offers: [] } );
    state.nostrTerms.requestedContent = values.requestedContent;
    state.startDate = values.startDate;
    state.step = "AddCollaborator";
    setState({...state})
  }

  return <Card sx={{mb: 2, textAlign: "justify"}}>
    <CardTitle text="Nostr message and start date" />
    <CardContent>
      <BulletPoint label="Collaborators will post your message on Nostr under their own name: Phrase it like they would." />
      <BulletPoint label="There are no length restrictions. You can use links." />
      <BulletPoint label="Nostr campains are 100% decentralized, disputes are handled by submitting proof to a smart contract." />
      <BulletPoint label="If they don't post the message within 12 hours after the campaign started, we'll submit a challenge." />
      <BulletPoint label="They will keep your message published for the duration of the campaign, which is 14 days." />
      <BulletPoint label="If the collaborator deletes their message before the campaign ends, we will report it." />
      <BulletPoint label="We'll make our best effort to monitor this campaign, but you can always challenge and report them yourself." />
      <BulletPoint label="Challenges and disputes have a cost associated to them." />
      <TextField
        label="Message content"
        value={values.requestedContent}
        onChange={(e) => updateValue("requestedContent", e.target.value) }
        error={ values.isDirty && !!values.requestedContentError }
        helperText={ values.isDirty && values.requestedContentError }
        multiline
        fullWidth
        minRows={5}
        maxRows={40}
      />
      <LocalizationProvider dateAdapter={AdapterDayjs} adapterLocale="es-ES">
        <DateField
          size="small"
          fullWidth
          onChange={(value) => { updateValue("startDate", value.unix())} }
          value={values.startDate && dayjs.unix(values.startDate)}
          label="Start Date"
          error={ values.isDirty && !!values.startDateError }
          helperText={ (values.isDirty && values.startDateError) || "Start date, in your local time." }
          sx={{flexGrow: 1}}
        />
      </LocalizationProvider>
      <Button onClick={validateAndNext} fullWidth sx={{mt: "1em"}} size="large" variant="contained">Next: Add collaborators.</Button>
    </CardContent>
  </Card>
}

const NostrBaseDone = ({state, setState}) => {
  const change = () => setState( (s) => ({...s, step: "Base" }) )
  
  const dateString = dayjs.unix(state.startDate).toString();

  return <Done>
    <BulletPoint noGutter label={`Campaign starts: ${dateString}`}>
      { state.step != "AllDone" &&
        <Button size="small" sx={{ ml: 1 }} variant="outlined" onClick={change}>Change</Button> 
      }
    </BulletPoint>
  </Done>;
}

const ClassicBase = ({state, setState}) => {
  const [values, setValues] = useSafeSetState(() => ({
    startDate: state.startDate,
    rulesText: state.nostrTerms?.requestedContent,
    oracleAddress: "0x82c1954901dbA125b3Af4e26526EB86Ebc020e7A",
    oracleFee: 5,
    isValid: true,
    isDirty: false,
  }));

  const updateValue = (field, value) => {
    const updated = {...values, [field]: value};
    setValues(validate(updated));
  }
    
  const validate = (vals) => {
    vals.startDateError = null;
    vals.socialNetworkError = null;
    vals.rulesTextError = null;
    vals.oracleAddressError = null;
    vals.oracleFeeError = null;


    if (!vals.startDate || vals.startDate < dayjs().unix()) {
      vals.startDateError = "You must provide a future date as start time.";
    }
    
    if (!vals.socialNetwork) {
      vals.socialNetworkError = "Select a social network for your campaign.";
    }

    if (_.isEmpty(_.trim(vals.rulesText))) {
      vals.rulesTextError = "You must describe your campaign instructions and rules.";
    }

    if(!vals.oracleFee || parseUnits(vals.oracleFee.toString(), 18) < 0 ) {
      vals.oracleFeeError = "You must assign a reward in DoC for the trusted third party.";
    }

    if(!vals.oracleAddress || !vals.oracleAddress.match(/^0x[a-fA-F0-9]{40}$/)){
      vals.oracleAddressError = "Enter a valid RSK address starting with 0x...";
    }

    vals.isValid = !vals.startDateError && !vals.socialNetworkError && !vals.rulesTextError && !vals.oracleAddressError && !vals.oracleFeeError;

    return {...vals};
  }

  const validateAndNext = () => {
    let validated = validate(values);
    if(!validated.isValid) {
      return setValues({...validated, isDirty: true});
    }

    state.startDate = values.startDate;
    state.step = "AddCollaborator";
    state.classicTerms = (state.classicTerms || { offers: [] } );
    state.classicTerms = {
      ...state.classicTerms,
      socialNetwork: validated.socialNetwork,
      rulesText: validated.rulesText,
      oracleFee: validated.oracleFee,
      oracleAddress: validated.oracleAddress,
    };
    setState({...state})
  }

  return <Card sx={{mb: 2, textAlign: "justify"}}>
    <CardTitle text="Collaborator instructions and start date" />
    <CardContent>
      <BulletPoint label="Give detailed instructions to collaborators about the message they must post. An example also helps." />
      <BulletPoint label="Keep in mind the rules of the social network you're running your campaign on." />
      <BulletPoint label="Classic campaigns use an impartial trusted third party to resolve disputes." />
      <BulletPoint label="If they don't post the message within 12 hours after the campaign started, we'll submit a challenge." />
      <BulletPoint label="They will keep your message published for the duration of the campaign, which is 14 days." />
      <BulletPoint label="If the collaborator deletes their message before the campaign ends, we will report it." />
      <BulletPoint label="We'll make our best effort to monitor this campaign, but you can always challenge and report them yourself." />
      <BulletPoint label="Challenges and disputes have a cost associated to them." />
      <FormControl
        fullWidth
        error={ values.isDirty && !!values.socialNetworkError }
      >
        <InputLabel id="social-network-label">Social Network</InputLabel>
        <Select
          labelId="social-network-label"
          label="Social Network"
          onChange={(e) => updateValue("socialNetwork", e.target.value)}
          value={values.socialNetwork || "" }
        >
          <MenuItem value={1}>Instagram</MenuItem>
          <MenuItem value={2}>Twitter</MenuItem>
          <MenuItem value={3}>Youtube</MenuItem>
          <MenuItem value={4}>Facebook</MenuItem>
          <MenuItem value={5}>TikTok</MenuItem>
        </Select>
        <FormHelperText>
          { (values.isDirty && values.socialNetworkError) || "Choose a social network" }
        </FormHelperText>
      </FormControl>
      <TextField
        label="Campaign instructions and rules"
        multiline
        fullWidth
        onChange={(e) => updateValue("rulesText", e.target.value)  }
        value={values.rulesText}
        error={values.isDirty && !!values.rulesTextError }
        helperText={ (values.isDirty && values.rulesTextError) || "Write your instructions as short and clear as possible." }
        minRows={5}
        maxRows={40}
      />
      <LocalizationProvider dateAdapter={AdapterDayjs} adapterLocale="es-ES">
        <DateField
          size="small"
          fullWidth
          onChange={(value) => { updateValue("startDate", value.unix())} }
          value={values.startDate && dayjs.unix(values.startDate)}
          label="Start Date"
          error={ values.isDirty && !!values.startDateError }
          helperText={ (values.isDirty && values.startDateError) || "Start date, in your local time." }
          sx={{flexGrow: 1}}
        />
      </LocalizationProvider>
      <Box mt="1em">
        You can use a different trusted third party. Make sure your trusted third party agrees before changing these.
      </Box>
      <TextField
        label="Trusted third party address"
        onChange={(e) => updateValue("oracleAddress", e.target.value) }
        value={ values.oracleAddress }
        error={ values.isDirty && !!values.oracleAddressError }
        fullWidth
        helperText={
          (values.isDirty && values.oracleAddressError) ||
          "The default oracle is Constata.eu"
        }
      />
      <TextField
        label="Trusted third party reward"
        value={values.oracleFee || ""}
        onChange={(e) => setValue(e, "oracleFee") }
        error={ values.isDirty && !!values.oracleFeeError }
        helperText={ (values.isDirty && values.oracleFeeError) }
        fullWidth
        type="number"
        InputLabelProps={{ shrink: true, }}
      />
      <Button onClick={validateAndNext} fullWidth sx={{mt: "1em"}} size="large" variant="contained">Next: Add collaborators.</Button>
    </CardContent>
  </Card>
}

const ClassicBaseDone = ({state, setState}) => {
  const change = () => setState( (s) => ({...s, step: "Base" }) )
  return <Done>
    <BulletPoint noGutter label="Campaign starts: 20/2/2024">
      { state.step != "AllDone" &&
        <Button size="small" sx={{ ml: 1 }} variant="outlined" onClick={change}>Change</Button> 
      }
    </BulletPoint>
  </Done>;
}

const NostrAddCollaborator = ({state, setState}) => {
  const [values, setValues] = useSafeSetState({
    isDirty: false,
    isValid: true,
  });

  const setValue = (e, field) => setValues((v) => ({...v, [field]: e.target.value }))

  const addCollaborator = () => {
    let validated = validate(values);
    if(!validated.isValid) {
      return setValues({...validated, isDirty: true});
    }

    state.nostrTerms = state.nostrTerms || { offers: [] };
    state.nostrTerms.offers.push(values);
    setValues({isDirty: false, isValid: true});
    setState({...state})
  }

  const validate = (vals) => {
    vals.rewardAmountError = null;
    vals.rewardAddressError = null;
    vals.nostrNpubError = null;

    if(!vals.rewardAmount || parseUnits(vals.rewardAmount.toString(), 18) < 0 ) {
      vals.rewardAmountError = "You must assign a reward to this collaborator.";
    }
    
    if(!vals.rewardAddress || !vals.rewardAddress.match(/^0x[a-fA-F0-9]{40}$/)){
      vals.rewardAddressError = "Enter a valid RSK address starting with 0x...";
    }

    try {
      nip19.decode(vals.nostrNpub)
    } catch {
      vals.nostrNpubError = "Use the nostr user's pubkey that starts with 'npub'";
    }

    vals.isValid = !vals.nostrNpubError && !vals.rewardAddressError && !vals.rewardAmountError;
  
    return {...vals};

  }

  const nextStep = () => setState( (s) => ({...s, step: "SummaryAndPay"}) )
  
  return <Card sx={{mb: 2, textAlign: "justify"}}>
    <CardTitle text="Add collaborator" />
    <CardContent>
      <BulletPoint label="Collaborators get paid in the DoC stablecoin." />
      <TextField
        value={values.rewardAmount || ""}
        onChange={(e) => setValue(e, "rewardAmount") }
        label="Reward Amount"
        error={ values.isDirty && !!values.rewardAmountError }
        helperText={ values.isDirty && values.rewardAmountError }
        fullWidth
        type="number"
        InputLabelProps={{ shrink: true, }}
      />
      <TextField
        value={values.rewardAddress || ""}
        onChange={(e) => setValue(e, "rewardAddress") }
        error={ values.isDirty && !!values.rewardAddressError }
        helperText={ values.isDirty && values.rewardAddressError }
        label="Their RSK address"
        fullWidth
        />
      <TextField
        value={values.nostrNpub || ""}
        onChange={(e) => setValue(e, "nostrNpub") }
        error={ values.isDirty && !!values.nostrNpubError }
        helperText={ values.isDirty && values.nostrNpubError }
        label="Their Nostr Pubkey"
        fullWidth
        />
      <Box mt="1em" display="flex" gap="1em" flexWrap="wrap">
        <Button onClick={() => addCollaborator() } sx={{flexGrow: 1}} size="large" variant="contained">Add this</Button>
        { state.nostrTerms?.offers.length > 0 && 
          <Button onClick={() => nextStep() } sx={{flexGrow: 1}} size="large" variant="contained">Continue</Button>
        }
      </Box>
    </CardContent>
  </Card>
}

const NostrAddCollaboratorDone = ({state, setState, position}) => {
  const onRemove = () => {
    state.nostrTerms.offers.splice(position, 1);
    state.step = "AddCollaborator";
    setState({...state})
  }
  const {rewardAmount, rewardAddress, nostrNpub } = state.nostrTerms.offers[position];
  const shortNpub = `${nostrNpub.substring(0,10)}…${nostrNpub.substring(58)}`;
  const shortRewardAddress = `${rewardAddress.substring(0,6)}…${rewardAddress.substring(38)}`;

  return <Done>
    <BulletPoint noGutter label={`Nostr user ${shortNpub} will get ${rewardAmount} DoC to address ${shortRewardAddress}`}>
      { state.step != "AllDone" &&
        <Button size="small" sx={{ ml: 1 }} variant="outlined" onClick={onRemove}>Remove</Button> 
      }
    </BulletPoint>
  </Done>;
}

const ClassicAddCollaborator = ({state, setState}) => {
  const [values, setValues] = useSafeSetState({
    isDirty: false,
    isValid: true,
  });

  const setValue = (e, field) => setValues((v) => ({...v, [field]: e.target.value }))

  const addCollaborator = () => {
    let validated = validate(values);
    if(!validated.isValid) {
      return setValues({...validated, isDirty: true});
    }

    state.classicTerms = state.classicTerms || { offers: [] };
    state.classicTerms.offers.push(values);
    setValues({isDirty: false, isValid: true});
    setState({...state})
  }

  const validate = (vals) => {
    vals.rewardAmountError = null;
    vals.rewardAddressError = null;
    vals.usernameError = null;

    if(!vals.rewardAmount || parseUnits(vals.rewardAmount.toString(), 18) < 0 ) {
      vals.rewardAmountError = "You must assign a reward to this collaborator.";
    }
    
    if(!vals.rewardAddress || !vals.rewardAddress.match(/^0x[a-fA-F0-9]{40}$/)){
      vals.rewardAddressError = "Enter a valid RSK address starting with 0x...";
    }

    if(_.isEmpty(_.trim(vals.username))) {
      vals.usernameError = "Enter the collaborator's username";
    }

    vals.isValid = !vals.usernameError && !vals.rewardAddressError && !vals.rewardAmountError;
  
    return {...vals};
  }

  const nextStep = () => setState( (s) => ({...s, step: "SummaryAndPay"}) )
  
  return <Card sx={{mb: 2, textAlign: "justify"}}>
    <CardTitle text="Add collaborator" />
    <CardContent>
      <BulletPoint label="Collaborators get paid in the DoC stablecoin." />
      <TextField
        value={values.rewardAmount || ""}
        onChange={(e) => setValue(e, "rewardAmount") }
        label="Reward Amount"
        error={ values.isDirty && !!values.rewardAmountError }
        helperText={ values.isDirty && values.rewardAmountError }
        fullWidth
        type="number"
        InputLabelProps={{ shrink: true, }}
      />
      <TextField
        value={values.rewardAddress || ""}
        onChange={(e) => setValue(e, "rewardAddress") }
        error={ values.isDirty && !!values.rewardAddressError }
        helperText={ values.isDirty && values.rewardAddressError }
        label="Their RSK address"
        fullWidth
        />
      <TextField
        value={values.username || ""}
        onChange={(e) => setValue(e, "username") }
        error={ values.isDirty && !!values.usernameError }
        helperText={ values.isDirty && values.usernameError }
        label="Their username on the social network"
        fullWidth
        />
      <Box mt="1em" display="flex" gap="1em" flexWrap="wrap">
        <Button onClick={() => addCollaborator() } sx={{flexGrow: 1}} size="large" variant="contained">Add this</Button>
        { state.classicTerms?.offers.length > 0 && 
          <Button onClick={() => nextStep() } sx={{flexGrow: 1}} size="large" variant="contained">Continue</Button>
        }
      </Box>
    </CardContent>
  </Card>
}

const ClassicAddCollaboratorDone = ({state, setState, position}) => {
  const onRemove = () => {
    state.classicTerms.offers.splice(position, 1);
    state.step = "AddCollaborator";
    setState({...state})
  }
  const {rewardAmount, rewardAddress, username } = state.classicTerms.offers[position];
  const shortRewardAddress = `${rewardAddress.substring(0,6)}…${rewardAddress.substring(38)}`;

  return <Done>
    <BulletPoint noGutter label={`User ${username} will get ${rewardAmount} DoC to address ${shortRewardAddress}`}>
      { state.step != "AllDone" &&
        <Button size="small" sx={{ ml: 1 }} variant="outlined" onClick={onRemove}>Remove</Button> 
      }
    </BulletPoint>
  </Done>;
}

const SummaryAndPay = ({state, setState}) => {
  const [loading, setLoading] = useSafeSetState(false); 
  const [error, setError] = useSafeSetState(null);

  const submitCampaign = async () => {
    setError(null);
    setLoading(true);
    
    try {
      if (state.classic) {
        await submitClassicCampaign();
      } else {
        await submitNostrCampaign();
      }
      setState({...state, step: "AllDone" })
    } catch(e) {
      setError(e.toString());
    }

    setLoading(false);
  }
  
  const submitNostrCampaign = async () => {
    const { asami, asamiAddress, doc, signer } = await useContracts();

    const offers = state.nostrTerms.offers.map( (o) => {
      const rewardAmount = parseUnits(o.rewardAmount.toString(), 18);
      const nostrHexPubkey = nip19.decode(o.nostrNpub).data;
      const point = schnorr.math.liftX(Buffer.from(nostrHexPubkey, 'hex'));
      const nostrAffineX = BigInt(point.affineX.toString());
      const nostrAffineY = BigInt(point.affineY.toString());
      return { rewardAddress: o.rewardAddress, rewardAmount, nostrHexPubkey, nostrAffineX, nostrAffineY };
    });
    
    
    const fees = await asami.calculateCampaignFees(offers.length);
    const campaignAmount = _.sumBy(offers, (o) => o.rewardAmount ) + fees;

    const approval = await doc.approve(asamiAddress, campaignAmount, signer);
    await approval.wait();

    const creation = await asami.connect(signer).nostrCreateCampaign(
      campaignAmount,
      state.startDate,
      state.nostrTerms.requestedContent,
      offers,
    );

    await creation.wait();
  }

  async function sha256(input) {
    const msgBuffer = new TextEncoder().encode(input);
    const hashBuffer = await crypto.subtle.digest('SHA-256', msgBuffer);

    const myArray = new Uint8Array(32);
    myArray.set(hashBuffer);
    return myArray;
  }

  const submitClassicCampaign = async () => {
    const { asami, asamiAddress, doc, signer } = await useContracts();
    const terms = state.classicTerms;
    const offers = terms.offers.map( (o) => {
      return { 
        rewardAddress: o.rewardAddress,
        username: o.username,
        rewardAmount: parseUnits(o.rewardAmount.toString(), 18),
      };
    });
    
    const fees = await asami.calculateCampaignFees(offers.length);
    const campaignAmount = _.sumBy(offers, (o) => o.rewardAmount ) + fees;
    const rulesHash = await sha256(terms.rulesText);

    const approval = await doc.approve(asamiAddress, campaignAmount, signer);
    await approval.wait();

    debugger;

    const creation = await asami.connect(signer).classicCreateCampaign({
      funding: campaignAmount,
      startDate: state.startDate,
      socialNetworkId: terms.socialNetwork,
      rulesUrl: "tbd",
      rulesHash,
      oracleAddress: terms.oracleAddress,
      oracleFee: parseUnits(terms.oracleFee.toString(), 18),
      newOffers: offers
    });

    await creation.wait();
  }

  return <Card sx={{mb: 2, textAlign: "justify"}}>
    <CardTitle text="Summary and payment" />
    <CardContent>
      <BulletPoint label="Your campaign comes out at 100 DoC." />
      <BulletPoint label="That's 90 for rewards and 10 for Asami." />
      <BulletPoint label="If any collaborator bails out, the reward will be refunded." />
    </CardContent>
    { error && <Alert severity="error">Unexpected error ocurred: {error}</Alert> }
    <CardActions>
      <LoadingButton loading={loading} onClick={submitCampaign} fullWidth loadingIndicator="Loading…" size="large" variant="contained">
        Pay and start campaign.
      </LoadingButton>
    </CardActions>
  </Card>
}

const SummaryAndPayDone = ({state, setState}) => {
  return <Done>
    <BulletPoint noGutter label="Payment received."></BulletPoint>
  </Done>;
}

const AllDone = ({state, setState}) => {
  return <Card sx={{mb: 2, textAlign: "justify"}}>
    <CardTitle text="Campaign is running" />
    <CardContent>
      <BulletPoint label="This campaign will start on ..." />
      <BulletPoint label="You can see the campaign creation recepit at: " />
      <BulletPoint label="Follow its progress in your dashboard." />
    </CardContent>
    <CardActions>
      <Button fullWidth size="large" variant="contained">Go to dashboard.</Button>
    </CardActions>
  </Card>
}

export const Done = ({children}: { children: JSX.Element } ) => 
  <Card sx={{mb: 2}}>
    <CardContent sx={{ py: "0.5em !important" }}>
      { children }
    </CardContent>
  </Card>;

export default CampaignWizard;
