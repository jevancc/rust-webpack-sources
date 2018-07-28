let StringCache = require("../StringCache");
let WasmObjectPool = require("../WasmObjectPool");
let wasm = require("../build/webpack_sources");

function defineConst(obj, prop, getter) {
    Object.defineProperty(obj, prop, {
        configurable: false,
        enumerable: false,
        get: getter,
        set: function(v) {
            throw new Error("Constant source map can not be modified");
        }
    });
}

function defineEnum(obj, prop, getter, setter) {
    Object.defineProperty(obj, prop, {
        configurable: false,
        enumerable: true,
        get: getter,
        set: setter
    });
}

function defineHidden(obj, prop, val) {
    Object.defineProperty(obj, prop, {
        configurable: false,
        enumerable: false,
        writable: true,
        value: val
    });
}

const Null = Symbol(null);
function createStringWithSourceMap(WasmStringWithSourceMap, file, isNames) {
    let stringWithSourceMap = new Object();
    let sourceMap = new Object();
    let cachedS = Null;
    let cachedSources = Null;
    let cachedSourcesContent = Null;
    let cachedMappings = Null;
    let cachedNames = Null;
    let modified = false;

    WasmObjectPool.add(WasmStringWithSourceMap);

    sourceMap.file = file;
    sourceMap.version = 3;

    defineEnum(
        sourceMap,
        "sources",
        function() {
            if (cachedSources === Null) {
                cachedSources = StringCache.resolveIntArray(
                    WasmStringWithSourceMap.sources()
                );
            }
            return cachedSources;
        },
        function(v) {
            cachedSources = v;
            modified = true;
        }
    );

    defineEnum(
        sourceMap,
        "sourcesContent",
        function() {
            if (cachedSourcesContent === Null) {
                cachedSourcesContent = StringCache.resolveIntArray(
                    WasmStringWithSourceMap.sources_content()
                );
                if (cachedSourcesContent.length === 0) {
                    cachedSourcesContent = undefined;
                }
            }
            return (cachedSourcesContent || []).length > 0
                ? cachedSourcesContent
                : undefined;
        },
        function(v) {
            cachedSourcesContent = v;
            modified = true;
        }
    );

    defineEnum(
        sourceMap,
        "mappings",
        function() {
            if (cachedMappings === Null) {
                cachedMappings = WasmStringWithSourceMap.mappings();
            }
            return cachedMappings;
        },
        function(v) {
            cachedMappings = v;
            modified = true;
        }
    );

    if (isNames) {
        defineEnum(
            sourceMap,
            "names",
            function() {
                if (cachedNames === Null) {
                    cachedNames = StringCache.resolveIntArray(
                        WasmStringWithSourceMap.names()
                    );
                }
                return cachedNames;
            },
            function(v) {
                cachedNames = v;
                modified = true;
            }
        );
    }

    defineConst(sourceMap, "_wasmObj", function() {
        if (!modified && WasmStringWithSourceMap.ptr) {
            return WasmStringWithSourceMap;
        } else {
            return null;
        }
    });

    stringWithSourceMap.map = sourceMap;
    defineEnum(
        stringWithSourceMap,
        "source",
        function() {
            if (cachedS === Null) {
                cachedS = WasmStringWithSourceMap.s();
            }
            return cachedS;
        },
        function(v) {
            cachedS = v;
        }
    );

    return stringWithSourceMap;
}
module.exports = createStringWithSourceMap;
