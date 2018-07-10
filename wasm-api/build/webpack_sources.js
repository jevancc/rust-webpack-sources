/* tslint:disable */
var wasm;

const TextEncoder = require("util").TextEncoder;

let cachedEncoder = new TextEncoder("utf-8");

let cachegetUint8Memory = null;
function getUint8Memory() {
    if (
        cachegetUint8Memory === null ||
        cachegetUint8Memory.buffer !== wasm.memory.buffer
    )
        cachegetUint8Memory = new Uint8Array(wasm.memory.buffer);
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
    if (cachedGlobalArgumentPtr === null)
        cachedGlobalArgumentPtr = wasm.__wbindgen_global_argument_ptr();
    return cachedGlobalArgumentPtr;
}

let cachegetUint32Memory = null;
function getUint32Memory() {
    if (
        cachegetUint32Memory === null ||
        cachegetUint32Memory.buffer !== wasm.memory.buffer
    )
        cachegetUint32Memory = new Uint32Array(wasm.memory.buffer);
    return cachegetUint32Memory;
}

let cachegetUint64Memory = null;
function getUint64Memory() {
    if (
        cachegetUint64Memory === null ||
        cachegetUint64Memory.buffer !== wasm.memory.buffer
    )
        cachegetUint64Memory = new BigUint64Array(wasm.memory.buffer);
    return cachegetUint64Memory;
}

function passArray32ToWasm(arg) {
    const ptr = wasm.__wbindgen_malloc(arg.length * 4);
    getUint32Memory().set(arg, ptr / 4);
    return [ptr, arg.length];
}

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

module.exports._sourcelistmap_map_generated_code_identical = function(arg0) {
    const ptr0 = arg0.ptr;
    arg0.ptr = 0;
    return _SourceListMap.__construct(
        wasm._sourcelistmap_map_generated_code_identical(ptr0)
    );
};

module.exports._sourcelistmap_map_generated_code_test = function(arg0) {
    const ptr0 = arg0.ptr;
    arg0.ptr = 0;
    return _SourceListMap.__construct(
        wasm._sourcelistmap_map_generated_code_test(ptr0)
    );
};

module.exports._sourcelistmap_map_generated_code_prefix = function(arg0, arg1) {
    const ptr0 = arg0.ptr;
    arg0.ptr = 0;
    const [ptr1, len1] = passStringToWasm(arg1);
    return _SourceListMap.__construct(
        wasm._sourcelistmap_map_generated_code_prefix(ptr0, ptr1, len1)
    );
};

const __wbg_f_log_clog_n_target = console.log;

module.exports.__wbg_f_log_clog_n = function(arg0, arg1) {
    let varg0 = getStringFromWasm(arg0, arg1);
    __wbg_f_log_clog_n_target(varg0);
};

class StringVec {
    static __construct(ptr) {
        return new StringVec(ptr);
    }

    constructor(ptr) {
        this.ptr = ptr;
    }

