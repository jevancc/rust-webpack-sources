/*
	MIT License http://www.opensource.org/licenses/mit-license.php
	Author Tobias Koppers @sokra
*/
"use strict";

let SourceNode = require("./wasm-source-map").SourceNode;
let SourceListMap = require("./wasm-source-list-map").SourceListMap;
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

        this._js_source = source;
        this._js_prefix = prefix;
    }

    source() {
        return this._source();
    }

    size() {
        return this._size();
    }

    updateHash(hash) {
        if (typeof this._js_source === "string") hash.update(this._js_source);
        else this._js_source.updateHash(hash);
        if (typeof this._js_prefix === "string") hash.update(this._js_prefix);
        else this._js_prefix.updateHash(hash);
    }
}

require("./SourceAndMapMixin")(PrefixSource.prototype);

PrefixSource.prototype.type = "PrefixSource";
module.exports = PrefixSource;
