import {
  useTranslate,
  useGetOne,
  useNotify,
  useDataProvider,
} from "react-admin";
import { formatAddress } from "../lib/formatters";
import {
  Box,
  CardContent,
  Typography,
  Button,
  Divider,
  Card,
  Stack,
} from "@mui/material";
import { AmountField } from "../components/custom_fields";
import { formatEther, parseEther } from "ethers";
import { Head2, Head3, Lead } from "../components/theme";
import { getAuthKeys } from "../lib/auth_provider";
import ClaimAccountButton from "./claim_account";
import { useContracts } from "../components/contracts_context";
import WalletIcon from "@mui/icons-material/Wallet";
import NorthEastIcon from "@mui/icons-material/NorthEast";
import { AttributeTable } from "../components/attribute_table";
import HourglassTopIcon from "@mui/icons-material/HourglassTop";

const BalanceCard = () => {
  const { data, isLoading } = useGetOne(
    "Account",
    { id: getAuthKeys().session.accountId },
    { refetchInterval: 10000 },
  );

  let content;

  if (isLoading || !data) {
    content = <></>;
  } else if (data.status == "CLAIMED") {
    content = <Done account={data} />;
  } else {
    content = <Unclaimed account={data} />;
  }

  return (
    <Card sx={{ mb: "1em" }} id="balance-card">
      {content}
    </Card>
  );
};

const Unclaimed = ({ account }) => {
  const translate = useTranslate();
  return (
    <Stack height="100%">
      <CardContent sx={{ pb: 0 }}>
        <Head3 sx={{ mb: "1em" }}>
          {translate("balance_card.unclaimed.title")}
        </Head3>
        <AttributeTable
          record={account}
          fontSize="1em !important"
          resource="Account"
        >
          <AmountField source="unclaimedDocBalance" currency="" />
          <AmountField source="unclaimedAsamiBalance" currency="" />
        </AttributeTable>
      </CardContent>
      <CardContent sx={{ pt: 0 }}>
        {account.status == "MANAGED" && <NotRequested />}
        {account.status == "CLAIMING" && <ProcessingRequest />}
      </CardContent>
    </Stack>
  );
};

const NotRequested = () => {
  const translate = useTranslate();
  return (
    <>
      <Typography my="1em" id="account-summary-claim-none">
        {translate("balance_card.unclaimed.not_requested")}
      </Typography>
      <ClaimAccountButton
        id="balance-card-claim-account-button"
        color="secondary"
        variant="outlined"
        label={translate("balance_card.unclaimed.not_requested_button")}
      />
    </>
  );
};

const ProcessingRequest = () => {
  const translate = useTranslate();
  return (
    <Stack gap="1em" alignItems="center">
      <HourglassTopIcon sx={{ fontSize: "7em", color: "secondary.main" }} />
      <Typography id="account-summary-claim-pending">
        {translate("balance_card.unclaimed.pending")}
      </Typography>
    </Stack>
  );
};

