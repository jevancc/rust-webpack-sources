/* tslint:disable */
var wasm;
const SourceNode = require('source-map').SourceNode;

const TextEncoder = require('util').TextEncoder;

let cachedEncoder = new TextEncoder('utf-8');

let cachegetUint8Memory = null;
function getUint8Memory() {
    if (cachegetUint8Memory === null ||
        cachegetUint8Memory.buffer !== wasm.memory.buffer)
        cachegetUint8Memory = new Uint8Array(wasm.memory.buffer);
    return cachegetUint8Memory;
}

function passStringToWasm(arg) {

    const buf = cachedEncoder.encode(arg);
    const ptr = wasm.__wbindgen_malloc(buf.length);
    getUint8Memory().set(buf, ptr);
    return [ptr, buf.length];
}

const TextDecoder = require('util').TextDecoder;

let cachedDecoder = new TextDecoder('utf-8');

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
    if (cachegetUint32Memory === null ||
        cachegetUint32Memory.buffer !== wasm.memory.buffer)
        cachegetUint32Memory = new Uint32Array(wasm.memory.buffer);
    return cachegetUint32Memory;
}

let stack = [];

let slab = [];

function getObject(idx) {
    if ((idx & 1) === 1) {
        return stack[idx >> 1];
    } else {
        const val = slab[idx >> 1];

    return val.obj;

    }
}

let slab_next = 0;

function dropRef(idx) {

    let obj = slab[idx >> 1];

    obj.cnt -= 1;
    if (obj.cnt > 0)
        return;

    // If we hit 0 then free up our space in the slab
    slab[idx >> 1] = slab_next;
    slab_next = idx >> 1;
}

function takeObject(idx) {
    const ret = getObject(idx);
    dropRef(idx);
    return ret;
}

module.exports._from_string_with_source_map = function(arg0, arg1, arg2, arg3) {
    const [ptr0, len0] = passStringToWasm(arg0);
    const ptr1 = arg1.ptr;
    arg1.ptr = 0;
    const ptr2 = arg2.ptr;
    arg2.ptr = 0;
    const [ptr3, len3] = passStringToWasm(arg3);
    try {
        return _SourceListMap.__construct(wasm._from_string_with_source_map(ptr0, len0, ptr1, ptr2, ptr3, len3));
    } finally {
        wasm.__wbindgen_free(ptr0, len0 * 1);
        wasm.__wbindgen_free(ptr3, len3 * 1);
    }
};

function addHeapObject(obj) {
    if (slab_next === slab.length)
        slab.push(slab.length + 1);
    const idx = slab_next;
    const next = slab[idx];

    slab_next = next;

    slab[idx] = { obj, cnt: 1 };
    return idx << 1;
}

module.exports.__wbg_f_new_number_number_string_new_number_number_string_SourceNode = function(arg0, arg1, arg2, arg3) {
    let varg2 = getStringFromWasm(arg2, arg3);
    return addHeapObject(new SourceNode(arg0, arg1, varg2));
};

module.exports.__wbg_f_new_null_null_null_new_null_null_null_SourceNode = function() {
    return addHeapObject(new SourceNode());
};

const __wbg_f_add_add_string_SourceNode_target = SourceNode.prototype.add;

module.exports.__wbg_f_add_add_string_SourceNode = function(arg0, arg1, arg2) {
    let varg1 = getStringFromWasm(arg1, arg2);
    __wbg_f_add_add_string_SourceNode_target.call(getObject(arg0), varg1);
};

const __wbg_f_add_add_sourcenode_SourceNode_target = SourceNode.prototype.add;

module.exports.__wbg_f_add_add_sourcenode_SourceNode = function(arg0, arg1) {
    __wbg_f_add_add_sourcenode_SourceNode_target.call(getObject(arg0), getObject(arg1));
};

const __wbg_f_setSourceContent_setSourceContent_SourceNode_target = SourceNode.prototype.setSourceContent;

