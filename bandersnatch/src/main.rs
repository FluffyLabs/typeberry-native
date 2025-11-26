include!("./lib.rs");

const KEYS_HEX: &str = "aa2b95f7572875b0d0f186552ae745ba8222fc0b5bd456554bfe51c68938f8bcf16e5352840afb47e206b5c89f560f2611835855cf2e6ebad1acc9520a72591d5e465beb01dbafe160ce8216047f2155dd0569f058afd52dcea601025a8d161d48e5fcdce10e0b64ec4eebd0d9211c7bac2f27ce54bca6f7776ff6fee86ab3e33d5e5a51aab2b048f8686ecd79712a80e3265a114cc73f14bdb2a59233fb66d07f6190116d118d643a98878e294ccf62b509e214299931aad8ff9764181a4e33";
const EXPECTED_COMMITMENT: &str = "008387a131593447e4e1c3d4e220c322e42d33207fa77cd0fedb39fc3491479ca47a2d82295252e278fa3eec78185982ed82ae0c8fd691335e703d663fb5be02b3def15380789320636b2479beab5a03ccb3f0909ffea59d859fcdc7e187e45a8c92e630ae2b14e758ab0960e372172203f4c9a41777dadd529971d7ab9d23ab29fe0e9c85ec450505dde7f5ac038274cf";
const SEED: &str = "007596986419e027e65499cc87027a236bf4a78b5e8bd7f675759d73e7a9c799";
const EXPECTED_PUBLIC_KEY: &str =
    "ff71c6c03ff88adb5ed52c9681de1629a54e702fc14729f6b50d2f0a76f185b3";

fn main() {
    println!("Testing ring commitment...");
    let keys = hex::decode(KEYS_HEX).unwrap();
    println!("Keys: 0x{}", hex::encode(&keys));
    let commitment = ring_commitment(&keys);
    println!("Commitment: 0x{}", hex::encode(&commitment));
    assert_eq!(hex::encode(&commitment), EXPECTED_COMMITMENT);

    println!("\nTesting key generation...");
    let seed = hex::decode(SEED).unwrap();
    println!("Seed: {SEED}");
    let key = derive_public_key(&seed);
    // we skip 1st byte, bcs its a result status
    let public_key = &key[1..];
    println!("Public key: {}", hex::encode(public_key));
    assert_eq!(hex::encode(public_key), EXPECTED_PUBLIC_KEY);

    println!("Testing seal geneation...");
    let seed = b"example seed";
    let input = b"example input";
    let aux_data = b"example aux data";
    let seal = generate_seal(seed, input, aux_data);
    println!("Seal: 0x{}", hex::encode(&seal));

    let pub_key = derive_public_key(seed);
    let result = verify_seal(&pub_key[1..], &seal, input, aux_data);
    println!("Result: 0x{}", hex::encode(&result[1..]));
}
