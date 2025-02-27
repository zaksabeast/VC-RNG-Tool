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
 * @param {PokeOptions} opts
 * @returns {Spread[]}
 */
export function generate_starters(opts) {
    _assertClass(opts, PokeOptions);
    var ptr0 = opts.__destroy_into_raw();
    const ret = wasm.generate_starters(ptr0);
    var v2 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
    return v2;
}

/**
 * @param {PokeOptions} opts
 * @returns {Spread[]}
 */
export function generate_celebi(opts) {
    _assertClass(opts, PokeOptions);
    var ptr0 = opts.__destroy_into_raw();
    const ret = wasm.generate_celebi(ptr0);
    var v2 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
    return v2;
}

/**
 * @param {RandOptions} opts
 * @returns {RngState[]}
 */
export function generate_rng_states(opts) {
    _assertClass(opts, RandOptions);
    var ptr0 = opts.__destroy_into_raw();
    const ret = wasm.generate_rng_states(ptr0);
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

const PokeOptionsFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_pokeoptions_free(ptr >>> 0, 1));

export class PokeOptions {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(PokeOptions.prototype);
        obj.__wbg_ptr = ptr;
        PokeOptionsFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        PokeOptionsFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_pokeoptions_free(ptr, 0);
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
     * @returns {PokeOptions}
     */
    static new(adiv, sdiv, adiv_index, sdiv_index, state, start_advance, end_advance, filter) {
        const ret = wasm.pokeoptions_new(adiv, sdiv, adiv_index, sdiv_index, state, start_advance, end_advance, filter);
        return PokeOptions.__wrap(ret);
    }
}

const RandOptionsFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_randoptions_free(ptr >>> 0, 1));

export class RandOptions {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(RandOptions.prototype);
        obj.__wbg_ptr = ptr;
        RandOptionsFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        RandOptionsFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_randoptions_free(ptr, 0);
    }
    /**
     * @param {number} adiv
     * @param {number} sdiv
     * @param {number} adiv_index
     * @param {number} sdiv_index
     * @param {number} state
     * @param {number} start_advance
     * @param {number} end_advance
     * @returns {RandOptions}
     */
    static new(adiv, sdiv, adiv_index, sdiv_index, state, start_advance, end_advance) {
        const ret = wasm.randoptions_new(adiv, sdiv, adiv_index, sdiv_index, state, start_advance, end_advance);
        return RandOptions.__wrap(ret);
    }
}

const RngStateFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_rngstate_free(ptr >>> 0, 1));

export class RngState {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(RngState.prototype);
        obj.__wbg_ptr = ptr;
        RngStateFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        RngStateFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_rngstate_free(ptr, 0);
    }
    /**
     * @returns {number}
     */
    get rand() {
        const ret = wasm.__wbg_get_rngstate_rand(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set rand(arg0) {
        wasm.__wbg_set_rngstate_rand(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get advance() {
        const ret = wasm.__wbg_get_rngstate_advance(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @param {number} arg0
     */
    set advance(arg0) {
        wasm.__wbg_set_rngstate_advance(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get add_div() {
        const ret = wasm.__wbg_get_rngstate_add_div(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set add_div(arg0) {
        wasm.__wbg_set_rngstate_add_div(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get sub_div() {
        const ret = wasm.__wbg_get_rngstate_sub_div(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set sub_div(arg0) {
        wasm.__wbg_set_rngstate_sub_div(this.__wbg_ptr, arg0);
    }
}

const SpreadFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_spread_free(ptr >>> 0, 1));

export class Spread {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(Spread.prototype);
        obj.__wbg_ptr = ptr;
        SpreadFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        SpreadFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_spread_free(ptr, 0);
    }
    /**
     * @returns {number}
     */
    get state() {
        const ret = wasm.__wbg_get_rngstate_rand(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set state(arg0) {
        wasm.__wbg_set_rngstate_rand(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get advance() {
        const ret = wasm.__wbg_get_rngstate_advance(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @param {number} arg0
     */
    set advance(arg0) {
        wasm.__wbg_set_rngstate_advance(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {boolean}
     */
    get shiny() {
        const ret = wasm.__wbg_get_spread_shiny(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * @param {boolean} arg0
     */
    set shiny(arg0) {
        wasm.__wbg_set_spread_shiny(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {boolean}
     */
    get max_dv() {
        const ret = wasm.__wbg_get_spread_max_dv(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * @param {boolean} arg0
     */
    set max_dv(arg0) {
        wasm.__wbg_set_spread_max_dv(this.__wbg_ptr, arg0);
    }
}

export function __wbg_rngstate_new(arg0) {
    const ret = RngState.__wrap(arg0);
    return ret;
};

export function __wbg_spread_new(arg0) {
    const ret = Spread.__wrap(arg0);
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

