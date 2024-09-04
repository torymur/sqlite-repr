import { RawInterpreter } from './snippets/dioxus-interpreter-js-7e2aed97ebee2c55/inline0.js';
import { setAttributeInner } from './snippets/dioxus-interpreter-js-7e2aed97ebee2c55/src/js/common.js';
import { WebDioxusChannel } from './snippets/dioxus-interpreter-js-7e2aed97ebee2c55/src/js/eval.js';
import { get_form_data } from './snippets/dioxus-web-10186f9fcc0b4418/inline0.js';
import { get_select_data } from './snippets/dioxus-web-10186f9fcc0b4418/inline1.js';

let wasm;

const heap = new Array(128).fill(undefined);

heap.push(undefined, null, true, false);

function getObject(idx) { return heap[idx]; }

let heap_next = heap.length;

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    if (typeof(heap_next) !== 'number') throw new Error('corrupt heap');

    heap[idx] = obj;
    return idx;
}

const cachedTextDecoder = (typeof TextDecoder !== 'undefined' ? new TextDecoder('utf-8', { ignoreBOM: true, fatal: true }) : { decode: () => { throw Error('TextDecoder not available') } } );

if (typeof TextDecoder !== 'undefined') { cachedTextDecoder.decode(); };

let cachedUint8ArrayMemory0 = null;

function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
}

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));
}

function dropObject(idx) {
    if (idx < 132) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

function takeObject(idx) {
    const ret = getObject(idx);
    dropObject(idx);
    return ret;
}

function _assertBoolean(n) {
    if (typeof(n) !== 'boolean') {
        throw new Error(`expected a boolean argument, found ${typeof(n)}`);
    }
}

function _assertNum(n) {
    if (typeof(n) !== 'number') throw new Error(`expected a number argument, found ${typeof(n)}`);
}

function isLikeNone(x) {
    return x === undefined || x === null;
}

let cachedDataViewMemory0 = null;

function getDataViewMemory0() {
    if (cachedDataViewMemory0 === null || cachedDataViewMemory0.buffer.detached === true || (cachedDataViewMemory0.buffer.detached === undefined && cachedDataViewMemory0.buffer !== wasm.memory.buffer)) {
        cachedDataViewMemory0 = new DataView(wasm.memory.buffer);
    }
    return cachedDataViewMemory0;
}

let WASM_VECTOR_LEN = 0;

const cachedTextEncoder = (typeof TextEncoder !== 'undefined' ? new TextEncoder('utf-8') : { encode: () => { throw Error('TextEncoder not available') } } );

const encodeString = (typeof cachedTextEncoder.encodeInto === 'function'
    ? function (arg, view) {
    return cachedTextEncoder.encodeInto(arg, view);
}
    : function (arg, view) {
    const buf = cachedTextEncoder.encode(arg);
    view.set(buf);
    return {
        read: arg.length,
        written: buf.length
    };
});

function passStringToWasm0(arg, malloc, realloc) {

    if (typeof(arg) !== 'string') throw new Error(`expected a string argument, found ${typeof(arg)}`);

    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length, 1) >>> 0;
        getUint8ArrayMemory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len, 1) >>> 0;

    const mem = getUint8ArrayMemory0();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }

    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3, 1) >>> 0;
        const view = getUint8ArrayMemory0().subarray(ptr + offset, ptr + len);
        const ret = encodeString(arg, view);
        if (ret.read !== arg.length) throw new Error('failed to pass whole string');
        offset += ret.written;
        ptr = realloc(ptr, len, offset, 1) >>> 0;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

function _assertBigInt(n) {
    if (typeof(n) !== 'bigint') throw new Error(`expected a bigint argument, found ${typeof(n)}`);
}

function debugString(val) {
    // primitive types
    const type = typeof val;
    if (type == 'number' || type == 'boolean' || val == null) {
        return  `${val}`;
    }
    if (type == 'string') {
        return `"${val}"`;
    }
    if (type == 'symbol') {
        const description = val.description;
        if (description == null) {
            return 'Symbol';
        } else {
            return `Symbol(${description})`;
        }
    }
    if (type == 'function') {
        const name = val.name;
        if (typeof name == 'string' && name.length > 0) {
            return `Function(${name})`;
        } else {
            return 'Function';
        }
    }
    // objects
    if (Array.isArray(val)) {
        const length = val.length;
        let debug = '[';
        if (length > 0) {
            debug += debugString(val[0]);
        }
        for(let i = 1; i < length; i++) {
            debug += ', ' + debugString(val[i]);
        }
        debug += ']';
        return debug;
    }
    // Test for built-in
    const builtInMatches = /\[object ([^\]]+)\]/.exec(toString.call(val));
    let className;
    if (builtInMatches.length > 1) {
        className = builtInMatches[1];
    } else {
        // Failed to match the standard '[object ClassName]'
        return toString.call(val);
    }
    if (className == 'Object') {
        // we're a user defined class or Object
        // JSON.stringify avoids problems with cycles, and is generally much
        // easier than looping through ownProperties of `val`.
        try {
            return 'Object(' + JSON.stringify(val) + ')';
        } catch (_) {
            return 'Object';
        }
    }
    // errors
    if (val instanceof Error) {
        return `${val.name}: ${val.message}\n${val.stack}`;
    }
    // TODO we could test for more things here, like `Set`s and `Map`s.
    return className;
}

const CLOSURE_DTORS = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(state => {
    wasm.__wbindgen_export_2.get(state.dtor)(state.a, state.b)
});

function makeMutClosure(arg0, arg1, dtor, f) {
    const state = { a: arg0, b: arg1, cnt: 1, dtor };
    const real = (...args) => {
        // First up with a closure we increment the internal reference
        // count. This ensures that the Rust closure environment won't
        // be deallocated while we're invoking it.
        state.cnt++;
        const a = state.a;
        state.a = 0;
        try {
            return f(a, state.b, ...args);
        } finally {
            if (--state.cnt === 0) {
                wasm.__wbindgen_export_2.get(state.dtor)(a, state.b);
                CLOSURE_DTORS.unregister(state);
            } else {
                state.a = a;
            }
        }
    };
    real.original = state;
    CLOSURE_DTORS.register(real, state, state);
    return real;
}

function logError(f, args) {
    try {
        return f.apply(this, args);
    } catch (e) {
        let error = (function () {
            try {
                return e instanceof Error ? `${e.message}\n\nStack:\n${e.stack}` : e.toString();
            } catch(_) {
                return "<failed to stringify thrown value>";
            }
        }());
        console.error("wasm-bindgen: imported JS function that was not marked as `catch` threw an error:", error);
        throw e;
    }
}

let stack_pointer = 128;

