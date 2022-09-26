import * as path from 'path';
import HtmlWebpackPlugin from 'html-webpack-plugin';

const OUTPUT_DIR = path.resolve('./dist');

export default {
  mode: 'development',
  entry: {
    examples: {
      import: './examples/index'
    },
  },
  output: {
    filename: '[name].min.js',
    path: OUTPUT_DIR,
  },
  module: {
    rules: [
      {
        test: /\.m?js$/,
        enforce: 'pre',
        use: 'source-map-loader',
      },
      {
        test: /\.(m?jsx?|tsx?)$/,
        use: {
          loader: 'babel-loader',
          options: {
            envName: 'webpack'
          }
        },
        exclude: /node_modules/,
      },
    ],
  },
  resolve: {
    extensions: [ '.js', '.mjs', '.ts', '.tsx' ]
  },
  plugins: [
    new HtmlWebpackPlugin()
  ],
  devtool: 'source-map',
};
