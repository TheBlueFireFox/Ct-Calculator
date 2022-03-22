const CopyWebpackPlugin = require("copy-webpack-plugin");
const TerserPlugin = require("terser-webpack-plugin");
const HtmlMinimizerPlugin = require("html-minimizer-webpack-plugin");
const MiniCssExtractPlugin = require("mini-css-extract-plugin");
const CssMinimizerPlugin = require("css-minimizer-webpack-plugin");
const {
        CleanWebpackPlugin
} = require('clean-webpack-plugin');
const path = require('path');

module.exports = {
        entry: "./bootstrap.js",
        output: {
                path: path.resolve(__dirname, "dist"),
                filename: "bootstrap.js",
        },
        experiments: {
                asyncWebAssembly: true,
        },
        mode: "development",
        plugins: [
                new CleanWebpackPlugin(),
                new MiniCssExtractPlugin(),
                new CopyWebpackPlugin({
                        patterns: ['index.html', 'style.css', 'favicon.png', 'apfelringe.png']
                }),
        ],
        module: {
                rules: [{
                        test: /.s?css$/,
                        use: [MiniCssExtractPlugin.loader, "css-loader", "sass-loader"],
                }],
        },
        optimization: {
                minimize: true,
                minimizer: [
                        new TerserPlugin(),
                        new HtmlMinimizerPlugin(),
                        new CssMinimizerPlugin(),
                ],
        },
};
