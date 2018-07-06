/*
	MIT License http://www.opensource.org/licenses/mit-license.php
	Author Tobias Koppers @sokra
*/
"use strict";
var SourceNode = require("./wasm-source-map").SourceNode;
var SourceListMap = require("./wasm-source-list-map").SourceListMap;
var wasm = require("./build/webpack_sources");

class ConcatSource extends wasm._ConcatSource {
    constructor() {
        super(0);
        this.ptr = ConcatSource._new().ptr;
        this.children = [];
        for (var i = 0; i < arguments.length; i++) {
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
            var children = item.children;
            for (var j = 0; j < children.length; j++)
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
        var children = this.children;
        for (var i = 0; i < children.length; i++) {
            var item = children[i];
            if (typeof item === "string") hash.update(item);
            else item.updateHash(hash);
        }
    }
}

require("./SourceAndMapMixin")(ConcatSource.prototype);

ConcatSource.prototype.type = "ConcatSource";
module.exports = ConcatSource;
