let wasmBinding = null;
let nativeBinding = null;
function isNode() {
    return (typeof process !== "undefined" && process.versions != null && process.versions.node != null);
}
async function loadNativeBinding() {
    if (!isNode()) {
        return null;
    }
    try {
        const native = await import("@typeberry/bandersnatch-native");
        return native.default || native;
    }
    catch {
        return null;
    }
}
async function loadWasmBinding(wasmModule) {
    const wasmBindingModule = await import("../wasm-binding/pkg/bandersnatch_wasm.js");
    await wasmBindingModule.default({ module_or_path: wasmModule });
    return wasmBindingModule;
}
function createApi() {
    return {
        isNativeBinding,
        ringCommitment,
        derivePublicKey,
        verifyHeaderSeals,
        verifySeal,
        generateSeal,
        vrfOutputHash,
        batchVerifyTickets,
    };
}
export default async function init(options) {
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
export function isInitialized() {
    return wasmBinding !== null || nativeBinding !== null;
}
export function isNativeBinding() {
    return nativeBinding !== null;
}
function assertInitialized() {
    if (!isInitialized()) {
        throw new Error("Bandersnatch binding not initialized. Call init() first.");
    }
}
export function ringCommitment(keys) {
    assertInitialized();
    if (nativeBinding) {
        return nativeBinding.ringCommitment(keys);
    }
    return wasmBinding.ring_commitment(keys);
}
export function derivePublicKey(seed) {
    assertInitialized();
    if (nativeBinding) {
        return nativeBinding.derivePublicKey(seed);
    }
    return wasmBinding.derive_public_key(seed);
}
export function verifyHeaderSeals(signerKey, sealData, sealPayload, unsealedHeader, entropyData, entropyPrefix) {
    assertInitialized();
    if (nativeBinding) {
        return nativeBinding.verifyHeaderSeals(signerKey, sealData, sealPayload, unsealedHeader, entropyData, entropyPrefix);
    }
    return wasmBinding.verify_header_seals(signerKey, sealData, sealPayload, unsealedHeader, entropyData, entropyPrefix);
}
export function verifySeal(signerKey, sealData, payload, auxData) {
    assertInitialized();
    if (nativeBinding) {
        return nativeBinding.verifySeal(signerKey, sealData, payload, auxData);
    }
    return wasmBinding.verify_seal(signerKey, sealData, payload, auxData);
}
export function generateSeal(secretSeed, input, auxData) {
    assertInitialized();
    if (nativeBinding) {
        return nativeBinding.generateSeal(secretSeed, input, auxData);
    }
    return wasmBinding.generate_seal(secretSeed, input, auxData);
}
export function vrfOutputHash(secretSeed, input) {
    assertInitialized();
    if (nativeBinding) {
        return nativeBinding.vrfOutputHash(secretSeed, input);
    }
    return wasmBinding.vrf_output_hash(secretSeed, input);
}
export function batchVerifyTickets(ringSize, commitment, ticketsData, vrfInputDataLen) {
    assertInitialized();
    if (nativeBinding) {
        return nativeBinding.batchVerifyTickets(ringSize, commitment, ticketsData, vrfInputDataLen);
    }
    return wasmBinding.batch_verify_tickets(ringSize, commitment, ticketsData, vrfInputDataLen);
}
