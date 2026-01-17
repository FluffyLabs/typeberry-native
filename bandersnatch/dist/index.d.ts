import type * as WasmBinding from "../wasm-binding/pkg/bandersnatch_wasm";
export type BandersnatchApi = {
    isNativeBinding: () => boolean;
    ringCommitment: (keys: Uint8Array) => Uint8Array;
    derivePublicKey: (seed: Uint8Array) => Uint8Array;
    verifyHeaderSeals: (signerKey: Uint8Array, sealData: Uint8Array, sealPayload: Uint8Array, unsealedHeader: Uint8Array, entropyData: Uint8Array, entropyPrefix: Uint8Array) => Uint8Array;
    verifySeal: (signerKey: Uint8Array, sealData: Uint8Array, payload: Uint8Array, auxData: Uint8Array) => Uint8Array;
    generateSeal: (secretSeed: Uint8Array, input: Uint8Array, auxData: Uint8Array) => Uint8Array;
    vrfOutputHash: (secretSeed: Uint8Array, input: Uint8Array) => Uint8Array;
    batchVerifyTickets: (ringSize: number, commitment: Uint8Array, ticketsData: Uint8Array, vrfInputDataLen: number) => Uint8Array;
};
export type InitOptions = {
    module_or_path?: WasmBinding.InitInput | Promise<WasmBinding.InitInput>;
};
export default function init(options?: InitOptions): Promise<BandersnatchApi>;
export declare function isInitialized(): boolean;
export declare function isNativeBinding(): boolean;
export declare function ringCommitment(keys: Uint8Array): Uint8Array;
export declare function derivePublicKey(seed: Uint8Array): Uint8Array;
export declare function verifyHeaderSeals(signerKey: Uint8Array, sealData: Uint8Array, sealPayload: Uint8Array, unsealedHeader: Uint8Array, entropyData: Uint8Array, entropyPrefix: Uint8Array): Uint8Array;
export declare function verifySeal(signerKey: Uint8Array, sealData: Uint8Array, payload: Uint8Array, auxData: Uint8Array): Uint8Array;
export declare function generateSeal(secretSeed: Uint8Array, input: Uint8Array, auxData: Uint8Array): Uint8Array;
export declare function vrfOutputHash(secretSeed: Uint8Array, input: Uint8Array): Uint8Array;
export declare function batchVerifyTickets(ringSize: number, commitment: Uint8Array, ticketsData: Uint8Array, vrfInputDataLen: number): Uint8Array;
//# sourceMappingURL=index.d.ts.map