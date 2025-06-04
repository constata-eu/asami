interface PeerMeta {
    description?: string | undefined | null;
    icons?: string[] | undefined | null;
    name?: string | undefined | null;
    url?: string | undefined | null;
}
interface PeerInfo {
    name: string;
    logo: string;
}
export declare function getPeerInfo(meta: PeerMeta | undefined | null): PeerInfo | null;
export {};
//# sourceMappingURL=getPeerLogo.d.ts.map