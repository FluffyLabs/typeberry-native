use ed25519_consensus::{Error as Ed25519Error, Signature, VerificationKey};
use std::io::{self, Cursor, Read};
use wasm_bindgen::prelude::wasm_bindgen;

mod test;

const KEY_LENGTH: usize = 32;
const SIGNATURE_LENGTH: usize = 64;

type Message = Vec<u8>;

/**
 * Verify Ed25519 signatures one by one.
 *
 *  ed25519-consensus always does strict verification (ZIP-215 compatible). */
#[wasm_bindgen]
pub fn verify_ed25519(
    data: &[u8], // [key (32 bytes), signature (64 bytes), message_length (1 byte), message ({message_length} bytes)]
) -> Vec<u8> {
    let mut results = vec![];
    let mut cursor = Cursor::new(data);

    while cursor.position() < data.len() as u64 {
        let verificaton_result = read_chunk(&mut cursor).and_then(|(key, signature, message)| {
            key.verify(&signature, &message).map_err(Error::from)
        });

        results.push(match verificaton_result {
            Ok(_) => 1,
            Err(_) => 0,
        });
    }

    results
}

/**
 * ed25519-consensus doesn't have built-in batch verification.
 */
#[wasm_bindgen]
pub fn verify_ed25519_batch(
    data: &[u8], // [key (32 bytes), signature (64 bytes), message_length (1 byte) message (message_length bytes)]
) -> bool {
    let mut cursor = Cursor::new(data);

    while cursor.position() < data.len() as u64 {
        let chunk = read_chunk(&mut cursor);

        match chunk {
            Ok((key, signature, message)) => {
                if key.verify(&signature, &message).is_err() {
                    return false;
                }
            }
            Err(_) => return false,
        }
    }

    true
}

fn read_chunk(cursor: &mut Cursor<&[u8]>) -> Result<(VerificationKey, Signature, Message), Error> {
    let mut key = [0u8; KEY_LENGTH];
    let mut sig = [0u8; SIGNATURE_LENGTH];
    let mut msg_len = [0u8; 1];
    cursor.read_exact(&mut key)?;
    cursor.read_exact(&mut sig)?;
    cursor.read_exact(&mut msg_len)?;

    let mut msg = vec![0u8; msg_len[0] as usize];
    cursor.read_exact(&mut msg)?;

    Ok((
        VerificationKey::try_from(key.as_slice())?,
        Signature::try_from(sig.as_slice())?,
        msg,
    ))
}

pub enum Error {
    Io(io::Error),
    Crypto(Ed25519Error),
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<Ed25519Error> for Error {
    fn from(value: Ed25519Error) -> Self {
        Self::Crypto(value)
    }
}
