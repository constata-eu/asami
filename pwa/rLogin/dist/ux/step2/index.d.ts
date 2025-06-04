import React from 'react';
import { SDR, SD, Data } from '../../lib/sdr';
interface Step2Props {
    sdr: {
        credentials: string[];
        claims: string[];
    };
    fetchSelectiveDisclosureRequest: () => Promise<Data>;
    backendUrl: string;
    onConfirm: (data: SD) => void;
    providerName?: string;
}
declare const SelectiveDisclosure: ({ sdr, backendUrl, fetchSelectiveDisclosureRequest, onConfirm, providerName }: Step2Props) => React.JSX.Element;
export { SelectiveDisclosure, SDR, SD, Data };
//# sourceMappingURL=index.d.ts.map