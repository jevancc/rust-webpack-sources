/*
	MIT License http://www.opensource.org/licenses/mit-license.php
	Author Tobias Koppers @sokra
*/
"use strict";

var SourceNode = require("./wasm-source-map").SourceNode;
var SourceListMap = require("./wasm-source-list-map").SourceListMap;
var wasm = require("./build/webpack_sources");

class LineToLineMappedSource extends wasm._LineToLineMappedSource {
    constructor(value, name, originalSource) {
        super(0);
        this.ptr = LineToLineMappedSource._new().ptr;
        this._value = value;
        this._name = name;
        this._originalSource = originalSource;
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
        hash.update(this._value);
        hash.update(this._originalSource);
    }
}

require("./SourceAndMapMixin")(LineToLineMappedSource.prototype);

LineToLineMappedSource.prototype.type = "LineToLineMappedSource";
module.exports = LineToLineMappedSource;
