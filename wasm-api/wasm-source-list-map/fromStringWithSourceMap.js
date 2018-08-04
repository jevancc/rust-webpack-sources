"use strict";

const SourceListMap = require("./SourceListMap");
const StringCache = require("../StringCache");
const Types = require("./Types");
const wasm = require("../build/webpack_sources");

module.exports = function fromStringWithSourceMap(code, map) {
    let sources = (map.sources || []).map(StringCache.add);
    let sourcesContent = (map.sourcesContent || []).map(
        StringCache.addUnchecked
    );
    let mappings = map.mappings;

    let slp = new SourceListMap(Types.Null);
    slp.ptr = wasm._from_string_with_source_map(
        code,
        sources,
        sourcesContent,
        mappings
    ).ptr;

    return slp;
};