function addBorrowedObject(obj) {
    if (stack_pointer == 1) throw new Error('out of js stack');
    heap[--stack_pointer] = obj;
    return stack_pointer;
}
function __wbg_adapter_48(arg0, arg1, arg2) {
    try {
        _assertNum(arg0);
        _assertNum(arg1);
        wasm._dyn_core__ops__function__FnMut___A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h9283ee076627a186(arg0, arg1, addBorrowedObject(arg2));
    } finally {
        heap[stack_pointer++] = undefined;
    }
}

function __wbg_adapter_51(arg0, arg1) {
    _assertNum(arg0);
    _assertNum(arg1);
    wasm._dyn_core__ops__function__FnMut_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h034aa9f1cb092c78(arg0, arg1);
}

function __wbg_adapter_54(arg0, arg1, arg2) {
    try {
        _assertNum(arg0);
        _assertNum(arg1);
        wasm._dyn_core__ops__function__FnMut___A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__hb808d29990395616(arg0, arg1, addBorrowedObject(arg2));
    } finally {
        heap[stack_pointer++] = undefined;
    }
}

function makeClosure(arg0, arg1, dtor, f) {
    const state = { a: arg0, b: arg1, cnt: 1, dtor };
    const real = (...args) => {
        // First up with a closure we increment the internal reference
        // count. This ensures that the Rust closure environment won't
        // be deallocated while we're invoking it.
        state.cnt++;
        try {
            return f(state.a, state.b, ...args);
        } finally {
            if (--state.cnt === 0) {
                wasm.__wbindgen_export_2.get(state.dtor)(state.a, state.b);
                state.a = 0;
                CLOSURE_DTORS.unregister(state);
            }
        }
    };
    real.original = state;
    CLOSURE_DTORS.register(real, state, state);
    return real;
}
function __wbg_adapter_57(arg0, arg1, arg2) {
    _assertNum(arg0);
    _assertNum(arg1);
    wasm._dyn_core__ops__function__Fn__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h5eae400ff4d57717(arg0, arg1, addHeapObject(arg2));
}

function __wbg_adapter_60(arg0, arg1, arg2) {
    _assertNum(arg0);
    _assertNum(arg1);
    wasm._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__hc8b177126e70c0d6(arg0, arg1, addHeapObject(arg2));
}

function handleError(f, args) {
    try {
        return f.apply(this, args);
    } catch (e) {
        wasm.__wbindgen_exn_store(addHeapObject(e));
    }
}

function passArrayJsValueToWasm0(array, malloc) {
    const ptr = malloc(array.length * 4, 4) >>> 0;
    const mem = getDataViewMemory0();
    for (let i = 0; i < array.length; i++) {
        mem.setUint32(ptr + 4 * i, addHeapObject(array[i]), true);
    }
    WASM_VECTOR_LEN = array.length;
    return ptr;
}

function getArrayJsValueFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    const mem = getDataViewMemory0();
    const result = [];
    for (let i = ptr; i < ptr + 4 * len; i += 4) {
        result.push(takeObject(mem.getUint32(i, true)));
    }
    return result;
}

const JSOwnerFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_jsowner_free(ptr >>> 0, 1));
/**
*/
export class JSOwner {

    constructor() {
        throw new Error('cannot invoke `new` directly');
    }

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(JSOwner.prototype);
        obj.__wbg_ptr = ptr;
        JSOwnerFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        JSOwnerFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_jsowner_free(ptr, 0);
    }
}

async function __wbg_load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);

            } catch (e) {
                if (module.headers.get('Content-Type') != 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else {
                    throw e;
                }
            }
        }

        const bytes = await module.arrayBuffer();
        return await WebAssembly.instantiate(bytes, imports);

    } else {
        const instance = await WebAssembly.instantiate(module, imports);

        if (instance instanceof WebAssembly.Instance) {
            return { instance, module };

        } else {
            return instance;
        }
    }
}

