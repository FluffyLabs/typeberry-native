import { createRequire } from 'module';

const require = createRequire(import.meta.url);

function loadNativeBinding() {
  const platform = process.platform;
  const arch = process.arch;

  let nativeBinding = null;
  let loadError = null;

  const platformBindings = {
    darwin: {
      arm64: '@typeberry/bandersnatch-native-darwin-arm64',
    },
    linux: {
      x64: '@typeberry/bandersnatch-native-linux-x64-gnu',
    },
  };

  const platformArch = platformBindings[platform]?.[arch];
  
  if (platformArch) {
    try {
      nativeBinding = require(platformArch);
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
export const batchVerifyTickets = binding.batchVerifyTickets;

export default binding;
