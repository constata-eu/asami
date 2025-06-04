export type Transaction = {
    to: string
    from?: string
    nonce?: number
    data?: string
    value?: string | number
    gasLimit?: number
    gasPrice?: number
}

export type EthSendTransactionParams = [transaction: Transaction]
export type PersonalSignParams = [data: string, account: string]
export type SignParams = [account: string, data: string]

export interface IRLoginEIP1193Provider {
    request(args: { method: 'net_version' }): Promise<string>
    request(args: { method: 'eth_chainId' }): Promise<string>

    request(args: { method: 'eth_accounts' }): Promise<string[]>
    request(args: { method: 'eth_requestAccounts' }): Promise<string[]>

    request(args: { method: 'eth_sendTransaction', params: EthSendTransactionParams }): Promise<string>

    request(args: { method: 'personal_sign', params: PersonalSignParams }): Promise<string>
    request(args: { method: 'eth_sign', params: SignParams }): Promise<string>

    request(args: { method: string, params?: any[] }): Promise<any>
    sendAsync(request: { method: string; params?: any;}, cb: any): void;

    on(method: string): void;
    removeAllListeners(): void;
}
