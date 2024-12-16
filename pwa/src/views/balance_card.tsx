import { useTranslate, useGetOne, useNotify, useDataProvider} from "react-admin";
import { formatAddress } from '../lib/formatters';
import { Box, CardContent, Typography, Button, Divider } from "@mui/material";
import { formatEther, parseEther } from "ethers";
import { Head2 } from '../components/theme';
import { DeckCard } from './layout';
import { FunctionField, SimpleShowLayout} from 'react-admin';
import { getAuthKeys } from '../lib/auth_provider';
import ClaimAccountButton from './claim_account';
import { useContracts } from "../components/contracts_context";
import WalletIcon from '@mui/icons-material/Wallet';
import NorthEastIcon from '@mui/icons-material/NorthEast';

const truncateEther = (str) => Math.trunc(+(formatEther(str)) * 1e4) / 1e4

const BalanceCard = () => {
  const {data, isLoading } = useGetOne(
    "Account",
    { id: getAuthKeys().session.accountId },
    { refetchInterval: 10000 }
  );

  let content;

  if (isLoading || !data) {
    content = <></>;
  } else if(data.status == "CLAIMED") {
    content = <Done account={data} />;
  } else {
    content = <Unclaimed account={data} />;
  }
  
  return <DeckCard id="balance-card">{ content }</DeckCard>;
}

const Unclaimed = ({account}) => {
  const translate = useTranslate();
  return <>
    <CardContent>
      <Head2>{ translate("balance_card.title") }</Head2>
      <SimpleShowLayout record={account} sx={{ p: 0, my: 1}}>
        <FunctionField source="unclaimedDocBalance" label={ translate("balance_card.unclaimed.doc_label") }
          render={ record => `${truncateEther(record.unclaimedDocBalance)} DOC` } />
        <FunctionField source="unclaimedAsamiBalance" label={ translate("balance_card.unclaimed.asami_label") }
          render={ record => `${truncateEther(record.unclaimedAsamiBalance)} ASAMI` }  />
      </SimpleShowLayout>
    </CardContent>
    <Divider />
    { account.status == "MANAGED" && <NotRequested/> }
    { account.status == "CLAIMING" && <ProcessingRequest/> }
  </>;
}

const NotRequested = () => {
  const translate = useTranslate();
  return <>
    <CardContent>
      <Typography mb="1em" id="account-summary-claim-none" variant="body2">
        { translate("balance_card.unclaimed.not_requested") }
      </Typography>
      <ClaimAccountButton id="balance-card-claim-account-button" color="inverted" variant="outlined"
        label={ translate("balance_card.unclaimed.not_requested_button") }/>
    </CardContent>
  </>;
}

const ProcessingRequest = () => {
  const translate = useTranslate();
  return <CardContent>
    <Typography id="account-summary-claim-pending" variant="body2">
      { translate("balance_card.unclaimed.pending") }
    </Typography>
  </CardContent>;
}

