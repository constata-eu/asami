import TrezorConnect, { EthereumTransaction, Unsuccessful } from '@trezor/connect-web'
import { Transaction } from '@ethereumjs/tx'
import { RLoginEIP1193Provider, RLoginEIP1193ProviderOptions } from '@rsksmart/rlogin-eip1193-proxy-subprovider'
import { EthSendTransactionParams, SignParams, PersonalSignParams } from '@rsksmart/rlogin-eip1193-types'
import { getDPathByChainId } from '@rsksmart/rlogin-dpath'
import { createTransaction } from '@rsksmart/rlogin-transactions'

type TrezorOptions = {
  manifestEmail: string
  manifestAppUrl: string
}

export type TrezorProviderOptions = RLoginEIP1193ProviderOptions & TrezorOptions & {
  debug?: boolean
  dPath?: string
}

const convertToHex = (v: string | number) => typeof v === 'string' && v.slice(0, 2) === '0x' ? v : `0x${v.toString(16)}`

export class TrezorProvider extends RLoginEIP1193Provider {
  public readonly isTrezor = true

  path: string

  opts: TrezorOptions
  initialized = false
  connected = false

  debug = false

  constructor ({
    rpcUrl, chainId,
    debug, dPath,
    manifestEmail, manifestAppUrl
  }: TrezorProviderOptions) {
    super({ rpcUrl, chainId })

    this.debug = !!debug

    this.path = dPath || getDPathByChainId(chainId)
    this.opts = { manifestEmail, manifestAppUrl }
  }

  #logger = (...params: any) => this.debug && console.log(...params)

  /**
   * Attempt to parse an UNKNOWN_ERROR returned from Trezor.
   *
   * @param err Error Object
   * @param reject Reject from the parent's promise
   * @returns returns the rejected promise
   */
  #handleTrezorError = (message: string, code?: string): string => {
    this.#logger('🦄 try to interpret the error: ', { message, code })
    return code ? `Trezor: ${code} - ${message}` : message
  }

  #validateIsConnected () {
    if (!this.initialized || !this.connected) throw new Error('You need to connect the device first')
  }

  async connect (): Promise<any> {
    if (!this.initialized) {
      this.#logger('🦄 attempting to initialize!')
      try {
        await TrezorConnect.init({
          lazyLoad: true, // this param will prevent iframe injection until TrezorConnect.method will be called
          manifest: {
            email: this.opts.manifestEmail,
            appUrl: this.opts.manifestAppUrl,
            appName: "AsamiClub"
          }
        })
        this.initialized = true
      } catch (e) {
        if (e.message === 'TrezorConnect has been already initialized') {
          return this.chooseAccount(this.path)
        }

        throw new Error(this.#handleTrezorError(e.message))
      }
    }

    return this.chooseAccount(this.path)
  }

  async chooseAccount (dpath: string): Promise<RLoginEIP1193Provider> {
    this.#logger('🦄 attempting to connect!')
    const result = await TrezorConnect.ethereumGetAddress({ path: dpath, showOnTrezor: false })
    if (result.success) {
      this.connected = true
      this.selectedAddress = result.payload.address.toLowerCase()
      this.path = dpath
    } else {
      const unsuccessfulResponse = result as Unsuccessful
      throw new Error(this.#handleTrezorError(unsuccessfulResponse.payload.error, unsuccessfulResponse.payload.code))
    }
    return this
  }

  async getAddresses (dPaths: string[]): Promise<{path: string, address:string}[]> {
    const bundle = dPaths.map((path) => ({
      path,
      showOnTrezor: false
    }))

    return TrezorConnect.ethereumGetAddress({ bundle })
      .then((results) => results.payload)
      .then((accounts: any) => accounts.map((account: any) => ({
        dPath: account.serializedPath,
        address: account.address
      })))
  }

  private async validateConnectionAndSign (message: string): Promise<string> {
    this.#validateIsConnected()

    const result = await TrezorConnect.ethereumSignMessage({ path: this.path, message, hex: true })
    if (result.success) {
      return `0x${result.payload.signature}`
    } else {
      const unsuccessfulResponse = result as Unsuccessful
      throw new Error(this.#handleTrezorError(unsuccessfulResponse.payload.error, unsuccessfulResponse.payload.code))
    }
  }

  personalSign (params: PersonalSignParams): Promise<string> {
    return this.validateConnectionAndSign(params[0])
  }

  ethSign (params: SignParams): Promise<string> {
    return this.validateConnectionAndSign(params[1])
  }

  async ethSendTransaction (params: EthSendTransactionParams): Promise<string> {
    this.#validateIsConnected()

    const transaction = await createTransaction(this.provider, this.selectedAddress!, params[0])
    const tx = {
      ...transaction,
      nonce: convertToHex(transaction.nonce),
      gasPrice: convertToHex(transaction.gasPrice),
      gasLimit: convertToHex(transaction.gasLimit),
      chainId: this.chainId
    }

    this.#logger('🦄 sending tx request to device', tx)

    const result = await TrezorConnect.ethereumSignTransaction({
      path: this.path,
      transaction: tx as EthereumTransaction
    })

    if (result.success) {
      const signedTransaction = new Transaction({
        ...transaction,
        ...result.payload
      })
      return await this.provider.sendRawTransaction(`0x${signedTransaction.serialize().toString('hex')}`)
    } else {
      const unsuccessfulResponse = result as Unsuccessful
      throw new Error(this.#handleTrezorError(unsuccessfulResponse.payload.error, unsuccessfulResponse.payload.code))
    }
  }
}
