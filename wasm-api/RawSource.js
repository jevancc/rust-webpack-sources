/*
	MIT License http://www.opensource.org/licenses/mit-license.php
	Author Tobias Koppers @sokra
*/
"use strict";
let SourceNode = require("./wasm-source-map").SourceNode;
let SourceListMap = require("./wasm-source-list-map").SourceListMap;
let SourcesPool = require("./SourcesPool");
let wasm = require("./build/webpack_sources");

let ptrCache = new Map();
class RawSource extends wasm._RawSource {
    constructor(value) {
        super(0);
        let cachedPtr = ptrCache.get(value);
        if (cachedPtr) {
            this.ptr = cachedPtr;
        } else {
            this.ptr = RawSource._new_string(value).ptr;
            ptrCache.set(value, this.ptr);
            SourcesPool.add(this);
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

RawSource.prototype.type = "RawSource";
module.exports = RawSource;
