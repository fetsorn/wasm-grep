import path from 'path';
import HtmlWebpackPlugin from 'html-webpack-plugin';
import WasmPackPlugin from '@wasm-tool/wasm-pack-plugin'

const __dirname = path.dirname(new URL(import.meta.url).pathname)

export default {
    mode: 'development',
    entry: './tests/bundler.mjs',
    output: {
        path: path.resolve(__dirname, 'dist/bundler'),
        filename: 'index.js',
    },
    plugins: [
        new HtmlWebpackPlugin(),

        new WasmPackPlugin({
            crateDirectory: path.resolve(__dirname, '.'),
            outDir: 'pkg/bundler',
            extraArgs: '--target bundler'
            // extraArgs: '-- --typescript --target bundler --mode normal',
        }),
    ],
    experiments: {
        syncWebAssembly: true,
        topLevelAwait: true
    },
    watchOptions: {
        aggregateTimeout: 200,
        poll: 200,
    },
}
