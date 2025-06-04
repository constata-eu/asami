import { IProviderUserOptions } from 'web3modal';
/**
 * Takes an array of Web3 Providers from web3modal and checks if the first item is
 * Metamask or Web3. If true, add additional checks to see if it matches the providers
 * above. If so, rewrite provider
 * @param providers an array of providers from web3Modal
 * @returns updated array of providers
 */
export declare const checkRLoginInjectedProviders: (providers: IProviderUserOptions[]) => IProviderUserOptions[];
//# sourceMappingURL=injectedProviders.d.ts.map