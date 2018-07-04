/*
	MIT License http://www.opensource.org/licenses/mit-license.php
	Author Tobias Koppers @sokra
*/
"use strict";

var SourceNode = require("./wasm-source-map").SourceNode;
var SourceListMap = require("./wasm-source-list-map").SourceListMap;
var fromStringWithSourceMap = require("./wasm-source-list-map")
    .fromStringWithSourceMap;
var SourceMapConsumer = require("source-map").SourceMapConsumer;
var SourceMapGenerator = require("source-map").SourceMapGenerator;
var StringVec = require("./wasm-source-list-map/utils").StringVec;
var wasm = require("./build/webpack_sources");

class SourceMapSource extends wasm._SourceMapSource {
    constructor(value, name, sourceMap, originalSource, innerSourceMap) {
        super(0);
        this._value = value;
        this._name = name;
        this._sourceMap = sourceMap;
        this._originalSource = originalSource;
        this._innerSourceMap = innerSourceMap;

        var sources = JSON.stringify(sourceMap.sources || []);
        var sourcesContent = JSON.stringify(sourceMap.sourcesContent || []);
        var mappings = sourceMap.mappings;
        this.ptr = SourceMapSource._new_string_string_map(value, name,
            sources, sourcesContent, mappings
        ).ptr;


        var innerSourceMap = this._innerSourceMap;
        var sourceMap = this._sourceMap;
        if (innerSourceMap) {
            sourceMap = SourceMapGenerator.fromSourceMap(
                new SourceMapConsumer(sourceMap)
            );
            if (this._originalSource)
                sourceMap.setSourceContent(this._name, this._originalSource);
            innerSourceMap = new SourceMapConsumer(innerSourceMap);
            sourceMap.applySourceMap(innerSourceMap, this._name);
            sourceMap = sourceMap.toJSON();
        }
        let consumer = new SourceMapConsumer(sourceMap);
        let parsed_mappings = [];
        let parsed_sources = [];
        consumer.eachMapping((mapping) => {
            parsed_mappings.push([
                [mapping.generatedLine, mapping.generatedColumn],
                mapping.source,
                mapping.name,
                mapping.originalLine ? [mapping.originalLine, mapping.originalColumn] : null
            ]);
        });
        consumer.sources.forEach((file) => {
            let content = consumer.sourceContentFor(file);
            parsed_sources.push([file, content]);
        });
        this._set_source_map_consumer_string(JSON.stringify({ mappings: parsed_mappings, sources: parsed_sources }));
    }

    source() {
        return this._value;
    }

    size() {
        return this._value.length;
    }

    node(options) {
        var node = new SourceNode(-2);
        options = options || {};
        node.ptr = this._node_bool_bool(!(options.columns === false), !(options.module === false)).ptr;
        return node;
    }

    listMap(options) {
        var map = new SourceListMap(-2);
        options = options || {};
        map.ptr = this._list_map_bool_bool(!(options.columns === false), !(options.module === false)).ptr;
        return map;
    }

    updateHash(hash) {
        hash.update(this._value);
        if (this._originalSource) hash.update(this._originalSource);
    }
}

require("./SourceAndMapMixin")(SourceMapSource.prototype);

SourceMapSource.prototype.type = "SourceMapSource";
module.exports = SourceMapSource;
