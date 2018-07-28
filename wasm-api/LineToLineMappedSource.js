/*
	MIT License http://www.opensource.org/licenses/mit-license.php
	Author Tobias Koppers @sokra
*/
"use strict";

let SourceNode = require("./wasm-source-map").SourceNode;
let SourceListMap = require("./wasm-source-list-map").SourceListMap;
let StringCache = require("./StringCache");
let WasmObjectPool = require("./WasmObjectPool");
let wasm = require("./build/webpack_sources");

class LineToLineMappedSource extends wasm._LineToLineMappedSource {
    constructor(value, name, originalSource) {
        super(0);
        this._value = value;
        this._name = name;
        this._nameIndex = StringCache.add(name);
        this._originalSource = originalSource;
        this._originalSourceIndex = StringCache.add(originalSource);
        this.ptr = LineToLineMappedSource._new_string_sidx_sidx(
            value,
            this._nameIndex,
            this._originalSourceIndex
        ).ptr;
        WasmObjectPool.add(this);
    }

    source() {
        return this._value;
    }

    size() {
        return this._value.length;
    }

    updateHash(hash) {
        hash.update(this._value);
        hash.update(this._originalSource);
    }
}

require("./SourceAndMapMixin")(LineToLineMappedSource.prototype);

LineToLineMappedSource.prototype.type = "LineToLineMappedSource";
module.exports = LineToLineMappedSource;