const Done = ({ account }) => {
  const translate = useTranslate();

  const unclaimedAsami = BigInt(account.unclaimedAsamiBalance);
  const unclaimedDoc = BigInt(account.unclaimedDocBalance);
  const hasEnoughRbtc = BigInt(account.rbtcBalance) > 0.00001;
  const hasUnclaimed = unclaimedAsami > 0 || unclaimedDoc > 0;

  const claimRegular = hasUnclaimed && hasEnoughRbtc;
  const claimGasless = hasUnclaimed && unclaimedDoc >= parseEther("1");

  return (
    <>
      <CardContent
        sx={{ height: "100%", display: "flex", flexDirection: "column" }}
      >
        <Head2>{translate("balance_card.claimed.title")}</Head2>

        <Typography
          mb="1em"
          id="no-pending-balance-messsage"
          variant="caption"
          paragraph={true}
        >
          {translate("balance_card.claimed.no_pending_balance")}
        </Typography>
        <Box mb="0.5em">
          <AttributeTable
            fontSize="0.9em !important"
            record={account}
            resource="Account"
          >
            <AmountField source="unclaimedDocBalance" currency="" />
            <AmountField source="unclaimedAsamiBalance" currency="" />
          </AttributeTable>
        </Box>
        {claimRegular && <ClaimButton account={account} />}
        {claimGasless && (
          <Box mt="1em">
            <GaslessClaimButton disabled={account.allowsGasless} />
            <Typography
              mt="0.5em"
              mb="0"
              id="suggest-gasless"
              variant="caption"
              paragraph={true}
            >
              {translate("balance_card.claimed.suggest_gasless")}
            </Typography>
          </Box>
        )}
        {hasUnclaimed && !claimRegular && !claimGasless && (
          <Box mt="1em">
            <Button
              fullWidth
              id="disabled-claim-button"
              variant="contained"
              color="primary"
              disabled={true}
            >
              {translate("balance_card.claimed.claim_button_label")}
            </Button>
            <Typography
              mt="1em"
              id="cant-withdraw-message"
              variant="caption"
              paragraph={true}
            >
              {translate("balance_card.claimed.cant_withdraw")}
            </Typography>
          </Box>
        )}
        {!hasUnclaimed && (
          <Box mt="1em">
            <Button
              fullWidth
              id="disabled-claim-button"
              variant="contained"
              color="primary"
              disabled={true}
            >
              {translate("balance_card.claimed.claim_button_label")}
            </Button>
          </Box>
        )}
        <Box flexGrow={1} />
        <Head3 sx={{ mt: "1em", mb: "0.2em" }}>
          {translate("balance_card.claimed.on_your_wallet")}
        </Head3>
        {account.addr && (
          <Typography mb="0.5em">{formatAddress(account.addr)}</Typography>
        )}
        <AttributeTable
          fontSize="0.9em !important"
          record={account}
          resource="Account"
        >
          <Stack direction="row" source="docBalance">
            <AmountField source="docBalance" currency="" />
            <AddToWallet symbol="DOC" account={account} />
          </Stack>
          <Stack direction="row" source="asamiBalance">
            <AmountField source="asamiBalance" currency="" />
            <AddToWallet symbol="ASAMI" account={account} />
          </Stack>
        </AttributeTable>
      </CardContent>
    </>
  );
};

const AddToWallet = ({ symbol, account }) => {
  const notify = useNotify();
  const { contracts } = useContracts();

  const addToken = async () => {
    const { provider, docAddress, asamiAddress } = await contracts(
      account.addr,
    );
    const address = symbol == "DOC" ? docAddress : asamiAddress;

    try {
      await provider.request({
        method: "wallet_watchAsset",
        params: {
          type: "ERC20",
          options: {
            address: address,
            symbol: symbol,
            decimals: 18,
            // A string URL of the token logo.
            //image: tokenImage,
          },
        },
      });
      notify("balance_card.claimed.added", {
        type: "success",
        messageArgs: { symbol },
      });
    } catch (e) {
      notify("balance_card.claimed.not_added", {
        type: "error",
        messageArgs: { symbol },
      });
    }
  };

  return (
    <Button
      id={`add-to-wallet-${symbol}`}
      sx={{ padding: "0.1em", justifyContent: "flex-end" }}
      size="small"
      color="secondary"
      onClick={addToken}
    >
      <WalletIcon sx={{ fontSize: "20px" }} />
      <NorthEastIcon sx={{ fontSize: "15px" }} />
    </Button>
  );
};

const ClaimButton = ({ account }) => {
  const translate = useTranslate();
  const notify = useNotify();
  const { contracts } = useContracts();

  const onClick = async () => {
    const { asami } = await contracts(account.addr);
    await asami.claimBalances();
    notify("balance_card.claimed.claim_success", { type: "success" });
  };

  return (
    <Button
      fullWidth
      id="claim-balances-button"
      variant="contained"
      color="primary"
      onClick={onClick}
    >
      {translate("balance_card.claimed.claim_button_label")}
    </Button>
  );
};

const GaslessClaimButton = ({ disabled }) => {
  const translate = useTranslate();
  const dataProvider = useDataProvider();
  const notify = useNotify();

  const onClick = async () => {
    await dataProvider.create("GaslessAllowance", { data: {} });
    notify("balance_card.claimed.gasless_allowance_success", {
      type: "success",
    });
  };

  return (
    <Button
      fullWidth
      id="gasless-claim-button"
      variant="outlined"
      color="secondary"
      size="small"
      onClick={onClick}
      disabled={disabled}
    >
      {translate("balance_card.claimed.gasless_claim_button_label")}
    </Button>
  );
};

export default BalanceCard;
