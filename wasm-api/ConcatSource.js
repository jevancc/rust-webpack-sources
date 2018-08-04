/*
	MIT License http://www.opensource.org/licenses/mit-license.php
	Author Tobias Koppers @sokra
*/
"use strict";

const SourceNode = require("./wasm-source-map").SourceNode;
const SourceListMap = require("./wasm-source-list-map").SourceListMap;
const WasmObjectPool = require("./WasmObjectPool");
const Types = require("./Types");
const wasm = require("./build/webpack_sources");

class ConcatSource extends wasm._ConcatSource {
    constructor() {
        super(0);
        this.ptr = ConcatSource._new().ptr;
        this.children = [];
        for (let i = 0; i < arguments.length; i++) {
            this.add(arguments[i]);
        }
        WasmObjectPool.add(this);
    }

    add(item) {
        if (typeof item === "string") {
            this._add_string(item);
        } else {
            switch (item.type) {
                case Types.RawSource:
                    this._add_raw_source(item);
                    break;
                case Types.OriginalSource:
                    this._add_original_source(item);
                    break;
                case Types.ReplaceSource:
                    this._add_replace_source(item);
                    break;
                case Types.PrefixSource:
                    this._add_prefix_source(item);
                    break;
                case Types.ConcatSource:
                    this._add_concat_source(item);
                    break;
                case Types.LineToLineMappedSource:
                    this._add_line_to_line_mapped_source(item);
                    break;
                case Types.SourceMapSource:
                    this._add_source_map_source(item);
                    break;
                default:
                    throw new TypeError("Invalid source type");
            }
        }

        if (item.isConcatSource) {
            let children = item.children;
            for (let j = 0; j < children.length; j++)
                this.children.push(children[j]);
        } else {
            this.children.push(item);
        }
    }

    source() {
        return this._source();
    }

    size() {
        return this._size();
    }

    updateHash(hash) {
        let children = this.children;
        for (let i = 0; i < children.length; i++) {
            let item = children[i];
            if (typeof item === "string") hash.update(item);
            else item.updateHash(hash);
        }
    }
}

require("./SourceAndMapMixin")(ConcatSource.prototype);

ConcatSource.prototype.type = Types.ConcatSource;
module.exports = ConcatSource;
