//! Bandersnatch VRF Core Library
//!
//! Pure Rust implementation of Bandersnatch VRF functionality without any
//! binding-specific code. This crate can be used by both WASM and native bindings.

use ark_vrf::ietf::Prover;
use ark_vrf::reexports::ark_serialize::{self, CanonicalDeserialize, CanonicalSerialize};
use ark_vrf::suites::bandersnatch;
use bandersnatch::{
    BandersnatchSha512Ell2, IetfProof, Input, Output, Public, RingProof, RingProofParams, Secret,
};

#[cfg(test)]
mod test;

/// Size of supported rings.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RingSize {
    /// Tiny ring for testing (6 members)
    Tiny,
    /// Full ring for production (1023 members)
    Full,
}

impl RingSize {
    /// Returns the number of members in the ring.
    pub fn size(&self) -> usize {
        match *self {
            RingSize::Tiny => 6,
            RingSize::Full => 1023,
        }
    }

    /// Create a RingSize from a numeric size.
    pub fn from_size(size: usize) -> Self {
        if size == Self::Full.size() {
            RingSize::Full
        } else {
            RingSize::Tiny
        }
    }
}

/// IETF VRF signature as described in section 2.2 of the Bandersnatch VRFs specification.
#[derive(CanonicalSerialize, CanonicalDeserialize)]
pub struct IetfVrfSignature {
    pub output: Output,
    pub proof: IetfProof,
}

/// Ring VRF signature as described in section 4.2 of the Bandersnatch VRFs specification.
#[derive(CanonicalSerialize, CanonicalDeserialize)]
pub struct RingVrfSignature {
    pub output: Output,
    /// Contains both the Pedersen proof and actual ring proof.
    pub proof: RingProof,
}

/// Get or initialize ring proof parameters for the given ring size.
pub fn ring_proof_params(ring_size: RingSize) -> &'static RingProofParams {
    use std::sync::OnceLock;
    static PARAMS_TINY: OnceLock<RingProofParams> = OnceLock::new();
    static PARAMS_FULL: OnceLock<RingProofParams> = OnceLock::new();

    let init = |size: usize| {
        use bandersnatch::PcsParams;
        let buf = include_bytes!("../../data/zcash-srs-2-11-uncompressed.bin");
        let pcs_params = PcsParams::deserialize_uncompressed_unchecked(&mut &buf[..])
            .expect("binary data invalid");
        RingProofParams::from_pcs_params(size, pcs_params).expect("invalid ring proof params")
    };

    match ring_size {
        RingSize::Tiny => PARAMS_TINY.get_or_init(|| init(ring_size.size())),
        RingSize::Full => PARAMS_FULL.get_or_init(|| init(ring_size.size())),
    }
}

/// Construct VRF Input Point from arbitrary data (section 1.2).
pub fn vrf_input_point(vrf_input_data: &[u8]) -> Result<Input, Error> {
    Input::new(vrf_input_data).ok_or(Error::InvalidPointData)
}

/// Errors that can occur during VRF operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    /// Invalid point data provided.
    InvalidPointData,
    /// Invalid signature format.
    InvalidSignature,
    /// Signature verification failed.
    VerificationFailure,
}

/// Ring commitment type alias.
pub type RingCommitment = ark_vrf::ring::RingCommitment<BandersnatchSha512Ell2>;

/// Size of a public key in bytes.
pub const PUBLIC_KEY_SIZE: usize = 32;

/// Size of an IETF VRF signature in bytes.
pub const IETF_SIGNATURE_SIZE: usize = 96;

/// Size of a Ring VRF signature in bytes.
pub const RING_SIGNATURE_SIZE: usize = 784;

/// Deserialize a public key from bytes, returning the padding point on failure.
pub fn deserialize_public_key(chunk: &[u8]) -> Public {
    Public::deserialize_compressed_unchecked(chunk)
        .unwrap_or_else(|_| Public::from(RingProofParams::padding_point()))
}

/// Extract VRF output hash from output.
pub fn copy_vrf_output_hash(output: Output) -> [u8; 32] {
    let mut vrf_output_hash = [0u8; 32];
    vrf_output_hash.copy_from_slice(&output.hash()[..32]);
    vrf_output_hash
}

/// Verifier for VRF signatures.
pub struct Verifier;

impl Verifier {
    /// Anonymous VRF signature verification.
    ///
    /// Used for tickets verification.
    ///
    /// On success returns the VRF output hash.
    pub fn ring_vrf_verify(
        ring_size: RingSize,
        commitment: RingCommitment,
        vrf_input_data: &[u8],
        aux_data: &[u8],
        signature: &[u8],
    ) -> Result<[u8; 32], Error> {
        use ark_vrf::ring::Verifier as _;

        let signature = RingVrfSignature::deserialize_compressed_unchecked(signature)
            .map_err(|_| Error::InvalidSignature)?;

        let input = vrf_input_point(vrf_input_data)?;
        let output = signature.output;

        let ring_params = ring_proof_params(ring_size);

        // The verifier key is reconstructed from the commitment and the constant
        // verifier key component of the SRS in order to verify some proof.
        let verifier_key = ring_params.verifier_key_from_commitment(commitment);
        let verifier = ring_params.verifier(verifier_key);
        if Public::verify(input, output, aux_data, &signature.proof, &verifier).is_err() {
            return Err(Error::VerificationFailure);
        }

        Ok(copy_vrf_output_hash(output))
    }

