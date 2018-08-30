/* tslint:disable */
var wasm;

const TextEncoder = require("util").TextEncoder;

let cachedEncoder = new TextEncoder("utf-8");

let cachegetUint8Memory = null;
function getUint8Memory() {
    if (
        cachegetUint8Memory === null ||
        cachegetUint8Memory.buffer !== wasm.memory.buffer
    ) {
        cachegetUint8Memory = new Uint8Array(wasm.memory.buffer);
    }
    return cachegetUint8Memory;
}

function passStringToWasm(arg) {
    const buf = cachedEncoder.encode(arg);
    const ptr = wasm.__wbindgen_malloc(buf.length);
    getUint8Memory().set(buf, ptr);
    return [ptr, buf.length];
}

const TextDecoder = require("util").TextDecoder;

let cachedDecoder = new TextDecoder("utf-8");

function getStringFromWasm(ptr, len) {
    return cachedDecoder.decode(getUint8Memory().subarray(ptr, ptr + len));
}

let cachedGlobalArgumentPtr = null;
function globalArgumentPtr() {
    if (cachedGlobalArgumentPtr === null) {
        cachedGlobalArgumentPtr = wasm.__wbindgen_global_argument_ptr();
    }
    return cachedGlobalArgumentPtr;
}

let cachegetUint32Memory = null;
function getUint32Memory() {
    if (
        cachegetUint32Memory === null ||
        cachegetUint32Memory.buffer !== wasm.memory.buffer
    ) {
        cachegetUint32Memory = new Uint32Array(wasm.memory.buffer);
    }
    return cachegetUint32Memory;
}

function passArray32ToWasm(arg) {
    const ptr = wasm.__wbindgen_malloc(arg.length * 4);
    getUint32Memory().set(arg, ptr / 4);
    return [ptr, arg.length];
}
/**
 * @param {string} arg0
 * @param {Int32Array} arg1
 * @param {Int32Array} arg2
 * @param {string} arg3
 * @returns {_SourceListMap}
 */
module.exports._from_string_with_source_map = function(arg0, arg1, arg2, arg3) {
    const [ptr0, len0] = passStringToWasm(arg0);
    const [ptr1, len1] = passArray32ToWasm(arg1);
    const [ptr2, len2] = passArray32ToWasm(arg2);
    const [ptr3, len3] = passStringToWasm(arg3);
    try {
        return _SourceListMap.__construct(
            wasm._from_string_with_source_map(
                ptr0,
                len0,
                ptr1,
                len1,
                ptr2,
                len2,
                ptr3,
                len3
            )
        );
    } finally {
        wasm.__wbindgen_free(ptr1, len1 * 4);
        wasm.__wbindgen_free(ptr2, len2 * 4);
    }
};

let cachegetInt32Memory = null;
function getInt32Memory() {
    if (
        cachegetInt32Memory === null ||
        cachegetInt32Memory.buffer !== wasm.memory.buffer
    ) {
        cachegetInt32Memory = new Int32Array(wasm.memory.buffer);
    }
    return cachegetInt32Memory;
}

function getArrayI32FromWasm(ptr, len) {
    return getInt32Memory().subarray(ptr / 4, ptr / 4 + len);
}
/**
 * @param {_SourceListMap} arg0
 * @returns {_SourceListMap}
 */
module.exports._sourcelistmap_map_generated_code_identical = function(arg0) {
    const ptr0 = arg0.ptr;
    if (ptr0 === 0) {
        throw new Error("Attempt to use a moved value");
    }
    arg0.ptr = 0;
    return _SourceListMap.__construct(
        wasm._sourcelistmap_map_generated_code_identical(ptr0)
    );
};

/**
 * @param {_SourceListMap} arg0
 * @returns {_SourceListMap}
 */
module.exports._sourcelistmap_map_generated_code_test = function(arg0) {
    const ptr0 = arg0.ptr;
    if (ptr0 === 0) {
        throw new Error("Attempt to use a moved value");
    }
    arg0.ptr = 0;
    return _SourceListMap.__construct(
        wasm._sourcelistmap_map_generated_code_test(ptr0)
    );
};

/**
 * @param {_SourceListMap} arg0
 * @param {string} arg1
 * @returns {_SourceListMap}
 */
module.exports._sourcelistmap_map_generated_code_prefix = function(arg0, arg1) {
    const ptr0 = arg0.ptr;
    if (ptr0 === 0) {
        throw new Error("Attempt to use a moved value");
    }
    arg0.ptr = 0;
    const [ptr1, len1] = passStringToWasm(arg1);
    return _SourceListMap.__construct(
        wasm._sourcelistmap_map_generated_code_prefix(ptr0, ptr1, len1)
    );
};

function free_LineToLineMappedSource(ptr) {
    wasm.__wbg__linetolinemappedsource_free(ptr);
}
/**
 */
class _LineToLineMappedSource {
    static __construct(ptr) {
        return new _LineToLineMappedSource(ptr);
    }

    constructor(ptr) {
        this.ptr = ptr;
    }

    free() {
        const ptr = this.ptr;
        this.ptr = 0;
        free_LineToLineMappedSource(ptr);
    }
    /**
     * @param {string} arg0
     * @param {number} arg1
     * @param {number} arg2
     * @returns {_LineToLineMappedSource}
     */
    static _new_string_sidx_sidx(arg0, arg1, arg2) {
        const [ptr0, len0] = passStringToWasm(arg0);
        return _LineToLineMappedSource.__construct(
            wasm._linetolinemappedsource__new_string_sidx_sidx(
                ptr0,
                len0,
                arg1,
                arg2
            )
        );
    }
    /**
     * @returns {string}
     */
    _source() {
        if (this.ptr === 0) {
            throw new Error("Attempt to use a moved value");
        }
        const retptr = globalArgumentPtr();
        wasm._linetolinemappedsource__source(retptr, this.ptr);
        const mem = getUint32Memory();
        const rustptr = mem[retptr / 4];
        const rustlen = mem[retptr / 4 + 1];

        const realRet = getStringFromWasm(rustptr, rustlen).slice();
        wasm.__wbindgen_free(rustptr, rustlen * 1);
        return realRet;
    }
    /**
     * @returns {number}
     */
    _size() {
        if (this.ptr === 0) {
            throw new Error("Attempt to use a moved value");
        }
        return wasm._linetolinemappedsource__size(this.ptr);
    }
    /**
     * @param {boolean} arg0
     * @param {boolean} arg1
     * @returns {_SourceListMap}
     */
    _list_map_bool_bool(arg0, arg1) {
        if (this.ptr === 0) {
            throw new Error("Attempt to use a moved value");
        }
        return _SourceListMap.__construct(
            wasm._linetolinemappedsource__list_map_bool_bool(
                this.ptr,
                arg0 ? 1 : 0,
                arg1 ? 1 : 0
            )
        );
    }
    /**
     * @param {boolean} arg0
     * @param {boolean} arg1
     * @returns {_MSourceNode}
     */
    _node_bool_bool(arg0, arg1) {
        if (this.ptr === 0) {
            throw new Error("Attempt to use a moved value");
        }
        return _MSourceNode.__construct(
            wasm._linetolinemappedsource__node_bool_bool(
                this.ptr,
                arg0 ? 1 : 0,
                arg1 ? 1 : 0
            )
        );
    }
}
module.exports._LineToLineMappedSource = _LineToLineMappedSource;

