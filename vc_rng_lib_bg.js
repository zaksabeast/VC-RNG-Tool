let wasm;
export function __wbg_set_wasm(val) {
    wasm = val;
}


const lTextDecoder = typeof TextDecoder === 'undefined' ? (0, module.require)('util').TextDecoder : TextDecoder;

let cachedTextDecoder = new lTextDecoder('utf-8', { ignoreBOM: true, fatal: true });

cachedTextDecoder.decode();

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

function _assertClass(instance, klass) {
    if (!(instance instanceof klass)) {
        throw new Error(`expected instance of ${klass.name}`);
    }
}

let cachedDataViewMemory0 = null;

function getDataViewMemory0() {
    if (cachedDataViewMemory0 === null || cachedDataViewMemory0.buffer.detached === true || (cachedDataViewMemory0.buffer.detached === undefined && cachedDataViewMemory0.buffer !== wasm.memory.buffer)) {
        cachedDataViewMemory0 = new DataView(wasm.memory.buffer);
    }
    return cachedDataViewMemory0;
}

function getArrayJsValueFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    const mem = getDataViewMemory0();
    const result = [];
    for (let i = ptr; i < ptr + 4 * len; i += 4) {
        result.push(wasm.__wbindgen_export_0.get(mem.getUint32(i, true)));
    }
    wasm.__externref_drop_slice(ptr, len);
    return result;
}
/**
 * @param {Options} opts
 * @returns {Starter[]}
 */
export function generate_starters(opts) {
    _assertClass(opts, Options);
    var ptr0 = opts.__destroy_into_raw();
    const ret = wasm.generate_starters(ptr0);
    var v2 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
    return v2;
}

/**
 * @enum {0 | 1 | 2}
 */
export const Filter = Object.freeze({
    Any: 0, "0": "Any",
    Shiny: 1, "1": "Shiny",
    MaxDv: 2, "2": "MaxDv",
});

const OptionsFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_options_free(ptr >>> 0, 1));

export class Options {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(Options.prototype);
        obj.__wbg_ptr = ptr;
        OptionsFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        OptionsFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_options_free(ptr, 0);
    }
    /**
     * @param {number} adiv
     * @param {number} sdiv
     * @param {number} adiv_index
     * @param {number} sdiv_index
     * @param {number} state
     * @param {number} start_advance
     * @param {number} end_advance
     * @param {Filter} filter
     * @returns {Options}
     */
    static new(adiv, sdiv, adiv_index, sdiv_index, state, start_advance, end_advance, filter) {
        const ret = wasm.options_new(adiv, sdiv, adiv_index, sdiv_index, state, start_advance, end_advance, filter);
        return Options.__wrap(ret);
    }
}

const StarterFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_starter_free(ptr >>> 0, 1));

export class Starter {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(Starter.prototype);
        obj.__wbg_ptr = ptr;
        StarterFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        StarterFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_starter_free(ptr, 0);
    }
    /**
     * @returns {number}
     */
    get state() {
        const ret = wasm.__wbg_get_starter_state(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set state(arg0) {
        wasm.__wbg_set_starter_state(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get advance() {
        const ret = wasm.__wbg_get_starter_advance(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @param {number} arg0
     */
    set advance(arg0) {
        wasm.__wbg_set_starter_advance(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {boolean}
     */
    get shiny() {
        const ret = wasm.__wbg_get_starter_shiny(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * @param {boolean} arg0
     */
    set shiny(arg0) {
        wasm.__wbg_set_starter_shiny(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {boolean}
     */
    get max_dv() {
        const ret = wasm.__wbg_get_starter_max_dv(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * @param {boolean} arg0
     */
    set max_dv(arg0) {
        wasm.__wbg_set_starter_max_dv(this.__wbg_ptr, arg0);
    }
}

export function __wbg_starter_new(arg0) {
    const ret = Starter.__wrap(arg0);
    return ret;
};

export function __wbindgen_init_externref_table() {
    const table = wasm.__wbindgen_export_0;
    const offset = table.grow(4);
    table.set(0, undefined);
    table.set(offset + 0, undefined);
    table.set(offset + 1, null);
    table.set(offset + 2, true);
    table.set(offset + 3, false);
    ;
};

export function __wbindgen_throw(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
};

