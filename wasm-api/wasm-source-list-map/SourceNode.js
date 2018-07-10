"use strict";

let StringCache = require("../StringCache");
let wasm = require("../build/webpack_sources");

class SourceNode extends wasm._SourceNode {
    constructor(generatedCode, source, originalSource, startingLine) {
        super(0);
        if (generatedCode) {
            startingLine = startingLine || 1;
            if (source) {
                let source_idx = StringCache.add(source);
                let originalSource_idx = StringCache.add(originalSource);
                this.ptr = SourceNode._new_string_sidx_sidx_number(
                    generatedCode,
                    source_idx,
                    originalSource_idx,
                    startingLine
                ).ptr;
            } else {
                this.ptr = SourceNode._new_string_null_null_number(
                    generatedCode,
                    startingLine
                ).ptr;
            }
        }
    }

    clone() {
        let ret = new SourceNode();
        ret.ptr = wasm._sourcenode__clone(this.ptr);
        return ret;
    }
}

SourceNode.prototype.isSourceNode = true;
module.exports = SourceNode;