function free_OriginalSource(ptr) {
    wasm.__wbg__originalsource_free(ptr);
}
/**
 */
class _OriginalSource {
    static __construct(ptr) {
        return new _OriginalSource(ptr);
    }

    constructor(ptr) {
        this.ptr = ptr;
    }

    free() {
        const ptr = this.ptr;
        this.ptr = 0;
        free_OriginalSource(ptr);
    }
    /**
     * @param {string} arg0
     * @param {number} arg1
     * @param {number} arg2
     * @returns {_OriginalSource}
     */
    static _new_string_sidx_sidx(arg0, arg1, arg2) {
        const [ptr0, len0] = passStringToWasm(arg0);
        return _OriginalSource.__construct(
            wasm._originalsource__new_string_sidx_sidx(ptr0, len0, arg1, arg2)
        );
    }
    /**
     * @returns {string}
     */
    _source() {
        if (this.ptr === 0) {
            throw new Error("Attempt to use a moved value");
        }
        const retptr = globalArgumentPtr();
        wasm._originalsource__source(retptr, this.ptr);
        const mem = getUint32Memory();
        const rustptr = mem[retptr / 4];
        const rustlen = mem[retptr / 4 + 1];

        const realRet = getStringFromWasm(rustptr, rustlen).slice();
        wasm.__wbindgen_free(rustptr, rustlen * 1);
        return realRet;
    }
    /**
     * @returns {number}
     */
    _size() {
        if (this.ptr === 0) {
            throw new Error("Attempt to use a moved value");
        }
        return wasm._originalsource__size(this.ptr);
    }
    /**
     * @returns {number}
     */
    _name() {
        if (this.ptr === 0) {
            throw new Error("Attempt to use a moved value");
        }
        return wasm._originalsource__name(this.ptr);
    }
    /**
     * @param {boolean} arg0
     * @param {boolean} arg1
     * @returns {_SourceListMap}
     */
    _list_map_bool_bool(arg0, arg1) {
        if (this.ptr === 0) {
            throw new Error("Attempt to use a moved value");
        }
        return _SourceListMap.__construct(
            wasm._originalsource__list_map_bool_bool(
                this.ptr,
                arg0 ? 1 : 0,
                arg1 ? 1 : 0
            )
        );
    }
    /**
     * @param {boolean} arg0
     * @param {boolean} arg1
     * @returns {_MSourceNode}
     */
    _node_bool_bool(arg0, arg1) {
        if (this.ptr === 0) {
            throw new Error("Attempt to use a moved value");
        }
        return _MSourceNode.__construct(
            wasm._originalsource__node_bool_bool(
                this.ptr,
                arg0 ? 1 : 0,
                arg1 ? 1 : 0
            )
        );
    }
}
module.exports._OriginalSource = _OriginalSource;

function freeNodeVec(ptr) {
    wasm.__wbg_nodevec_free(ptr);
}
/**
 */
class NodeVec {
    static __construct(ptr) {
        return new NodeVec(ptr);
    }

    constructor(ptr) {
        this.ptr = ptr;
    }

    free() {
        const ptr = this.ptr;
        this.ptr = 0;
        freeNodeVec(ptr);
    }
    /**
     * @returns {NodeVec}
     */
    static new() {
        return NodeVec.__construct(wasm.nodevec_new());
    }
    /**
     * @param {string} arg0
     * @returns {void}
     */
    push_string(arg0) {
        if (this.ptr === 0) {
            throw new Error("Attempt to use a moved value");
        }
        const [ptr0, len0] = passStringToWasm(arg0);
        return wasm.nodevec_push_string(this.ptr, ptr0, len0);
    }
    /**
     * @param {_SourceNode} arg0
     * @returns {void}
     */
    push_sourcenode(arg0) {
        if (this.ptr === 0) {
            throw new Error("Attempt to use a moved value");
        }
        return wasm.nodevec_push_sourcenode(this.ptr, arg0.ptr);
    }
    /**
     * @param {_CodeNode} arg0
     * @returns {void}
     */
    push_codenode(arg0) {
        if (this.ptr === 0) {
            throw new Error("Attempt to use a moved value");
        }
        return wasm.nodevec_push_codenode(this.ptr, arg0.ptr);
    }
    /**
     * @param {_SingleLineNode} arg0
     * @returns {void}
     */
    push_singlelinenode(arg0) {
        if (this.ptr === 0) {
            throw new Error("Attempt to use a moved value");
        }
        return wasm.nodevec_push_singlelinenode(this.ptr, arg0.ptr);
    }
    /**
     * @param {_SourceListMap} arg0
     * @returns {void}
     */
    push_sourcelistmap(arg0) {
        if (this.ptr === 0) {
            throw new Error("Attempt to use a moved value");
        }
        return wasm.nodevec_push_sourcelistmap(this.ptr, arg0.ptr);
    }
}
module.exports.NodeVec = NodeVec;

function free_ConcatSource(ptr) {
    wasm.__wbg__concatsource_free(ptr);
}
/**
 */
class _ConcatSource {
    static __construct(ptr) {
        return new _ConcatSource(ptr);
    }

    constructor(ptr) {
        this.ptr = ptr;
    }

