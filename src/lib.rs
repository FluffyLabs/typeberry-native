//#![cfg_attr(not(feature = "std"), no_std)]

use wasm_bindgen::prelude::wasm_bindgen;

use ark_ec_vrfs::prelude::ark_ec::AffineRepr;
use ark_ec_vrfs::ring::RingSuite;
use ark_ec_vrfs::{pedersen::PedersenSuite, suites::bandersnatch::edwards as bandersnatch};
use ark_ec_vrfs::{prelude::ark_serialize, suites::bandersnatch::edwards::RingContext};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use bandersnatch::{
    AffinePoint, BandersnatchSha512Ell2, IetfProof, Input, Output, Public, RingProof, Secret,
};

#[derive(Clone, Copy)]
enum RingSize {
    Tiny,
    Full,
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

// "Static" ring context data
fn ring_context(ring_size: RingSize) -> &'static RingContext {
    use std::sync::OnceLock;
    static RING_CTX_TINY: OnceLock<RingContext> = OnceLock::new();
    static RING_CTX_FULL: OnceLock<RingContext> = OnceLock::new();

    let init = |size: usize| {
        use bandersnatch::PcsParams;
        let buf = include_bytes!("../data/zcash-srs-2-11-uncompressed.bin");
        let pcs_params = PcsParams::deserialize_uncompressed_unchecked(&mut &buf[..]).unwrap();
        RingContext::from_srs(size, pcs_params).unwrap()
    };

    match ring_size {
        RingSize::Tiny => RING_CTX_TINY.get_or_init(|| init(6)),
        RingSize::Full => RING_CTX_FULL.get_or_init(|| init(1023)),
    }
}

// Construct VRF Input Point from arbitrary data (section 1.2)
fn vrf_input_point(vrf_input_data: &[u8]) -> Input {
    Input::new(vrf_input_data).unwrap()
}

// Prover actor.
struct Prover {
    pub prover_idx: usize,
    pub secret: Secret,
    pub ring: Vec<Public>,
    pub size: RingSize,
}

impl Prover {
    pub fn new(size: RingSize, ring: Vec<Public>, prover_idx: usize) -> Self {
        Self {
            size,
            prover_idx,
            secret: Secret::from_seed(&prover_idx.to_le_bytes()),
            ring,
        }
    }

    /// VRF output hash.
    pub fn vrf_output(&self, vrf_input_data: &[u8]) -> Vec<u8> {
        let input = vrf_input_point(vrf_input_data);
        let output = self.secret.output(input);
        output.hash()[..32].try_into().unwrap()
    }

    /// Anonymous VRF signature.
    ///
    /// Used for tickets submission.
    pub fn ring_vrf_sign(&self, vrf_input_data: &[u8], aux_data: &[u8]) -> Vec<u8> {
        use ark_ec_vrfs::ring::Prover as _;

        let input = vrf_input_point(vrf_input_data);
        let output = self.secret.output(input);

        // Backend currently requires the wrapped type (plain affine points)
        let pts: Vec<_> = self.ring.iter().map(|pk| pk.0).collect();

        // Proof construction
        let ring_ctx = ring_context(self.size);
        let prover_key = ring_ctx.prover_key(&pts);
        let prover = ring_ctx.prover(prover_key, self.prover_idx);
        let proof = self.secret.prove(input, output, aux_data, &prover);

        // Output and Ring Proof bundled together (as per section 2.2)
        let signature = RingVrfSignature { output, proof };
        let mut buf = Vec::new();
        signature.serialize_compressed(&mut buf).unwrap();
        buf
    }

    /// Non-Anonymous VRF signature.
    ///
    /// Used for ticket claiming during block production.
    /// Not used with Safrole test vectors.
    pub fn ietf_vrf_sign(&self, vrf_input_data: &[u8], aux_data: &[u8]) -> Vec<u8> {
        use ark_ec_vrfs::ietf::Prover as _;

        let input = vrf_input_point(vrf_input_data);
        let output = self.secret.output(input);

        let proof = self.secret.prove(input, output, aux_data);

        // Output and IETF Proof bundled together (as per section 2.2)
        let signature = IetfVrfSignature { output, proof };
        let mut buf = Vec::new();
        signature.serialize_compressed(&mut buf).unwrap();
        buf
    }
}

