import { EthereumProvider } from '@walletconnect/ethereum-provider'
import { EthereumProviderOptions } from '@walletconnect/ethereum-provider/dist/types/EthereumProvider'

export class WalletConnect2Provider extends EthereumProvider {
  options: EthereumProviderOptions

  constructor (opts: EthereumProviderOptions) {
    super()
    this.options = opts
  }

  /**
   * When the user tries to connect, if the client has not been initialized, init it
   * This is to make it work with rLogin
   * @param opts
   * @return {Promise<void>}
   */
  async connect (opts) {
    if (!this.signer.client) {
      await this.initialize(this.options)
    }
    return super.connect(opts)
  }

  /**
   * In order to make it work with rLogin, this override had to be created
   * This because the EthereumProvider returns the response as it...
   * but in rLogin, we require the response as an object as: object.result
   * @param args
   * @param callback
   */
  sendAsync (args, callback) {
    return super.sendAsync(args, (error, response) => {
      const responseToSend = {
        result: undefined
      }
      if (response) {
        responseToSend.result = response
      }
      return callback(error, responseToSend)
    })
  }
}
