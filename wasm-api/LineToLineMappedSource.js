"use strict";

const SourceNode = require("./wasm-source-map").SourceNode;
const SourceListMap = require("./wasm-source-list-map").SourceListMap;
const StringCache = require("./StringCache");
const WasmObjectPool = require("./WasmObjectPool");
const Types = require("./Types");
const wasm = require("./build/webpack_sources");

class LineToLineMappedSource extends wasm._LineToLineMappedSource {
    constructor(value, name, originalSource) {
        super(0);
        this._value = value;
        this._name = name;
        this._nameIndex = StringCache.add(name);
        this._originalSource = originalSource;
        this._originalSourceIndex = StringCache.addUnchecked(originalSource);
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

LineToLineMappedSource.prototype.type = Types.LineToLineMappedSource;
module.exports = LineToLineMappedSource;
