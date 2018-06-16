"use strict";

var SourceNode = require("source-map").SourceNode;
var SourceMapConsumer = require("source-map").SourceMapConsumer;
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
        options = options || {};
        return this._node_bool(!(options.columns === false));
    }

    listMap(options) {
        return new SourceListMap(this._value, this._name, this._value);
    }

    updateHash(hash) {
        hash.update(this._value);
    }
}

require("./SourceAndMapMixin")(OriginalSource.prototype);

module.exports = OriginalSource;
