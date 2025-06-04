import React from 'react';
interface ModalProps {
    lightboxOffset: number;
    show: boolean;
    onClose: () => void;
    setLightboxRef: (c: HTMLDivElement | null) => void;
    mainModalCard: HTMLDivElement | null | undefined;
    big: boolean;
}
export declare const Modal: React.FC<ModalProps>;
export {};
//# sourceMappingURL=Modal.d.ts.map