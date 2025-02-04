include!("./lib.rs");

const KEYS_HEX: &str = "aa2b95f7572875b0d0f186552ae745ba8222fc0b5bd456554bfe51c68938f8bcf16e5352840afb47e206b5c89f560f2611835855cf2e6ebad1acc9520a72591d5e465beb01dbafe160ce8216047f2155dd0569f058afd52dcea601025a8d161d48e5fcdce10e0b64ec4eebd0d9211c7bac2f27ce54bca6f7776ff6fee86ab3e33d5e5a51aab2b048f8686ecd79712a80e3265a114cc73f14bdb2a59233fb66d07f6190116d118d643a98878e294ccf62b509e214299931aad8ff9764181a4e33";

fn hex_char_to_val(c: char) -> Option<u8> {
    match c {
        '0'..='9' => Some(c as u8 - b'0'),
        'a'..='f' => Some(c as u8 - b'a' + 10),
        'A'..='F' => Some(c as u8 - b'A' + 10),
        _ => None,
    }
}

fn hex_to_bytes(hex: &str) -> Result<Vec<u8>, String> {
    if hex.len() % 2 != 0 {
        return Err("Hex string must have an even number of digits".into());
    }

    let mut bytes = Vec::with_capacity(hex.len() / 2);
    let mut chars = hex.chars();

    while let (Some(c1), Some(c2)) = (chars.next(), chars.next()) {
        let high = hex_char_to_val(c1)
            .ok_or_else(|| format!("Invalid hex digit: {}", c1))?;
        let low = hex_char_to_val(c2)
            .ok_or_else(|| format!("Invalid hex digit: {}", c2))?;
        bytes.push(high << 4 | low);
    }

    Ok(bytes)
}

fn bytes_to_hex(bytes: &[u8]) -> String {
    // A lookup table for hex digits.
    const HEX_CHARS: &[u8; 16] = b"0123456789abcdef";

    // Pre-allocate a string with enough capacity.
    let mut hex_string = String::with_capacity(bytes.len() * 2);

    // For each byte, convert it to two hex characters.
    for &byte in bytes {
        // High nibble (first hex digit)
        hex_string.push(HEX_CHARS[(byte >> 4) as usize] as char);
        // Low nibble (second hex digit)
        hex_string.push(HEX_CHARS[(byte & 0x0F) as usize] as char);
    }

    hex_string
}

fn main() {
    let keys = hex_to_bytes(KEYS_HEX).unwrap();
    println!("Keys: 0x{}", bytes_to_hex(&keys));
    let commitment = ring_commitment(keys);
    println!("Commitment: 0x{}", bytes_to_hex(&commitment));
}

const EXPECTED_COMMITMENT: &str = "b3750bba87e39fb38579c880ff3b5c4e0aa90df8ff8be1ddc5fdd615c6780955f8fd85d99fd92a3f1d4585eb7ae8d627b01dd76d41720d73c9361a1dd2e830871155834c55db72de38fb875a9470faedb8cae54b34f7bfe196a9caca00c2911592e630ae2b14e758ab0960e372172203f4c9a41777dadd529971d7ab9d23ab29fe0e9c85ec450505dde7f5ac038274cf";
