"use strict";

var wasm = require("../build/webpack_sources");

class SourceNode extends wasm._MSourceNode {
    constructor(line, column, source, chunks) {
        super(0);
        if (line !== -2) {
            if (source) {
                this.ptr = SourceNode._new_number_number_string_null(
                    line,
                    column,
                    source
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
        var json;
        if (typeof args.file === "string") {
            json = this._to_string_with_source_map_string(args.file);
        } else {
            json = this._to_string_with_source_map_null();
        }

        return json;
    }
}

SourceNode.prototype.isSourceNode = true;
module.exports = SourceNode;
