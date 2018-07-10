"use strict";

let StringCache = require("../StringCache");
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
        let parsed = JSON.parse(this._to_string_with_source_map_null());
        return {
            source: parsed.source,
            map: {
                file: args.file,
                mappings: parsed.map.mappings,
                names: (parsed.map.names || []).map(StringCache.at),
                sources: (parsed.map.sources || []).map(StringCache.at),
                sourcesContent:
                    (parsed.map.sources_content || []).length > 0
                        ? (parsed.map.sources_content || []).map(StringCache.at)
                        : undefined,
                version: parsed.map.version || 3
            }
        };
    }
}

SourceNode.prototype.isSourceNode = true;
module.exports = SourceNode;
