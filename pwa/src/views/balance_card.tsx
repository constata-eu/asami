import { useTranslate, useGetOne, useNotify } from "react-admin";
import { formatAddress } from '../lib/formatters';
import { Box, CardContent, Typography, Button, Divider } from "@mui/material";
import { formatEther } from "ethers";
import { Head2 } from '../components/theme';
import { DeckCard } from './layout';
import { FunctionField, SimpleShowLayout} from 'react-admin';
import { getAuthKeys } from '../lib/auth_provider';
import ClaimAccountButton from './claim_account';
import { useContracts } from "../components/contracts_context";
import WalletIcon from '@mui/icons-material/Wallet';
import NorthEastIcon from '@mui/icons-material/NorthEast';

const BalanceCard = () => {
  const {data, isLoading } = useGetOne(
    "Account",
    { id: getAuthKeys().session.accountId },
    { refetchInterval: 10000 }
  );

  let content;

  if (isLoading || !data) {
    content = <></>;
  } else if(data.status == "DONE") {
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
        <FunctionField label={ translate("balance_card.unclaimed.doc_label") }
          render={ record => `${formatEther(record.unclaimedDocRewards)} DOC` } />
        <FunctionField label={ translate("balance_card.unclaimed.asami_label") }
          render={ record => `${formatEther(record.unclaimedAsamiTokens)} ASAMI` }  />
        <FunctionField
          label={ translate("balance_card.account_id_label")}
          render={ record => `${BigInt(record.id)}` }
        />
      </SimpleShowLayout>
    </CardContent>
    <Divider />
    { account.status == null ? <NotRequested/> : <ProcessingRequest/> }
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
  return <>
    <CardContent>
      <Head2>{ translate("balance_card.title") }</Head2>
      <SimpleShowLayout record={account} sx={{ p: 0, my: 1}}>
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
        <FunctionField label={ translate("balance_card.claimed.address") }
          render={ record => formatAddress(record.addr) } />
        <FunctionField label={ translate("balance_card.account_id_label")} render={ record => `${BigInt(record.id)}` }  />
      </SimpleShowLayout>
    </CardContent>
    <Divider />
    <CardContent>
      <Typography id="account-summary-claim-done" variant="body2">
        { translate("balance_card.claimed.participate_in_governance") }
      </Typography>
    </CardContent>
  </>;
}

const BalanceWithAddButton = ({symbol, account}) => {
  const notify = useNotify();
  const translate = useTranslate();
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
      <Typography>${formatEther(balance)} {symbol}</Typography>
      <Button id={`add-to-wallet-${symbol}`} sx={{ padding: "0.1em" }} size="small" color="inverted" onClick={addToken}>
        <WalletIcon sx={{ fontSize: "20px"}}/>
        <NorthEastIcon sx={{ fontSize: "15px"}}/>
      </Button>
    </Box>}
  />;
}

export default BalanceCard;
