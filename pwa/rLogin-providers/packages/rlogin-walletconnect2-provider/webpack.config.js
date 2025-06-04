const path = require('path')
const webpack = require('webpack')

module.exports = {
  mode: 'production',
  entry: './src/index.ts',
  devtool: 'source-map',
  module: {
    rules: [
      {
        test: /\.ts$/,
        use: 'ts-loader',
        exclude: /node_modules/
      }
    ]
  },
  resolve: {
    extensions: ['.ts', '.js']
  },
  plugins: [
    new webpack.DefinePlugin({ process: { env: { DEBUG: false } } }),
    new webpack.optimize.LimitChunkCountPlugin({ maxChunks: 1 })
  ],
  output: {
    filename: 'bundle.js',
    path: path.resolve(__dirname, 'dist'),
    library: 'rLoginWalletConnect2Provider',
    libraryTarget: 'umd',
    umdNamedDefine: true,
    globalObject: 'this',
    clean: true
  }
}
