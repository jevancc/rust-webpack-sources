/*
	MIT License http://www.opensource.org/licenses/mit-license.php
	Author Tobias Koppers @sokra
*/
"use strict";
let SourceNode = require("./wasm-source-map").SourceNode;
let SourceListMap = require("./wasm-source-list-map").SourceListMap;
let wasm = require("./build/webpack_sources");

class ConcatSource extends wasm._ConcatSource {
    constructor() {
        super(0);
        this.ptr = ConcatSource._new().ptr;
        this.children = [];
        for (let i = 0; i < arguments.length; i++) {
            this.add(arguments[i]);
        }
    }

    add(item) {
        if (typeof item === "string") {
            this._add_string(item);
        } else if (item.type === "RawSource") {
            this._add_raw_source(item);
        } else if (item.type === "OriginalSource") {
            this._add_original_source(item);
        } else if (item.type === "ReplaceSource") {
            this._add_replace_source(item);
        } else if (item.type === "PrefixSource") {
            this._add_prefix_source(item);
        } else if (item.type === "ConcatSource") {
            this._add_concat_source(item);
        } else if (item.type === "LineToLineMappedSource") {
            this._add_line_to_line_mapped_source(item);
        } else if (item.type === "SourceMapSource") {
            this._add_source_map_source(item);
        } else {
            throw new Error("Invalid source");
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

ConcatSource.prototype.type = "ConcatSource";
module.exports = ConcatSource;
