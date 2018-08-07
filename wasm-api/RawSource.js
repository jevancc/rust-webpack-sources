"use strict";

const SourceNode = require("./wasm-source-map").SourceNode;
const SourceListMap = require("./wasm-source-list-map").SourceListMap;
const WasmObjectPool = require("./WasmObjectPool");
const Types = require("./Types");
const wasm = require("./build/webpack_sources");

let ptrCache = new Map();
class RawSource extends wasm._RawSource {
    static _clearPtrCache() {
        ptrCache.clear();
    }

    constructor(value) {
        super(0);
        let cachedPtr = ptrCache.get(value);
        if (cachedPtr) {
            this.ptr = cachedPtr;
        } else {
            this.ptr = RawSource._new_string(value).ptr;
            ptrCache.set(value, this.ptr);
            WasmObjectPool.add(this);
        }
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

RawSource.prototype.type = Types.RawSource;
module.exports = RawSource;
