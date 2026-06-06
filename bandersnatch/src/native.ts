import { createRequire } from 'module';

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
    proverKeyIndices: Uint8Array,
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
}

export async function loadNativeBinding(): Promise<NativeBinding> {
  const require = createRequire(import.meta.url);

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
