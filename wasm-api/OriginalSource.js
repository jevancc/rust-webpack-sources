"use strict";

var SourceNode = require("./wasm-source-map").SourceNode;
var SourceListMap = require("./wasm-source-list-map").SourceListMap;
var wasm = require("./build/webpack_sources");

class OriginalSource extends wasm._OriginalSource {
    constructor(value, name) {
        super(0);
        this.ptr = OriginalSource._new_string_string(value, name).ptr;
        this._value = value;
        this._name = name;
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
    }
}

require("./SourceAndMapMixin")(OriginalSource.prototype);

OriginalSource.prototype.type = "OriginalSource";
module.exports = OriginalSource;
