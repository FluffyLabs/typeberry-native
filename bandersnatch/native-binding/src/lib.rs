use bandersnatch_core::ffi;
use napi::bindgen_prelude::*;
use napi_derive::napi;

#[napi]
pub fn ring_commitment(keys: Buffer) -> Buffer {
    ffi::ring_commitment(keys.as_ref()).into()
}

#[napi]
pub fn derive_public_key(seed: Buffer) -> Buffer {
    ffi::derive_public_key(seed.as_ref()).into()
}

#[napi]
pub fn verify_header_seals(
    signer_key: Buffer,
    seal_data: Buffer,
    seal_payload: Buffer,
    unsealed_header: Buffer,
    entropy_data: Buffer,
    entropy_prefix: Buffer,
) -> Buffer {
    ffi::verify_header_seals(
        signer_key.as_ref(),
        seal_data.as_ref(),
        seal_payload.as_ref(),
        unsealed_header.as_ref(),
        entropy_data.as_ref(),
        entropy_prefix.as_ref(),
    )
    .into()
}

#[napi]
pub fn verify_seal(
    signer_key: Buffer,
    seal_data: Buffer,
    payload: Buffer,
    aux_data: Buffer,
) -> Buffer {
    ffi::verify_seal(
        signer_key.as_ref(),
        seal_data.as_ref(),
        payload.as_ref(),
        aux_data.as_ref(),
    )
    .into()
}

#[napi]
pub fn generate_seal(secret_seed: Buffer, input: Buffer, aux_data: Buffer) -> Buffer {
    ffi::generate_seal(secret_seed.as_ref(), input.as_ref(), aux_data.as_ref()).into()
}

#[napi]
pub fn vrf_output_hash(secret_seed: Buffer, input: Buffer) -> Buffer {
    ffi::vrf_output_hash(secret_seed.as_ref(), input.as_ref()).into()
}

#[napi]
pub fn batch_verify_tickets(
    ring_size: u32,
    commitment: Buffer,
    tickets_data: Buffer,
    vrf_input_data_len: u32,
) -> Buffer {
    ffi::batch_verify_tickets(
        ring_size,
        commitment.as_ref(),
        tickets_data.as_ref(),
        vrf_input_data_len,
    )
    .into()
}
