"use strict";

const CodeNode = require("./CodeNode");
const SourceNode = require("./SourceNode");
const SingleLineNode = require("./SingleLineNode");
const StringVec = require("./utils").StringVec;
const NodeVec = require("./utils").NodeVec;
const StringCache = require("../StringCache");
const WasmObjectPool = require("../WasmObjectPool");
const createStringWithSourceMap = require("../utils/createStringWithSourceMap");
const Types = require("./Types");
const wasm = require("../build/webpack_sources");

class SourceListMap extends wasm._SourceListMap {
    constructor(generatedCode, source, originalSource) {
        super(0);
        if (generatedCode !== Types.Null) {
            if (Array.isArray(generatedCode)) {
                let nodes = NodeVec(generatedCode);
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
        let nodes = NodeVec([generatedCode]);
        if (source) {
            let sourceIndex = StringCache.add(source);
            let originalSourceIndex = StringCache.addUnchecked(originalSource);
            this._add_node_sidx_sidx(nodes, sourceIndex, originalSourceIndex);
        } else {
            this._add_node(nodes);
        }
        this.stringWithSourceMapCache = null;
    }

    prepend(generatedCode, source, originalSource) {
        let nodes = NodeVec([generatedCode]);
        if (source) {
            let sourceIndex = StringCache.add(source);
            let originalSourceIndex = StringCache.addUnchecked(originalSource);
            this._prepend_node_sidx_sidx(
                nodes,
                sourceIndex,
                originalSourceIndex
            );
        } else {
            this._prepend_node(nodes);
        }
        this.stringWithSourceMapCache = null;
    }

    mapGeneratedCode(fnIdx) {
        let newSlp = new SourceListMap(Types.Null);
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
            case this.MappingFunction.Replace:
                let replacements = [];
                for (let i in arguments[1]) {
                    let repl = arguments[1][i];
                    replacements.push([
                        Math.floor(repl[0] * 16),
                        Math.floor(repl[1] * 16),
                        repl[2],
                        repl[3]
                    ]);
                }
                newSlp.ptr = wasm._sourcelistmap_map_generated_code_replace(
                    this,
                    JSON.stringify(replacements)
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

    toStringWithSourceMap(args) {
        let stringWithSourceMap = this._to_string_with_source_map_null();
        return createStringWithSourceMap(stringWithSourceMap, args.file);
    }

    static isSourceListMap(obj) {
        return obj instanceof wasm._SourceListMap;
    }
}

SourceListMap.prototype.MappingFunction = {
    Test: 1,
    Identical: 2,
    Prefix: 3,
    Replace: 4
};
SourceListMap.prototype.type = Types.SourceListMap;
module.exports = SourceListMap;
