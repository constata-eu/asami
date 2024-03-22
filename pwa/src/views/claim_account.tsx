import { useDataProvider, useNotify} from 'react-admin';
import { useContracts } from "../components/contracts_context";
import { Button } from "@mui/material";

const ClaimAccountButton = ({id, label, variant, color}) => {
  const notify = useNotify();
  const dataProvider = useDataProvider();
  const { signLoginMessage } = useContracts();

  const createClaimRequest = async () => {
    try {
      const signature = await signLoginMessage();
      await dataProvider.create("ClaimAccountRequest", { data: { input: { signature }}});
      notify("claim_account.success", { type: "success" })
    } catch(e) {
      notify("claim_account.error", { type: "error"})
    }
  }

  return (<Button fullWidth id={id} variant={variant} color={color} onClick={createClaimRequest}>
    {label}
  </Button>);
};

export default ClaimAccountButton;

