/*
	MIT License http://www.opensource.org/licenses/mit-license.php
	Author Tobias Koppers @sokra
*/
"use strict";

var SourceNode = require("./wasm-source-map").SourceNode;
var SourceListMap = require("./wasm-source-list-map").SourceListMap;
var fromStringWithSourceMap = require("./wasm-source-list-map")
    .fromStringWithSourceMap;
var SourceMapConsumer = require("source-map").SourceMapConsumer;
var wasm = require("./build/webpack_sources");

class ReplaceSource extends wasm._ReplaceSource {
    constructor(source, name) {
        super(0);
        this._js_source = source;
        this._js_name = name;
        this._source_cache = null;
        this._replacements = null;

        if (typeof source === "string") {
            this.ptr = ReplaceSource._new_string(source).ptr;
        } else if (source.type === "RawSource") {
            this.ptr = ReplaceSource._new_raw_source(source).ptr;
        } else if (source.type === "OriginalSource") {
            this.ptr = ReplaceSource._new_original_source(source).ptr;
        } else if (source.type === "ReplaceSource") {
            this.ptr = ReplaceSource._new_replace_source(source).ptr;
        } else if (source.type === "PrefixSource") {
            this.ptr = ReplaceSource._new_prefix_source(source).ptr;
        } else if (source.type === "ConcatSource") {
            this.ptr = ReplaceSource._new_concat_source(source).ptr;
        } else if (source.type === "LineToLineMappedSource") {
            this.ptr = ReplaceSource._new_line_to_line_mapped_source(source).ptr;
        } else if (source.type === "SourceMapSource") {
            this.ptr = ReplaceSource._new_source_map_source(source).ptr;
        } else {
            throw new Error("Invalid source");
        }
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
        // return this._source;
    }

    replacements() {
        if (this._replacements == null) {
            this._replacements = JSON.parse(this._replacements_to_string());
        }
        return this._replacements;
    }

    node(options) {
        var node = new SourceNode(-2);
        options = options || {};
        node.ptr = this._node_bool_bool(!(options.columns === false), !(options.module === false)).ptr;
        return node;
    }

    listMap(options) {
        var map = new SourceListMap(-2);
        options = options || {};
        map.ptr = this._list_map_bool_bool(!(options.columns === false), !(options.module === false)).ptr;
        return map;
    }

    updateHash(hash) {
        var source = this.source();
        hash.update(source || "");
    }
}

require("./SourceAndMapMixin")(ReplaceSource.prototype);

ReplaceSource.prototype.type = "ReplaceSource";
module.exports = ReplaceSource;