module.exports.__wbg_f_setSourceContent_setSourceContent_SourceNode = function(arg0, arg1, arg2, arg3, arg4) {
    let varg1 = getStringFromWasm(arg1, arg2);
    let varg3 = getStringFromWasm(arg3, arg4);
    __wbg_f_setSourceContent_setSourceContent_SourceNode_target.call(getObject(arg0), varg1, varg3);
};

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
    return _SingleLineNode.__construct(wasm._singlelinenode__new_string_null_null_number(ptr0, len0, arg1));
}
static _new_string_string_string_number(arg0, arg1, arg2, arg3) {
    const [ptr0, len0] = passStringToWasm(arg0);
    const [ptr1, len1] = passStringToWasm(arg1);
    const [ptr2, len2] = passStringToWasm(arg2);
    return _SingleLineNode.__construct(wasm._singlelinenode__new_string_string_string_number(ptr0, len0, ptr1, len1, ptr2, len2, arg3));
}
_clone() {
    return _SingleLineNode.__construct(wasm._singlelinenode__clone(this.ptr));
}
}
module.exports._SingleLineNode = _SingleLineNode;

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
    return _SourceNode.__construct(wasm._sourcenode__new_string_null_null_number(ptr0, len0, arg1));
}
static _new_string_string_string_number(arg0, arg1, arg2, arg3) {
    const [ptr0, len0] = passStringToWasm(arg0);
    const [ptr1, len1] = passStringToWasm(arg1);
    const [ptr2, len2] = passStringToWasm(arg2);
    return _SourceNode.__construct(wasm._sourcenode__new_string_string_string_number(ptr0, len0, ptr1, len1, ptr2, len2, arg3));
}
_clone() {
    return _SourceNode.__construct(wasm._sourcenode__clone(this.ptr));
}
}
module.exports._SourceNode = _SourceNode;

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
_add_node_string_string(arg0, arg1, arg2) {
    const ptr0 = arg0.ptr;
    arg0.ptr = 0;
    const [ptr1, len1] = passStringToWasm(arg1);
    const [ptr2, len2] = passStringToWasm(arg2);
    return wasm._sourcelistmap__add_node_string_string(this.ptr, ptr0, ptr1, len1, ptr2, len2);
}
_prepend_node(arg0) {
    const ptr0 = arg0.ptr;
    arg0.ptr = 0;
    return wasm._sourcelistmap__prepend_node(this.ptr, ptr0);
}
_prepend_node_string_string(arg0, arg1, arg2) {
    const ptr0 = arg0.ptr;
    arg0.ptr = 0;
    const [ptr1, len1] = passStringToWasm(arg1);
    const [ptr2, len2] = passStringToWasm(arg2);
    return wasm._sourcelistmap__prepend_node_string_string(this.ptr, ptr0, ptr1, len1, ptr2, len2);
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
_to_string_with_source_map_string(arg0) {
    const [ptr0, len0] = passStringToWasm(arg0);
    const retptr = globalArgumentPtr();
    wasm._sourcelistmap__to_string_with_source_map_string(retptr, this.ptr, ptr0, len0);
    const mem = getUint32Memory();
    const ptr = mem[retptr / 4];
    const len = mem[retptr / 4 + 1];
    const realRet = getStringFromWasm(ptr, len).slice();
    wasm.__wbindgen_free(ptr, len * 1);
    return realRet;
}
_map_generated_code_identical() {
    return _SourceListMap.__construct(wasm._sourcelistmap__map_generated_code_identical(this.ptr));
}
_map_generated_code_test() {
    return _SourceListMap.__construct(wasm._sourcelistmap__map_generated_code_test(this.ptr));
}
_map_generated_code_prefix(arg0) {
    const [ptr0, len0] = passStringToWasm(arg0);
    return _SourceListMap.__construct(wasm._sourcelistmap__map_generated_code_prefix(this.ptr, ptr0, len0));
}
}
module.exports._SourceListMap = _SourceListMap;

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
        static _new_string_string(arg0, arg1) {
    const [ptr0, len0] = passStringToWasm(arg0);
    const [ptr1, len1] = passStringToWasm(arg1);
    return _OriginalSource.__construct(wasm._originalsource__new_string_string(ptr0, len0, ptr1, len1));
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
    const retptr = globalArgumentPtr();
    wasm._originalsource__name(retptr, this.ptr);
    const mem = getUint32Memory();
    const ptr = mem[retptr / 4];
    const len = mem[retptr / 4 + 1];
    const realRet = getStringFromWasm(ptr, len).slice();
    wasm.__wbindgen_free(ptr, len * 1);
    return realRet;
}
_node_bool(arg0) {
    return takeObject(wasm._originalsource__node_bool(this.ptr, arg0 ? 1 : 0));
}
}
module.exports._OriginalSource = _OriginalSource;

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
        static _new() {
    return _ReplaceSource.__construct(wasm._replacesource__new());
}
_replace_number_number_string(arg0, arg1, arg2) {
    const [ptr2, len2] = passStringToWasm(arg2);
    return wasm._replacesource__replace_number_number_string(this.ptr, arg0, arg1, ptr2, len2);
}
_insert_number_string(arg0, arg1) {
    const [ptr1, len1] = passStringToWasm(arg1);
    return wasm._replacesource__insert_number_string(this.ptr, arg0, ptr1, len1);
}
_source_string(arg0) {
    const [ptr0, len0] = passStringToWasm(arg0);
    const retptr = globalArgumentPtr();
    wasm._replacesource__source_string(retptr, this.ptr, ptr0, len0);
    const mem = getUint32Memory();
    const ptr = mem[retptr / 4];
    const len = mem[retptr / 4 + 1];
    const realRet = getStringFromWasm(ptr, len).slice();
    wasm.__wbindgen_free(ptr, len * 1);
    return realRet;
}
_list_map_sourcelistmap(arg0) {
    return _SourceListMap.__construct(wasm._replacesource__list_map_sourcelistmap(this.ptr, arg0.ptr));
}
_replacements_to_string() {
    const retptr = globalArgumentPtr();
    wasm._replacesource__replacements_to_string(retptr, this.ptr);
    const mem = getUint32Memory();
    const ptr = mem[retptr / 4];
    const len = mem[retptr / 4 + 1];
    const realRet = getStringFromWasm(ptr, len).slice();
    wasm.__wbindgen_free(ptr, len * 1);
    return realRet;
}
}
module.exports._ReplaceSource = _ReplaceSource;

