import React from 'react';
import { Data, DataField, SD } from '../../lib/sdr';
interface SelectiveDisclosureProps {
    data: Data;
    requestedData: {
        credentials: string[];
        claims: string[];
    };
    backendUrl: string;
    onConfirm: (sd: SD) => void;
    onRetry: () => void;
}
interface DataListProps {
    dataField: DataField;
    areCredentials: boolean;
    select: (key: string, value: string) => void;
}
declare const DataList: ({ dataField, areCredentials, select }: DataListProps) => React.JSX.Element;
declare const SelectiveDisclosureResponse: ({ data: { credentials, claims }, requestedData, backendUrl, onConfirm, onRetry }: SelectiveDisclosureProps) => React.JSX.Element;
export { DataList, SelectiveDisclosureResponse };
//# sourceMappingURL=SelectiveDisclosureResponse.d.ts.map