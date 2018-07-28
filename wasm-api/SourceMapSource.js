/*
	MIT License http://www.opensource.org/licenses/mit-license.php
	Author Tobias Koppers @sokra
*/
"use strict";

let SourceNode = require("./wasm-source-map").SourceNode;
let SourceListMap = require("./wasm-source-list-map").SourceListMap;
let fromStringWithSourceMap = require("./wasm-source-list-map")
    .fromStringWithSourceMap;
let SourceMapConsumer = require("source-map").SourceMapConsumer;
let SourceMapGenerator = require("source-map").SourceMapGenerator;
let StringCache = require("./StringCache");
let WasmObjectPool = require("./WasmObjectPool");
let wasm = require("./build/webpack_sources");

class SourceMapSource extends wasm._SourceMapSource {
    constructor(value, name, sourceMap, originalSource, innerSourceMap) {
        super(0);
        this._value = value;
        this._valueIndex = StringCache.add(value);
        this._name = name;
        this._sourceMap = sourceMap;

        if (sourceMap._wasmObj) {
            this.ptr = SourceMapSource._new_string_sidx_string_wasmmap(
                value,
                this._valueIndex,
                name,
                sourceMap._wasmObj
            ).ptr;
        } else {
            let sources = (sourceMap.sources || []).map(StringCache.add);
            let sourcesContent = (sourceMap.sourcesContent || []).map(
                StringCache.add
            );
            let mappings = sourceMap.mappings;
            let names = (sourceMap.names || []).map(StringCache.add);
            this.ptr = SourceMapSource._new_string_sidx_string_map(
                value,
                this._valueIndex,
                name,
                sources,
                sourcesContent,
                mappings,
                names
            ).ptr;
        }

        if (originalSource) {
            this._originalSource = originalSource;
            this._set_original_source_sidx(StringCache.add(originalSource));
        }
        if (innerSourceMap) {
            this._innerSourceMap = innerSourceMap;
            if (innerSourceMap._wasmObj) {
                this._set_inner_source_map_wasmmap(innerSourceMap._wasmObj);
            } else {
                let innerSources = (innerSourceMap.sources || []).map(
                    StringCache.add
                );
                let innerSourcesContent = (
                    innerSourceMap.sourcesContent || []
                ).map(StringCache.add);
                let innerMappings = innerSourceMap.mappings;
                let innerNames = (innerSourceMap.names || []).map(
                    StringCache.add
                );
                this._set_inner_source_map_map(
                    innerSources,
                    innerSourcesContent,
                    innerMappings,
                    innerNames
                );
            }
        }
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
        if (this._originalSource) {
            hash.update(this._originalSource);
        }
    }
}

require("./SourceAndMapMixin")(SourceMapSource.prototype);

SourceMapSource.prototype.type = "SourceMapSource";
module.exports = SourceMapSource;
