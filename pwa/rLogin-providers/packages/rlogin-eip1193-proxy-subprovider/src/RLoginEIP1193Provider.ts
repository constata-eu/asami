import HttpProvider from 'ethjs-provider-http'
import Eth from 'ethjs-query'
import { IRLoginEIP1193Provider, EthSendTransactionParams, PersonalSignParams, SignParams } from '@rsksmart/rlogin-eip1193-types'

export type RLoginEIP1193ProviderOptions = { rpcUrl: string, chainId: number }

class ProviderRpcError extends Error {
  code: number;
  data?: unknown;

  constructor (message: string, code: number, data?: unknown) {
    super(message)
    this.code = code
    this.data = data
  }
}

export abstract class RLoginEIP1193Provider implements IRLoginEIP1193Provider {
  protected selectedAddress?: string
  readonly chainId: number

  protected provider: typeof Eth

  constructor ({ rpcUrl, chainId }: RLoginEIP1193ProviderOptions) {
    if (!rpcUrl) throw new Error('rpcUrl is required')
    if (!chainId) throw new Error('chainId is required')

    this.chainId = chainId

    this.provider = new Eth(new HttpProvider(rpcUrl))
  }

  abstract ethSendTransaction (params: EthSendTransactionParams): Promise<string>;
  abstract personalSign (params: PersonalSignParams): Promise<string>;
  abstract ethSign (params: SignParams): Promise<string>;

  private validateSender (sender: string) {
    if (sender.toLowerCase() !== this.selectedAddress.toLowerCase()) throw new ProviderRpcError('The requested account has not been authorized by the user', 4100)
  }

  async request ({ method, params }): Promise<any> {
    console.log('🦄 incoming request:', method, params)

    switch (method) {
      case 'eth_accounts':
      case 'eth_requestAccounts':
        return [this.selectedAddress.toLowerCase()]

      case 'eth_chainId':
      case 'net_version':
        return `0x${this.chainId.toString(16)}`

      case 'eth_sign':
        this.validateSender(params[0])
        return this.ethSign(params)

      case 'personal_sign':
        this.validateSender(params[1])
        return this.personalSign(params)

      case 'eth_sendTransaction': {
        const { from } = (params as EthSendTransactionParams)[0]
        if (from) this.validateSender((params as EthSendTransactionParams)[0].from)
        else params[0].from = this.selectedAddress
        return this.ethSendTransaction(params)
      }

      default:
        return new Promise((resolve, reject) => {
          this.provider.rpc.sendAsync({ method, params }, (err, data) => {
            if (err) return reject(err)
            resolve(data)
          })
        })
    }
  }

  /**
   * Support deprecated sendAsync method, pass them to the request method
   * @param request sendAsync method
   * @param cb callback function that returns (error, success)
   */
  sendAsync (request: { method: string, params?: any }, cb: any) {
    this.request({ method: request.method, params: request.params })
      .then((response: any) => cb(null, { result: response }))
      .catch((error: Error) => cb(error, null))
  }

  on (method: string) {
    console.log('🦄 registering action ', method)
  }

  removeAllListeners () {
    console.log('🦄 removeAllListeners')
  }
}
