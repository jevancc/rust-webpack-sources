/*
	MIT License http://www.opensource.org/licenses/mit-license.php
	Author Tobias Koppers @sokra
*/
"use strict";
var SourceNode = require("./wasm-source-map").SourceNode;
var SourceListMap = require("./wasm-source-list-map").SourceListMap;
var wasm = require("./build/webpack_sources");

class RawSource extends wasm._RawSource {
    constructor(value) {
        super(0);
        this.ptr = RawSource._new_string(value).ptr;
        this._value = value;
    }

    source() {
        return this._value;
    }

    size() {
        return this._value.length;
    }

    map(options) {
        return null;
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
    }

    sourceAndMap(options) {
        return {
            source: this.source(),
            map: this.map()
        };
    }
}

RawSource.prototype.type = "RawSource";
module.exports = RawSource;
