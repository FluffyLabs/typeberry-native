include!("./lib.rs");

const KEYS_HEX: &str = "aa2b95f7572875b0d0f186552ae745ba8222fc0b5bd456554bfe51c68938f8bcf16e5352840afb47e206b5c89f560f2611835855cf2e6ebad1acc9520a72591d5e465beb01dbafe160ce8216047f2155dd0569f058afd52dcea601025a8d161d48e5fcdce10e0b64ec4eebd0d9211c7bac2f27ce54bca6f7776ff6fee86ab3e33d5e5a51aab2b048f8686ecd79712a80e3265a114cc73f14bdb2a59233fb66d07f6190116d118d643a98878e294ccf62b509e214299931aad8ff9764181a4e33";
// const SIGNATURE_HEX: &str = "b342bf8f6fa69c745daad2e99c92929b1da2b840f67e5e8015ac22dd1076343eec00b574369c92536db6d65e5d6a3bc01fc8612cccbece4a32a8439e5ce646491be3b7b13eab6be3190eb57ddbde2a4c20472768563b12aa1e1b8b299cd41b8d2b566c14a7f8643a5d976ced0a18d12e32c660d59c66c271332138269cb0fe9c6707eb7821301ae4c172a9648036cf719fa2f64f272a008dd58f0734c00a440f7f5a457abb3c9271ffe61f8dc998ddda0499a783836b54e5b727f44a2eecb017b0341a1868fc476a0d6da019b5f2b4e521903b00e937f23b17ea49d6928c615841da5442e5b070079af6cdbbaed964a9b044dcf1ae69ce2e2febec37f6369910a0b20b9dce71b4cd3396e44a90a0a4c404cb170d7ffd2c5467f152bd5daf40b3b81e067e10509356f4b6e98a990c37346348d20fd47aab334c3acf2b51d58e63e0e5eecf84f646b0df292ee25b2d7267831b1ae29540a0604bc50f213f7c981752b740d026f8c4734de9c721f0fee4254053f763776a97e9b870d11e1a9c58b27db5b035b91c7276a18ed3654fb70b52a38b80503ba0cc88c53a04793bea963efdc3ae0e98fd0cda9330796ae9ca3d9795f4abe806978fab744e8c5659fb7469a5beb5bed213fdc1b036763c7f0d911627d43aa01340570c946e13d0951aa64d452fc7f7e842389fe25bd646d674931263656585c2f13336ff64a90674b0ae1df8b8a9f0fe28c8a049eaec1c788fde0d883958f9c08820751b9f9143bca8d656a12d28132eb81ebde2d21680cd0a6f589f9c8ebd2fe84917aa16492d91cf7f58aa92d1bb9b9905a88f8a9846637d77439641c33c9db9fe855ce5b532cbe00c6a828ee974f5918f970e106f2f60bc9cecebcf0a35a239934cbbb29f0ec99a412b247b54c96ac582b52179a8fae3a0f8956cfb6fc541902bef749a034a9a59cb08715fbed847592784097b2d20b923a14bb2f8319508341bf39b16b104199477680af2a7b2a2dcddba88963b64229e1a25ce8cb9f7fbdee49c6ecfb31751c7a237b46578030c7f0c5088d4b54ede07c1ecdc3730a07ff78ccd8c0a020cf494f5059f21802cb8f866a680854ce7162bfa8a";

fn main() {
    println!("Testing ring commitment...");
    let keys = hex::decode(KEYS_HEX).unwrap();
    println!("Keys: 0x{}", hex::encode(&keys));
    let commitment = ring_commitment(&keys);
    println!("Commitment: 0x{}", hex::encode(&commitment));
    assert_eq!(hex::encode(&commitment), EXPECTED_COMMITMENT);

    println!("\nTesting key generation...");
    let seed = hex::decode(SEED).unwrap();
    println!("Seed: {}", SEED);
    let key_pair = derive_key_pair(&seed);
    // we skip 1st byte, bcs its a result status, and first key bcs its private key
    let public_key = &key_pair[1 + 64..];
    println!("Public key: {}", hex::encode(&public_key));
    assert_eq!(hex::encode(&public_key), EXPECTED_PUBLIC_KEY);

    // let signature = hex::decode(SIGNATURE_HEX).unwrap();
    // let hash = entropy_hash(&signature);
    // println!("Hash: 0x{}", hex::encode(&hash));
}
const SEED: &str = "007596986419e027e65499cc87027a236bf4a78b5e8bd7f675759d73e7a9c799";
const EXPECTED_PUBLIC_KEY: &str =
    "ff71c6c03ff88adb5ed52c9681de1629a54e702fc14729f6b50d2f0a76f185b3";
const EXPECTED_COMMITMENT: &str = "008387a131593447e4e1c3d4e220c322e42d33207fa77cd0fedb39fc3491479ca47a2d82295252e278fa3eec78185982ed82ae0c8fd691335e703d663fb5be02b3def15380789320636b2479beab5a03ccb3f0909ffea59d859fcdc7e187e45a8c92e630ae2b14e758ab0960e372172203f4c9a41777dadd529971d7ab9d23ab29fe0e9c85ec450505dde7f5ac038274cf";