module.exports.__wbindgen_object_clone_ref = function(idx) {
    // If this object is on the stack promote it to the heap.
    if ((idx & 1) === 1)
        return addHeapObject(getObject(idx));

    // Otherwise if the object is on the heap just bump the
    // refcount and move on
    const val = slab[idx >> 1];
    val.cnt += 1;
    return idx;
};

module.exports.__wbindgen_object_drop_ref = function(i) { dropRef(i); };

module.exports.__wbindgen_string_new = function(p, l) {
    return addHeapObject(getStringFromWasm(p, l));
};

module.exports.__wbindgen_number_new = function(i) { return addHeapObject(i); };

module.exports.__wbindgen_number_get = function(n, invalid) {
    let obj = getObject(n);
    if (typeof(obj) === 'number')
        return obj;
    getUint8Memory()[invalid] = 1;
    return 0;
};

module.exports.__wbindgen_undefined_new = function() { return addHeapObject(undefined); };

module.exports.__wbindgen_null_new = function() {
    return addHeapObject(null);
};

module.exports.__wbindgen_is_null = function(idx) {
    return getObject(idx) === null ? 1 : 0;
};

module.exports.__wbindgen_is_undefined = function(idx) {
    return getObject(idx) === undefined ? 1 : 0;
};

module.exports.__wbindgen_boolean_new = function(v) {
    return addHeapObject(v === 1);
};

module.exports.__wbindgen_boolean_get = function(i) {
    let v = getObject(i);
    if (typeof(v) === 'boolean') {
        return v ? 1 : 0;
    } else {
        return 2;
    }
};

module.exports.__wbindgen_symbol_new = function(ptr, len) {
    let a;
    if (ptr === 0) {
        a = Symbol();
    } else {
        a = Symbol(getStringFromWasm(ptr, len));
    }
    return addHeapObject(a);
};

module.exports.__wbindgen_is_symbol = function(i) {
    return typeof(getObject(i)) === 'symbol' ? 1 : 0;
};

module.exports.__wbindgen_string_get = function(i, len_ptr) {
    let obj = getObject(i);
    if (typeof(obj) !== 'string')
        return 0;
    const [ptr, len] = passStringToWasm(obj);
    getUint32Memory()[len_ptr / 4] = len;
    return ptr;
};

module.exports.__wbindgen_throw = function(ptr, len) {
    throw new Error(getStringFromWasm(ptr, len));
};

wasm = require('./webpack_sources_bg');