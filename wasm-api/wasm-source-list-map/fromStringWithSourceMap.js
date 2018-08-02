"use strict";

let wasm = require("../build/webpack_sources");
let SourceListMap = require("./SourceListMap");
let StringCache = require("../StringCache");

module.exports = function fromStringWithSourceMap(code, map) {
    let sources = (map.sources || []).map(StringCache.add);
    let sourcesContent = (map.sourcesContent || []).map(
        StringCache.addUnchecked
    );
    let mappings = map.mappings;

    let slp = new SourceListMap(-2);
    slp.ptr = wasm._from_string_with_source_map(
        code,
        sources,
        sourcesContent,
        mappings
    ).ptr;

    return slp;
};
