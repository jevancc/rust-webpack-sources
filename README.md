# rust-webpack-sources

The rust implementation of `webpack-sources` with WebAssembly Node.js API.

## Usage

The WebAssembly API of this project is compatible with `webpack-sources`. You can install
this package with yarn under an alias to try it in Webpack:
```
$ yarn add webpack-sources@npm:wasm-webpack-sources
```

## Important Notice
* This project is still under development, don't use it in production.
* The size of file is stored in 32bit signed integer, hence this package does not support files larger than 2GB.
* In order to use this package you will need Node.js version 8 or above.


## Reference
* webpack / [webpack-sources](https://github.com/webpack/webpack-sources)
* webpack / [webpack](https://github.com/webpack/webpack)
