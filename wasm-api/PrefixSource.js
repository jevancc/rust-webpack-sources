/*
	MIT License http://www.opensource.org/licenses/mit-license.php
	Author Tobias Koppers @sokra
*/
"use strict";

const SourceNode = require("./wasm-source-map").SourceNode;
const SourceListMap = require("./wasm-source-list-map").SourceListMap;
const WasmObjectPool = require("./WasmObjectPool");
const Types = require("./Types");
const wasm = require("./build/webpack_sources");

class PrefixSource extends wasm._PrefixSource {
    constructor(prefix, source) {
        super(0);
        if (typeof source === "string") {
            this.ptr = PrefixSource._new_string_string(prefix, source).ptr;
        } else {
            switch (source.type) {
                case Types.RawSource:
                    this.ptr = PrefixSource._new_string_raw_source(
                        prefix,
                        source
                    ).ptr;
                    break;
                case Types.OriginalSource:
                    this.ptr = PrefixSource._new_string_original_source(
                        prefix,
                        source
                    ).ptr;
                    break;
                case Types.ReplaceSource:
                    this.ptr = PrefixSource._new_string_replace_source(
                        prefix,
                        source
                    ).ptr;
                    break;
                case Types.PrefixSource:
                    this.ptr = PrefixSource._new_string_prefix_source(
                        prefix,
                        source
                    ).ptr;
                    break;
                case Types.ConcatSource:
                    this.ptr = PrefixSource._new_string_concat_source(
                        prefix,
                        source
                    ).ptr;
                    break;
                case Types.LineToLineMappedSource:
                    this.ptr = PrefixSource._new_string_line_to_line_mapped_source(
                        prefix,
                        source
                    ).ptr;
                    break;
                case Types.SourceMapSource:
                    this.ptr = PrefixSource._new_string_source_map_source(
                        prefix,
                        source
                    ).ptr;
                    break;
                default:
                    throw new TypeError("Invalid source type");
            }
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

PrefixSource.prototype.type = Types.PrefixSource;
module.exports = PrefixSource;
