"use strict";

const StringCache = require("../StringCache");
const WasmObjectPool = require("../WasmObjectPool");
const createStringWithSourceMap = require("../utils/createStringWithSourceMap");
const Types = require("./Types");
const wasm = require("../build/webpack_sources");

class SourceNode extends wasm._MSourceNode {
    constructor(line, column, source, chunks) {
        super(0);
        if (line !== Types.Null) {
            if (source) {
                let sourceIndex = StringCache.add(source);
                this.ptr = SourceNode._new_number_number_sidx_null(
                    line,
                    column,
                    sourceIndex
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
        } else if (chunk.type === Types.SourceNode) {
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

SourceNode.prototype.type = Types.SourceNode;
module.exports = SourceNode;
