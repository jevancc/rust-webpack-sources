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
var SourceMapGenerator = require("source-map").SourceMapGenerator;
var StringVec = require("./wasm-source-list-map/utils").StringVec;
var wasm = require("./build/webpack_sources");

class SourceMapSource extends wasm._SourceMapSource {
    constructor(value, name, sourceMap) {
        super(0);
        this._value = value;
        this._name = name;
        this._sourceMap = sourceMap;

        var sources = sourceMap.sources || [];
        var sourcesContent = sourceMap.sourcesContent || [];
        var mappings = sourceMap.mappings;
        var names = sourceMap.names || [];
        this.ptr = SourceMapSource._new_string_string_map(
            value,
            name,
            StringVec(sources),
            StringVec(sourcesContent),
            mappings,
            StringVec(names)
        ).ptr;
    }

    source() {
        return this._value;
    }

    size() {
        return this._value.length;
    }

    node(options) {
        var node = new SourceNode(-2);
        options = options || {};
        node.ptr = this._node_bool_bool(
            !(options.columns === false),
            !(options.module === false)
        ).ptr;
        return node;
    }

    listMap(options) {
        var map = new SourceListMap(-2);
        options = options || {};
        map.ptr = this._list_map_bool_bool(
            !(options.columns === false),
            !(options.module === false)
        ).ptr;
        return map;
    }

    updateHash(hash) {
        hash.update(this._value);
        if (this._originalSource) hash.update(this._originalSource);
    }
}

require("./SourceAndMapMixin")(SourceMapSource.prototype);

SourceMapSource.prototype.type = "SourceMapSource";
module.exports = SourceMapSource;
