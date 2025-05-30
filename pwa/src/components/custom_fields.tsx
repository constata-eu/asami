import { formatEther, formatUnits } from "ethers";
import { FunctionField, useRecordContext, useTranslate } from "react-admin";
import React from "react";
import { Button, Box, Typography, IconButton, Tooltip } from "@mui/material";
import ContentCopyIcon from "@mui/icons-material/ContentCopy";

export const AmountField = ({ source, label, currency }) => {
  const record = useRecordContext();
  if (!record?.[source]) return null;
  return (
    <FunctionField
      label={label}
      render={(record) =>
        `${truncateEther(record[source])} ${currency !== undefined ? currency : "DOC"}`
      }
    />
  );
};

function truncateEther(wei: BigNumberish, decimals = 4): string {
  const str = formatEther(wei);
  const dotIndex = str.indexOf(".");
  if (dotIndex === -1 || decimals === 0) return str;
  return str.slice(0, dotIndex + decimals + 1);
}

export const BigNumField = ({ source, label }) => {
  const record = useRecordContext();
  if (!record?.[source]) return null;
  return (
    <FunctionField
      label={label}
      render={(record) => `${formatUnits(record[source], 0)}`}
    />
  );
};

export const AmountInput = ({ source }) => (
  <NumberInput
    source={source}
    size="small"
    parse={(x) => (x ? toBeHex(parseEther(x.toString()), 32) : null)}
    format={(x) => (x ? formatEther(x) : "")}
  />
);

interface AddressFieldProps {
  source: string;
}

export const AddressField: React.FC<AddressFieldProps> = ({ source }) => {
  const t = useTranslate();
  const record = useRecordContext();
  const address = record[source];

  if (!address) {
    return "-";
  }

  const truncated = `${address.slice(0, 4)}…${address.slice(-5)}`;

  const handleCopy = async () => {
    try {
      await navigator.clipboard.writeText(address);
    } catch (err) {
      console.error(t("address_field.cannot_copy"), err);
    }
  };

  return (
    <Box display="flex" alignItems="center">
      <Tooltip title={t("address_field.copy")}>
        <Button size="small" onClick={handleCopy}>
          <Typography sx={{ fontSize: "0.9em" }}>{truncated}</Typography>
        </Button>
      </Tooltip>
    </Box>
  );
};
