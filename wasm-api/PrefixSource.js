/*
	MIT License http://www.opensource.org/licenses/mit-license.php
	Author Tobias Koppers @sokra
*/
"use strict";

let SourceNode = require("./wasm-source-map").SourceNode;
let SourceListMap = require("./wasm-source-list-map").SourceListMap;
let WasmObjectPool = require("./WasmObjectPool");
let wasm = require("./build/webpack_sources");

class PrefixSource extends wasm._PrefixSource {
    constructor(prefix, source) {
        super(0);
        if (typeof source === "string") {
            this.ptr = PrefixSource._new_string_string(prefix, source).ptr;
        } else if (source.type === "RawSource") {
            this.ptr = PrefixSource._new_string_raw_source(prefix, source).ptr;
        } else if (source.type === "OriginalSource") {
            this.ptr = PrefixSource._new_string_original_source(
                prefix,
                source
            ).ptr;
        } else if (source.type === "ReplaceSource") {
            this.ptr = PrefixSource._new_string_replace_source(
                prefix,
                source
            ).ptr;
        } else if (source.type === "PrefixSource") {
            this.ptr = PrefixSource._new_string_prefix_source(
                prefix,
                source
            ).ptr;
        } else if (source.type === "ConcatSource") {
            this.ptr = PrefixSource._new_string_concat_source(
                prefix,
                source
            ).ptr;
        } else if (source.type === "LineToLineMappedSource") {
            this.ptr = PrefixSource._new_string_line_to_line_mapped_source(
                prefix,
                source
            ).ptr;
        } else if (source.type === "SourceMapSource") {
            this.ptr = PrefixSource._new_string_source_map_source(
                prefix,
                source
            ).ptr;
        } else {
            throw new Error("Invalid source");
        }

        this._jsSource = source;
        this._jsPrefix = prefix;
        WasmObjectPool.add(this);
    }

    source() {
        return this._source();
    }

    size() {
        return this._size();
    }

    updateHash(hash) {
        if (typeof this._jsSource === "string") hash.update(this._jsSource);
        else this._jsSource.updateHash(hash);
        if (typeof this._jsPrefix === "string") hash.update(this._jsPrefix);
        else this._jsPrefix.updateHash(hash);
    }
}

require("./SourceAndMapMixin")(PrefixSource.prototype);

PrefixSource.prototype.type = "PrefixSource";
module.exports = PrefixSource;
