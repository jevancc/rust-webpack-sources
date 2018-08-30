# wasm-webpack-sources

The rust implementation of `webpack-sources` with WebAssembly Node.js API.

* [Benchmark](https://github.com/jevancc/webpack-sources-benchmark)
* [API document](https://github.com/webpack/webpack-sources)

## Usage


### 1. With `webpack-cli` (Recommend)
First, install `webpack-cli`, `webpack`, and `wasm-webpack-sources` in your local project:
```bash
npm install webpack wasm-webpack-sources https://github.com/jevancc/webpack-cli
```
After the installation, you can try the experimental WebAssembly package with argument `-r wasm-webpack-sources`:
```bash
npx webpack -r wasm-webpack-sources
```
When the module is successfully loaded, you will see the following message:
```
Override:
    <webpack-sources resolve> -> <wasm-webpack-sources resolve>
You are now using experimental package `wasm-webpack-sources`
```

### 2. With cloned webpack repository
You can install
this package with yarn under an alias to try it with your local Webpack:
```bash
yarn add webpack-sources@npm:wasm-webpack-sources
```
This command will replace `webpack-soruces` with `wasm-webpack-sources` in `node_modules`.

## Important Notice
* The size of file is stored in 32bit signed integer, hence this package does not support files larger than 2GB.
* In order to use this package you will need Node.js version 8 or above.

## FAQ
### 1. Do I need to modify webpack or my plugins to try this package?
No. The API of this package is 100% compatible with the JS package `webpack-sources`. You just need to override the requiring of `webpack-sources` with `wasm-webpack-sources` and everything can work perfectly.

### 2. How you handle deallocation?
There is no deallocation in current release.

The deallocation process invloves extra works to be made in webpack and plugins which use and create objects with `webpack-sources`. So far, these works haven't be done so it may impose the potential risk of memory leak.

Be careful when you are using it in the cases where process is not exited after the compilation such as `dev-server`.

## Build

The released npm package and this repository contains the latest WebAssembly binary, so there is no need for you to build the binary yourself to use it unless you want to test it.

This package is written in Rust with `wasm-bindgen`. To build the WebAssembly on your own, you need to install the  requirements:
* Rust nightly
* `wasm32-unknown-unknown` target
* `wasm-bindgen`

You can follow the [guide of `wasm-bindgen`](https://rustwasm.github.io/wasm-bindgen/whirlwind-tour/basic-usage.html) to setup everything you need.

After the enviroment is set-up, you can build the WebAssembly binary with the npm script:
```bash
npm run build:wasm
```
This script will place the binary and JS interface generated by `wasm-bindgen` in `wasm-api/build`.

## Reference
* webpack / [webpack-sources](https://github.com/webpack/webpack-sources)
* webpack / [webpack](https://github.com/webpack/webpack)
