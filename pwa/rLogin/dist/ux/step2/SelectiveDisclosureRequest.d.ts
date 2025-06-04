import React from 'react';
import { SDR } from '../../lib/sdr';
interface SelectiveDisclosureRequestProps {
    sdr: SDR;
    backendUrl: string;
    onConfirm: () => void;
}
declare const RequestsList: ({ requests }: {
    requests: string[];
}) => React.JSX.Element;
declare const SelectiveDisclosureRequest: ({ sdr: { credentials, claims }, backendUrl, onConfirm }: SelectiveDisclosureRequestProps) => React.JSX.Element;
export { RequestsList, SelectiveDisclosureRequest };
//# sourceMappingURL=SelectiveDisclosureRequest.d.ts.map