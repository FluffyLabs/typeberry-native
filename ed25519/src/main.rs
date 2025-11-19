include!("./lib.rs");

const KEY_HEX: &str = "3b6a27bcceb6a42d62a3a8d02a6f0d73653215771de243a63ac048a18b59da29";
const SIGNATURE_HEX: &str = "f23e45d7f8977a8eda61513bd5cab1451eb64f265edf340c415f25480123391364521f9bb4c14f840a0dae20eb4dc4a735c961d9966da51dde0d85281dc1dc0b";
const MESSAGE_HEX: &str =
    "6a616d5f67756172616e74656511da6d1f761ddf9bdb4c9d6e5303ebd41f61858d0a5647a1a7bfe089bf921be9";

fn main() {
    let key = hex::decode(KEY_HEX).unwrap();
    println!("Key: 0x{}", hex::encode(&key));
    let signature = hex::decode(SIGNATURE_HEX).unwrap();
    println!("Key: 0x{}", hex::encode(&signature));
    let message = hex::decode(MESSAGE_HEX).unwrap();
    println!("Message: 0x{}", hex::encode(&message));
    let message_len = message.len() as u8;
    println!("Message len: 0x{}", &message_len);
    let data = &[key, signature, vec![message_len], message].concat();
    println!("Data: 0x{}", hex::encode(&data));
    let verify_ed25519_result = verify_ed25519(&data.clone());
    println!("verify_ed25519 result: {:?}", verify_ed25519_result);
    let verify_ed25519_batch_result = verify_ed25519_batch(&data.clone());
    println!(
        "verify_ed25519_batch result: {:?}",
        verify_ed25519_batch_result
    );
}
