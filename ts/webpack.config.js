const path = require('path');

module.exports = {
    mode: "production",
    entry: './src/index.ts',
    output: {
        filename: 'index.js',
        path: path.resolve(__dirname, 'dist'),
        library: {
            name: "gfxSSL",
            type: "commonjs",
        },
        publicPath: '',
    },
    module: {
        rules: [
            {
                test: /\.ts$/,
                use: 'ts-loader',
                exclude: /node_modules/,
            }
        ]
    },
    resolve: {
        extensions: ['.ts', '.js', "json"],
        fallback: { assert: false, process: false, fs: false, util: false, path: false }
    }
};