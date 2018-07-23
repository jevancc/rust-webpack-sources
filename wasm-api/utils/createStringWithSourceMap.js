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
    let _cachedS = Null;
    let _cachedSources = Null;
    let _cachedSourcesContent = Null;
    let _cachedMappings = Null;
    let _cachedNames = Null;
    let modified = false;

    WasmObjectPool.add(WasmStringWithSourceMap);

    sourceMap.file = file;
    sourceMap.version = 3;

    defineEnum(
        sourceMap,
        "sources",
        function() {
            if (_cachedSources === Null) {
                _cachedSources = StringCache.resolveIntArray(
                    WasmStringWithSourceMap.sources()
                );
            }
            return _cachedSources;
        },
        function(v) {
            _cachedSources = v;
            modified = true;
        }
    );

    defineEnum(
        sourceMap,
        "sourcesContent",
        function() {
            if (_cachedSourcesContent === Null) {
                _cachedSourcesContent = StringCache.resolveIntArray(
                    WasmStringWithSourceMap.sources_content()
                );
                if (_cachedSourcesContent.length === 0) {
                    _cachedSourcesContent = undefined;
                }
            }
            return (_cachedSourcesContent || []).length > 0
                ? _cachedSourcesContent
                : undefined;
        },
        function(v) {
            _cachedSourcesContent = v;
            modified = true;
        }
    );

    defineEnum(
        sourceMap,
        "mappings",
        function() {
            if (_cachedMappings === Null) {
                _cachedMappings = WasmStringWithSourceMap.mappings();
            }
            return _cachedMappings;
        },
        function(v) {
            _cachedMappings = v;
            modified = true;
        }
    );

    if (isNames) {
        defineEnum(
            sourceMap,
            "names",
            function() {
                if (_cachedNames === Null) {
                    _cachedNames = StringCache.resolveIntArray(
                        WasmStringWithSourceMap.names()
                    );
                }
                return _cachedNames;
            },
            function(v) {
                _cachedNames = v;
                modified = true;
            }
        );
    }

    defineConst(sourceMap, "wasmObj", function() {
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
            if (_cachedS === Null) {
                _cachedS = WasmStringWithSourceMap.s();
            }
            return _cachedS;
        },
        function(v) {
            _cachedS = v;
        }
    );

    return stringWithSourceMap;
}
module.exports = createStringWithSourceMap;
