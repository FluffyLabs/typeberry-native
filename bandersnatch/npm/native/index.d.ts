export function ringCommitment(keys: Uint8Array): Uint8Array;
export function derivePublicKey(seed: Uint8Array): Uint8Array;
export function verifyHeaderSeals(
  signerKey: Uint8Array,
  sealData: Uint8Array,
  sealPayload: Uint8Array,
  unsealedHeader: Uint8Array,
  entropyData: Uint8Array,
  entropyPrefix: Uint8Array
): Uint8Array;
export function verifySeal(
  signerKey: Uint8Array,
  sealData: Uint8Array,
  payload: Uint8Array,
  auxData: Uint8Array
): Uint8Array;
export function generateSeal(
  secretSeed: Uint8Array,
  input: Uint8Array,
  auxData: Uint8Array
): Uint8Array;
export function vrfOutputHash(secretSeed: Uint8Array, input: Uint8Array): Uint8Array;
export function batchVerifyTickets(
  ringSize: number,
  commitment: Uint8Array,
  ticketsData: Uint8Array,
  vrfInputDataLen: number
): Uint8Array;

declare const binding: {
  ringCommitment: typeof ringCommitment;
  derivePublicKey: typeof derivePublicKey;
  verifyHeaderSeals: typeof verifyHeaderSeals;
  verifySeal: typeof verifySeal;
  generateSeal: typeof generateSeal;
  vrfOutputHash: typeof vrfOutputHash;
  batchVerifyTickets: typeof batchVerifyTickets;
};

export default binding;
