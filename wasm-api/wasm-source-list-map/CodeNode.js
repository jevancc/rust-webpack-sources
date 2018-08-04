"use strict";

const Types = require("./Types");
const wasm = require("../build/webpack_sources");

class CodeNode extends wasm._CodeNode {
    constructor(generatedCode) {
        super(0);
        if (generatedCode) {
            this.ptr = CodeNode._new_string(generatedCode).ptr;
        }
    }

    clone() {
        let ret = new CodeNode();
        ret.ptr = wasm._codenode__clone(this.ptr);
        return ret;
    }
}

CodeNode.prototype.type = Types.CodeNode;
module.exports = CodeNode;