    free() {
        const ptr = this.ptr;
        this.ptr = 0;
        free_ConcatSource(ptr);
    }
    /**
     * @returns {_ConcatSource}
     */
    static _new() {
        return _ConcatSource.__construct(wasm._concatsource__new());
    }
    /**
     * @param {string} arg0
     * @returns {void}
     */
    _add_string(arg0) {
        if (this.ptr === 0) {
            throw new Error("Attempt to use a moved value");
        }
        const [ptr0, len0] = passStringToWasm(arg0);
        return wasm._concatsource__add_string(this.ptr, ptr0, len0);
    }
    /**
     * @param {_RawSource} arg0
     * @returns {void}
     */
    _add_raw_source(arg0) {
        if (this.ptr === 0) {
            throw new Error("Attempt to use a moved value");
        }
        return wasm._concatsource__add_raw_source(this.ptr, arg0.ptr);
    }
    /**
     * @param {_OriginalSource} arg0
     * @returns {void}
     */
    _add_original_source(arg0) {
        if (this.ptr === 0) {
            throw new Error("Attempt to use a moved value");
        }
        return wasm._concatsource__add_original_source(this.ptr, arg0.ptr);
    }
    /**
     * @param {_ReplaceSource} arg0
     * @returns {void}
     */
    _add_replace_source(arg0) {
        if (this.ptr === 0) {
            throw new Error("Attempt to use a moved value");
        }
        return wasm._concatsource__add_replace_source(this.ptr, arg0.ptr);
    }
    /**
     * @param {_PrefixSource} arg0
     * @returns {void}
     */
    _add_prefix_source(arg0) {
        if (this.ptr === 0) {
            throw new Error("Attempt to use a moved value");
        }
        return wasm._concatsource__add_prefix_source(this.ptr, arg0.ptr);
    }
    /**
     * @param {_ConcatSource} arg0
     * @returns {void}
     */
    _add_concat_source(arg0) {
        if (this.ptr === 0) {
            throw new Error("Attempt to use a moved value");
        }
        return wasm._concatsource__add_concat_source(this.ptr, arg0.ptr);
    }
    /**
     * @param {_LineToLineMappedSource} arg0
     * @returns {void}
     */
    _add_line_to_line_mapped_source(arg0) {
        if (this.ptr === 0) {
            throw new Error("Attempt to use a moved value");
        }
        return wasm._concatsource__add_line_to_line_mapped_source(
            this.ptr,
            arg0.ptr
        );
    }
    /**
     * @param {_SourceMapSource} arg0
     * @returns {void}
     */
    _add_source_map_source(arg0) {
        if (this.ptr === 0) {
            throw new Error("Attempt to use a moved value");
        }
        return wasm._concatsource__add_source_map_source(this.ptr, arg0.ptr);
    }
    /**
     * @returns {string}
     */
    _source() {
        if (this.ptr === 0) {
            throw new Error("Attempt to use a moved value");
        }
        const retptr = globalArgumentPtr();
        wasm._concatsource__source(retptr, this.ptr);
        const mem = getUint32Memory();
        const rustptr = mem[retptr / 4];
        const rustlen = mem[retptr / 4 + 1];

        const realRet = getStringFromWasm(rustptr, rustlen).slice();
        wasm.__wbindgen_free(rustptr, rustlen * 1);
        return realRet;
    }
    /**
     * @returns {number}
     */
    _size() {
        if (this.ptr === 0) {
            throw new Error("Attempt to use a moved value");
        }
        return wasm._concatsource__size(this.ptr);
    }
    /**
     * @param {boolean} arg0
     * @param {boolean} arg1
     * @returns {_SourceListMap}
     */
    _list_map_bool_bool(arg0, arg1) {
        if (this.ptr === 0) {
            throw new Error("Attempt to use a moved value");
        }
        return _SourceListMap.__construct(
            wasm._concatsource__list_map_bool_bool(
                this.ptr,
                arg0 ? 1 : 0,
                arg1 ? 1 : 0
            )
        );
    }
    /**
     * @param {boolean} arg0
     * @param {boolean} arg1
     * @returns {_MSourceNode}
     */
    _node_bool_bool(arg0, arg1) {
        if (this.ptr === 0) {
            throw new Error("Attempt to use a moved value");
        }
        return _MSourceNode.__construct(
            wasm._concatsource__node_bool_bool(
                this.ptr,
                arg0 ? 1 : 0,
                arg1 ? 1 : 0
            )
        );
    }
}
module.exports._ConcatSource = _ConcatSource;

function free_PrefixSource(ptr) {
    wasm.__wbg__prefixsource_free(ptr);
}
/**
 */
class _PrefixSource {
    static __construct(ptr) {
        return new _PrefixSource(ptr);
    }

    constructor(ptr) {
        this.ptr = ptr;
    }

