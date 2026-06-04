import type * as WasmBinding from "../wasm-binding/pkg/bandersnatch_wasm";
import type { NativeBinding } from "./native.js";

type WasmBindingType = typeof WasmBinding;

let wasmBinding: WasmBindingType | null = null;
let nativeBinding: NativeBinding | null = null;
let nativeBindingError: string | null = null;

function isNode(): boolean {
  return (
    typeof process !== "undefined" && process.versions != null && process.versions.node != null
  );
}

async function loadNativeBinding(): Promise<NativeBinding | null> {
  if (!isNode()) {
    nativeBindingError = 'Invalid environment';
    return null;
  }

  try {
    const native = await import("./native.js");
    return native.loadNativeBinding();
  } catch (e) {
    nativeBindingError = `${e}`;
    return null;
  }
}

async function loadWasmBinding(
  wasmModule?: WasmBinding.InitInput | Promise<WasmBinding.InitInput>
): Promise<WasmBindingType> {
  const wasmBindingModule = await import("../wasm-binding/pkg/bandersnatch_wasm.js");
  await wasmBindingModule.default({ module_or_path: wasmModule });
  return wasmBindingModule;
}

export type BandersnatchApi = {
  isNativeBinding: () => boolean;
  ringCommitment: (keys: Uint8Array) => Uint8Array;
  derivePublicKey: (seed: Uint8Array) => Uint8Array;
  verifyHeaderSeals: (
    signerKey: Uint8Array,
    sealData: Uint8Array,
    sealPayload: Uint8Array,
    unsealedHeader: Uint8Array,
    entropyData: Uint8Array,
    entropyPrefix: Uint8Array
  ) => Uint8Array;
  verifySeal: (
    signerKey: Uint8Array,
    sealData: Uint8Array,
    payload: Uint8Array,
    auxData: Uint8Array
  ) => Uint8Array;
  generateSeal: (secretSeed: Uint8Array, input: Uint8Array, auxData: Uint8Array) => Uint8Array;
  vrfOutputHash: (secretSeed: Uint8Array, input: Uint8Array) => Uint8Array;
  generateRingVrf: (
    ringKeys: Uint8Array,
    proverKeyIndex: number,
    secretSeed: Uint8Array,
    vrfInputData: Uint8Array
  ) => Uint8Array;
  batchGenerateRingVrf: (
    ringKeys: Uint8Array,
    proverKeyIndex: number,
    secretSeed: Uint8Array,
    inputsData: Uint8Array,
    vrfInputDataLen: number
  ) => Uint8Array;
  batchGenerateRingVrfForValidators: (
    ringKeys: Uint8Array,
    proverKeyIndices: Uint32Array | readonly number[],
    secretSeedsData: Uint8Array,
    secretSeedDataLen: number,
    inputsData: Uint8Array,
    vrfInputDataLen: number
  ) => Uint8Array;
  batchVerifyTickets: (
    ringSize: number,
    commitment: Uint8Array,
    ticketsData: Uint8Array,
    vrfInputDataLen: number
  ) => Uint8Array;
};

function createApi(): BandersnatchApi {
  return {
    isNativeBinding,
    ringCommitment,
    derivePublicKey,
    verifyHeaderSeals,
    verifySeal,
    generateSeal,
    vrfOutputHash,
    generateRingVrf,
    batchGenerateRingVrf,
    batchGenerateRingVrfForValidators,
    batchVerifyTickets,
  };
}

export type InitOptions = {
  module_or_path?: WasmBinding.InitInput | Promise<WasmBinding.InitInput>;
};

export default async function init(options?: InitOptions): Promise<BandersnatchApi> {
  if (wasmBinding !== null || nativeBinding !== null) {
    return createApi();
  }

  const native = await loadNativeBinding();
  if (native) {
    nativeBinding = native;
    return createApi();
  }

  wasmBinding = await loadWasmBinding(options?.module_or_path);
  return createApi();
}

/**
 * Check if the binding is initialized already.
 */
export function isInitialized(): boolean {
  return wasmBinding !== null || nativeBinding !== null;
}

/**
 * Returns true if native binding is used.
 */
export function isNativeBinding(): boolean {
  return nativeBinding !== null;
}

/**
 * Returns native binding initialisation error (if any).
 */
export function getNativeBindingError(): string | null {
  return nativeBindingError;
}

function assertInitialized(): void {
  if (!isInitialized()) {
    throw new Error("Bandersnatch binding not initialized. Call init() first.");
  }
}

export function ringCommitment(keys: Uint8Array): Uint8Array {
  assertInitialized();
  if (nativeBinding) {
    return nativeBinding.ringCommitment(keys);
  }
  return wasmBinding!.ring_commitment(keys);
}

export function derivePublicKey(seed: Uint8Array): Uint8Array {
  assertInitialized();
  if (nativeBinding) {
    return nativeBinding.derivePublicKey(seed);
  }
  return wasmBinding!.derive_public_key(seed);
}

export function verifyHeaderSeals(
  signerKey: Uint8Array,
  sealData: Uint8Array,
  sealPayload: Uint8Array,
  unsealedHeader: Uint8Array,
  entropyData: Uint8Array,
  entropyPrefix: Uint8Array
): Uint8Array {
  assertInitialized();
  if (nativeBinding) {
    return nativeBinding.verifyHeaderSeals(
      signerKey,
      sealData,
      sealPayload,
      unsealedHeader,
      entropyData,
      entropyPrefix
    );
  }
  return wasmBinding!.verify_header_seals(
    signerKey,
    sealData,
    sealPayload,
    unsealedHeader,
    entropyData,
    entropyPrefix
  );
}

