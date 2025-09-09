//#![cfg_attr(not(feature = "std"), no_std)]

use wasm_bindgen::prelude::wasm_bindgen;

use ark_vrf::reexports::ark_serialize::{self, CanonicalDeserialize, CanonicalSerialize};
use ark_vrf::suites::bandersnatch;
use bandersnatch::{
    BandersnatchSha512Ell2, IetfProof, Input, Output, Public, RingProof, RingProofParams, Secret,
};

#[derive(Clone, Copy)]
enum RingSize {
    Tiny,
    Full,
}

impl RingSize {
    pub fn size(&self) -> usize {
        match *self {
            RingSize::Tiny => 6,
            RingSize::Full => 1023,
        }
    }
}

// This is the IETF `Prove` procedure output as described in section 2.2
// of the Bandersnatch VRFs specification
#[derive(CanonicalSerialize, CanonicalDeserialize)]
struct IetfVrfSignature {
    output: Output,
    proof: IetfProof,
}

// This is the IETF `Prove` procedure output as described in section 4.2
// of the Bandersnatch VRFs specification
#[derive(CanonicalSerialize, CanonicalDeserialize)]
struct RingVrfSignature {
    output: Output,
    // This contains both the Pedersen proof and actual ring proof.
    proof: RingProof,
}

fn ring_proof_params(ring_size: RingSize) -> &'static RingProofParams {
    use std::sync::OnceLock;
    static PARAMS_TINY: OnceLock<RingProofParams> = OnceLock::new();
    static PARAMS_FULL: OnceLock<RingProofParams> = OnceLock::new();

    let init = |size: usize| {
        use bandersnatch::PcsParams;
        let buf = include_bytes!("../data/zcash-srs-2-11-uncompressed.bin");
        let pcs_params = PcsParams::deserialize_uncompressed_unchecked(&mut &buf[..])
            .expect("binary data invalid");
        RingProofParams::from_pcs_params(size, pcs_params).expect("invalid ring proof params")
    };

    match ring_size {
        RingSize::Tiny => PARAMS_TINY.get_or_init(|| init(ring_size.size())),
        RingSize::Full => PARAMS_FULL.get_or_init(|| init(ring_size.size())),
    }
}

// Construct VRF Input Point from arbitrary data (section 1.2)
fn vrf_input_point(vrf_input_data: &[u8]) -> Result<Input, Error> {
    Input::new(vrf_input_data).ok_or(Error::InvalidPointData)
}

#[derive(Debug)]
pub enum Error {
    InvalidPointData,
    InvalidSignature,
    VerificationFailure,
}

type RingCommitment = ark_vrf::ring::RingCommitment<BandersnatchSha512Ell2>;

// Verifier actor.
struct Verifier {
    pub commitment: RingCommitment,
    pub ring: Vec<Public>,
    pub ring_size: RingSize,
}

impl Verifier {
    fn new(ring_size: RingSize, ring: Vec<Public>) -> Self {
        // Backend currently requires the wrapped type (plain affine points)
        let pts: Vec<_> = ring.iter().map(|pk| pk.0).collect();
        let verifier_key = ring_proof_params(ring_size).verifier_key(&pts);
        let commitment = verifier_key.commitment();
        Self {
            ring,
            commitment,
            ring_size,
        }
    }

    /// Anonymous VRF signature verification.
    ///
    /// Used for tickets verification.
    ///
    /// On success returns the VRF output hash.
    pub fn ring_vrf_verify(
        &self,
        vrf_input_data: &[u8],
        aux_data: &[u8],
        signature: &[u8],
    ) -> Result<[u8; 32], Error> {
        use ark_vrf::ring::Verifier as _;

        let signature = RingVrfSignature::deserialize_compressed(signature)
            .map_err(|_| Error::InvalidSignature)?;

        let input = vrf_input_point(vrf_input_data)?;
        let output = signature.output;

        let ring_params = ring_proof_params(self.ring_size);

        // The verifier key is reconstructed from the commitment and the constant
        // verifier key component of the SRS in order to verify some proof.
        // As an alternative we can construct the verifier key using the
        // RingContext::verifier_key() method, but is more expensive.
        // In other words, we prefer computing the commitment once, when the keyset changes.
        let verifier_key = ring_params.verifier_key_from_commitment(self.commitment.clone());
        let verifier = ring_params.verifier(verifier_key);
        if Public::verify(input, output, aux_data, &signature.proof, &verifier).is_err() {
            println!("Ring signature verification failure");
            return Err(Error::VerificationFailure);
        }
        println!("Ring signature verified");

        // This truncated hash is the actual value used as ticket-id/score in JAM
        Ok(vrf_output_hash(output))
    }