    free() {
        const ptr = this.ptr;
        this.ptr = 0;
        free_PrefixSource(ptr);
    }
    /**
     * @param {string} arg0
     * @param {string} arg1
     * @returns {_PrefixSource}
     */
    static _new_string_string(arg0, arg1) {
        const [ptr0, len0] = passStringToWasm(arg0);
        const [ptr1, len1] = passStringToWasm(arg1);
        return _PrefixSource.__construct(
            wasm._prefixsource__new_string_string(ptr0, len0, ptr1, len1)
        );
    }
    /**
     * @param {string} arg0
     * @param {_RawSource} arg1
     * @returns {_PrefixSource}
     */
    static _new_string_raw_source(arg0, arg1) {
        const [ptr0, len0] = passStringToWasm(arg0);
        return _PrefixSource.__construct(
            wasm._prefixsource__new_string_raw_source(ptr0, len0, arg1.ptr)
        );
    }
    /**
     * @param {string} arg0
     * @param {_OriginalSource} arg1
     * @returns {_PrefixSource}
     */
    static _new_string_original_source(arg0, arg1) {
        const [ptr0, len0] = passStringToWasm(arg0);
        return _PrefixSource.__construct(
            wasm._prefixsource__new_string_original_source(ptr0, len0, arg1.ptr)
        );
    }
    /**
     * @param {string} arg0
     * @param {_ReplaceSource} arg1
     * @returns {_PrefixSource}
     */
    static _new_string_replace_source(arg0, arg1) {
        const [ptr0, len0] = passStringToWasm(arg0);
        return _PrefixSource.__construct(
            wasm._prefixsource__new_string_replace_source(ptr0, len0, arg1.ptr)
        );
    }
    /**
     * @param {string} arg0
     * @param {_PrefixSource} arg1
     * @returns {_PrefixSource}
     */
    static _new_string_prefix_source(arg0, arg1) {
        const [ptr0, len0] = passStringToWasm(arg0);
        return _PrefixSource.__construct(
            wasm._prefixsource__new_string_prefix_source(ptr0, len0, arg1.ptr)
        );
    }
    /**
     * @param {string} arg0
     * @param {_ConcatSource} arg1
     * @returns {_PrefixSource}
     */
    static _new_string_concat_source(arg0, arg1) {
        const [ptr0, len0] = passStringToWasm(arg0);
        return _PrefixSource.__construct(
            wasm._prefixsource__new_string_concat_source(ptr0, len0, arg1.ptr)
        );
    }
    /**
     * @param {string} arg0
     * @param {_LineToLineMappedSource} arg1
     * @returns {_PrefixSource}
     */
    static _new_string_line_to_line_mapped_source(arg0, arg1) {
        const [ptr0, len0] = passStringToWasm(arg0);
        return _PrefixSource.__construct(
            wasm._prefixsource__new_string_line_to_line_mapped_source(
                ptr0,
                len0,
                arg1.ptr
            )
        );
    }
    /**
     * @param {string} arg0
     * @param {_SourceMapSource} arg1
     * @returns {_PrefixSource}
     */
    static _new_string_source_map_source(arg0, arg1) {
        const [ptr0, len0] = passStringToWasm(arg0);
        return _PrefixSource.__construct(
            wasm._prefixsource__new_string_source_map_source(
                ptr0,
                len0,
                arg1.ptr
            )
        );
    }
    /**
     * @returns {string}
     */
    _source() {
        if (this.ptr === 0) {
            throw new Error("Attempt to use a moved value");
        }
        const retptr = globalArgumentPtr();
        wasm._prefixsource__source(retptr, this.ptr);
        const mem = getUint32Memory();
        const rustptr = mem[retptr / 4];
        const rustlen = mem[retptr / 4 + 1];

        const realRet = getStringFromWasm(rustptr, rustlen).slice();
        wasm.__wbindgen_free(rustptr, rustlen * 1);
        return realRet;
    }
    /**
     * @returns {number}
     */
    _size() {
        if (this.ptr === 0) {
            throw new Error("Attempt to use a moved value");
        }
        return wasm._prefixsource__size(this.ptr);
    }
    /**
     * @param {boolean} arg0
     * @param {boolean} arg1
     * @returns {_SourceListMap}
     */
    _list_map_bool_bool(arg0, arg1) {
        if (this.ptr === 0) {
            throw new Error("Attempt to use a moved value");
        }
        return _SourceListMap.__construct(
            wasm._prefixsource__list_map_bool_bool(
                this.ptr,
                arg0 ? 1 : 0,
                arg1 ? 1 : 0
            )
        );
    }
    /**
     * @param {boolean} arg0
     * @param {boolean} arg1
     * @returns {_MSourceNode}
     */
    _node_bool_bool(arg0, arg1) {
        if (this.ptr === 0) {
            throw new Error("Attempt to use a moved value");
        }
        return _MSourceNode.__construct(
            wasm._prefixsource__node_bool_bool(
                this.ptr,
                arg0 ? 1 : 0,
                arg1 ? 1 : 0
            )
        );
    }
}
module.exports._PrefixSource = _PrefixSource;

function freeJsStringWithSourceMap(ptr) {
    wasm.__wbg_jsstringwithsourcemap_free(ptr);
}
/**
 */
class JsStringWithSourceMap {
    static __construct(ptr) {
        return new JsStringWithSourceMap(ptr);
    }

    constructor(ptr) {
        this.ptr = ptr;
    }

    free() {
        const ptr = this.ptr;
        this.ptr = 0;
        freeJsStringWithSourceMap(ptr);
    }
    /**
     * @returns {string}
     */
    s() {
        if (this.ptr === 0) {
            throw new Error("Attempt to use a moved value");
        }
        const retptr = globalArgumentPtr();
        wasm.jsstringwithsourcemap_s(retptr, this.ptr);
        const mem = getUint32Memory();
        const rustptr = mem[retptr / 4];
        const rustlen = mem[retptr / 4 + 1];

        const realRet = getStringFromWasm(rustptr, rustlen).slice();
        wasm.__wbindgen_free(rustptr, rustlen * 1);
        return realRet;
    }
    /**
     * @returns {number}
     */
    version() {
        if (this.ptr === 0) {
            throw new Error("Attempt to use a moved value");
        }
        return wasm.jsstringwithsourcemap_version(this.ptr);
    }
    /**
     * @returns {number}
     */
    file() {
        if (this.ptr === 0) {
            throw new Error("Attempt to use a moved value");
        }
        return wasm.jsstringwithsourcemap_file(this.ptr);
    }
    /**
     * @returns {Int32Array}
     */
    sources() {
        if (this.ptr === 0) {
            throw new Error("Attempt to use a moved value");
        }
        const retptr = globalArgumentPtr();
        wasm.jsstringwithsourcemap_sources(retptr, this.ptr);
        const mem = getUint32Memory();
        const rustptr = mem[retptr / 4];
        const rustlen = mem[retptr / 4 + 1];

        const realRet = getArrayI32FromWasm(rustptr, rustlen).slice();
        wasm.__wbindgen_free(rustptr, rustlen * 4);
        return realRet;
    }
    /**
     * @returns {Int32Array}
     */
    sources_content() {
        if (this.ptr === 0) {
            throw new Error("Attempt to use a moved value");
        }
        const retptr = globalArgumentPtr();
        wasm.jsstringwithsourcemap_sources_content(retptr, this.ptr);
        const mem = getUint32Memory();
        const rustptr = mem[retptr / 4];
        const rustlen = mem[retptr / 4 + 1];

        const realRet = getArrayI32FromWasm(rustptr, rustlen).slice();
        wasm.__wbindgen_free(rustptr, rustlen * 4);
        return realRet;
    }
    /**
     * @returns {Int32Array}
     */
    names() {
        if (this.ptr === 0) {
            throw new Error("Attempt to use a moved value");
        }
        const retptr = globalArgumentPtr();
        wasm.jsstringwithsourcemap_names(retptr, this.ptr);
        const mem = getUint32Memory();
        const rustptr = mem[retptr / 4];
        const rustlen = mem[retptr / 4 + 1];

        const realRet = getArrayI32FromWasm(rustptr, rustlen).slice();
        wasm.__wbindgen_free(rustptr, rustlen * 4);
        return realRet;
    }
    /**
     * @returns {string}
     */
    mappings() {
        if (this.ptr === 0) {
            throw new Error("Attempt to use a moved value");
        }
        const retptr = globalArgumentPtr();
        wasm.jsstringwithsourcemap_mappings(retptr, this.ptr);
        const mem = getUint32Memory();
        const rustptr = mem[retptr / 4];
        const rustlen = mem[retptr / 4 + 1];

        const realRet = getStringFromWasm(rustptr, rustlen).slice();
        wasm.__wbindgen_free(rustptr, rustlen * 1);
        return realRet;
    }
}
module.exports.JsStringWithSourceMap = JsStringWithSourceMap;