const Done = ({account}) => {
  const translate = useTranslate();

	const unclaimedAsami = BigInt(account.unclaimedAsamiBalance);
	const unclaimedDoc = BigInt(account.unclaimedDocBalance);
	const hasEnoughRbtc = BigInt(account.rbtcBalance) > 0.00001;
	const hasUnclaimed = unclaimedAsami > 0 || unclaimedDoc > 0;

	const claimRegular = hasUnclaimed && hasEnoughRbtc;
	const claimGasless = hasUnclaimed && unclaimedDoc >= 1;

  return <>
    <CardContent>
      <Head2>{ translate("balance_card.title") }</Head2>
      <SimpleShowLayout record={account} sx={{ p: 0, my: 1}}>
        <FunctionField label={ translate("balance_card.claimed.address") }
          render={ record => formatAddress(record.addr) } />
        <BalanceWithAddButton
          label={ translate("balance_card.claimed.doc_label") }
          symbol="DOC"
          account={account}
        />
        <BalanceWithAddButton
          label={ translate("balance_card.claimed.asami_label") }
          symbol="ASAMI"
          account={account}
        />
        <FunctionField source="unclaimedDocBalance" label={ translate("balance_card.unclaimed.doc_label") }
          render={ record => `${truncateEther(record.unclaimedDocBalance)} DOC` } />
        <FunctionField source="unclaimedAsamiBalance" label={ translate("balance_card.unclaimed.asami_label") }
          render={ record => `${truncateEther(record.unclaimedAsamiBalance)} ASAMI` }  />
      </SimpleShowLayout>
			{ hasUnclaimed && hasEnoughRbtc && <ClaimButton account={account} /> }
			{ hasUnclaimed && unclaimedDoc > 1 && <Box mt="1em">
				<GaslessClaimButton disabled={account.allowsGasless} />
				<Typography mt="0.5em" mb="0" id="suggest-gasless" variant="caption" paragraph={true}>
					{ translate("balance_card.claimed.suggest_gasless") }
				</Typography>
			</Box> }
			{ hasUnclaimed && !hasEnoughRbtc && unclaimedDoc < 1 && <Box mt="1em">
				<Button fullWidth id="disabled-claim-button" variant="contained" color="primary" disabled={true}>
					{ translate("balance_card.claimed.claim_button_label") }
				</Button>
				<Typography mt="1em" id="cant-withdraw-message" variant="caption" paragraph={true}>
					{ translate("balance_card.claimed.cant_withdraw") }
				</Typography>
			</Box> }
			{ !hasUnclaimed && <Box mt="1em">
				<Button fullWidth id="disabled-claim-button" variant="contained" color="primary" disabled={true}>
					{ translate("balance_card.claimed.claim_button_label") }
				</Button>
				<Typography mt="1em" id="no-pending-balance-messsage" variant="caption" paragraph={true}>
					{ translate("balance_card.claimed.no_pending_balance") }
				</Typography>
			</Box> }
    </CardContent>
  </>;
}

const BalanceWithAddButton = ({symbol, account}) => {
  const notify = useNotify();
  const { contracts } = useContracts();

  const balance = symbol == "DOC" ? account.docBalance : account.asamiBalance;

  const addToken = async () => {
    const { provider, docAddress, asamiAddress } = await contracts(account.addr);
    const address = symbol == "DOC" ? docAddress : asamiAddress;

    try {
      await provider 
        .request({
          method: "wallet_watchAsset",
          params: {
            type: "ERC20",
            options: {
              address: address,
              symbol: symbol,
              decimals: 18,
              // A string URL of the token logo.
              //image: tokenImage,
            }
          }
        });
      notify("balance_card.claimed.added", { type: "success", messageArgs: {symbol}});
    } catch(e) {
      notify("balance_card.claimed.not_added", { type: "error", messageArgs: {symbol}});
    }
  }

  return <FunctionField
    render={ () => <Box display="flex" gap="1em" alignItems="center" justifyContent="space-between">
      <Typography>${truncateEther(balance)} {symbol}</Typography>
      <Button id={`add-to-wallet-${symbol}`} sx={{ padding: "0.1em" }} size="small" color="inverted" onClick={addToken}>
        <WalletIcon sx={{ fontSize: "20px"}}/>
        <NorthEastIcon sx={{ fontSize: "15px"}}/>
      </Button>
    </Box>}
  />;
}

const ClaimButton = ({account}) => {
  const translate = useTranslate();
  const notify = useNotify();
  const { contracts } = useContracts();

  const onClick = async () => {
      const { asami } = await contracts(account.addr);
      await asami.claimBalances();
			notify("balance_card.claimed.claim_success", { type: "success" })
  }

  return (<Button fullWidth id="claim-balances-button" variant="contained" color="primary" onClick={onClick}>
    { translate("balance_card.claimed.claim_button_label") }
  </Button>);
};

const GaslessClaimButton = ({disabled}) => {
  const translate = useTranslate();
  const dataProvider = useDataProvider();
  const notify = useNotify();

  const onClick = async () => {
		await dataProvider.create("GaslessAllowance", { data: {}});
    notify("balance_card.claimed.gasless_allowance_success", { type: "success" })
  }

  return (<Button fullWidth id="gasless-claim-button" variant="outlined" color="inverted" size="small" onClick={onClick} disabled={disabled}>
    { translate("balance_card.claimed.gasless_claim_button_label") }
  </Button>);
};

export default BalanceCard;
