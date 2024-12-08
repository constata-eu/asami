import { formatEther, formatUnits } from "ethers";
import { FunctionField, useRecordContext } from 'react-admin';
  
export const AmountField = ({source, label}) => {
  const record = useRecordContext();
  if (!record?.[source]) return null;
  return <FunctionField label={label} render={record => `${formatEther(record[source])} DOC`} />
}

export const BigNumField = ({source, label}) => {
  const record = useRecordContext();
  if (!record?.[source]) return null;
  return <FunctionField label={label} render={record => `${formatUnits(record[source], 0)}`} />
}