function free_SourceMapSource(ptr) {
    wasm.__wbg__sourcemapsource_free(ptr);
}
/**
 */
class _SourceMapSource {
    static __construct(ptr) {
        return new _SourceMapSource(ptr);
    }

    constructor(ptr) {
        this.ptr = ptr;
    }

    free() {
        const ptr = this.ptr;
        this.ptr = 0;
        free_SourceMapSource(ptr);
    }
    /**
     * @param {string} arg0
     * @param {number} arg1
     * @param {number} arg2
     * @param {Int32Array} arg3
     * @param {Int32Array} arg4
     * @param {string} arg5
     * @param {Int32Array} arg6
     * @returns {_SourceMapSource}
     */
    static _new_string_sidx_string_map(
        arg0,
        arg1,
        arg2,
        arg3,
        arg4,
        arg5,
        arg6
    ) {
        const [ptr0, len0] = passStringToWasm(arg0);
        const [ptr3, len3] = passArray32ToWasm(arg3);
        const [ptr4, len4] = passArray32ToWasm(arg4);
        const [ptr5, len5] = passStringToWasm(arg5);
        const [ptr6, len6] = passArray32ToWasm(arg6);
        try {
            return _SourceMapSource.__construct(
                wasm._sourcemapsource__new_string_sidx_string_map(
                    ptr0,
                    len0,
                    arg1,
                    arg2,
                    ptr3,
                    len3,
                    ptr4,
                    len4,
                    ptr5,
                    len5,
                    ptr6,
                    len6
                )
            );
        } finally {
            wasm.__wbindgen_free(ptr3, len3 * 4);
            wasm.__wbindgen_free(ptr4, len4 * 4);
            wasm.__wbindgen_free(ptr6, len6 * 4);
        }
    }
    /**
     * @param {string} arg0
     * @param {number} arg1
     * @param {number} arg2
     * @param {JsStringWithSourceMap} arg3
     * @returns {_SourceMapSource}
     */
    static _new_string_sidx_string_wasmmap(arg0, arg1, arg2, arg3) {
        const [ptr0, len0] = passStringToWasm(arg0);
        const ptr3 = arg3.ptr;
        if (ptr3 === 0) {
            throw new Error("Attempt to use a moved value");
        }
        arg3.ptr = 0;
        return _SourceMapSource.__construct(
            wasm._sourcemapsource__new_string_sidx_string_wasmmap(
                ptr0,
                len0,
                arg1,
                arg2,
                ptr3
            )
        );
    }
    /**
     * @param {number} arg0
     * @returns {void}
     */
    _set_original_source_sidx(arg0) {
        if (this.ptr === 0) {
            throw new Error("Attempt to use a moved value");
        }
        return wasm._sourcemapsource__set_original_source_sidx(this.ptr, arg0);
    }
    /**
     * @param {Int32Array} arg0
     * @param {Int32Array} arg1
     * @param {string} arg2
     * @param {Int32Array} arg3
     * @returns {void}
     */
    _set_inner_source_map_map(arg0, arg1, arg2, arg3) {
        if (this.ptr === 0) {
            throw new Error("Attempt to use a moved value");
        }
        const [ptr0, len0] = passArray32ToWasm(arg0);
        const [ptr1, len1] = passArray32ToWasm(arg1);
        const [ptr2, len2] = passStringToWasm(arg2);
        const [ptr3, len3] = passArray32ToWasm(arg3);
        try {
            return wasm._sourcemapsource__set_inner_source_map_map(
                this.ptr,
                ptr0,
                len0,
                ptr1,
                len1,
                ptr2,
                len2,
                ptr3,
                len3
            );
        } finally {
            wasm.__wbindgen_free(ptr0, len0 * 4);
            wasm.__wbindgen_free(ptr1, len1 * 4);
            wasm.__wbindgen_free(ptr3, len3 * 4);
        }
    }
    /**
     * @param {JsStringWithSourceMap} arg0
     * @returns {void}
     */
    _set_inner_source_map_wasmmap(arg0) {
        if (this.ptr === 0) {
            throw new Error("Attempt to use a moved value");
        }
        const ptr0 = arg0.ptr;
        if (ptr0 === 0) {
            throw new Error("Attempt to use a moved value");
        }
        arg0.ptr = 0;
        return wasm._sourcemapsource__set_inner_source_map_wasmmap(
            this.ptr,
            ptr0
        );
    }
    /**
     * @returns {string}
     */
    _source() {
        if (this.ptr === 0) {
            throw new Error("Attempt to use a moved value");
        }
        const retptr = globalArgumentPtr();
        wasm._sourcemapsource__source(retptr, this.ptr);
        const mem = getUint32Memory();
        const rustptr = mem[retptr / 4];
        const rustlen = mem[retptr / 4 + 1];

        const realRet = getStringFromWasm(rustptr, rustlen).slice();
        wasm.__wbindgen_free(rustptr, rustlen * 1);
        return realRet;
    }
    /**
     * @returns {number}
     */
    _size() {
        if (this.ptr === 0) {
            throw new Error("Attempt to use a moved value");
        }
        return wasm._sourcemapsource__size(this.ptr);
    }
    /**
     * @param {boolean} arg0
     * @param {boolean} arg1
     * @returns {_SourceListMap}
     */
    _list_map_bool_bool(arg0, arg1) {
        if (this.ptr === 0) {
            throw new Error("Attempt to use a moved value");
        }
        return _SourceListMap.__construct(
            wasm._sourcemapsource__list_map_bool_bool(
                this.ptr,
                arg0 ? 1 : 0,
                arg1 ? 1 : 0
            )
        );
    }
    /**
     * @param {boolean} arg0
     * @param {boolean} arg1
     * @returns {_MSourceNode}
     */
    _node_bool_bool(arg0, arg1) {
        if (this.ptr === 0) {
            throw new Error("Attempt to use a moved value");
        }
        return _MSourceNode.__construct(
            wasm._sourcemapsource__node_bool_bool(
                this.ptr,
                arg0 ? 1 : 0,
                arg1 ? 1 : 0
            )
        );
    }
}
module.exports._SourceMapSource = _SourceMapSource;

