"use strict";

let StringCache = require("../StringCache");
let WasmObjectPool = require("../WasmObjectPool");
let createStringWithSourceMap = require("../utils/createStringWithSourceMap");
let wasm = require("../build/webpack_sources");

class SourceNode extends wasm._MSourceNode {
    constructor(line, column, source, chunks) {
        super(0);
        if (line !== -2) {
            if (source) {
                let source_idx = StringCache.add(source);
                this.ptr = SourceNode._new_number_number_sidx_null(
                    line,
                    column,
                    source_idx
                ).ptr;
            } else {
                this.ptr = SourceNode._new_null_null_null_null().ptr;
            }
            if (chunks != null) {
                this.add(chunks);
            }
        }
    }

    add(chunk) {
        if (Array.isArray(chunk)) {
            chunk.forEach(function(ck) {
                this.add(ck);
            }, this);
        } else if (chunk.isSourceNode) {
            this._add_sourcenode(chunk);
        } else if (typeof chunk === "string") {
            this._add_string(chunk);
        } else {
            throw new TypeError(
                "Expected a SourceNode, string, or an array of SourceNodes and strings. Got " +
                    aChunk
            );
        }
    }

    toStringWithSourceMap(args) {
        let stringWithSourceMap = this._to_string_with_source_map_null();
        return createStringWithSourceMap(stringWithSourceMap, args.file, true);
    }
}

SourceNode.prototype.isSourceNode = true;
module.exports = SourceNode;
