"use strict";

let SourceNode = require("./wasm-source-map").SourceNode;
let SourceListMap = require("./wasm-source-list-map").SourceListMap;
let StringCache = require("./StringCache");
let SourcesPool = require("./SourcesPool");
let wasm = require("./build/webpack_sources");

let ptrCache = new Map();
class OriginalSource extends wasm._OriginalSource {
    constructor(value, name) {
        super(0);
        this._value = value;
        this._value_idx = StringCache.add(value);
        this._name = name;
        this._name_idx = StringCache.add(name);
        if (name === "webpack/bootstrap") {
            let cachedPtr = ptrCache.get(this._value_idx);
            if (cachedPtr) {
                this.ptr = cachedPtr;
            } else {
                this.ptr = OriginalSource._new_string_sidx_sidx(
                    value,
                    this._value_idx,
                    this._name_idx
                ).ptr;
                ptrCache.set(this._value_idx, this.ptr);
                SourcesPool.add(this);
            }
        } else {
            this.ptr = OriginalSource._new_string_sidx_sidx(
                value,
                this._value_idx,
                this._name_idx
            ).ptr;
            SourcesPool.add(this);
        }
    }

    source() {
        return this._value;
    }

    size() {
        return this._value.length;
    }

    updateHash(hash) {
        hash.update(this._value);
    }
}

require("./SourceAndMapMixin")(OriginalSource.prototype);

OriginalSource.prototype.type = "OriginalSource";
module.exports = OriginalSource;