function free_SourceListMap(ptr) {
    wasm.__wbg__sourcelistmap_free(ptr);
}
/**
 */
class _SourceListMap {
    static __construct(ptr) {
        return new _SourceListMap(ptr);
    }

    constructor(ptr) {
        this.ptr = ptr;
    }

    free() {
        const ptr = this.ptr;
        this.ptr = 0;
        free_SourceListMap(ptr);
    }
    /**
     * @returns {_SourceListMap}
     */
    static _new() {
        return _SourceListMap.__construct(wasm._sourcelistmap__new());
    }
    /**
     * @param {NodeVec} arg0
     * @returns {_SourceListMap}
     */
    static _new_nodes(arg0) {
        const ptr0 = arg0.ptr;
        if (ptr0 === 0) {
            throw new Error("Attempt to use a moved value");
        }
        arg0.ptr = 0;
        return _SourceListMap.__construct(wasm._sourcelistmap__new_nodes(ptr0));
    }
    /**
     * @param {NodeVec} arg0
     * @returns {void}
     */
    _add_node(arg0) {
        if (this.ptr === 0) {
            throw new Error("Attempt to use a moved value");
        }
        const ptr0 = arg0.ptr;
        if (ptr0 === 0) {
            throw new Error("Attempt to use a moved value");
        }
        arg0.ptr = 0;
        return wasm._sourcelistmap__add_node(this.ptr, ptr0);
    }
    /**
     * @param {NodeVec} arg0
     * @param {number} arg1
     * @param {number} arg2
     * @returns {void}
     */
    _add_node_sidx_sidx(arg0, arg1, arg2) {
        if (this.ptr === 0) {
            throw new Error("Attempt to use a moved value");
        }
        const ptr0 = arg0.ptr;
        if (ptr0 === 0) {
            throw new Error("Attempt to use a moved value");
        }
        arg0.ptr = 0;
        return wasm._sourcelistmap__add_node_sidx_sidx(
            this.ptr,
            ptr0,
            arg1,
            arg2
        );
    }
    /**
     * @param {NodeVec} arg0
     * @returns {void}
     */
    _prepend_node(arg0) {
        if (this.ptr === 0) {
            throw new Error("Attempt to use a moved value");
        }
        const ptr0 = arg0.ptr;
        if (ptr0 === 0) {
            throw new Error("Attempt to use a moved value");
        }
        arg0.ptr = 0;
        return wasm._sourcelistmap__prepend_node(this.ptr, ptr0);
    }
    /**
     * @param {NodeVec} arg0
     * @param {number} arg1
     * @param {number} arg2
     * @returns {void}
     */
    _prepend_node_sidx_sidx(arg0, arg1, arg2) {
        if (this.ptr === 0) {
            throw new Error("Attempt to use a moved value");
        }
        const ptr0 = arg0.ptr;
        if (ptr0 === 0) {
            throw new Error("Attempt to use a moved value");
        }
        arg0.ptr = 0;
        return wasm._sourcelistmap__prepend_node_sidx_sidx(
            this.ptr,
            ptr0,
            arg1,
            arg2
        );
    }
    /**
     * @returns {string}
     */
    _to_string() {
        if (this.ptr === 0) {
            throw new Error("Attempt to use a moved value");
        }
        const retptr = globalArgumentPtr();
        wasm._sourcelistmap__to_string(retptr, this.ptr);
        const mem = getUint32Memory();
        const rustptr = mem[retptr / 4];
        const rustlen = mem[retptr / 4 + 1];

        const realRet = getStringFromWasm(rustptr, rustlen).slice();
        wasm.__wbindgen_free(rustptr, rustlen * 1);
        return realRet;
    }
    /**
     * @returns {JsStringWithSourceMap}
     */
    _to_string_with_source_map_null() {
        if (this.ptr === 0) {
            throw new Error("Attempt to use a moved value");
        }
        return JsStringWithSourceMap.__construct(
            wasm._sourcelistmap__to_string_with_source_map_null(this.ptr)
        );
    }
}
module.exports._SourceListMap = _SourceListMap;

function free_SourceNode(ptr) {
    wasm.__wbg__sourcenode_free(ptr);
}
/**
 */
class _SourceNode {
    static __construct(ptr) {
        return new _SourceNode(ptr);
    }

    constructor(ptr) {
        this.ptr = ptr;
    }

    free() {
        const ptr = this.ptr;
        this.ptr = 0;
        free_SourceNode(ptr);
    }
    /**
     * @param {string} arg0
     * @param {number} arg1
     * @returns {_SourceNode}
     */
    static _new_string_null_null_number(arg0, arg1) {
        const [ptr0, len0] = passStringToWasm(arg0);
        return _SourceNode.__construct(
            wasm._sourcenode__new_string_null_null_number(ptr0, len0, arg1)
        );
    }
    /**
     * @param {string} arg0
     * @param {number} arg1
     * @param {number} arg2
     * @param {number} arg3
     * @returns {_SourceNode}
     */
    static _new_string_sidx_sidx_number(arg0, arg1, arg2, arg3) {
        const [ptr0, len0] = passStringToWasm(arg0);
        return _SourceNode.__construct(
            wasm._sourcenode__new_string_sidx_sidx_number(
                ptr0,
                len0,
                arg1,
                arg2,
                arg3
            )
        );
    }
    /**
     * @returns {_SourceNode}
     */
    _clone() {
        if (this.ptr === 0) {
            throw new Error("Attempt to use a moved value");
        }
        return _SourceNode.__construct(wasm._sourcenode__clone(this.ptr));
    }
}
module.exports._SourceNode = _SourceNode;

