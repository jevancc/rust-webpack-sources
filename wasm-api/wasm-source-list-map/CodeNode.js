"use strict";

var wasm = require("../build/webpack_sources");

class CodeNode extends wasm._CodeNode {
    constructor(generatedCode) {
        super(0);
        if (generatedCode) {
            this.ptr = CodeNode._new_string(generatedCode).ptr;
        }
    }

    clone() {
        var ret = new CodeNode();
        ret.ptr = wasm._codenode__clone(this.ptr);
        return ret;
    }
}

CodeNode.prototype.isCodeNode = true;
module.exports = CodeNode;
