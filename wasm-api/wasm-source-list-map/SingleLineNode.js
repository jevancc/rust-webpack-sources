"use strict";

let StringCache = require("../StringCache");
let wasm = require("../build/webpack_sources");

class SingleLineNode extends wasm._SingleLineNode {
    constructor(generatedCode, source, originalSource, startingLine) {
        super(0);
        if (generatedCode) {
            startingLine = startingLine || 1;
            if (source) {
                let sourceIndex = StringCache.add(source);
                let originalSourceIndex = StringCache.addUnchecked(originalSource);
                this.ptr = SingleLineNode._new_string_sidx_sidx_number(
                    generatedCode,
                    sourceIndex,
                    originalSourceIndex,
                    startingLine
                ).ptr;
            } else {
                this.ptr = SingleLineNode._new_ntring_null_null_number(
                    generatedCode,
                    startingLine
                ).ptr;
            }
        }
    }

    clone() {
        let ret = new SingleLineNode();
        ret.ptr = wasm._singlelinenode__clone(this.ptr);
        return ret;
    }
}

SingleLineNode.prototype.isSingleLineNode = true;
module.exports = SingleLineNode;