type RingCommitment = ark_ec_vrfs::ring::RingCommitment<BandersnatchSha512Ell2>;

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
        let verifier_key = ring_context(ring_size).verifier_key(&pts);
        let commitment = verifier_key.commitment();
        Self { ring, commitment, ring_size }
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
    ) -> Result<[u8; 32], ()> {
        use ark_ec_vrfs::ring::Verifier as _;

        let signature = RingVrfSignature::deserialize_compressed(signature).unwrap();

        let input = vrf_input_point(vrf_input_data);
        let output = signature.output;

        let ring_ctx = ring_context(self.ring_size);

        // The verifier key is reconstructed from the commitment and the constant
        // verifier key component of the SRS in order to verify some proof.
        // As an alternative we can construct the verifier key using the
        // RingContext::verifier_key() method, but is more expensive.
        // In other words, we prefer computing the commitment once, when the keyset changes.
        let verifier_key = ring_ctx.verifier_key_from_commitment(self.commitment.clone());
        let verifier = ring_ctx.verifier(verifier_key);
        if Public::verify(input, output, aux_data, &signature.proof, &verifier).is_err() {
            println!("Ring signature verification failure");
            return Err(());
        }
        println!("Ring signature verified");

        // This truncated hash is the actual value used as ticket-id/score in JAM
        let vrf_output_hash: [u8; 32] = output.hash()[..32].try_into().unwrap();
        println!(" vrf-output-hash: {}", hex::encode(vrf_output_hash));
        Ok(vrf_output_hash)
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
    ) -> Result<[u8; 32], ()> {
        use ark_ec_vrfs::ietf::Verifier as _;

        let signature = IetfVrfSignature::deserialize_compressed(signature).unwrap();

        let input = vrf_input_point(vrf_input_data);
        let output = signature.output;

        let public = &self.ring[signer_key_index];
        if public
            .verify(input, output, aux_data, &signature.proof)
            .is_err()
        {
            println!("Ring signature verification failure");
            return Err(());
        }
        println!("Ietf signature verified");

        // This is the actual value used as ticket-id/score
        // NOTE: as far as vrf_input_data is the same, this matches the one produced
        // using the ring-vrf (regardless of aux_data).
        let vrf_output_hash: [u8; 32] = output.hash()[..32].try_into().unwrap();
        println!(" vrf-output-hash: {}", hex::encode(vrf_output_hash));
        Ok(vrf_output_hash)
    }
}

macro_rules! measure_time {
    ($func_name:expr, $func_call:expr) => {{
        let start = std::time::Instant::now();
        let result = $func_call;
        let duration = start.elapsed();
        println!("* Time taken by {}: {:?}", $func_name, duration);
        result
    }};
}

fn print_point(name: &str, p: AffinePoint) {
    println!("------------------------------");
    println!("[{name}]");
    println!("X: {}", p.x);
    println!("Y: {}", p.y);
    let mut buf = Vec::new();
    p.serialize_compressed(&mut buf).unwrap();
    println!("Compressed: 0x{}", hex::encode(buf));
}

fn print_points(ring_size: RingSize) {
    println!("==============================");
    print_point("Group Base", AffinePoint::generator());
    print_point("Blinding Base", BandersnatchSha512Ell2::BLINDING_BASE);
    print_point("Ring Padding", ring_context(ring_size).padding_point());
    print_point("Accumulator Base", BandersnatchSha512Ell2::ACCUMULATOR_BASE);
    println!("==============================");
}

#[wasm_bindgen]
pub fn verify_safrole() -> bool {
    let ring_size = RingSize::Full;
    let ring_len: i32 = 1023;

    print_points(ring_size);

    let mut ring: Vec<_> = (0..ring_len)
        .map(|i| Secret::from_seed(&i.to_le_bytes()).public())
        .collect();
    let prover_key_index = 3;

    // NOTE: any key can be replaced with the padding point
    let padding_point = Public::from(ring_context(ring_size).padding_point());
    ring[2] = padding_point;
    ring[7] = padding_point;

    let prover = Prover::new(ring_size, ring.clone(), prover_key_index);
    let verifier = Verifier::new(ring_size, ring);

    let vrf_input_data = b"foo";

    //--- Anonymous VRF

    let aux_data = b"bar";

    // Prover signs some data.
    let ring_signature = measure_time! {
        "ring-vrf-sign",
        prover.ring_vrf_sign(vrf_input_data, aux_data)
    };

    // Verifier checks it without knowing who is the signer.
    let ring_vrf_output_hash = measure_time! {
        "ring-vrf-verify",
        verifier.ring_vrf_verify(vrf_input_data, aux_data, &ring_signature).unwrap()
    };

    //--- Non anonymous VRF

    let other_aux_data = b"hello";

    // Prover signs the same vrf-input data (we want the output to match)
    // But different aux data.
    let ietf_signature = measure_time! {
        "ietf-vrf-sign",
        prover.ietf_vrf_sign(vrf_input_data, other_aux_data)
    };

    // Verifier checks the signature knowing the signer identity.
    let ietf_vrf_output_hash = measure_time! {
        "ietf-vrf-verify",
        verifier.ietf_vrf_verify(vrf_input_data, other_aux_data, &ietf_signature, prover_key_index).unwrap()
    };

    // Must match
    assert_eq!(ring_vrf_output_hash, ietf_vrf_output_hash);

    // We don't need to produce a signature to get the vrf output
    let vrf_output_hash = prover.vrf_output(vrf_input_data);
    assert_eq!(vrf_output_hash, ietf_vrf_output_hash);

    true
}

#[wasm_bindgen]
pub fn ring_commitment(
    keys: Vec<u8>
) -> Vec<u8> {
    let keys: Vec<Public> = keys.chunks(32).map(|chunk| {
        Public::deserialize_compressed(chunk).unwrap()
    }).collect();
    let ring_size = if keys.len() == 1023 { RingSize::Full } else { RingSize::Tiny };
    let verifier = Verifier::new(ring_size, keys);

    let mut buf = Vec::new();
    verifier.commitment.serialize_compressed(&mut buf).unwrap();
    buf    
}

#[wasm_bindgen]
pub fn get_entropy_hash(
    signature: &[u8],
) -> Vec<u8> {
    let signature = RingVrfSignature::deserialize_compressed(signature).unwrap();
    let output: ark_ec_vrfs::Output<BandersnatchSha512Ell2> = signature.output;
    let vrf_output_hash: [u8; 32] = output.hash()[..32].try_into().unwrap();
    vrf_output_hash.to_vec()
}