    /// Non-Anonymous VRF signature verification.
    ///
    /// Used for ticket claim verification during block import.
    ///
    /// On success returns the VRF output hash.
    pub fn ietf_vrf_verify(
        vrf_input_data: &[u8],
        aux_data: &[u8],
        signature: &[u8],
        signer_public_key: Public,
    ) -> Result<[u8; 32], Error> {
        use ark_vrf::ietf::Verifier as _;

        let signature = IetfVrfSignature::deserialize_compressed_unchecked(signature)
            .map_err(|_| Error::InvalidSignature)?;

        let input = vrf_input_point(vrf_input_data)?;
        let output = signature.output;

        if signer_public_key
            .verify(input, output, aux_data, &signature.proof)
            .is_err()
        {
            return Err(Error::VerificationFailure);
        }

        Ok(copy_vrf_output_hash(output))
    }
}

/// Generate ring commitment given a slice of public keys.
pub fn compute_ring_commitment(keys: &[Public], ring_size: RingSize) -> Result<Vec<u8>, Error> {
    let pts: Vec<_> = keys.iter().map(|pk| pk.0).collect();
    let verifier_key = ring_proof_params(ring_size).verifier_key(&pts);
    let commitment = verifier_key.commitment();
    let mut buf = Vec::new();
    commitment
        .serialize_compressed(&mut buf)
        .map_err(|_| Error::InvalidSignature)?;
    Ok(buf)
}

/// Derive a public key from a seed.
pub fn derive_public_key_from_seed(seed: &[u8]) -> Result<Vec<u8>, Error> {
    let secret = Secret::from_seed(seed);
    let mut buf = Vec::new();
    secret
        .public()
        .serialize_compressed(&mut buf)
        .map_err(|_| Error::InvalidSignature)?;
    Ok(buf)
}

/// Generate an IETF VRF seal (signature).
pub fn generate_ietf_seal(
    secret_seed: &[u8],
    input: &[u8],
    aux_data: &[u8],
) -> Result<Vec<u8>, Error> {
    let secret = Secret::from_seed(secret_seed);
    let input_point = Input::new(input).ok_or(Error::InvalidPointData)?;

    let output = secret.output(input_point);
    let proof = secret.prove(input_point, output, aux_data);

    let sig = IetfVrfSignature { output, proof };
    let mut result = Vec::new();
    sig.serialize_compressed(&mut result)
        .map_err(|_| Error::InvalidSignature)?;
    Ok(result)
}

/// Compute VRF output hash from a secret seed and input data.
pub fn compute_vrf_output_hash(secret_seed: &[u8], input: &[u8]) -> Result<[u8; 32], Error> {
    let secret = Secret::from_seed(secret_seed);
    let input_point = Input::new(input).ok_or(Error::InvalidPointData)?;
    let output = secret.output(input_point);
    Ok(copy_vrf_output_hash(output))
}

/// Verify both header seal and entropy source in a single call.
pub fn verify_header_seals_impl(
    signer_key: &[u8],
    seal_data: &[u8],
    seal_payload: &[u8],
    unsealed_header: &[u8],
    entropy_data: &[u8],
    entropy_prefix: &[u8],
) -> Result<([u8; 32], [u8; 32]), Error> {
    let public_key = deserialize_public_key(signer_key);

    let seal = Verifier::ietf_vrf_verify(seal_payload, unsealed_header, seal_data, public_key)?;

    let mut entropy_payload = Vec::with_capacity(entropy_prefix.len() + seal.len());
    entropy_payload.extend_from_slice(entropy_prefix);
    entropy_payload.extend_from_slice(&seal);

    let entropy = Verifier::ietf_vrf_verify(&entropy_payload, &[], entropy_data, public_key)?;

    Ok((seal, entropy))
}

/// Verify a seal and return the VRF output hash.
pub fn verify_seal_impl(
    signer_key: &[u8],
    seal_data: &[u8],
    payload: &[u8],
    aux_data: &[u8],
) -> Result<[u8; 32], Error> {
    let public_key = deserialize_public_key(signer_key);
    Verifier::ietf_vrf_verify(payload, aux_data, seal_data, public_key)
}