    free() {
        const ptr = this.ptr;
        this.ptr = 0;
        wasm.__wbg_stringvec_free(ptr);
    }
    static new() {
        return StringVec.__construct(wasm.stringvec_new());
    }
    push_string(arg0) {
        const [ptr0, len0] = passStringToWasm(arg0);
        return wasm.stringvec_push_string(this.ptr, ptr0, len0);
    }
}
module.exports.StringVec = StringVec;

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
        wasm.__wbg__codenode_free(ptr);
    }
    static _new_string(arg0) {
        const [ptr0, len0] = passStringToWasm(arg0);
        return _CodeNode.__construct(wasm._codenode__new_string(ptr0, len0));
    }
    _clone() {
        return _CodeNode.__construct(wasm._codenode__clone(this.ptr));
    }
}
module.exports._CodeNode = _CodeNode;

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
        wasm.__wbg__prefixsource_free(ptr);
    }
    static _new_string_string(arg0, arg1) {
        const [ptr0, len0] = passStringToWasm(arg0);
        const [ptr1, len1] = passStringToWasm(arg1);
        return _PrefixSource.__construct(
            wasm._prefixsource__new_string_string(ptr0, len0, ptr1, len1)
        );
    }
    static _new_string_raw_source(arg0, arg1) {
        const [ptr0, len0] = passStringToWasm(arg0);
        return _PrefixSource.__construct(
            wasm._prefixsource__new_string_raw_source(ptr0, len0, arg1.ptr)
        );
    }
    static _new_string_original_source(arg0, arg1) {
        const [ptr0, len0] = passStringToWasm(arg0);
        return _PrefixSource.__construct(
            wasm._prefixsource__new_string_original_source(ptr0, len0, arg1.ptr)
        );
    }
    static _new_string_replace_source(arg0, arg1) {
        const [ptr0, len0] = passStringToWasm(arg0);
        return _PrefixSource.__construct(
            wasm._prefixsource__new_string_replace_source(ptr0, len0, arg1.ptr)
        );
    }
    static _new_string_prefix_source(arg0, arg1) {
        const [ptr0, len0] = passStringToWasm(arg0);
        return _PrefixSource.__construct(
            wasm._prefixsource__new_string_prefix_source(ptr0, len0, arg1.ptr)
        );
    }
    static _new_string_concat_source(arg0, arg1) {
        const [ptr0, len0] = passStringToWasm(arg0);
        return _PrefixSource.__construct(
            wasm._prefixsource__new_string_concat_source(ptr0, len0, arg1.ptr)
        );
    }
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
    _source() {
        const retptr = globalArgumentPtr();
        wasm._prefixsource__source(retptr, this.ptr);
        const mem = getUint32Memory();
        const ptr = mem[retptr / 4];
        const len = mem[retptr / 4 + 1];
        const realRet = getStringFromWasm(ptr, len).slice();
        wasm.__wbindgen_free(ptr, len * 1);
        return realRet;
    }
    _size() {
        return wasm._prefixsource__size(this.ptr);
    }
    _list_map_bool_bool(arg0, arg1) {
        return _SourceListMap.__construct(
            wasm._prefixsource__list_map_bool_bool(
                this.ptr,
                arg0 ? 1 : 0,
                arg1 ? 1 : 0
            )
        );
    }
    _node_bool_bool(arg0, arg1) {
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
        wasm.__wbg__sourcemapsource_free(ptr);
    }
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
    _source() {
        const retptr = globalArgumentPtr();
        wasm._sourcemapsource__source(retptr, this.ptr);
        const mem = getUint32Memory();
        const ptr = mem[retptr / 4];
        const len = mem[retptr / 4 + 1];
        const realRet = getStringFromWasm(ptr, len).slice();
        wasm.__wbindgen_free(ptr, len * 1);
        return realRet;
    }
    _size() {
        return wasm._sourcemapsource__size(this.ptr);
    }
    _list_map_bool_bool(arg0, arg1) {
        return _SourceListMap.__construct(
            wasm._sourcemapsource__list_map_bool_bool(
                this.ptr,
                arg0 ? 1 : 0,
                arg1 ? 1 : 0
            )
        );
    }
    _node_bool_bool(arg0, arg1) {
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
        wasm.__wbg__sourcenode_free(ptr);
    }
    static _new_string_null_null_number(arg0, arg1) {
        const [ptr0, len0] = passStringToWasm(arg0);
        return _SourceNode.__construct(
            wasm._sourcenode__new_string_null_null_number(ptr0, len0, arg1)
        );
    }
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
    _clone() {
        return _SourceNode.__construct(wasm._sourcenode__clone(this.ptr));
    }
}
module.exports._SourceNode = _SourceNode;

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
        wasm.__wbg__replacesource_free(ptr);
    }
    static _new_string(arg0) {
        const [ptr0, len0] = passStringToWasm(arg0);
        return _ReplaceSource.__construct(
            wasm._replacesource__new_string(ptr0, len0)
        );
    }
    static _new_raw_source(arg0) {
        return _ReplaceSource.__construct(
            wasm._replacesource__new_raw_source(arg0.ptr)
        );
    }
    static _new_original_source(arg0) {
        return _ReplaceSource.__construct(
            wasm._replacesource__new_original_source(arg0.ptr)
        );
    }
    static _new_replace_source(arg0) {
        return _ReplaceSource.__construct(
            wasm._replacesource__new_replace_source(arg0.ptr)
        );
    }
    static _new_prefix_source(arg0) {
        return _ReplaceSource.__construct(
            wasm._replacesource__new_prefix_source(arg0.ptr)
        );
    }
    static _new_concat_source(arg0) {
        return _ReplaceSource.__construct(
            wasm._replacesource__new_concat_source(arg0.ptr)
        );
    }
    static _new_line_to_line_mapped_source(arg0) {
        return _ReplaceSource.__construct(
            wasm._replacesource__new_line_to_line_mapped_source(arg0.ptr)
        );
    }
    static _new_source_map_source(arg0) {
        return _ReplaceSource.__construct(
            wasm._replacesource__new_source_map_source(arg0.ptr)
        );
    }
    _replace_number_number_string_number_number(arg0, arg1, arg2, arg3, arg4) {
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
    _insert_number_string_number(arg0, arg1, arg2) {
        const [ptr1, len1] = passStringToWasm(arg1);
        return wasm._replacesource__insert_number_string_number(
            this.ptr,
            arg0,
            ptr1,
            len1,
            arg2
        );
    }
    _source() {
        const retptr = globalArgumentPtr();
        wasm._replacesource__source(retptr, this.ptr);
        const mem = getUint32Memory();
        const ptr = mem[retptr / 4];
        const len = mem[retptr / 4 + 1];
        const realRet = getStringFromWasm(ptr, len).slice();
        wasm.__wbindgen_free(ptr, len * 1);
        return realRet;
    }
    _size() {
        return wasm._replacesource__size(this.ptr);
    }
    _list_map_bool_bool(arg0, arg1) {
        return _SourceListMap.__construct(
            wasm._replacesource__list_map_bool_bool(
                this.ptr,
                arg0 ? 1 : 0,
                arg1 ? 1 : 0
            )
        );
    }
    _node_bool_bool(arg0, arg1) {
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
        wasm.__wbg__msourcenode_free(ptr);
    }
    static _new_number_number_sidx_null(arg0, arg1, arg2) {
        return _MSourceNode.__construct(
            wasm._msourcenode__new_number_number_sidx_null(arg0, arg1, arg2)
        );
    }
    static _new_null_null_null_null() {
        return _MSourceNode.__construct(
            wasm._msourcenode__new_null_null_null_null()
        );
    }
    _add_string(arg0) {
        const [ptr0, len0] = passStringToWasm(arg0);
        return wasm._msourcenode__add_string(this.ptr, ptr0, len0);
    }
    _add_sourcenode(arg0) {
        const ptr0 = arg0.ptr;
        arg0.ptr = 0;
        return wasm._msourcenode__add_sourcenode(this.ptr, ptr0);
    }
    _to_string_with_source_map_sidx(arg0) {
        const retptr = globalArgumentPtr();
        wasm._msourcenode__to_string_with_source_map_sidx(
            retptr,
            this.ptr,
            arg0
        );
        const mem = getUint32Memory();
        const ptr = mem[retptr / 4];
        const len = mem[retptr / 4 + 1];
        const realRet = getStringFromWasm(ptr, len).slice();
        wasm.__wbindgen_free(ptr, len * 1);
        return realRet;
    }
    _to_string_with_source_map_null() {
        const retptr = globalArgumentPtr();
        wasm._msourcenode__to_string_with_source_map_null(retptr, this.ptr);
        const mem = getUint32Memory();
        const ptr = mem[retptr / 4];
        const len = mem[retptr / 4 + 1];
        const realRet = getStringFromWasm(ptr, len).slice();
        wasm.__wbindgen_free(ptr, len * 1);
        return realRet;
    }
}
module.exports._MSourceNode = _MSourceNode;

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
        wasm.__wbg__concatsource_free(ptr);
    }
    static _new() {
        return _ConcatSource.__construct(wasm._concatsource__new());
    }
    _add_string(arg0) {
        const [ptr0, len0] = passStringToWasm(arg0);
        return wasm._concatsource__add_string(this.ptr, ptr0, len0);
    }
    _add_raw_source(arg0) {
        return wasm._concatsource__add_raw_source(this.ptr, arg0.ptr);
    }
    _add_original_source(arg0) {
        return wasm._concatsource__add_original_source(this.ptr, arg0.ptr);
    }
    _add_replace_source(arg0) {
        return wasm._concatsource__add_replace_source(this.ptr, arg0.ptr);
    }
    _add_prefix_source(arg0) {
        return wasm._concatsource__add_prefix_source(this.ptr, arg0.ptr);
    }
    _add_concat_source(arg0) {
        return wasm._concatsource__add_concat_source(this.ptr, arg0.ptr);
    }
    _add_line_to_line_mapped_source(arg0) {
        return wasm._concatsource__add_line_to_line_mapped_source(
            this.ptr,
            arg0.ptr
        );
    }
    _add_source_map_source(arg0) {
        return wasm._concatsource__add_source_map_source(this.ptr, arg0.ptr);
    }
    _source() {
        const retptr = globalArgumentPtr();
        wasm._concatsource__source(retptr, this.ptr);
        const mem = getUint32Memory();
        const ptr = mem[retptr / 4];
        const len = mem[retptr / 4 + 1];
        const realRet = getStringFromWasm(ptr, len).slice();
        wasm.__wbindgen_free(ptr, len * 1);
        return realRet;
    }
    _size() {
        return wasm._concatsource__size(this.ptr);
    }
    _list_map_bool_bool(arg0, arg1) {
        return _SourceListMap.__construct(
            wasm._concatsource__list_map_bool_bool(
                this.ptr,
                arg0 ? 1 : 0,
                arg1 ? 1 : 0
            )
        );
    }
    _node_bool_bool(arg0, arg1) {
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
        wasm.__wbg__singlelinenode_free(ptr);
    }
    static _new_string_null_null_number(arg0, arg1) {
        const [ptr0, len0] = passStringToWasm(arg0);
        return _SingleLineNode.__construct(
            wasm._singlelinenode__new_string_null_null_number(ptr0, len0, arg1)
        );
    }
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
    _clone() {
        return _SingleLineNode.__construct(
            wasm._singlelinenode__clone(this.ptr)
        );
    }
}
module.exports._SingleLineNode = _SingleLineNode;

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
        wasm.__wbg__originalsource_free(ptr);
    }
    static _new_string_sidx_sidx(arg0, arg1, arg2) {
        const [ptr0, len0] = passStringToWasm(arg0);
        return _OriginalSource.__construct(
            wasm._originalsource__new_string_sidx_sidx(ptr0, len0, arg1, arg2)
        );
    }
    _source() {
        const retptr = globalArgumentPtr();
        wasm._originalsource__source(retptr, this.ptr);
        const mem = getUint32Memory();
        const ptr = mem[retptr / 4];
        const len = mem[retptr / 4 + 1];
        const realRet = getStringFromWasm(ptr, len).slice();
        wasm.__wbindgen_free(ptr, len * 1);
        return realRet;
    }
    _size() {
        return wasm._originalsource__size(this.ptr);
    }
    _name() {
        return wasm._originalsource__name(this.ptr);
    }
    _list_map_bool_bool(arg0, arg1) {
        return _SourceListMap.__construct(
            wasm._originalsource__list_map_bool_bool(
                this.ptr,
                arg0 ? 1 : 0,
                arg1 ? 1 : 0
            )
        );
    }
    _node_bool_bool(arg0, arg1) {
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
        wasm.__wbg__linetolinemappedsource_free(ptr);
    }
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
    _source() {
        const retptr = globalArgumentPtr();
        wasm._linetolinemappedsource__source(retptr, this.ptr);
        const mem = getUint32Memory();
        const ptr = mem[retptr / 4];
        const len = mem[retptr / 4 + 1];
        const realRet = getStringFromWasm(ptr, len).slice();
        wasm.__wbindgen_free(ptr, len * 1);
        return realRet;
    }
    _size() {
        return wasm._linetolinemappedsource__size(this.ptr);
    }
    _list_map_bool_bool(arg0, arg1) {
        return _SourceListMap.__construct(
            wasm._linetolinemappedsource__list_map_bool_bool(
                this.ptr,
                arg0 ? 1 : 0,
                arg1 ? 1 : 0
            )
        );
    }
    _node_bool_bool(arg0, arg1) {
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
        wasm.__wbg__rawsource_free(ptr);
    }
    static _new_string(arg0) {
        const [ptr0, len0] = passStringToWasm(arg0);
        return _RawSource.__construct(wasm._rawsource__new_string(ptr0, len0));
    }
    _source() {
        const retptr = globalArgumentPtr();
        wasm._rawsource__source(retptr, this.ptr);
        const mem = getUint32Memory();
        const ptr = mem[retptr / 4];
        const len = mem[retptr / 4 + 1];
        const realRet = getStringFromWasm(ptr, len).slice();
        wasm.__wbindgen_free(ptr, len * 1);
        return realRet;
    }
    _size() {
        return wasm._rawsource__size(this.ptr);
    }
    _list_map_bool_bool(arg0, arg1) {
        return _SourceListMap.__construct(
            wasm._rawsource__list_map_bool_bool(
                this.ptr,
                arg0 ? 1 : 0,
                arg1 ? 1 : 0
            )
        );
    }
    _node_bool_bool(arg0, arg1) {
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
        wasm.__wbg__sourcelistmap_free(ptr);
    }
    static _new() {
        return _SourceListMap.__construct(wasm._sourcelistmap__new());
    }
    static _new_nodes(arg0) {
        const ptr0 = arg0.ptr;
        arg0.ptr = 0;
        return _SourceListMap.__construct(wasm._sourcelistmap__new_nodes(ptr0));
    }
    _add_node(arg0) {
        const ptr0 = arg0.ptr;
        arg0.ptr = 0;
        return wasm._sourcelistmap__add_node(this.ptr, ptr0);
    }
    _add_node_sidx_sidx(arg0, arg1, arg2) {
        const ptr0 = arg0.ptr;
        arg0.ptr = 0;
        return wasm._sourcelistmap__add_node_sidx_sidx(
            this.ptr,
            ptr0,
            arg1,
            arg2
        );
    }
    _prepend_node(arg0) {
        const ptr0 = arg0.ptr;
        arg0.ptr = 0;
        return wasm._sourcelistmap__prepend_node(this.ptr, ptr0);
    }
    _prepend_node_sidx_sidx(arg0, arg1, arg2) {
        const ptr0 = arg0.ptr;
        arg0.ptr = 0;
        return wasm._sourcelistmap__prepend_node_sidx_sidx(
            this.ptr,
            ptr0,
            arg1,
            arg2
        );
    }
    _to_string() {
        const retptr = globalArgumentPtr();
        wasm._sourcelistmap__to_string(retptr, this.ptr);
        const mem = getUint32Memory();
        const ptr = mem[retptr / 4];
        const len = mem[retptr / 4 + 1];
        const realRet = getStringFromWasm(ptr, len).slice();
        wasm.__wbindgen_free(ptr, len * 1);
        return realRet;
    }
    _to_string_with_source_map() {
        const retptr = globalArgumentPtr();
        wasm._sourcelistmap__to_string_with_source_map(retptr, this.ptr);
        const mem = getUint32Memory();
        const ptr = mem[retptr / 4];
        const len = mem[retptr / 4 + 1];
        const realRet = getStringFromWasm(ptr, len).slice();
        wasm.__wbindgen_free(ptr, len * 1);
        return realRet;
    }
}
module.exports._SourceListMap = _SourceListMap;

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
        wasm.__wbg_nodevec_free(ptr);
    }
    static new() {
        return NodeVec.__construct(wasm.nodevec_new());
    }
    push_string(arg0) {
        const [ptr0, len0] = passStringToWasm(arg0);
        return wasm.nodevec_push_string(this.ptr, ptr0, len0);
    }
    push_sourcenode(arg0) {
        return wasm.nodevec_push_sourcenode(this.ptr, arg0.ptr);
    }
    push_codenode(arg0) {
        return wasm.nodevec_push_codenode(this.ptr, arg0.ptr);
    }
    push_singlelinenode(arg0) {
        return wasm.nodevec_push_singlelinenode(this.ptr, arg0.ptr);
    }
    push_sourcelistmap(arg0) {
        return wasm.nodevec_push_sourcelistmap(this.ptr, arg0.ptr);
    }
}
module.exports.NodeVec = NodeVec;

module.exports.__wbindgen_throw = function(ptr, len) {
    throw new Error(getStringFromWasm(ptr, len));
};

wasm = require("./webpack_sources_bg");
