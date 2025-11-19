#[cfg(test)]
mod tests {
    use crate::{verify_ed25519, verify_ed25519_batch};

    #[test]
    fn should_verify_ed25519() {
        let data = hex::decode(
            "3b6a27bcceb6a42d62a3a8d02a6f0d73653215771de243a63ac048a18b59da29f23e45d7f8977a8eda61513bd5cab1451eb64f265edf340c415f25480123391364521f9bb4c14f840a0dae20eb4dc4a735c961d9966da51dde0d85281dc1dc0b2d6a616d5f67756172616e74656511da6d1f761ddf9bdb4c9d6e5303ebd41f61858d0a5647a1a7bfe089bf921be9").unwrap()
        ;

        let result = verify_ed25519(&data.clone());
        assert_eq!(result, vec![1]);
    }

    #[test]
    fn should_batch_verify_ed25519() {
        let data = hex::decode(
            "3b6a27bcceb6a42d62a3a8d02a6f0d73653215771de243a63ac048a18b59da29f23e45d7f8977a8eda61513bd5cab1451eb64f265edf340c415f25480123391364521f9bb4c14f840a0dae20eb4dc4a735c961d9966da51dde0d85281dc1dc0b2d6a616d5f67756172616e74656511da6d1f761ddf9bdb4c9d6e5303ebd41f61858d0a5647a1a7bfe089bf921be9").unwrap()
        ;

        let result = verify_ed25519_batch(&data.clone());
        assert_eq!(result, true);
    }
}
