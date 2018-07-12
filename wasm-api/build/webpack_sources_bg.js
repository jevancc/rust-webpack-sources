let imports = {};
imports['./webpack_sources'] = require('./webpack_sources');

            const join = require('path').join;
            const bytes = require('fs').readFileSync(join(__dirname, 'webpack_sources_bg.wasm'));
            const wasmModule = new WebAssembly.Module(bytes);
            const wasmInstance = new WebAssembly.Instance(wasmModule, imports);
            module.exports = wasmInstance.exports;
        