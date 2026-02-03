import { createRequire } from 'module';

const require = createRequire(import.meta.url);

export interface NativeBinding {
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
    ringSize: number,
    secretSeed: Uint8Array,
    inputsData: Uint8Array,
    vrfInputDataLen: number,
    ringKeys: Uint8Array,
    proverKeyIndex: number
  ) => Uint8Array;
  batchVerifyTickets: (
    ringSize: number,
    commitment: Uint8Array,
    ticketsData: Uint8Array,
    vrfInputDataLen: number
  ) => Uint8Array;
}

function loadNativeBinding(): NativeBinding {
  // process is defined in env.d.ts but might be undefined in some envs. 
  // However this file is meant for Node.js usage.
  const platform = process?.platform;
  const arch = process?.arch;

  let nativeBinding: NativeBinding | null = null;
  let loadError: unknown = null;

  const platformBindings: Record<string, Record<string, string>> = {
    darwin: {
      arm64: '@typeberry/bandersnatch-native-darwin-arm64',
    },
    linux: {
      x64: '@typeberry/bandersnatch-native-linux-x64-gnu',
    },
  };

  const platformArch = platform && arch ? platformBindings[platform]?.[arch] : undefined;
  
  if (platformArch) {
    try {
      nativeBinding = require(platformArch) as NativeBinding;
    } catch (e) {
      loadError = e;
    }
  } else {
    loadError = new Error(`Unsupported platform: ${platform}-${arch}`);
  }

  if (!nativeBinding) {
    throw loadError || new Error('Failed to load native binding');
  }

  return nativeBinding;
}

const binding = loadNativeBinding();

export const ringCommitment = binding.ringCommitment;
export const derivePublicKey = binding.derivePublicKey;
export const verifyHeaderSeals = binding.verifyHeaderSeals;
export const verifySeal = binding.verifySeal;
export const generateSeal = binding.generateSeal;
export const vrfOutputHash = binding.vrfOutputHash;
export const batchGenerateRingVrf = binding.batchGenerateRingVrf;
export const batchVerifyTickets = binding.batchVerifyTickets;

export default binding;
