"use strict";

const StringCache = require("../StringCache");
const Types = require("./Types");
const wasm = require("../build/webpack_sources");

class SourceNode extends wasm._SourceNode {
    constructor(generatedCode, source, originalSource, startingLine) {
        super(0);
        if (generatedCode) {
            startingLine = startingLine || 1;
            if (source) {
                let sourceIndex = StringCache.add(source);
                let originalSourceIndex = StringCache.addUnchecked(
                    originalSource
                );
                this.ptr = SourceNode._new_string_sidx_sidx_number(
                    generatedCode,
                    sourceIndex,
                    originalSourceIndex,
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

SourceNode.prototype.type = Types.SourceNode;
module.exports = SourceNode;
