import React from 'react';
import { AccountInterface } from './ChooseDPath';
interface Interface {
    account: AccountInterface;
    onClick: () => void;
    selected: boolean;
    balancePrefix?: string;
}
declare const AccountRow: React.FC<Interface>;
export default AccountRow;
//# sourceMappingURL=AccountRow.d.ts.map