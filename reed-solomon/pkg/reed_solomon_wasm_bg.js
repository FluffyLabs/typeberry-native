let wasm;
export function __wbg_set_wasm(val) {
    wasm = val;
}


const heap = new Array(128).fill(undefined);

heap.push(undefined, null, true, false);

let heap_next = heap.length;

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
}

function getObject(idx) { return heap[idx]; }

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

function isLikeNone(x) {
    return x === undefined || x === null;
}

function _assertClass(instance, klass) {
    if (!(instance instanceof klass)) {
        throw new Error(`expected instance of ${klass.name}`);
    }
    return instance.ptr;
}

let cachedDataViewMemory0 = null;

function getDataViewMemory0() {
    if (cachedDataViewMemory0 === null || cachedDataViewMemory0.buffer.detached === true || (cachedDataViewMemory0.buffer.detached === undefined && cachedDataViewMemory0.buffer !== wasm.memory.buffer)) {
        cachedDataViewMemory0 = new DataView(wasm.memory.buffer);
    }
    return cachedDataViewMemory0;
}
/**
* @param {number} recovery_count
* @param {ShardsCollection} shards
* @returns {ShardsCollection}
*/
export function encode(recovery_count, shards) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        _assertClass(shards, ShardsCollection);
        var ptr0 = shards.__destroy_into_raw();
        wasm.encode(retptr, recovery_count, ptr0);
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
        if (r2) {
            throw takeObject(r1);
        }
        return ShardsCollection.__wrap(r0);
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
}

/**
* @param {number} original_count
* @param {number} recovery_count
* @param {ShardsCollection} shards
* @returns {ShardsCollection}
*/
export function decode(original_count, recovery_count, shards) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        _assertClass(shards, ShardsCollection);
        var ptr0 = shards.__destroy_into_raw();
        wasm.decode(retptr, original_count, recovery_count, ptr0);
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
        if (r2) {
            throw takeObject(r1);
        }
        return ShardsCollection.__wrap(r0);
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
}

const ShardsCollectionFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_shardscollection_free(ptr >>> 0, 1));
/**
* Collection of shards (either input or output).
*
* To efficiently pass data between JS and WASM all of the shards
* are passed as one big vector of bytes.
* It's assumed that every shard has the same length (`shard_len`).
* If the shards are NOT passed in the exact order they were created
* it's possible to pass `indices` array.
* A value of `indices` array at position `idx` is the shard index
* that resides at `[ idx * shard_len .. idx * shard_len + shard_len )`
* in `data` array.
*
* This collection is only used to get the data from JS or pass the data back.
* Internally we convert it to [`RsShardsCollection`], which copies
* the memory to/from WASM.
*/
export class ShardsCollection {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(ShardsCollection.prototype);
        obj.__wbg_ptr = ptr;
        ShardsCollectionFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        ShardsCollectionFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_shardscollection_free(ptr, 0);
    }
    /**
    * Number of shards within the collection.
    * @returns {number}
    */
    get length() {
        const ret = wasm.__wbg_get_shardscollection_length(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
    * Number of shards within the collection.
    * @param {number} arg0
    */
    set length(arg0) {
        wasm.__wbg_set_shardscollection_length(this.__wbg_ptr, arg0);
    }
    /**
    * The length of each shard.
    * @returns {number}
    */
    get shard_len() {
        const ret = wasm.__wbg_get_shardscollection_shard_len(this.__wbg_ptr);
        return ret;
    }
    /**
    * The length of each shard.
    * @param {number} arg0
    */
    set shard_len(arg0) {
        wasm.__wbg_set_shardscollection_shard_len(this.__wbg_ptr, arg0);
    }
    /**
    * @param {number} shard_len
    * @param {Uint8Array} data
    * @param {Uint16Array | undefined} [indices]
    */
    constructor(shard_len, data, indices) {
        const ret = wasm.shardscollection_new(shard_len, addHeapObject(data), isLikeNone(indices) ? 0 : addHeapObject(indices));
        this.__wbg_ptr = ret >>> 0;
        ShardsCollectionFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
    * Extract the `indices` from this shards container.
    *
    * Should be called on the JS side to avoid copying.
    * NOTE that subsequent calls to that method will return `None`.
    * @returns {Uint16Array | undefined}
    */
    take_indices() {
        const ret = wasm.shardscollection_take_indices(this.__wbg_ptr);
        return takeObject(ret);
    }
    /**
    * Take the underlying `data` to the JS side.
    *
    * NOTE this object is destroyed after the data is consumed,
    * so make sure to [`take_indices`] first.
    * @returns {Uint8Array}
    */
    take_data() {
        const ptr = this.__destroy_into_raw();
        const ret = wasm.shardscollection_take_data(ptr);
        return takeObject(ret);
    }
}

export function __wbindgen_memory() {
    const ret = wasm.memory;
    return addHeapObject(ret);
};

export function __wbg_buffer_b7b08af79b0b0974(arg0) {
    const ret = getObject(arg0).buffer;
    return addHeapObject(ret);
};

export function __wbg_newwithbyteoffsetandlength_8a2cb9ca96b27ec9(arg0, arg1, arg2) {
    const ret = new Uint8Array(getObject(arg0), arg1 >>> 0, arg2 >>> 0);
    return addHeapObject(ret);
};

export function __wbindgen_object_drop_ref(arg0) {
    takeObject(arg0);
};

export function __wbg_new_ea1883e1e5e86686(arg0) {
    const ret = new Uint8Array(getObject(arg0));
    return addHeapObject(ret);
};

export function __wbg_newwithbyteoffsetandlength_bd3d5191e8925067(arg0, arg1, arg2) {
    const ret = new Uint16Array(getObject(arg0), arg1 >>> 0, arg2 >>> 0);
    return addHeapObject(ret);
};

export function __wbg_new_51798470384ee5a8(arg0) {
    const ret = new Uint16Array(getObject(arg0));
    return addHeapObject(ret);
};

export function __wbg_length_8339fcf5d8ecd12e(arg0) {
    const ret = getObject(arg0).length;
    return ret;
};

export function __wbg_length_ff22981e43417ccf(arg0) {
    const ret = getObject(arg0).length;
    return ret;
};

export function __wbindgen_string_new(arg0, arg1) {
    const ret = getStringFromWasm0(arg0, arg1);
    return addHeapObject(ret);
};

export function __wbg_set_d1e79e2388520f18(arg0, arg1, arg2) {
    getObject(arg0).set(getObject(arg1), arg2 >>> 0);
};

export function __wbg_set_e83c20bbf4b38a6b(arg0, arg1, arg2) {
    getObject(arg0).set(getObject(arg1), arg2 >>> 0);
};

export function __wbindgen_throw(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
};