/// Batch verify multiple tickets.
pub fn batch_verify_tickets_impl(
    ring_size: RingSize,
    commitment_bytes: &[u8],
    tickets_data: &[u8],
    vrf_input_data_len: usize,
) -> Vec<Result<[u8; 32], Error>> {
    let chunk_size = vrf_input_data_len + RING_SIGNATURE_SIZE;
    let commitment = match RingCommitment::deserialize_compressed_unchecked(commitment_bytes) {
        Ok(c) => c,
        Err(_) => {
            return tickets_data
                .chunks(chunk_size)
                .map(|_| Err(Error::InvalidSignature))
                .collect();
        }
    };

    tickets_data
        .chunks(chunk_size)
        .map(|chunk| {
            let signature = &chunk[0..RING_SIGNATURE_SIZE];
            let vrf_input_data = &chunk[RING_SIGNATURE_SIZE..];

            Verifier::ring_vrf_verify(
                ring_size,
                commitment.clone(),
                vrf_input_data,
                &[],
                signature,
            )
        })
        .collect()
}

pub mod ffi {
    //! FFI-ready functions that return `Vec<u8>` with status byte prefix.
    //! These are used by both WASM and native bindings.

    use super::*;

    const RESULT_OK: u8 = 0;
    const RESULT_ERR: u8 = 1;

    pub fn ring_commitment(keys: &[u8]) -> Vec<u8> {
        let ring_size = if keys.len() / PUBLIC_KEY_SIZE == RingSize::Full.size() {
            RingSize::Full
        } else {
            RingSize::Tiny
        };

        let public_keys: Vec<_> = keys
            .chunks(PUBLIC_KEY_SIZE)
            .map(deserialize_public_key)
            .collect();

        match compute_ring_commitment(&public_keys, ring_size) {
            Ok(commitment) => {
                let mut result = vec![RESULT_OK];
                result.extend(commitment);
                result
            }
            Err(_) => vec![RESULT_ERR],
        }
    }

    pub fn derive_public_key(seed: &[u8]) -> Vec<u8> {
        match derive_public_key_from_seed(seed) {
            Ok(key) => {
                let mut result = vec![RESULT_OK];
                result.extend(key);
                result
            }
            Err(_) => vec![RESULT_ERR],
        }
    }

    pub fn verify_header_seals(
        signer_key: &[u8],
        seal_data: &[u8],
        seal_payload: &[u8],
        unsealed_header: &[u8],
        entropy_data: &[u8],
        entropy_prefix: &[u8],
    ) -> Vec<u8> {
        match verify_header_seals_impl(
            signer_key,
            seal_data,
            seal_payload,
            unsealed_header,
            entropy_data,
            entropy_prefix,
        ) {
            Ok((seal, entropy)) => {
                let mut result = Vec::with_capacity(1 + 32 + 32);
                result.push(RESULT_OK);
                result.extend_from_slice(&seal);
                result.extend_from_slice(&entropy);
                result
            }
            Err(_) => {
                let mut result = vec![RESULT_ERR];
                result.extend([0u8; 64]);
                result
            }
        }
    }

    pub fn verify_seal(
        signer_key: &[u8],
        seal_data: &[u8],
        payload: &[u8],
        aux_data: &[u8],
    ) -> Vec<u8> {
        match verify_seal_impl(signer_key, seal_data, payload, aux_data) {
            Ok(entropy) => {
                let mut result = vec![RESULT_OK];
                result.extend(entropy);
                result
            }
            Err(_) => {
                let mut result = vec![RESULT_ERR];
                result.extend([0u8; 32]);
                result
            }
        }
    }

    pub fn generate_seal(secret_seed: &[u8], input: &[u8], aux_data: &[u8]) -> Vec<u8> {
        match generate_ietf_seal(secret_seed, input, aux_data) {
            Ok(seal) => {
                let mut result = vec![RESULT_OK];
                result.extend(seal);
                result
            }
            Err(_) => vec![RESULT_ERR],
        }
    }

    pub fn vrf_output_hash(secret_seed: &[u8], input: &[u8]) -> Vec<u8> {
        match compute_vrf_output_hash(secret_seed, input) {
            Ok(hash) => {
                let mut result = vec![RESULT_OK];
                result.extend_from_slice(&hash);
                result
            }
            Err(_) => vec![RESULT_ERR],
        }
    }

    pub fn batch_verify_tickets(
        ring_size: u32,
        commitment: &[u8],
        tickets_data: &[u8],
        vrf_input_data_len: u32,
    ) -> Vec<u8> {
        let ring_size = RingSize::from_size(ring_size as usize);
        let results = batch_verify_tickets_impl(
            ring_size,
            commitment,
            tickets_data,
            vrf_input_data_len as usize,
        );

        results.into_iter().fold(Vec::new(), |mut acc, result| {
            match result {
                Ok(entropy) => {
                    acc.push(RESULT_OK);
                    acc.extend(entropy);
                }
                Err(_) => {
                    acc.push(RESULT_ERR);
                    acc.extend([0u8; 32]);
                }
            }
            acc
        })
    }
}
