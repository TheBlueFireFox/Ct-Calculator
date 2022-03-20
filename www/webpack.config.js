const CopyWebpackPlugin = require("copy-webpack-plugin");
const { CleanWebpackPlugin } = require('clean-webpack-plugin');
const path = require('path');

module.exports = {
  entry: "./bootstrap.js",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "bootstrap.js",
  },
  experiments : { asyncWebAssembly: true, },
  mode: "development",
  plugins: [
    new CleanWebpackPlugin(),
    new CopyWebpackPlugin({
            patterns: ['index.html', 'style.css', 'favicon.png', 'apfelringe.png']
    }), 
  ],
};
