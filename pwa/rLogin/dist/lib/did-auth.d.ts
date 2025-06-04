import { EIP1193Provider } from './provider';
import { SD } from './sdr';
export type AuthKeys = {
    refreshToken: string;
    accessToken: string;
};
export declare const requestSignup: (backendUrl: string, did: string) => Promise<{
    challenge: any;
    sdr: {
        credentials: any;
        claims: any;
    };
} | {
    challenge: any;
    sdr?: undefined;
}>;
export declare const confirmAuth: (provider: EIP1193Provider, address: string, backendUrl: string, did: string, challenge: string, onConnect: (provider: any, authKeys: AuthKeys) => Promise<void>, sd?: SD) => Promise<void>;
//# sourceMappingURL=did-auth.d.ts.map