function free_RawSource(ptr) {
    wasm.__wbg__rawsource_free(ptr);
}
/**
 */
class _RawSource {
    static __construct(ptr) {
        return new _RawSource(ptr);
    }

    constructor(ptr) {
        this.ptr = ptr;
    }

    free() {
        const ptr = this.ptr;
        this.ptr = 0;
        free_RawSource(ptr);
    }
    /**
     * @param {string} arg0
     * @returns {_RawSource}
     */
    static _new_string(arg0) {
        const [ptr0, len0] = passStringToWasm(arg0);
        return _RawSource.__construct(wasm._rawsource__new_string(ptr0, len0));
    }
    /**
     * @returns {string}
     */
    _source() {
        if (this.ptr === 0) {
            throw new Error("Attempt to use a moved value");
        }
        const retptr = globalArgumentPtr();
        wasm._rawsource__source(retptr, this.ptr);
        const mem = getUint32Memory();
        const rustptr = mem[retptr / 4];
        const rustlen = mem[retptr / 4 + 1];

        const realRet = getStringFromWasm(rustptr, rustlen).slice();
        wasm.__wbindgen_free(rustptr, rustlen * 1);
        return realRet;
    }
    /**
     * @returns {number}
     */
    _size() {
        if (this.ptr === 0) {
            throw new Error("Attempt to use a moved value");
        }
        return wasm._rawsource__size(this.ptr);
    }
    /**
     * @param {boolean} arg0
     * @param {boolean} arg1
     * @returns {_SourceListMap}
     */
    _list_map_bool_bool(arg0, arg1) {
        if (this.ptr === 0) {
            throw new Error("Attempt to use a moved value");
        }
        return _SourceListMap.__construct(
            wasm._rawsource__list_map_bool_bool(
                this.ptr,
                arg0 ? 1 : 0,
                arg1 ? 1 : 0
            )
        );
    }
    /**
     * @param {boolean} arg0
     * @param {boolean} arg1
     * @returns {_MSourceNode}
     */
    _node_bool_bool(arg0, arg1) {
        if (this.ptr === 0) {
            throw new Error("Attempt to use a moved value");
        }
        return _MSourceNode.__construct(
            wasm._rawsource__node_bool_bool(
                this.ptr,
                arg0 ? 1 : 0,
                arg1 ? 1 : 0
            )
        );
    }
}
module.exports._RawSource = _RawSource;

function free_ReplaceSource(ptr) {
    wasm.__wbg__replacesource_free(ptr);
}
/**
 */
class _ReplaceSource {
    static __construct(ptr) {
        return new _ReplaceSource(ptr);
    }

    constructor(ptr) {
        this.ptr = ptr;
    }

    free() {
        const ptr = this.ptr;
        this.ptr = 0;
        free_ReplaceSource(ptr);
    }
    /**
     * @param {string} arg0
     * @returns {_ReplaceSource}
     */
    static _new_string(arg0) {
        const [ptr0, len0] = passStringToWasm(arg0);
        return _ReplaceSource.__construct(
            wasm._replacesource__new_string(ptr0, len0)
        );
    }
    /**
     * @param {_RawSource} arg0
     * @returns {_ReplaceSource}
     */
    static _new_raw_source(arg0) {
        return _ReplaceSource.__construct(
            wasm._replacesource__new_raw_source(arg0.ptr)
        );
    }
    /**
     * @param {_OriginalSource} arg0
     * @returns {_ReplaceSource}
     */
    static _new_original_source(arg0) {
        return _ReplaceSource.__construct(
            wasm._replacesource__new_original_source(arg0.ptr)
        );
    }
    /**
     * @param {_ReplaceSource} arg0
     * @returns {_ReplaceSource}
     */
    static _new_replace_source(arg0) {
        return _ReplaceSource.__construct(
            wasm._replacesource__new_replace_source(arg0.ptr)
        );
    }
    /**
     * @param {_PrefixSource} arg0
     * @returns {_ReplaceSource}
     */
    static _new_prefix_source(arg0) {
        return _ReplaceSource.__construct(
            wasm._replacesource__new_prefix_source(arg0.ptr)
        );
    }
    /**
     * @param {_ConcatSource} arg0
     * @returns {_ReplaceSource}
     */
    static _new_concat_source(arg0) {
        return _ReplaceSource.__construct(
            wasm._replacesource__new_concat_source(arg0.ptr)
        );
    }
    /**
     * @param {_LineToLineMappedSource} arg0
     * @returns {_ReplaceSource}
     */
    static _new_line_to_line_mapped_source(arg0) {
        return _ReplaceSource.__construct(
            wasm._replacesource__new_line_to_line_mapped_source(arg0.ptr)
        );
    }
    /**
     * @param {_SourceMapSource} arg0
     * @returns {_ReplaceSource}
     */
    static _new_source_map_source(arg0) {
        return _ReplaceSource.__construct(
            wasm._replacesource__new_source_map_source(arg0.ptr)
        );
    }
    /**
     * @param {number} arg0
     * @param {number} arg1
     * @param {string} arg2
     * @param {number} arg3
     * @param {number} arg4
     * @returns {void}
     */
    _replace_number_number_string_number_number(arg0, arg1, arg2, arg3, arg4) {
        if (this.ptr === 0) {
            throw new Error("Attempt to use a moved value");
        }
        const [ptr2, len2] = passStringToWasm(arg2);
        return wasm._replacesource__replace_number_number_string_number_number(
            this.ptr,
            arg0,
            arg1,
            ptr2,
            len2,
            arg3,
            arg4
        );
    }
    /**
     * @param {number} arg0
     * @param {string} arg1
     * @param {number} arg2
     * @returns {void}
     */
    _insert_number_string_number(arg0, arg1, arg2) {
        if (this.ptr === 0) {
            throw new Error("Attempt to use a moved value");
        }
        const [ptr1, len1] = passStringToWasm(arg1);
        return wasm._replacesource__insert_number_string_number(
            this.ptr,
            arg0,
            ptr1,
            len1,
            arg2
        );
    }
    /**
     * @returns {string}
     */
    _source() {
        if (this.ptr === 0) {
            throw new Error("Attempt to use a moved value");
        }
        const retptr = globalArgumentPtr();
        wasm._replacesource__source(retptr, this.ptr);
        const mem = getUint32Memory();
        const rustptr = mem[retptr / 4];
        const rustlen = mem[retptr / 4 + 1];

        const realRet = getStringFromWasm(rustptr, rustlen).slice();
        wasm.__wbindgen_free(rustptr, rustlen * 1);
        return realRet;
    }
    /**
     * @returns {number}
     */
    _size() {
        if (this.ptr === 0) {
            throw new Error("Attempt to use a moved value");
        }
        return wasm._replacesource__size(this.ptr);
    }
    /**
     * @param {boolean} arg0
     * @param {boolean} arg1
     * @returns {_SourceListMap}
     */
    _list_map_bool_bool(arg0, arg1) {
        if (this.ptr === 0) {
            throw new Error("Attempt to use a moved value");
        }
        return _SourceListMap.__construct(
            wasm._replacesource__list_map_bool_bool(
                this.ptr,
                arg0 ? 1 : 0,
                arg1 ? 1 : 0
            )
        );
    }
    /**
     * @param {boolean} arg0
     * @param {boolean} arg1
     * @returns {_MSourceNode}
     */
    _node_bool_bool(arg0, arg1) {
        if (this.ptr === 0) {
            throw new Error("Attempt to use a moved value");
        }
        return _MSourceNode.__construct(
            wasm._replacesource__node_bool_bool(
                this.ptr,
                arg0 ? 1 : 0,
                arg1 ? 1 : 0
            )
        );
    }
}
module.exports._ReplaceSource = _ReplaceSource;

