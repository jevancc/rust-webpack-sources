"use strict";

var wasm = require("../build/webpack_sources");
var SourceListMap = require("./SourceListMap");

module.exports = function fromStringWithSourceMap(code, map) {
    var sources = StringVec(map.sources || []);
    var sourcesContent = StringVec(map.sourcesContent || []);
    var mappings = map.mappings;

    var slp = new SourceListMap(-1);
    slp.ptr = wasm._from_string_with_source_map(
        code,
        sources,
        sourcesContent,
        mappings
    ).ptr;

    return slp;
};

var StringVec = require("./utils").StringVec;
