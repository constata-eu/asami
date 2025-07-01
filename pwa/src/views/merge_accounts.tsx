import { Alert, AlertTitle, Button, Stack, Typography } from "@mui/material";
import { useState } from "react";
import { useAuthenticated, useDataProvider, useGetOne } from "react-admin";
import { useParams } from "react-router-dom";
import { getAuthKeys } from "../lib/auth_provider";
import { useContracts } from "../components/contracts_context";

export const MergeAccounts = () => {
  const { isPending, isError } = useAuthenticated(); // redirects to login if not authenticated

  if (isPending || isError) {
    return <></>;
  }
  return <AccountMergeManager />;
};

const AccountMergeManager = (onClick) => {
  const [step, setStep] = useState("WELCOME");
  const [request, setRequest] = useState(null);
  const dataProvider = useDataProvider();

  // Attempt loading an existing merge request.
  // Set the current step if one is found.
  //useEffect();

  // createRequest()
  const createRequest = async () => {
    const response = await dataProvider.create("AccountMerge", { data: {} });
    setRequest(response.data);
    setStep("PENDING");
  };

  return (
    <>
      {step == "WELCOME" && <Welcome onClick={() => createRequest()} />}
      {step == "PENDING" && <Pending request={request} />}
      {step == "DESTINATION_SIGNED" && <DestinationSigned request={request} />}
    </>
  );
};

const Welcome = ({ onClick }) => {
  return (
    <Stack>
      <Typography>Ok, so you want to merge your account</Typography>
      <Typography>
        - So, you had an account and want to change your wallet, we'll help you
        merge both accounts. - Your main wallet is going to be this one, all
        future rewards will be claimed to the new one. - You won't be able to
        log-in to asami using your old wallet. - Any RBTC, ASAMI or DOC will
        remain on your old wallet, you'll have to move them to your new wallet
        if you want to. - The asami smart contract will remember you had another
        wallet. - Before we start: Make sure your old account has no pending DOC
        or ASAMI rewards, if it does, claim them first. - The process is like
        this: - We'll generate a special link here, which you'll have to visit
        from your existing account, which you may have on another device. -
        You'll have to approve the merge on your old account first. - Then
        you'll have to confirm the merge process again here.
      </Typography>
      <Button sx={{ mt: "1em" }} variant="outlined" fullWidth onClick={onClick}>
        Let's start merging.
      </Button>
    </Stack>
  );
};

const Pending = ({ request }) => {
  return (
    <>
      <Typography>
        In the device where your old account is, visit https://somelink and
        approve the merge request. You have 10 minutes to do this. But you can
        start over as many times as you need if this time expires. If you are
        using Rabby or Metamask you can click the address bar and enter the url
        there. Maybe show a QR here too ?
      </Typography>
      <Typography>{request.code}</Typography>
      <Button sx={{ mt: "1em" }} variant="outlined" fullWidth>
        I changed my mind, I'll merge later.
      </Button>
    </>
  );
};

const DestinationSigned = ({ request }) => {
  return (
    <>
      <Typography>
        Ok, you just approved your merge request with wallet ending in "XYZXYZ".
        Is that correct?
      </Typography>
      <Button sx={{ mt: "1em" }} variant="outlined" fullWidth>
        Yes, that's correct.
      </Button>
    </>
  );
};

export const SignMergeFromDestination = () => {
  const { isPending, isError } = useAuthenticated(); // redirects to login if not authenticated

  if (isPending || isError) {
    return <></>;
  }
  return <SignMergeFromDestinationInner />;
};

const SignMergeFromDestinationInner = () => {
  const navigate = useNavigate();
  const { code } = useParams<{ code: string }>();
  const { data, isLoading } = useGetOne(
    "Account",
    { id: getAuthKeys().session.accountId },
    { refetchInterval: 10000 },
  );
  const { signLoginMessage } = useContracts();

  const signAcceptance = async () => {
    const code = await signLoginMessage(false);
  };

  if (isLoading || !data) {
    return <></>;
  }

  if (!data.addr) {
    return (
      <Typography>
        This destination account has no wallet associated to it. Maybe you need
        to log-out and log back in with the right account.
      </Typography>
    );
  }

  const hasPending =
    BigInt(data.unclaimedDocBalance) > BigInt(0) ||
    BigInt(data.unclaimedAsamiBalance) > BigInt(0);

  return (
    <>
      <Typography>STEP 2: ACCEPT MERGING YOUR ACCOUNT</Typography>

      <Typography>
        This process only changes this old wallet to a new one, while keeping
        your X account linked, as well as your score and campaign and
        collaboration history. It does not move funds from the old wallet, if
        you want to move your RBTC, DOC or ASAMI you'll have to send the funds
        from the old wallet to the new one yourself.
      </Typography>
      {hasPending && (
        <Typography>
          You have unclaimed ASAMI and DOC. We suggest you claim them before you
          continue merging your accounts.
        </Typography>
      )}

      <Typography>
        You will not be able to log-in to asami using your wallet wallet
        anymore.
      </Typography>

      <Alert severity="error">
        <AlertTitle>Don't sign if you did not start this process.</AlertTitle>
        You won't be able to access your asami account using wallet {
          data.addr
        }{" "}
        anymore. If you didn't start this process, or someone shared this link
        with you, they may be trying to steal your Asami account.
      </Alert>

      <Button
        sx={{ mt: "1em" }}
        variant="outlined"
        fullWidth
        onClick={signAcceptance}
      >
        Sign merge acceptance.
      </Button>
      <Button
        sx={{ mt: "1em" }}
        variant="outlined"
        fullWidth
        onClick={navigate("/")}
      >
        I'll do this later.
      </Button>
    </>
  );
};