export function verifySeal(
  signerKey: Uint8Array,
  sealData: Uint8Array,
  payload: Uint8Array,
  auxData: Uint8Array
): Uint8Array {
  assertInitialized();
  if (nativeBinding) {
    return nativeBinding.verifySeal(signerKey, sealData, payload, auxData);
  }
  return wasmBinding!.verify_seal(signerKey, sealData, payload, auxData);
}

export function generateSeal(
  secretSeed: Uint8Array,
  input: Uint8Array,
  auxData: Uint8Array
): Uint8Array {
  assertInitialized();
  if (nativeBinding) {
    return nativeBinding.generateSeal(secretSeed, input, auxData);
  }
  return wasmBinding!.generate_seal(secretSeed, input, auxData);
}

export function vrfOutputHash(secretSeed: Uint8Array, input: Uint8Array): Uint8Array {
  assertInitialized();
  if (nativeBinding) {
    return nativeBinding.vrfOutputHash(secretSeed, input);
  }
  return wasmBinding!.vrf_output_hash(secretSeed, input);
}

/**
 * Generate one ring VRF ticket for a concrete VRF input.
 *
 * This is the single-attempt form of `batchGenerateRingVrf`: callers encode the
 * desired attempt into `vrfInputData` and receive one `status || signature`
 * record, where the signature is 784 bytes.
 */
export function generateRingVrf(
  ringKeys: Uint8Array,
  proverKeyIndex: number,
  secretSeed: Uint8Array,
  vrfInputData: Uint8Array
): Uint8Array {
  assertInitialized();
  if (nativeBinding) {
    return nativeBinding.generateRingVrf(ringKeys, proverKeyIndex, secretSeed, vrfInputData);
  }
  return wasmBinding!.generate_ring_vrf(ringKeys, proverKeyIndex, secretSeed, vrfInputData);
}

export function batchGenerateRingVrf(
  ringKeys: Uint8Array,
  proverKeyIndex: number,
  secretSeed: Uint8Array,
  inputsData: Uint8Array,
  vrfInputDataLen: number
): Uint8Array {
  assertInitialized();
  if (nativeBinding) {
    return nativeBinding.batchGenerateRingVrf(
      ringKeys,
      proverKeyIndex,
      secretSeed,
      inputsData,
      vrfInputDataLen
    );
  }
  return wasmBinding!.batch_generate_ring_vrf(
    ringKeys,
    proverKeyIndex,
    secretSeed,
    inputsData,
    vrfInputDataLen
  );
}

function encodeProverKeyIndices(indices: Uint32Array | readonly number[]): Uint8Array {
  const result = new Uint8Array(indices.length * 4);
  const view = new DataView(result.buffer, result.byteOffset, result.byteLength);

  for (let i = 0; i < indices.length; i++) {
    const index = indices[i];
    if (!Number.isInteger(index) || index < 0 || index > 0xffffffff) {
      throw new RangeError(`Invalid prover key index at position ${i}: ${index}`);
    }
    view.setUint32(i * 4, index, true);
  }

  return result;
}

/**
 * Batch-generate ring VRF tickets for multiple validators.
 *
 * `secretSeedsData` is fixed-width concatenated seed data, split by
 * `secretSeedDataLen`. `proverKeyIndices` and secret seeds must have the same
 * count. The returned records are ordered validator-major, then input-major:
 * `validator_0/input_0`, `validator_0/input_1`, ..., `validator_1/input_0`, ...
 *
 * Each record is `status byte || signature (784 bytes)`. If the validator
 * metadata is malformed, the function returns a single error status byte.
 */
export function batchGenerateRingVrfForValidators(
  ringKeys: Uint8Array,
  proverKeyIndices: Uint32Array | readonly number[],
  secretSeedsData: Uint8Array,
  secretSeedDataLen: number,
  inputsData: Uint8Array,
  vrfInputDataLen: number
): Uint8Array {
  assertInitialized();
  const proverKeyIndicesData = encodeProverKeyIndices(proverKeyIndices);
  if (nativeBinding) {
    return nativeBinding.batchGenerateRingVrfForValidators(
      ringKeys,
      proverKeyIndicesData,
      secretSeedsData,
      secretSeedDataLen,
      inputsData,
      vrfInputDataLen
    );
  }
  return wasmBinding!.batch_generate_ring_vrf_for_validators(
    ringKeys,
    proverKeyIndicesData,
    secretSeedsData,
    secretSeedDataLen,
    inputsData,
    vrfInputDataLen
  );
}

/**
 * Batch-verify ring VRF tickets against a single ring commitment.
 *
 * Verification is all-or-nothing: every ticket is aggregated into one batched
 * pairing check, so the result reports a single pass/fail for the whole batch
 * (an individual failing ticket cannot be identified).
 *
 * `ticketsData` is the concatenation of `signature (784 bytes) || vrfInput
 * (vrfInputDataLen bytes)` per ticket. The returned buffer is:
 *
 * - On success: `[0x00, entropyHash_0 (32B), entropyHash_1 (32B), ...]` — the
 *   status byte followed by one VRF output hash per ticket, in input order.
 * - On failure: `[0x01, 0x00 * (numTickets * 32)]` — the status byte followed
 *   by a zero-filled region of the same length; the hashes must be ignored.
 *
 * `numTickets` equals `ticketsData.length / (vrfInputDataLen + 784)`, so the
 * response length is identical for success and failure.
 */
export function batchVerifyTickets(
  ringSize: number,
  commitment: Uint8Array,
  ticketsData: Uint8Array,
  vrfInputDataLen: number
): Uint8Array {
  assertInitialized();
  if (nativeBinding) {
    return nativeBinding.batchVerifyTickets(ringSize, commitment, ticketsData, vrfInputDataLen);
  }
  return wasmBinding!.batch_verify_tickets(ringSize, commitment, ticketsData, vrfInputDataLen);
}
