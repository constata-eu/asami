import DCentRPCProvider from '@rsksmart/dcent-provider'
import { getDPathByChainId } from '@rsksmart/rlogin-dpath'
import { RLoginEIP1193Provider, RLoginEIP1193ProviderOptions } from '@rsksmart/rlogin-eip1193-proxy-subprovider'
import { EthSendTransactionParams, PersonalSignParams, Transaction } from '@rsksmart/rlogin-eip1193-types'
import { createTransaction } from '@rsksmart/rlogin-transactions'

export type DCentProviderOptions = RLoginEIP1193ProviderOptions & {
  dPath?: string
  debug?: boolean
}

export class DCentProvider extends RLoginEIP1193Provider {
  public readonly isDcent = true

  path: string
  rpcUrl: string

  #debug: boolean

  appEthInitialized = false
  appEthConnected = false
  enabled = false

  dcentProvider: typeof DCentRPCProvider

  constructor ({ rpcUrl, chainId, dPath, debug }: DCentProviderOptions) {
    super({ rpcUrl, chainId })

    this.#debug = debug
    this.path = dPath || getDPathByChainId(chainId)
    this.rpcUrl = rpcUrl

    this.dcentProvider = new DCentRPCProvider({
      rpcUrl: this.rpcUrl,
      chainId: this.chainId,
      dpath: this.path
    })
  }

  /**
   * Simple logger
   *
   * @param params any
   * @returns null
   */
  #logger = (...params: any) => this.#debug && console.log(...params)

  #validateIsConnected () {
    if (!this.enabled) throw new Error('You need to connect the device first')
  }

  /**
   * Connect to the Dcent physical device.
   *
   * @returns Dcent EIP1193 Provider Wrapper
   */
  async connect (): Promise<any> {
    const accounts = await this.dcentProvider.enable()
    this.selectedAddress = accounts[0]
    this.enabled = true
    return this
  }

  /**
   * Sign personal message with Dcent.
   *
   * @param message
   * @returns
   */
  async personalSign (params: PersonalSignParams): Promise<string> {
    this.#validateIsConnected()
    this.#logger('🦄 attempting to sign message!')
    const messageHex = Buffer.from(params[0]).toString('hex')
    return await this.dcentProvider.send(
      'personal_sign',
      [`0x${messageHex}`, params[1]]
    )
  }

  /**
   * Sign personal message with Dcent.
   *
   * @param message
   * @returns
   */
  async ethSign (params: PersonalSignParams): Promise<string> {
    this.#validateIsConnected()
    this.#logger('🦄 attempting to sign message!')
    const messageHex = params[1]
    return await this.dcentProvider.send(
      'eth_sign',
      [params[0], messageHex]
    )
  }

  /**
   * Create enable and send transaction using Dcent provider.
   *
   * @param to
   * @param value
   * @param data
   * @returns Tx object, signature include.
   */
  async ethSendTransaction (params: EthSendTransactionParams): Promise<string> {
    this.#validateIsConnected()

    const transaction: Transaction & { gas?: Transaction['gasLimit'] } = await createTransaction(this.provider, this.selectedAddress!, params[0])

    // Ref: https://github.com/DcentWallet/dcent-provider/blob/a0be498c41efb6657a299379a37eed07dbb5eee1/example/src/components/DemoFunction.vue#L119
    transaction.gas = transaction.gasLimit
    delete transaction.gasLimit

    this.#logger('🦄 sending tx request to device', transaction)

    return await this.dcentProvider.send('eth_sendTransaction', transaction)
  }
}
