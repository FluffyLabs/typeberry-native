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
    return native.default || native;
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
  batchGenerateRingVrf: (
    ringKeys: Uint8Array,
    proverKeyIndex: number,
    secretSeed: Uint8Array,
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
    batchGenerateRingVrf,
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
