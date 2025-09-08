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