function free_CodeNode(ptr) {
    wasm.__wbg__codenode_free(ptr);
}
/**
 */
class _CodeNode {
    static __construct(ptr) {
        return new _CodeNode(ptr);
    }

    constructor(ptr) {
        this.ptr = ptr;
    }

    free() {
        const ptr = this.ptr;
        this.ptr = 0;
        free_CodeNode(ptr);
    }
    /**
     * @param {string} arg0
     * @returns {_CodeNode}
     */
    static _new_string(arg0) {
        const [ptr0, len0] = passStringToWasm(arg0);
        return _CodeNode.__construct(wasm._codenode__new_string(ptr0, len0));
    }
    /**
     * @returns {_CodeNode}
     */
    _clone() {
        if (this.ptr === 0) {
            throw new Error("Attempt to use a moved value");
        }
        return _CodeNode.__construct(wasm._codenode__clone(this.ptr));
    }
}
module.exports._CodeNode = _CodeNode;

function free_SingleLineNode(ptr) {
    wasm.__wbg__singlelinenode_free(ptr);
}
/**
 */
class _SingleLineNode {
    static __construct(ptr) {
        return new _SingleLineNode(ptr);
    }

    constructor(ptr) {
        this.ptr = ptr;
    }

    free() {
        const ptr = this.ptr;
        this.ptr = 0;
        free_SingleLineNode(ptr);
    }
    /**
     * @param {string} arg0
     * @param {number} arg1
     * @returns {_SingleLineNode}
     */
    static _new_string_null_null_number(arg0, arg1) {
        const [ptr0, len0] = passStringToWasm(arg0);
        return _SingleLineNode.__construct(
            wasm._singlelinenode__new_string_null_null_number(ptr0, len0, arg1)
        );
    }
    /**
     * @param {string} arg0
     * @param {number} arg1
     * @param {number} arg2
     * @param {number} arg3
     * @returns {_SingleLineNode}
     */
    static _new_string_sidx_sidx_number(arg0, arg1, arg2, arg3) {
        const [ptr0, len0] = passStringToWasm(arg0);
        return _SingleLineNode.__construct(
            wasm._singlelinenode__new_string_sidx_sidx_number(
                ptr0,
                len0,
                arg1,
                arg2,
                arg3
            )
        );
    }
    /**
     * @returns {_SingleLineNode}
     */
    _clone() {
        if (this.ptr === 0) {
            throw new Error("Attempt to use a moved value");
        }
        return _SingleLineNode.__construct(
            wasm._singlelinenode__clone(this.ptr)
        );
    }
}
module.exports._SingleLineNode = _SingleLineNode;

function free_MSourceNode(ptr) {
    wasm.__wbg__msourcenode_free(ptr);
}
/**
 */
class _MSourceNode {
    static __construct(ptr) {
        return new _MSourceNode(ptr);
    }

    constructor(ptr) {
        this.ptr = ptr;
    }

    free() {
        const ptr = this.ptr;
        this.ptr = 0;
        free_MSourceNode(ptr);
    }
    /**
     * @param {number} arg0
     * @param {number} arg1
     * @param {number} arg2
     * @returns {_MSourceNode}
     */
    static _new_number_number_sidx_null(arg0, arg1, arg2) {
        return _MSourceNode.__construct(
            wasm._msourcenode__new_number_number_sidx_null(arg0, arg1, arg2)
        );
    }
    /**
     * @returns {_MSourceNode}
     */
    static _new_null_null_null_null() {
        return _MSourceNode.__construct(
            wasm._msourcenode__new_null_null_null_null()
        );
    }
    /**
     * @param {string} arg0
     * @returns {void}
     */
    _add_string(arg0) {
        if (this.ptr === 0) {
            throw new Error("Attempt to use a moved value");
        }
        const [ptr0, len0] = passStringToWasm(arg0);
        return wasm._msourcenode__add_string(this.ptr, ptr0, len0);
    }
    /**
     * @param {_MSourceNode} arg0
     * @returns {void}
     */
    _add_sourcenode(arg0) {
        if (this.ptr === 0) {
            throw new Error("Attempt to use a moved value");
        }
        const ptr0 = arg0.ptr;
        if (ptr0 === 0) {
            throw new Error("Attempt to use a moved value");
        }
        arg0.ptr = 0;
        return wasm._msourcenode__add_sourcenode(this.ptr, ptr0);
    }
    /**
     * @returns {JsStringWithSourceMap}
     */
    _to_string_with_source_map_null() {
        if (this.ptr === 0) {
            throw new Error("Attempt to use a moved value");
        }
        return JsStringWithSourceMap.__construct(
            wasm._msourcenode__to_string_with_source_map_null(this.ptr)
        );
    }
}
module.exports._MSourceNode = _MSourceNode;

module.exports.__wbindgen_throw = function(ptr, len) {
    throw new Error(getStringFromWasm(ptr, len));
};

wasm = require("./webpack_sources_bg");
