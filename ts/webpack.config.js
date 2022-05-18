const path = require('path');

module.exports = {
    mode: "production",
    entry: './src/index.ts',
    output: {
        globalObject: "this",
        filename: 'index.js',
        path: path.resolve(__dirname, 'dist'),
        library: {
            type: "umd",
        },
        publicPath: '',
    },
    experiments: {
        asyncWebAssembly: true,
        syncWebAssembly: true,
    },
    module: {
        rules: [
            {
                test: /\.tsx?$/,
                use: 'ts-loader',
                exclude: /node_modules/,
            }
        ]
    },
    resolve: {
        extensions: ['.tsx', '.ts', '.js', "json", "wasm"],
        fallback: { assert: false, process: false, fs: false, util: false, path: false }
    },
    optimization: {
        minimize: false
    }
};