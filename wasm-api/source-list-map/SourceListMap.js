"use strict";

var wasm = require("../build/webpack_sources");
var CodeNode = require("./CodeNode");
var SourceNode = require("./SourceNode");
var SingleLineNode = require("./SingleLineNode");
var StringVec = require("./utils").StringVec;
var NodeVec = require("./utils").NodeVec;

class SourceListMap extends wasm._SourceListMap {
    constructor(generatedCode, source, originalSource) {
        super(0);
        if (generatedCode !== -1) {
            if (Array.isArray(generatedCode)) {
                var nodes = NodeVec(generatedCode);
                this.ptr = SourceListMap._new_nodes(nodes);
            } else {
                this.ptr = SourceListMap._new().ptr;
                if (generatedCode || source) {
                    this.add(generatedCode, source, originalSource);
                }
            }
            this.stringWithSourceMapCache = null;
        }
    }

    add(generatedCode, source, originalSource) {
        var nodes = NodeVec([generatedCode]);
        if (source) {
            this._add_node_string_string(nodes, source, originalSource);
        } else {
            this._add_node(nodes);
        }
        this.stringWithSourceMapCache = null;
    }

    prepend(generatedCode, source, originalSource) {
        var nodes = NodeVec([generatedCode]);
        if (source) {
            this._prepend_node_string_string(nodes, source, originalSource);
        } else {
            this._prepend_node(nodes);
        }
        this.stringWithSourceMapCache = null;
    }

    mapGeneratedCode(fnIdx) {
        var newSlp = new SourceListMap(-1);
        switch (fnIdx) {
            case this.MappingFunction.Test:
                newSlp.ptr = wasm._sourcelistmap_map_generated_code_test(
                    this
                ).ptr;
                break;
            case this.MappingFunction.Identical:
                newSlp.ptr = wasm._sourcelistmap_map_generated_code_identical(
                    this
                ).ptr;
                break;
            case this.MappingFunction.Prefix:
                newSlp.ptr = wasm._sourcelistmap_map_generated_code_prefix(
                    this,
                    arguments[1]
                ).ptr;
                break;
            default:
                throw new Error("Invalid mapping function index");
        }
        return newSlp;
    }

    toString() {
        return this._to_string();
    }

    toStringWithSourceMap(options) {
        if (!this.stringWithSourceMapCache) {
            this.stringWithSourceMapCache = JSON.parse(
                this._to_string_with_source_map()
            );
        }
        this.stringWithSourceMapCache.map.file = options.file;
        return this.stringWithSourceMapCache;
    }

    static isSourceListMap(obj) {
        return obj instanceof wasm._SourceListMap;
    }
}

SourceListMap.prototype.MappingFunction = {
    Test: 1,
    Identical: 2,
    Prefix: 3
};
SourceListMap.prototype.isSourceListMap = true;
module.exports = SourceListMap;
