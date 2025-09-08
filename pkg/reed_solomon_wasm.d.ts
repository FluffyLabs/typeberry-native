/* tslint:disable */
/* eslint-disable */
/**
* @param {number} recovery_count
* @param {ShardsCollection} shards
* @returns {ShardsCollection}
*/
export function encode(recovery_count: number, shards: ShardsCollection): ShardsCollection;
/**
* @param {number} original_count
* @param {number} recovery_count
* @param {ShardsCollection} shards
* @returns {ShardsCollection}
*/
export function decode(original_count: number, recovery_count: number, shards: ShardsCollection): ShardsCollection;
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
  free(): void;
/**
* @param {number} shard_len
* @param {Uint8Array} data
* @param {Uint16Array | undefined} [indices]
*/
  constructor(shard_len: number, data: Uint8Array, indices?: Uint16Array);
/**
* Extract the `indices` from this shards container.
*
* Should be called on the JS side to avoid copying.
* NOTE that subsequent calls to that method will return `None`.
* @returns {Uint16Array | undefined}
*/
  take_indices(): Uint16Array | undefined;
/**
* Take the underlying `data` to the JS side.
*
* NOTE this object is destroyed after the data is consumed,
* so make sure to [`take_indices`] first.
* @returns {Uint8Array}
*/
  take_data(): Uint8Array;
/**
* Number of shards within the collection.
*/
  length: number;
/**
* The length of each shard.
*/
  shard_len: number;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_shardscollection_free: (a: number, b: number) => void;
  readonly __wbg_get_shardscollection_length: (a: number) => number;
  readonly __wbg_set_shardscollection_length: (a: number, b: number) => void;
  readonly __wbg_get_shardscollection_shard_len: (a: number) => number;
  readonly __wbg_set_shardscollection_shard_len: (a: number, b: number) => void;
  readonly shardscollection_new: (a: number, b: number, c: number) => number;
  readonly shardscollection_take_indices: (a: number) => number;
  readonly shardscollection_take_data: (a: number) => number;
  readonly encode: (a: number, b: number, c: number) => void;
  readonly decode: (a: number, b: number, c: number, d: number) => void;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
*
* @returns {InitOutput}
*/
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
