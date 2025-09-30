#[cfg(test)]
mod tests {
    use crate::ring_commitment;

    #[test]
    fn should_get_ring_commitment() {
        let keys_str = "ff71c6c03ff88adb5ed52c9681de1629a54e702fc14729f6b50d2f0a76f185b3dee6d555b82024f1ccf8a1e37e60fa60fd40b1958c4bb3006af78647950e1b919326edb21e5541717fde24ec085000b28709847b8aab1ac51f84e94b37ca1b660746846d17469fb2f95ef365efcab9f4e22fa1feb53111c995376be8019981cc151e5c8fe2b9d8a606966a79edd2f9e5db47e83947ce368ccba53bf6ba20a40b2105650944fcd101621fd5bb3124c9fd191d114b7ad936c1d79d734f9f21392e";
        let keys = hex::decode(keys_str).unwrap();

        let start = std::time::Instant::now();
        let commitment = ring_commitment(&keys);
        let duration1 = start.elapsed();
        println!("First call took: {:?}", duration1);

        let start = std::time::Instant::now();
        let commitment2 = ring_commitment(&keys);
        let duration2 = start.elapsed();
        println!("Second call took: {:?}", duration2);

        println!("Difference: {:?}", duration1.saturating_sub(duration2));

        assert_eq!(hex::encode(&commitment), "00af39b7de5fcfb9fb8a46b1645310529ce7d08af7301d9758249da4724ec698eb127f489b58e49ae9ab85027509116962a135fc4d97b66fbbed1d3df88cd7bf5cc6e5d7391d261a4b552246648defcb64ad440d61d69ec61b5473506a48d58e1992e630ae2b14e758ab0960e372172203f4c9a41777dadd529971d7ab9d23ab29fe0e9c85ec450505dde7f5ac038274cf");

        assert_eq!(hex::encode(&commitment), hex::encode(&commitment2));
    }
}
