/* tslint:disable */
/* eslint-disable */
/**
 *
 * * Verify Ed25519 signatures one by one using strict verification.
 * *
 * * This function is slower but does strict verification.
 * 
 */
export function verify_ed25519(data: Uint8Array): Uint8Array;
/**
 *
 * * Verify Ed25519 signatures using build-in batch verification.
 * *
 * * This function is faster but does not do strict verification.
 * * See https://crates.io/crates/ed25519-dalek#batch-verification for more information.
 * 
 */
export function verify_ed25519_batch(data: Uint8Array): boolean;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly verify_ed25519: (a: number, b: number) => [number, number];
  readonly verify_ed25519_batch: (a: number, b: number) => number;
  readonly __wbindgen_export_0: WebAssembly.Table;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_start: () => void;
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
