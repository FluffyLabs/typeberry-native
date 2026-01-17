use bandersnatch_core::ffi;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn ring_commitment(keys: &[u8]) -> Vec<u8> {
    ffi::ring_commitment(keys)
}

#[wasm_bindgen]
pub fn derive_public_key(seed: &[u8]) -> Vec<u8> {
    ffi::derive_public_key(seed)
}

#[wasm_bindgen]
pub fn verify_header_seals(
    signer_key: &[u8],
    seal_data: &[u8],
    seal_payload: &[u8],
    unsealed_header: &[u8],
    entropy_data: &[u8],
    entropy_prefix: &[u8],
) -> Vec<u8> {
    ffi::verify_header_seals(
        signer_key,
        seal_data,
        seal_payload,
        unsealed_header,
        entropy_data,
        entropy_prefix,
    )
}

#[wasm_bindgen]
pub fn verify_seal(
    signer_key: &[u8],
    seal_data: &[u8],
    payload: &[u8],
    aux_data: &[u8],
) -> Vec<u8> {
    ffi::verify_seal(signer_key, seal_data, payload, aux_data)
}

#[wasm_bindgen]
pub fn generate_seal(secret_seed: &[u8], input: &[u8], aux_data: &[u8]) -> Vec<u8> {
    ffi::generate_seal(secret_seed, input, aux_data)
}

#[wasm_bindgen]
pub fn vrf_output_hash(secret_seed: &[u8], input: &[u8]) -> Vec<u8> {
    ffi::vrf_output_hash(secret_seed, input)
}

#[wasm_bindgen]
pub fn batch_verify_tickets(
    ring_size: u32,
    commitment: &[u8],
    tickets_data: &[u8],
    vrf_input_data_len: u32,
) -> Vec<u8> {
    ffi::batch_verify_tickets(ring_size, commitment, tickets_data, vrf_input_data_len)
}
