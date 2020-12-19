const path = require("path");
const CopyPlugin = require("copy-webpack-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const MiniCssExtractPlugin = require("mini-css-extract-plugin");

const dist = path.resolve(__dirname, "dist");

const isProduction = process.env.NODE_ENV === "production";


module.exports = {
  mode: "production",
  entry: {
    index: "./src/index.tsx"
  },
  devtool: 'inline-source-map',
  output: {
    path: dist,
    filename: "[name].js"
  },
  devServer: {
    contentBase: dist,
  },
  resolve: {
    extensions: [ '.tsx', '.ts', '.js', '.wasm' ],
  },
  module: {
    rules: [
      {
        test: /\.tsx?$/,
        use: 'ts-loader',
        exclude: /node_modules/,
      },
      {
        test: /\.(glb|gltf|babylon)$/,
        use:[{
          loader: 'file-loader',
          options: { outputPath: 'assets/models/' }
        }]
      },
      {
        test: /\.(png)$/,
        use:[{
          loader: 'file-loader',
          options: { outputPath: 'assets/images/' }
        }]
      },
      {
        test: /\.less$/,
        use: [
          isProduction ? MiniCssExtractPlugin.loader : { loader: 'style-loader' },
          { loader: 'css-loader' },
          { loader: 'less-loader'},
        ]
      },      
    ],
  },
  plugins: [
    new CopyPlugin([
      path.resolve(__dirname, "static")
    ]),
    new WasmPackPlugin({
      crateDirectory: path.resolve(__dirname, "../", "wasm-business-module"),
      outDir: path.resolve(__dirname, "pkg")
    }),
    new MiniCssExtractPlugin({
      filename: "[name].css"
    }),
  ]
};
