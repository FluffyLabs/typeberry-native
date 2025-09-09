use ed25519_dalek::{ed25519, Signature, VerifyingKey};
use std::io::{self, Cursor, Read};
use wasm_bindgen::prelude::wasm_bindgen;

const KEY_LENGTH: usize = 32;
const SIGNATURE_LENGTH: usize = 64;

type Message = Vec<u8>;

/**
 * Verify Ed25519 signatures one by one using strict verification.
 *
 * This function is slower but does strict verification.
 */
#[wasm_bindgen]
pub fn verify_ed25519(
    data: &[u8], // [key (32 bytes), signature (64 bytes), message_length (1 byte), message ({message_length} bytes)]
) -> Vec<u8> {
    let mut results = vec![];
    let mut cursor = Cursor::new(data);

    while cursor.position() < data.len() as u64 {
        let verificaton_result = read_chunk(&mut cursor)
            .and_then(|(key, signature, message)| Ok(key.verify_strict(&message, &signature)?));

        results.push(match verificaton_result {
            Ok(_) => 1,
            Err(_) => 0,
        });
    }

    results
}

/**
 * Verify Ed25519 signatures using build-in batch verification.
 *
 * This function is faster but does not do strict verification.
 * See https://crates.io/crates/ed25519-dalek#batch-verification for more information.
 */
#[wasm_bindgen]
pub fn verify_ed25519_batch(
    data: &[u8], // [key (32 bytes), signature (64 bytes), message_length (1 byte) message (message_length bytes)]
) -> bool {
    let mut cursor = Cursor::new(data);
    let mut keys = vec![];
    let mut signatures = vec![];
    let mut messages = vec![];
    while cursor.position() < data.len() as u64 {
        let chunk = read_chunk(&mut cursor);

        if let Ok((key, signature, message)) = chunk {
            keys.push(key);
            signatures.push(signature);
            messages.push(message);
        } else {
            return false;
        }
    }

    let messages_refs: Vec<&[u8]> = messages.iter().map(|msg| msg.as_slice()).collect();

    ed25519_dalek::verify_batch(&messages_refs, &signatures, &keys).is_ok()
}

fn read_chunk(cursor: &mut Cursor<&[u8]>) -> Result<(VerifyingKey, Signature, Message), Error> {
    let mut key = [0u8; KEY_LENGTH];
    let mut sig = [0u8; SIGNATURE_LENGTH];
    let mut msg_len = [0u8; 1];
    cursor.read_exact(&mut key)?;
    cursor.read_exact(&mut sig)?;
    cursor.read_exact(&mut msg_len)?;

    let mut msg = vec![0u8; msg_len[0] as usize];
    cursor.read_exact(&mut msg)?;
    Ok((
        VerifyingKey::from_bytes(&key)?,
        Signature::from_bytes(&sig),
        msg,
    ))
}

pub enum Error {
    Io(io::Error),
    Crypto(ed25519::Error),
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<ed25519::Error> for Error {
    fn from(value: ed25519::Error) -> Self {
        Self::Crypto(value)
    }
}