function __wbg_get_imports() {
    const imports = {};
    imports.wbg = {};
    imports.wbg.__wbindgen_object_clone_ref = function(arg0) {
        const ret = getObject(arg0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_string_new = function(arg0, arg1) {
        const ret = getStringFromWasm0(arg0, arg1);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_object_drop_ref = function(arg0) {
        takeObject(arg0);
    };
    imports.wbg.__wbindgen_cb_drop = function(arg0) {
        const obj = takeObject(arg0).original;
        if (obj.cnt-- == 1) {
            obj.a = 0;
            return true;
        }
        const ret = false;
        _assertBoolean(ret);
        return ret;
    };
    imports.wbg.__wbg_mark_40e050a77cc39fea = function() { return logError(function (arg0, arg1) {
        performance.mark(getStringFromWasm0(arg0, arg1));
    }, arguments) };
    imports.wbg.__wbg_log_c9486ca5d8e2cbe8 = function() { return logError(function (arg0, arg1) {
        let deferred0_0;
        let deferred0_1;
        try {
            deferred0_0 = arg0;
            deferred0_1 = arg1;
            console.log(getStringFromWasm0(arg0, arg1));
        } finally {
            wasm.__wbindgen_free(deferred0_0, deferred0_1, 1);
        }
    }, arguments) };
    imports.wbg.__wbg_log_aba5996d9bde071f = function() { return logError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7) {
        let deferred0_0;
        let deferred0_1;
        try {
            deferred0_0 = arg0;
            deferred0_1 = arg1;
            console.log(getStringFromWasm0(arg0, arg1), getStringFromWasm0(arg2, arg3), getStringFromWasm0(arg4, arg5), getStringFromWasm0(arg6, arg7));
        } finally {
            wasm.__wbindgen_free(deferred0_0, deferred0_1, 1);
        }
    }, arguments) };
    imports.wbg.__wbg_measure_aa7a73f17813f708 = function() { return handleError(function (arg0, arg1, arg2, arg3) {
        let deferred0_0;
        let deferred0_1;
        let deferred1_0;
        let deferred1_1;
        try {
            deferred0_0 = arg0;
            deferred0_1 = arg1;
            deferred1_0 = arg2;
            deferred1_1 = arg3;
            performance.measure(getStringFromWasm0(arg0, arg1), getStringFromWasm0(arg2, arg3));
        } finally {
            wasm.__wbindgen_free(deferred0_0, deferred0_1, 1);
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }, arguments) };
    imports.wbg.__wbindgen_boolean_get = function(arg0) {
        const v = getObject(arg0);
        const ret = typeof(v) === 'boolean' ? (v ? 1 : 0) : 2;
        _assertNum(ret);
        return ret;
    };
    imports.wbg.__wbindgen_is_bigint = function(arg0) {
        const ret = typeof(getObject(arg0)) === 'bigint';
        _assertBoolean(ret);
        return ret;
    };
    imports.wbg.__wbindgen_number_get = function(arg0, arg1) {
        const obj = getObject(arg1);
        const ret = typeof(obj) === 'number' ? obj : undefined;
        if (!isLikeNone(ret)) {
            _assertNum(ret);
        }
        getDataViewMemory0().setFloat64(arg0 + 8 * 1, isLikeNone(ret) ? 0 : ret, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, !isLikeNone(ret), true);
    };
    imports.wbg.__wbindgen_string_get = function(arg0, arg1) {
        const obj = getObject(arg1);
        const ret = typeof(obj) === 'string' ? obj : undefined;
        var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    };
    imports.wbg.__wbindgen_is_object = function(arg0) {
        const val = getObject(arg0);
        const ret = typeof(val) === 'object' && val !== null;
        _assertBoolean(ret);
        return ret;
    };
    imports.wbg.__wbindgen_in = function(arg0, arg1) {
        const ret = getObject(arg0) in getObject(arg1);
        _assertBoolean(ret);
        return ret;
    };
    imports.wbg.__wbindgen_bigint_from_i64 = function(arg0) {
        const ret = arg0;
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_jsval_eq = function(arg0, arg1) {
        const ret = getObject(arg0) === getObject(arg1);
        _assertBoolean(ret);
        return ret;
    };
    imports.wbg.__wbindgen_bigint_from_u64 = function(arg0) {
        const ret = BigInt.asUintN(64, arg0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_error_new = function(arg0, arg1) {
        const ret = new Error(getStringFromWasm0(arg0, arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_is_string = function(arg0) {
        const ret = typeof(getObject(arg0)) === 'string';
        _assertBoolean(ret);
        return ret;
    };
    imports.wbg.__wbg_getformdata_42029f06cbdc8637 = function() { return logError(function (arg0) {
        const ret = get_form_data(getObject(arg0));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_getselectdata_d0a959dad128957f = function() { return logError(function (arg0, arg1) {
        const ret = get_select_data(getObject(arg1));
        const ptr1 = passArrayJsValueToWasm0(ret, wasm.__wbindgen_malloc);
        const len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    }, arguments) };
    imports.wbg.__wbindgen_is_undefined = function(arg0) {
        const ret = getObject(arg0) === undefined;
        _assertBoolean(ret);
        return ret;
    };
    imports.wbg.__wbg_rustRecv_1b6f5c2d10fa7141 = function() { return logError(function (arg0) {
        const ret = getObject(arg0).rustRecv();
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_new_abda76e883ba8a5f = function() { return logError(function () {
        const ret = new Error();
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_stack_658279fe44541cf6 = function() { return logError(function (arg0, arg1) {
        const ret = getObject(arg1).stack;
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    }, arguments) };
    imports.wbg.__wbg_error_f851667af71bcfc6 = function() { return logError(function (arg0, arg1) {
        let deferred0_0;
        let deferred0_1;
        try {
            deferred0_0 = arg0;
            deferred0_1 = arg1;
            console.error(getStringFromWasm0(arg0, arg1));
        } finally {
            wasm.__wbindgen_free(deferred0_0, deferred0_1, 1);
        }
    }, arguments) };
    imports.wbg.__wbg_initialize_ed565fb65dd7bcd0 = function() { return logError(function (arg0, arg1, arg2) {
        getObject(arg0).initialize(takeObject(arg1), getObject(arg2));
    }, arguments) };
    imports.wbg.__wbg_saveTemplate_e7948f8b7f54cfc9 = function() { return logError(function (arg0, arg1, arg2, arg3) {
        var v0 = getArrayJsValueFromWasm0(arg1, arg2).slice();
        wasm.__wbindgen_free(arg1, arg2 * 4, 4);
        getObject(arg0).saveTemplate(v0, arg3);
    }, arguments) };
    imports.wbg.__wbg_getNode_d4db5a1d3bc15eae = function() { return logError(function (arg0, arg1) {
        const ret = getObject(arg0).getNode(arg1 >>> 0);
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_new_0924b9f1bd6dd838 = function() { return logError(function (arg0) {
        const ret = new RawInterpreter(arg0 >>> 0);
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_updatememory_0fc68f206c0463de = function() { return logError(function (arg0, arg1) {
        getObject(arg0).update_memory(takeObject(arg1));
    }, arguments) };
    imports.wbg.__wbg_run_47308773c14de567 = function() { return logError(function (arg0) {
        getObject(arg0).run();
    }, arguments) };
    imports.wbg.__wbg_new_c44bca3b6f94b9dc = function() { return logError(function (arg0) {
        const ret = new WebDioxusChannel(JSOwner.__wrap(arg0));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_weak_cd28ee08d42a73ad = function() { return logError(function (arg0) {
        const ret = getObject(arg0).weak();
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_rustSend_3b93da3cdc175722 = function() { return logError(function (arg0, arg1) {
        getObject(arg0).rustSend(takeObject(arg1));
    }, arguments) };
    imports.wbg.__wbg_setAttributeInner_2820d9f570125088 = function() { return logError(function (arg0, arg1, arg2, arg3, arg4, arg5) {
        setAttributeInner(takeObject(arg0), getStringFromWasm0(arg1, arg2), takeObject(arg3), arg4 === 0 ? undefined : getStringFromWasm0(arg4, arg5));
    }, arguments) };
    imports.wbg.__wbindgen_number_new = function(arg0) {
        const ret = arg0;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_queueMicrotask_48421b3cc9052b68 = function() { return logError(function (arg0) {
        const ret = getObject(arg0).queueMicrotask;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbindgen_is_function = function(arg0) {
        const ret = typeof(getObject(arg0)) === 'function';
        _assertBoolean(ret);
        return ret;
    };
    imports.wbg.__wbg_queueMicrotask_12a30234db4045d3 = function() { return logError(function (arg0) {
        queueMicrotask(getObject(arg0));
    }, arguments) };
    imports.wbg.__wbindgen_jsval_loose_eq = function(arg0, arg1) {
        const ret = getObject(arg0) == getObject(arg1);
        _assertBoolean(ret);
        return ret;
    };
    imports.wbg.__wbg_String_88810dfeb4021902 = function() { return logError(function (arg0, arg1) {
        const ret = String(getObject(arg1));
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    }, arguments) };
    imports.wbg.__wbg_set_841ac57cff3d672b = function() { return logError(function (arg0, arg1, arg2) {
        getObject(arg0)[takeObject(arg1)] = takeObject(arg2);
    }, arguments) };
    imports.wbg.__wbg_error_71d6845bf00a930f = function() { return logError(function (arg0, arg1) {
        var v0 = getArrayJsValueFromWasm0(arg0, arg1).slice();
        wasm.__wbindgen_free(arg0, arg1 * 4, 4);
        console.error(...v0);
    }, arguments) };
    imports.wbg.__wbg_instanceof_Window_5012736c80a01584 = function() { return logError(function (arg0) {
        let result;
        try {
            result = getObject(arg0) instanceof Window;
        } catch (_) {
            result = false;
        }
        const ret = result;
        _assertBoolean(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_document_8554450897a855b9 = function() { return logError(function (arg0) {
        const ret = getObject(arg0).document;
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_location_af118da6c50d4c3f = function() { return logError(function (arg0) {
        const ret = getObject(arg0).location;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_history_489e13d0b625263c = function() { return handleError(function (arg0) {
        const ret = getObject(arg0).history;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_scrollX_9ecf42cb7727879a = function() { return handleError(function (arg0) {
        const ret = getObject(arg0).scrollX;
        return ret;
    }, arguments) };
    imports.wbg.__wbg_scrollY_6c77ecd791d604dc = function() { return handleError(function (arg0) {
        const ret = getObject(arg0).scrollY;
        return ret;
    }, arguments) };
    imports.wbg.__wbg_scrollTo_19dc1dbbc8c19fa8 = function() { return logError(function (arg0, arg1, arg2) {
        getObject(arg0).scrollTo(arg1, arg2);
    }, arguments) };
    imports.wbg.__wbg_cancelAnimationFrame_f80ecdad075d1d55 = function() { return handleError(function (arg0, arg1) {
        getObject(arg0).cancelAnimationFrame(arg1);
    }, arguments) };
    imports.wbg.__wbg_requestAnimationFrame_b4b782250b9c2c88 = function() { return handleError(function (arg0, arg1) {
        const ret = getObject(arg0).requestAnimationFrame(getObject(arg1));
        _assertNum(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_createElement_5921e9eb06b9ec89 = function() { return handleError(function (arg0, arg1, arg2) {
        const ret = getObject(arg0).createElement(getStringFromWasm0(arg1, arg2));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_createElementNS_78308ee7091c53f7 = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
        const ret = getObject(arg0).createElementNS(arg1 === 0 ? undefined : getStringFromWasm0(arg1, arg2), getStringFromWasm0(arg3, arg4));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_createTextNode_8bce33cf33bf8f6e = function() { return logError(function (arg0, arg1, arg2) {
        const ret = getObject(arg0).createTextNode(getStringFromWasm0(arg1, arg2));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_getElementById_f56c8e6a15a6926d = function() { return logError(function (arg0, arg1, arg2) {
        const ret = getObject(arg0).getElementById(getStringFromWasm0(arg1, arg2));
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_instanceof_Element_cc034878d52a64fa = function() { return logError(function (arg0) {
        let result;
        try {
            result = getObject(arg0) instanceof Element;
        } catch (_) {
            result = false;
        }
        const ret = result;
        _assertBoolean(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_getAttribute_e867e037f066c410 = function() { return logError(function (arg0, arg1, arg2, arg3) {
        const ret = getObject(arg1).getAttribute(getStringFromWasm0(arg2, arg3));
        var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    }, arguments) };
    imports.wbg.__wbg_getBoundingClientRect_35fc4d8fa10e0463 = function() { return logError(function (arg0) {
        const ret = getObject(arg0).getBoundingClientRect();
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_scrollIntoView_569698f4895545a5 = function() { return logError(function (arg0, arg1) {
        getObject(arg0).scrollIntoView(getObject(arg1));
    }, arguments) };
    imports.wbg.__wbg_toggleAttribute_38f9aa9917ec0dbb = function() { return handleError(function (arg0, arg1, arg2) {
        const ret = getObject(arg0).toggleAttribute(getStringFromWasm0(arg1, arg2));
        _assertBoolean(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_instanceof_HtmlElement_ee6cb55e6b90ae79 = function() { return logError(function (arg0) {
        let result;
        try {
            result = getObject(arg0) instanceof HTMLElement;
        } catch (_) {
            result = false;
        }
        const ret = result;
        _assertBoolean(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_blur_2097e550054a8dc8 = function() { return handleError(function (arg0) {
        getObject(arg0).blur();
    }, arguments) };
    imports.wbg.__wbg_focus_06621101cc79f5d8 = function() { return handleError(function (arg0) {
        getObject(arg0).focus();
    }, arguments) };
    imports.wbg.__wbg_length_a547e4226b069330 = function() { return logError(function (arg0) {
        const ret = getObject(arg0).length;
        _assertNum(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_get_6bee5bc8192fd59e = function() { return logError(function (arg0, arg1) {
        const ret = getObject(arg0)[arg1 >>> 0];
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_files_a4b6a9811697ac84 = function() { return logError(function (arg0) {
        const ret = getObject(arg0).files;
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_error_09480e4aadca50ad = function() { return logError(function (arg0) {
        console.error(getObject(arg0));
    }, arguments) };
    imports.wbg.__wbg_animationName_a25d2fb38e676a5c = function() { return logError(function (arg0, arg1) {
        const ret = getObject(arg1).animationName;
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    }, arguments) };
    imports.wbg.__wbg_elapsedTime_7c25b13bb9c48d8b = function() { return logError(function (arg0) {
        const ret = getObject(arg0).elapsedTime;
        return ret;
    }, arguments) };
    imports.wbg.__wbg_pseudoElement_275ba9e63313f644 = function() { return logError(function (arg0, arg1) {
        const ret = getObject(arg1).pseudoElement;
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    }, arguments) };
    imports.wbg.__wbg_top_322638693276a225 = function() { return logError(function (arg0) {
        const ret = getObject(arg0).top;
        return ret;
    }, arguments) };
    imports.wbg.__wbg_left_ec3af38bed003a86 = function() { return logError(function (arg0) {
        const ret = getObject(arg0).left;
        return ret;
    }, arguments) };
    imports.wbg.__wbg_setscrollRestoration_05ceea7938b9d948 = function() { return handleError(function (arg0, arg1) {
        getObject(arg0).scrollRestoration = ["auto","manual",][arg1];
    }, arguments) };
    imports.wbg.__wbg_state_b863826253700666 = function() { return handleError(function (arg0) {
        const ret = getObject(arg0).state;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_back_2b44401f98571e5e = function() { return handleError(function (arg0) {
        getObject(arg0).back();
    }, arguments) };
    imports.wbg.__wbg_forward_5d07c36d03f1d798 = function() { return handleError(function (arg0) {
        getObject(arg0).forward();
    }, arguments) };
    imports.wbg.__wbg_pushState_fc8b2d0c45854901 = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5) {
        getObject(arg0).pushState(getObject(arg1), getStringFromWasm0(arg2, arg3), arg4 === 0 ? undefined : getStringFromWasm0(arg4, arg5));
    }, arguments) };
    imports.wbg.__wbg_replaceState_c3213575ed65bac2 = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5) {
        getObject(arg0).replaceState(getObject(arg1), getStringFromWasm0(arg2, arg3), arg4 === 0 ? undefined : getStringFromWasm0(arg4, arg5));
    }, arguments) };
    imports.wbg.__wbg_instanceof_HtmlTextAreaElement_75bfdd55ca1a4a97 = function() { return logError(function (arg0) {
        let result;
        try {
            result = getObject(arg0) instanceof HTMLTextAreaElement;
        } catch (_) {
            result = false;
        }
        const ret = result;
        _assertBoolean(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_value_2bde81aca5570118 = function() { return logError(function (arg0, arg1) {
        const ret = getObject(arg1).value;
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    }, arguments) };
    imports.wbg.__wbg_setbehavior_689064979b5313ca = function() { return logError(function (arg0, arg1) {
        getObject(arg0).behavior = ["auto","instant","smooth",][arg1];
    }, arguments) };
    imports.wbg.__wbg_data_ee8c1a738c70cbe1 = function() { return logError(function (arg0, arg1) {
        const ret = getObject(arg1).data;
        var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    }, arguments) };
    imports.wbg.__wbg_type_de7927c168e34c94 = function() { return logError(function (arg0, arg1) {
        const ret = getObject(arg1).type;
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    }, arguments) };
    imports.wbg.__wbg_target_b7cb1739bee70928 = function() { return logError(function (arg0) {
        const ret = getObject(arg0).target;
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_bubbles_e9828bf0dd0393c5 = function() { return logError(function (arg0) {
        const ret = getObject(arg0).bubbles;
        _assertBoolean(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_preventDefault_c55d86c27b2dfa6e = function() { return logError(function (arg0) {
        getObject(arg0).preventDefault();
    }, arguments) };
    imports.wbg.__wbg_screenX_a2b9e6e22863d206 = function() { return logError(function (arg0) {
        const ret = getObject(arg0).screenX;
        _assertNum(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_screenY_6c27a423dc6f291e = function() { return logError(function (arg0) {
        const ret = getObject(arg0).screenY;
        _assertNum(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_clientX_3967ecd5e962e1b2 = function() { return logError(function (arg0) {
        const ret = getObject(arg0).clientX;
        _assertNum(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_clientY_37d418b8d9c0bb52 = function() { return logError(function (arg0) {
        const ret = getObject(arg0).clientY;
        _assertNum(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_offsetX_e7047852d4b4b482 = function() { return logError(function (arg0) {
        const ret = getObject(arg0).offsetX;
        _assertNum(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_offsetY_76fc66e0e449645e = function() { return logError(function (arg0) {
        const ret = getObject(arg0).offsetY;
        _assertNum(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_ctrlKey_957c6c31b62b4550 = function() { return logError(function (arg0) {
        const ret = getObject(arg0).ctrlKey;
        _assertBoolean(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_shiftKey_8c0f9a5ca3ff8f93 = function() { return logError(function (arg0) {
        const ret = getObject(arg0).shiftKey;
        _assertBoolean(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_altKey_d3fbce7596aac8cf = function() { return logError(function (arg0) {
        const ret = getObject(arg0).altKey;
        _assertBoolean(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_metaKey_be0158b14b1cef4a = function() { return logError(function (arg0) {
        const ret = getObject(arg0).metaKey;
        _assertBoolean(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_button_460cdec9f2512a91 = function() { return logError(function (arg0) {
        const ret = getObject(arg0).button;
        _assertNum(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_buttons_a302533e27733599 = function() { return logError(function (arg0) {
        const ret = getObject(arg0).buttons;
        _assertNum(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_identifier_e39f89e9c0a1a3fc = function() { return logError(function (arg0) {
        const ret = getObject(arg0).identifier;
        _assertNum(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_screenX_97af391f958a6dac = function() { return logError(function (arg0) {
        const ret = getObject(arg0).screenX;
        _assertNum(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_screenY_f3de769d7575b4b0 = function() { return logError(function (arg0) {
        const ret = getObject(arg0).screenY;
        _assertNum(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_clientX_6ea27dc5cef626dd = function() { return logError(function (arg0) {
        const ret = getObject(arg0).clientX;
        _assertNum(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_clientY_78f18a39f2f06125 = function() { return logError(function (arg0) {
        const ret = getObject(arg0).clientY;
        _assertNum(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_pageX_ad400ce6aa9e836c = function() { return logError(function (arg0) {
        const ret = getObject(arg0).pageX;
        _assertNum(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_pageY_a82980c94c0070ce = function() { return logError(function (arg0) {
        const ret = getObject(arg0).pageY;
        _assertNum(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_radiusX_22810fa561a748ec = function() { return logError(function (arg0) {
        const ret = getObject(arg0).radiusX;
        _assertNum(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_radiusY_af409122cb130f38 = function() { return logError(function (arg0) {
        const ret = getObject(arg0).radiusY;
        _assertNum(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_rotationAngle_ded80096102994d6 = function() { return logError(function (arg0) {
        const ret = getObject(arg0).rotationAngle;
        return ret;
    }, arguments) };
    imports.wbg.__wbg_force_f43e873103b4f9c8 = function() { return logError(function (arg0) {
        const ret = getObject(arg0).force;
        return ret;
    }, arguments) };
    imports.wbg.__wbg_width_85b26c6209197aa9 = function() { return logError(function (arg0) {
        const ret = getObject(arg0).width;
        return ret;
    }, arguments) };
    imports.wbg.__wbg_height_f270f53dbfb3a632 = function() { return logError(function (arg0) {
        const ret = getObject(arg0).height;
        return ret;
    }, arguments) };
    imports.wbg.__wbg_instanceof_DragEvent_29469534b31c3761 = function() { return logError(function (arg0) {
        let result;
        try {
            result = getObject(arg0) instanceof DragEvent;
        } catch (_) {
            result = false;
        }
        const ret = result;
        _assertBoolean(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_dataTransfer_2fb708051ee2c64c = function() { return logError(function (arg0) {
        const ret = getObject(arg0).dataTransfer;
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_altKey_5a6eb49ec8194792 = function() { return logError(function (arg0) {
        const ret = getObject(arg0).altKey;
        _assertBoolean(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_ctrlKey_319ff2374dc7f372 = function() { return logError(function (arg0) {
        const ret = getObject(arg0).ctrlKey;
        _assertBoolean(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_shiftKey_f38dee34420e0d62 = function() { return logError(function (arg0) {
        const ret = getObject(arg0).shiftKey;
        _assertBoolean(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_metaKey_00fdcfadf1968d45 = function() { return logError(function (arg0) {
        const ret = getObject(arg0).metaKey;
        _assertBoolean(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_location_d7fe3090ad7e80d7 = function() { return logError(function (arg0) {
        const ret = getObject(arg0).location;
        _assertNum(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_repeat_8451a79b3608855b = function() { return logError(function (arg0) {
        const ret = getObject(arg0).repeat;
        _assertBoolean(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_isComposing_1c9533ed199eaf7b = function() { return logError(function (arg0) {
        const ret = getObject(arg0).isComposing;
        _assertBoolean(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_key_a626396efbca2b95 = function() { return logError(function (arg0, arg1) {
        const ret = getObject(arg1).key;
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    }, arguments) };
    imports.wbg.__wbg_code_01dc6af887ca9ecb = function() { return logError(function (arg0, arg1) {
        const ret = getObject(arg1).code;
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    }, arguments) };
    imports.wbg.__wbg_addEventListener_14b036ff7cb8747c = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).addEventListener(getStringFromWasm0(arg1, arg2), getObject(arg3), getObject(arg4));
    }, arguments) };
    imports.wbg.__wbg_removeEventListener_7878b86efe1ab901 = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).removeEventListener(getStringFromWasm0(arg1, arg2), getObject(arg3), arg4 !== 0);
    }, arguments) };
    imports.wbg.__wbg_instanceof_HtmlInputElement_88bf515ab1d9511d = function() { return logError(function (arg0) {
        let result;
        try {
            result = getObject(arg0) instanceof HTMLInputElement;
        } catch (_) {
            result = false;
        }
        const ret = result;
        _assertBoolean(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_checked_1502c0098821afb3 = function() { return logError(function (arg0) {
        const ret = getObject(arg0).checked;
        _assertBoolean(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_files_b94d8f21e2b53924 = function() { return logError(function (arg0) {
        const ret = getObject(arg0).files;
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_type_c29d14b7056ba1f2 = function() { return logError(function (arg0, arg1) {
        const ret = getObject(arg1).type;
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    }, arguments) };
    imports.wbg.__wbg_value_d4a95e7a0d390578 = function() { return logError(function (arg0, arg1) {
        const ret = getObject(arg1).value;
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    }, arguments) };
    imports.wbg.__wbg_length_f2469772b8ec9ea3 = function() { return logError(function (arg0) {
        const ret = getObject(arg0).length;
        _assertNum(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_item_9b2e820a37b4fd3f = function() { return logError(function (arg0, arg1) {
        const ret = getObject(arg0).item(arg1 >>> 0);
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_instanceof_HtmlFormElement_f1c49500e9c5fd42 = function() { return logError(function (arg0) {
        let result;
        try {
            result = getObject(arg0) instanceof HTMLFormElement;
        } catch (_) {
            result = false;
        }
        const ret = result;
        _assertBoolean(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_instanceof_HtmlSelectElement_c54017bd3db58d85 = function() { return logError(function (arg0) {
        let result;
        try {
            result = getObject(arg0) instanceof HTMLSelectElement;
        } catch (_) {
            result = false;
        }
        const ret = result;
        _assertBoolean(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_value_9991233a797a5fbc = function() { return logError(function (arg0, arg1) {
        const ret = getObject(arg1).value;
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    }, arguments) };
    imports.wbg.__wbg_sethref_9d76f6c9356e9638 = function() { return handleError(function (arg0, arg1, arg2) {
        getObject(arg0).href = getStringFromWasm0(arg1, arg2);
    }, arguments) };
    imports.wbg.__wbg_pathname_6e6871539b48a0e5 = function() { return handleError(function (arg0, arg1) {
        const ret = getObject(arg1).pathname;
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    }, arguments) };
    imports.wbg.__wbg_search_20c15d493b8602c5 = function() { return handleError(function (arg0, arg1) {
        const ret = getObject(arg1).search;
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    }, arguments) };
    imports.wbg.__wbg_hash_313d7fdf42f6e7d3 = function() { return handleError(function (arg0, arg1) {
        const ret = getObject(arg1).hash;
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    }, arguments) };
    imports.wbg.__wbg_pointerId_37ae0c4682f85248 = function() { return logError(function (arg0) {
        const ret = getObject(arg0).pointerId;
        _assertNum(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_width_70c5d7e6af348a38 = function() { return logError(function (arg0) {
        const ret = getObject(arg0).width;
        _assertNum(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_height_a5d778a85404eb96 = function() { return logError(function (arg0) {
        const ret = getObject(arg0).height;
        _assertNum(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_pressure_95cee3909a8549a9 = function() { return logError(function (arg0) {
        const ret = getObject(arg0).pressure;
        return ret;
    }, arguments) };
    imports.wbg.__wbg_tangentialPressure_84b6cda1a0a6cdd3 = function() { return logError(function (arg0) {
        const ret = getObject(arg0).tangentialPressure;
        return ret;
    }, arguments) };
    imports.wbg.__wbg_tiltX_03e16b531a55705c = function() { return logError(function (arg0) {
        const ret = getObject(arg0).tiltX;
        _assertNum(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_tiltY_c4495a000352e925 = function() { return logError(function (arg0) {
        const ret = getObject(arg0).tiltY;
        _assertNum(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_twist_097164cec18dfd43 = function() { return logError(function (arg0) {
        const ret = getObject(arg0).twist;
        _assertNum(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_pointerType_d375491a3013a9bc = function() { return logError(function (arg0, arg1) {
        const ret = getObject(arg1).pointerType;
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    }, arguments) };
    imports.wbg.__wbg_isPrimary_30dbb508f538b2c6 = function() { return logError(function (arg0) {
        const ret = getObject(arg0).isPrimary;
        _assertBoolean(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_propertyName_c388e5bef92996bc = function() { return logError(function (arg0, arg1) {
        const ret = getObject(arg1).propertyName;
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    }, arguments) };
    imports.wbg.__wbg_elapsedTime_6560901e49831891 = function() { return logError(function (arg0) {
        const ret = getObject(arg0).elapsedTime;
        return ret;
    }, arguments) };
    imports.wbg.__wbg_pseudoElement_fb75011eff7853cb = function() { return logError(function (arg0, arg1) {
        const ret = getObject(arg1).pseudoElement;
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    }, arguments) };
    imports.wbg.__wbg_setcapture_4818ebe9ef88b2f6 = function() { return logError(function (arg0, arg1) {
        getObject(arg0).capture = arg1 !== 0;
    }, arguments) };
    imports.wbg.__wbg_setonce_06b35a72a3fafc15 = function() { return logError(function (arg0, arg1) {
        getObject(arg0).once = arg1 !== 0;
    }, arguments) };
    imports.wbg.__wbg_setpassive_70ce6704aec553f6 = function() { return logError(function (arg0, arg1) {
        getObject(arg0).passive = arg1 !== 0;
    }, arguments) };
    imports.wbg.__wbg_instanceof_Node_807587297afc161b = function() { return logError(function (arg0) {
        let result;
        try {
            result = getObject(arg0) instanceof Node;
        } catch (_) {
            result = false;
        }
        const ret = result;
        _assertBoolean(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_ownerDocument_40abde2abeff331c = function() { return logError(function (arg0) {
        const ret = getObject(arg0).ownerDocument;
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_parentElement_fbf8d048e797326d = function() { return logError(function (arg0) {
        const ret = getObject(arg0).parentElement;
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_textContent_a049d1ce093c3d21 = function() { return logError(function (arg0, arg1) {
        const ret = getObject(arg1).textContent;
        var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    }, arguments) };
    imports.wbg.__wbg_appendChild_ac45d1abddf1b89b = function() { return handleError(function (arg0, arg1) {
        const ret = getObject(arg0).appendChild(getObject(arg1));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_pageX_659907bf27190521 = function() { return logError(function (arg0) {
        const ret = getObject(arg0).pageX;
        _assertNum(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_pageY_b68450e73ec8e0cd = function() { return logError(function (arg0) {
        const ret = getObject(arg0).pageY;
        _assertNum(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_deltaX_7f4a9de8338c7ca6 = function() { return logError(function (arg0) {
        const ret = getObject(arg0).deltaX;
        return ret;
    }, arguments) };
    imports.wbg.__wbg_deltaY_606f12aa66daba69 = function() { return logError(function (arg0) {
        const ret = getObject(arg0).deltaY;
        return ret;
    }, arguments) };
    imports.wbg.__wbg_deltaZ_26a5a3b333712a18 = function() { return logError(function (arg0) {
        const ret = getObject(arg0).deltaZ;
        return ret;
    }, arguments) };
    imports.wbg.__wbg_deltaMode_d6b849e45efd0f5e = function() { return logError(function (arg0) {
        const ret = getObject(arg0).deltaMode;
        _assertNum(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_result_3869032b57f861ac = function() { return handleError(function (arg0) {
        const ret = getObject(arg0).result;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_setonload_71d51f79887a9257 = function() { return logError(function (arg0, arg1) {
        getObject(arg0).onload = getObject(arg1);
    }, arguments) };
    imports.wbg.__wbg_new_8515b7401632bd44 = function() { return handleError(function () {
        const ret = new FileReader();
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_readAsArrayBuffer_6475a86a924a8856 = function() { return handleError(function (arg0, arg1) {
        getObject(arg0).readAsArrayBuffer(getObject(arg1));
    }, arguments) };
    imports.wbg.__wbg_readAsText_c056625cb937da4e = function() { return handleError(function (arg0, arg1) {
        getObject(arg0).readAsText(getObject(arg1));
    }, arguments) };
    imports.wbg.__wbg_name_ed3cda975cce080d = function() { return logError(function (arg0, arg1) {
        const ret = getObject(arg1).name;
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    }, arguments) };
    imports.wbg.__wbg_touches_91ecfe75e4e0bff0 = function() { return logError(function (arg0) {
        const ret = getObject(arg0).touches;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_targetTouches_1a1415d264d9114f = function() { return logError(function (arg0) {
        const ret = getObject(arg0).targetTouches;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_changedTouches_8a2627b3dec12eed = function() { return logError(function (arg0) {
        const ret = getObject(arg0).changedTouches;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_altKey_cac6fdedfe2f5107 = function() { return logError(function (arg0) {
        const ret = getObject(arg0).altKey;
        _assertBoolean(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_metaKey_795ff1b49c71ce3a = function() { return logError(function (arg0) {
        const ret = getObject(arg0).metaKey;
        _assertBoolean(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_ctrlKey_70bbb597e1c54bbb = function() { return logError(function (arg0) {
        const ret = getObject(arg0).ctrlKey;
        _assertBoolean(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_shiftKey_fd3ea494de0c3475 = function() { return logError(function (arg0) {
        const ret = getObject(arg0).shiftKey;
        _assertBoolean(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_charCodeAt_e6c7cb2b8c394908 = function() { return logError(function (arg0, arg1) {
        const ret = getObject(arg0).charCodeAt(arg1 >>> 0);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_get_3baa728f9d58d3f6 = function() { return logError(function (arg0, arg1) {
        const ret = getObject(arg0)[arg1 >>> 0];
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_length_ae22078168b726f5 = function() { return logError(function (arg0) {
        const ret = getObject(arg0).length;
        _assertNum(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_new_a220cf903aa02ca2 = function() { return logError(function () {
        const ret = new Array();
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_newnoargs_76313bd6ff35d0f2 = function() { return logError(function (arg0, arg1) {
        const ret = new Function(getStringFromWasm0(arg0, arg1));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_new_8608a2b51a5f6737 = function() { return logError(function () {
        const ret = new Map();
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_next_de3e9db4440638b2 = function() { return logError(function (arg0) {
        const ret = getObject(arg0).next;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_next_f9cb570345655b9a = function() { return handleError(function (arg0) {
        const ret = getObject(arg0).next();
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_done_bfda7aa8f252b39f = function() { return logError(function (arg0) {
        const ret = getObject(arg0).done;
        _assertBoolean(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_value_6d39332ab4788d86 = function() { return logError(function (arg0) {
        const ret = getObject(arg0).value;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_iterator_888179a48810a9fe = function() { return logError(function () {
        const ret = Symbol.iterator;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_get_224d16597dbbfd96 = function() { return handleError(function (arg0, arg1) {
        const ret = Reflect.get(getObject(arg0), getObject(arg1));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_call_1084a111329e68ce = function() { return handleError(function (arg0, arg1) {
        const ret = getObject(arg0).call(getObject(arg1));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_new_525245e2b9901204 = function() { return logError(function () {
        const ret = new Object();
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_length_c582046780042f90 = function() { return logError(function (arg0) {
        const ret = getObject(arg0).length;
        _assertNum(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_self_3093d5d1f7bcb682 = function() { return handleError(function () {
        const ret = self.self;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_window_3bcfc4d31bc012f8 = function() { return handleError(function () {
        const ret = window.window;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_globalThis_86b222e13bdf32ed = function() { return handleError(function () {
        const ret = globalThis.globalThis;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_global_e5a3fe56f8be9485 = function() { return handleError(function () {
        const ret = global.global;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_set_673dda6c73d19609 = function() { return logError(function (arg0, arg1, arg2) {
        getObject(arg0)[arg1 >>> 0] = takeObject(arg2);
    }, arguments) };
    imports.wbg.__wbg_isArray_8364a5371e9737d8 = function() { return logError(function (arg0) {
        const ret = Array.isArray(getObject(arg0));
        _assertBoolean(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_push_37c89022f34c01ca = function() { return logError(function (arg0, arg1) {
        const ret = getObject(arg0).push(getObject(arg1));
        _assertNum(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_instanceof_ArrayBuffer_61dfc3198373c902 = function() { return logError(function (arg0) {
        let result;
        try {
            result = getObject(arg0) instanceof ArrayBuffer;
        } catch (_) {
            result = false;
        }
        const ret = result;
        _assertBoolean(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_newwithargs_7bac380a0f0eff16 = function() { return logError(function (arg0, arg1, arg2, arg3) {
        const ret = new Function(getStringFromWasm0(arg0, arg1), getStringFromWasm0(arg2, arg3));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_call_89af060b4e1523f2 = function() { return handleError(function (arg0, arg1, arg2) {
        const ret = getObject(arg0).call(getObject(arg1), getObject(arg2));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_set_49185437f0ab06f8 = function() { return logError(function (arg0, arg1, arg2) {
        const ret = getObject(arg0).set(getObject(arg1), getObject(arg2));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_entries_2f5ddf03b53c6730 = function() { return logError(function (arg0) {
        const ret = getObject(arg0).entries();
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_isSafeInteger_7f1ed56200d90674 = function() { return logError(function (arg0) {
        const ret = Number.isSafeInteger(getObject(arg0));
        _assertBoolean(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_entries_7a0e06255456ebcd = function() { return logError(function (arg0) {
        const ret = Object.entries(getObject(arg0));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_resolve_570458cb99d56a43 = function() { return logError(function (arg0) {
        const ret = Promise.resolve(getObject(arg0));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_then_95e6edc0f89b73b1 = function() { return logError(function (arg0, arg1) {
        const ret = getObject(arg0).then(getObject(arg1));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_then_876bb3c633745cc6 = function() { return logError(function (arg0, arg1, arg2) {
        const ret = getObject(arg0).then(getObject(arg1), getObject(arg2));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_buffer_b7b08af79b0b0974 = function() { return logError(function (arg0) {
        const ret = getObject(arg0).buffer;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_new_ea1883e1e5e86686 = function() { return logError(function (arg0) {
        const ret = new Uint8Array(getObject(arg0));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_set_d1e79e2388520f18 = function() { return logError(function (arg0, arg1, arg2) {
        getObject(arg0).set(getObject(arg1), arg2 >>> 0);
    }, arguments) };
    imports.wbg.__wbg_length_8339fcf5d8ecd12e = function() { return logError(function (arg0) {
        const ret = getObject(arg0).length;
        _assertNum(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_instanceof_Uint8Array_247a91427532499e = function() { return logError(function (arg0) {
        let result;
        try {
            result = getObject(arg0) instanceof Uint8Array;
        } catch (_) {
            result = false;
        }
        const ret = result;
        _assertBoolean(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_stringify_bbf45426c92a6bf5 = function() { return handleError(function (arg0) {
        const ret = JSON.stringify(getObject(arg0));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbindgen_bigint_get_as_i64 = function(arg0, arg1) {
        const v = getObject(arg1);
        const ret = typeof(v) === 'bigint' ? v : undefined;
        if (!isLikeNone(ret)) {
            _assertBigInt(ret);
        }
        getDataViewMemory0().setBigInt64(arg0 + 8 * 1, isLikeNone(ret) ? BigInt(0) : ret, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, !isLikeNone(ret), true);
    };
    imports.wbg.__wbindgen_debug_string = function(arg0, arg1) {
        const ret = debugString(getObject(arg1));
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    };
    imports.wbg.__wbindgen_throw = function(arg0, arg1) {
        throw new Error(getStringFromWasm0(arg0, arg1));
    };
    imports.wbg.__wbindgen_memory = function() {
        const ret = wasm.memory;
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper322 = function() { return logError(function (arg0, arg1, arg2) {
        const ret = makeMutClosure(arg0, arg1, 71, __wbg_adapter_48);
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbindgen_closure_wrapper1126 = function() { return logError(function (arg0, arg1, arg2) {
        const ret = makeMutClosure(arg0, arg1, 607, __wbg_adapter_51);
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbindgen_closure_wrapper1128 = function() { return logError(function (arg0, arg1, arg2) {
        const ret = makeMutClosure(arg0, arg1, 607, __wbg_adapter_54);
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbindgen_closure_wrapper1425 = function() { return logError(function (arg0, arg1, arg2) {
        const ret = makeClosure(arg0, arg1, 759, __wbg_adapter_57);
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbindgen_closure_wrapper1626 = function() { return logError(function (arg0, arg1, arg2) {
        const ret = makeMutClosure(arg0, arg1, 812, __wbg_adapter_60);
        return addHeapObject(ret);
    }, arguments) };

    return imports;
}

function __wbg_init_memory(imports, memory) {

}

function __wbg_finalize_init(instance, module) {
    wasm = instance.exports;
    __wbg_init.__wbindgen_wasm_module = module;
    cachedDataViewMemory0 = null;
    cachedUint8ArrayMemory0 = null;


    wasm.__wbindgen_start();
    return wasm;
}

function initSync(module) {
    if (wasm !== undefined) return wasm;


    if (typeof module !== 'undefined' && Object.getPrototypeOf(module) === Object.prototype)
    ({module} = module)
    else
    console.warn('using deprecated parameters for `initSync()`; pass a single object instead')

    const imports = __wbg_get_imports();

    __wbg_init_memory(imports);

    if (!(module instanceof WebAssembly.Module)) {
        module = new WebAssembly.Module(module);
    }

    const instance = new WebAssembly.Instance(module, imports);

    return __wbg_finalize_init(instance, module);
}

async function __wbg_init(module_or_path) {
    if (wasm !== undefined) return wasm;


    if (typeof module_or_path !== 'undefined' && Object.getPrototypeOf(module_or_path) === Object.prototype)
    ({module_or_path} = module_or_path)
    else
    console.warn('using deprecated parameters for the initialization function; pass a single object instead')


    const imports = __wbg_get_imports();

    if (typeof module_or_path === 'string' || (typeof Request === 'function' && module_or_path instanceof Request) || (typeof URL === 'function' && module_or_path instanceof URL)) {
        module_or_path = fetch(module_or_path);
    }

    __wbg_init_memory(imports);

    const { instance, module } = await __wbg_load(await module_or_path, imports);

    return __wbg_finalize_init(instance, module);
}

export { initSync };
export default __wbg_init;
