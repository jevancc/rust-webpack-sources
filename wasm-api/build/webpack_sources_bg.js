const path = require("path").join(__dirname, "webpack_sources_bg.wasm");
const bytes = require("fs").readFileSync(path);
let imports = {};
imports["./webpack_sources"] = require("./webpack_sources");

const wasmModule = new WebAssembly.Module(bytes);
const wasmInstance = new WebAssembly.Instance(wasmModule, imports);
module.exports = wasmInstance.exports;
