const { createRequire } = require('module');

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

module.exports = binding;
module.exports.ringCommitment = binding.ringCommitment;
module.exports.derivePublicKey = binding.derivePublicKey;
module.exports.verifyHeaderSeals = binding.verifyHeaderSeals;
module.exports.verifySeal = binding.verifySeal;
module.exports.generateSeal = binding.generateSeal;
module.exports.vrfOutputHash = binding.vrfOutputHash;
module.exports.batchVerifyTickets = binding.batchVerifyTickets;