    /// Non-Anonymous VRF signature verification.
    ///
    /// Used for ticket claim verification during block import.
    /// Not used with Safrole test vectors.
    ///
    /// On success returns the VRF output hash.
    pub fn ietf_vrf_verify(
        &self,
        vrf_input_data: &[u8],
        aux_data: &[u8],
        signature: &[u8],
        signer_key_index: usize,
    ) -> Result<[u8; 32], Error> {
        use ark_vrf::ietf::Verifier as _;

        let signature = IetfVrfSignature::deserialize_compressed(signature)
            .map_err(|_| Error::InvalidSignature)?;

        let input = vrf_input_point(vrf_input_data)?;
        let output = signature.output;

        let public = &self.ring[signer_key_index];
        if public
            .verify(input, output, aux_data, &signature.proof)
            .is_err()
        {
            println!("Ring signature verification failure");
            return Err(Error::VerificationFailure);
        }
        println!("Ietf signature verified");

        // This is the actual value used as ticket-id/score
        // NOTE: as far as vrf_input_data is the same, this matches the one produced
        // using the ring-vrf (regardless of aux_data).
        Ok(vrf_output_hash(output))
    }
}

fn vrf_output_hash(output: Output) -> [u8; 32] {
    let mut vrf_output_hash = [0u8; 32];
    vrf_output_hash.copy_from_slice(&output.hash()[..32]);
    println!(" vrf-output-hash: {}", hex::encode(vrf_output_hash));
    vrf_output_hash
}

fn create_verifier(keys: &[u8]) -> Verifier {
    let ring_size = if keys.len() / HASH_SIZE == RingSize::Full.size() {
        RingSize::Full
    } else {
        RingSize::Tiny
    };
    let keys = keys
        .chunks(HASH_SIZE)
        .map(|chunk| {
            Public::deserialize_compressed(chunk)
                .unwrap_or_else(|_| Public::from(RingProofParams::padding_point()))
        })
        .collect();
    Verifier::new(ring_size, keys)
}

// Return types are always starting with a byte representing either
// OK or ERROR code.
const RESULT_OK: u8 = 0;
const RESULT_ERR: u8 = 1;

const HASH_SIZE: usize = 32;

#[wasm_bindgen]
pub fn ring_commitment(keys: &[u8]) -> Vec<u8> {
    let verifier = create_verifier(keys);
    let mut buf = Vec::new();
    buf.push(RESULT_OK);
    match verifier.commitment.serialize_compressed(&mut buf) {
        Ok(_) => buf,
        Err(_) => vec![RESULT_ERR],
    }
}

const SIGNATURE_SIZE: usize = 784;

/// Derive Private and Public Key from Seed
///
/// returns: `Vec<u8>` containing the exit (1 byte) status followed by the (32 bytes) public key
#[wasm_bindgen]
pub fn derive_public_key(seed: &[u8]) -> Vec<u8> {
    let secret = Secret::from_seed(seed);

    let mut result = vec![RESULT_OK];
    let mut buf = Vec::new();
    if secret.public().serialize_compressed(&mut buf).is_ok() {
        result.extend(buf);
    } else {
        return vec![RESULT_ERR];
    }

    result
}

/// Seal verification as defined in:
/// https://graypaper.fluffylabs.dev/#/68eaa1f/0eff000eff00?v=0.6.4
/// or
/// https://graypaper.fluffylabs.dev/#/68eaa1f/0e54010e5401?v=0.6.4
#[wasm_bindgen]
pub fn verify_seal(
    keys: &[u8],
    signer_key_index: u32,
    seal_data: &[u8], // VRF Signature (96 bytes)
    payload: &[u8],   // vrf_input_data (? bytes)
    aux_data: &[u8],  // aux_data (? bytes)
) -> Vec<u8> {
    let mut result = vec![];
    let verifier = create_verifier(keys);
    match verifier.ietf_vrf_verify(payload, aux_data, seal_data, signer_key_index as usize) {
        Ok(entropy) => {
            result.push(RESULT_OK);
            result.extend(entropy);
        }
        Err(_) => {
            result.push(RESULT_ERR);
            result.extend([0u8; 32]);
        }
    }
    result
}

/// Verify multiple tickets at once as defined in:
/// https://graypaper.fluffylabs.dev/#/68eaa1f/0f3e000f3e00?v=0.6.4
///
/// NOTE: the aux_data of VRF function is empty!
#[wasm_bindgen]
pub fn batch_verify_tickets(
    keys: &[u8],
    tickets_data: &[u8], // [proof/signature (784 bytes), vrf_input_data (? bytes); NO_OF_TICKETS]
    vrf_input_data_len: u32, // the data we prove over
) -> Vec<u8> {
    let verifier = create_verifier(keys);
    let chunk_size = vrf_input_data_len as usize + SIGNATURE_SIZE;
    tickets_data
        .chunks(chunk_size)
        .fold(vec![], |mut result, chunk| {
            let signature = &chunk[0..SIGNATURE_SIZE];
            let vrf_input_data = &chunk[SIGNATURE_SIZE..];

            match verifier.ring_vrf_verify(vrf_input_data, &[], signature) {
                Ok(entropy) => {
                    result.push(RESULT_OK);
                    result.extend(entropy);
                }
                Err(_) => {
                    result.push(RESULT_ERR);
                    result.extend([0u8; 32]);
                }
            };
            result
        })
}
