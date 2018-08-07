"use strict";

const SourceNode = require("./wasm-source-map").SourceNode;
const SourceListMap = require("./wasm-source-list-map").SourceListMap;
const fromStringWithSourceMap = require("./wasm-source-list-map")
    .fromStringWithSourceMap;
const SourceMapConsumer = require("source-map").SourceMapConsumer;
const WasmObjectPool = require("./WasmObjectPool");
const Types = require("./Types");
const wasm = require("./build/webpack_sources");

class ReplaceSource extends wasm._ReplaceSource {
    constructor(source, name) {
        super(0);
        this._jsSource = source;
        this._jsName = name;
        this._replacements = null;

        if (typeof source === "string") {
            this.ptr = ReplaceSource._new_string(source).ptr;
        } else {
            switch (source.type) {
                case Types.RawSource:
                    this.ptr = ReplaceSource._new_raw_source(source).ptr;
                    break;
                case Types.OriginalSource:
                    this.ptr = ReplaceSource._new_original_source(source).ptr;
                    break;
                case Types.ReplaceSource:
                    this.ptr = ReplaceSource._new_replace_source(source).ptr;
                    break;
                case Types.PrefixSource:
                    this.ptr = ReplaceSource._new_prefix_source(source).ptr;
                    break;
                case Types.ConcatSource:
                    this.ptr = ReplaceSource._new_concat_source(source).ptr;
                    break;
                case Types.LineToLineMappedSource:
                    this.ptr = ReplaceSource._new_line_to_line_mapped_source(
                        source
                    ).ptr;
                    break;
                case Types.SourceMapSource:
                    this.ptr = ReplaceSource._new_source_map_source(source).ptr;
                    break;
                default:
                    throw new TypeError("Invalid source type");
            }
        }
        WasmObjectPool.add(this);
    }

    replace(start, end, newValue) {
        if (typeof newValue !== "string")
            throw new Error(
                "insertion must be a string, but is a " + typeof newValue
            );

        this._replacements = null;
        this._replace_number_number_string_number_number(
            Math.floor(start),
            Math.floor(end),
            newValue,
            Math.floor((start % 1) * 16),
            Math.floor((end % 1) * 16)
        );
    }

    insert(pos, newValue) {
        if (typeof newValue !== "string")
            throw new Error(
                "insertion must be a string, but is a " +
                    typeof newValue +
                    ": " +
                    newValue
            );
        this._replacements = null;
        this._insert_number_string_number(
            Math.floor(pos),
            newValue,
            Math.floor((pos % 1) * 16)
        );
    }

    size() {
        return this._size();
    }

    source(options) {
        return this._source();
    }

    original() {
        throw new Error("ReplaceSource.original() is deprecated");
        // return this._jsSource;
    }

    replacements() {
        if (this._replacements == null) {
            this._replacements = JSON.parse(this._replacements_to_string());
        }
        return this._replacements;
    }

    updateHash(hash) {
        let source = this.source();
        hash.update(source || "");
    }
}

require("./SourceAndMapMixin")(ReplaceSource.prototype);

ReplaceSource.prototype.type = Types.ReplaceSource;
module.exports = ReplaceSource;
