const webpack = require('webpack')

module.exports = env => ({
  mode: env.production ? 'production' : 'development',
  entry: "./src/index.ts",
  output: {
    libraryTarget: "umd",
    library: "RLogin",
    umdNamedDefine: true,
    globalObject: "this",
    filename: "index.js"
  },
  resolve: {
    extensions: [".ts", ".tsx", ".js"],
    fallback: {
      "assert": require.resolve("assert/"),
      "stream": require.resolve("stream-browserify"),
      "http": require.resolve('stream-http'),
      "https": require.resolve('https-browserify'),
      "zlib": require.resolve('browserify-zlib'),
    }
  },
  plugins: [
    new webpack.ProvidePlugin({
      process: 'process/browser',
      Buffer: ['buffer', 'Buffer']
    })
  ],
  devtool: "source-map",
  module: {
    rules: [
      { test: /\.tsx?$/, loader: "ts-loader" },
      {
        test: /\.(png|svg|jpg|gif)$/,
        use: [
          {
            loader: "url-loader",
            options: {
              limit: 65535,
              name: "static/media/[name].[hash:8].[ext]"
            }
          }
        ]
      }
    ]
  }
});
