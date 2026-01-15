#[cfg(test)]
mod tests {
    use crate::{
        derive_public_key, generate_seal, ring_commitment, verify_header_seals, verify_seal,
        vrf_output_hash,
    };

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

        assert_eq!(
            hex::encode(&commitment),
            "00af39b7de5fcfb9fb8a46b1645310529ce7d08af7301d9758249da4724ec698eb127f489b58e49ae9ab85027509116962a135fc4d97b66fbbed1d3df88cd7bf5cc6e5d7391d261a4b552246648defcb64ad440d61d69ec61b5473506a48d58e1992e630ae2b14e758ab0960e372172203f4c9a41777dadd529971d7ab9d23ab29fe0e9c85ec450505dde7f5ac038274cf"
        );

        assert_eq!(hex::encode(&commitment), hex::encode(&commitment2));
    }

    #[test]
    fn should_derive_public_key_from_seed() {
        let seed = b"example seed";

        let result = derive_public_key(seed);

        let status = result[0];
        let public_key = &result[1..];

        assert_eq!(status, 0);
        assert_eq!(
            hex::encode(public_key),
            "a777e887df9b783c6734140cdd95f74615bfd083ec8189c98ef5e3892f1a2ac1"
        );
    }

    #[test]
    fn should_derive_public_key_from_seed2() {
        let seed = hex::decode("007596986419e027e65499cc87027a236bf4a78b5e8bd7f675759d73e7a9c799")
            .unwrap();

        let result = derive_public_key(&seed);

        let status = result[0];
        let public_key = &result[1..];

        assert_eq!(status, 0);
        assert_eq!(
            hex::encode(public_key),
            "ff71c6c03ff88adb5ed52c9681de1629a54e702fc14729f6b50d2f0a76f185b3"
        );
    }

    #[test]
    fn should_generate_correct_seal() {
        let seed = b"example seed";
        let input = b"example input";
        let aux_data = b"example aux data";
        let result = generate_seal(seed, input, aux_data);
        let status = result[0];
        let seal = &result[1..];

        assert_eq!(status, 0);
        assert_eq!(
            hex::encode(seal),
            "5a997d4d260d49d2e4e02d3f2aae9a2beeea52e7678be6589694bf83677cb7d85e383d4c699839a21a15f01e44d4d585190372d889110ea192337e4b87c7b419932bf668597b49bb4797f64bcbe843deb96393722cbfcc2c80365b483826531c"
        );
    }

    #[test]
    fn should_verify_correct_seal_and_return_id() {
        let pub_key =
            hex::decode("a777e887df9b783c6734140cdd95f74615bfd083ec8189c98ef5e3892f1a2ac1")
                .unwrap();
        let seal = hex::decode("5a997d4d260d49d2e4e02d3f2aae9a2beeea52e7678be6589694bf83677cb7d85e383d4c699839a21a15f01e44d4d585190372d889110ea192337e4b87c7b419932bf668597b49bb4797f64bcbe843deb96393722cbfcc2c80365b483826531c").unwrap();
        let input = b"example input";
        let aux_data = b"example aux data";

        let result = verify_seal(&pub_key, &seal, input, aux_data);
        let status = result[0];
        let id = &result[1..];

        assert_eq!(status, 0);
        assert_eq!(
            hex::encode(id),
            "5814cea12deefefd92c497453ac7defdcacabce180074926251d8f00e420a841"
        );
    }

    #[test]
    fn should_verify_incorrect_seal_and_return_error() {
        let incorrect_pub_key =
            hex::decode("b777e887df9b783c6734140cdd95f74615bfd083ec8189c98ef5e3892f1a2ac1")
                .unwrap();
        let seal = hex::decode("5a997d4d260d49d2e4e02d3f2aae9a2beeea52e7678be6589694bf83677cb7d85e383d4c699839a21a15f01e44d4d585190372d889110ea192337e4b87c7b419932bf668597b49bb4797f64bcbe843deb96393722cbfcc2c80365b483826531c").unwrap();
        let input = b"example input";
        let aux_data = b"example aux data";

        let result = verify_seal(&incorrect_pub_key, &seal, input, aux_data);
        let status = result[0];
        let id = &result[1..];

        assert_eq!(status, 1);
        assert_eq!(
            hex::encode(id),
            "0000000000000000000000000000000000000000000000000000000000000000"
        );
    }

    #[test]
    fn should_generate_seal_and_verify_it() {
        let seed = hex::decode("007596986419e027e65499cc87027a236bf4a78b5e8bd7f675759d73e7a9c799")
            .unwrap();
        let input = b"test input data";
        let aux_data = b"test auxiliary data";

        let pub_key_result = derive_public_key(&seed);
        assert_eq!(pub_key_result[0], 0);
        let pub_key = &pub_key_result[1..];

        let entropy = vrf_output_hash(&seed, input);
        assert_eq!(entropy[0], 0);

        let seal_result = generate_seal(&seed, input, aux_data);
        assert_eq!(seal_result[0], 0);

        let verify_result = verify_seal(pub_key, &seal_result[1..], input, aux_data);
        assert_eq!(verify_result[0], 0);

        assert_eq!(entropy, verify_result);
    }

    #[test]
    fn should_verify_header_seals() {
        let seed = hex::decode("007596986419e027e65499cc87027a236bf4a78b5e8bd7f675759d73e7a9c799")
            .unwrap();
        let seal_payload = b"seal vrf input";
        let unsealed_header = b"unsealed header bytes";
        let entropy_prefix = b"entropy_prefix:";

        let pub_key_result = derive_public_key(&seed);
        assert_eq!(pub_key_result[0], 0);
        let pub_key = &pub_key_result[1..];

        let seal_result = generate_seal(&seed, seal_payload, unsealed_header);
        assert_eq!(seal_result[0], 0);
        let seal_data = &seal_result[1..];

        let seal_verify = verify_seal(pub_key, seal_data, seal_payload, unsealed_header);
        assert_eq!(seal_verify[0], 0);
        let seal_output = &seal_verify[1..];

        let mut entropy_payload = Vec::with_capacity(entropy_prefix.len() + seal_output.len());
        entropy_payload.extend_from_slice(entropy_prefix);
        entropy_payload.extend_from_slice(seal_output);

        let entropy_result = generate_seal(&seed, &entropy_payload, &[]);
        assert_eq!(entropy_result[0], 0);
        let entropy_data = &entropy_result[1..];

        let result = verify_header_seals(
            pub_key,
            seal_data,
            seal_payload,
            unsealed_header,
            entropy_data,
            entropy_prefix,
        );

        assert_eq!(result[0], 0, "verify_header_seals should succeed");
        assert_eq!(result.len(), 65, "result should be 1 + 32 + 32 bytes");

        let returned_seal = &result[1..33];
        assert_eq!(returned_seal, seal_output);

        let entropy_verify = verify_seal(pub_key, entropy_data, &entropy_payload, &[]);
        assert_eq!(entropy_verify[0], 0);
        let expected_entropy = &entropy_verify[1..];
        let returned_entropy = &result[33..65];
        assert_eq!(returned_entropy, expected_entropy);
    }

    #[test]
    fn should_fail_verify_header_seals_with_invalid_seal() {
        let seed = hex::decode("007596986419e027e65499cc87027a236bf4a78b5e8bd7f675759d73e7a9c799")
            .unwrap();
        let seal_payload = b"seal vrf input";
        let unsealed_header = b"unsealed header bytes";
        let entropy_prefix = b"entropy_prefix:";

        let pub_key_result = derive_public_key(&seed);
        assert_eq!(pub_key_result[0], 0);
        let pub_key = &pub_key_result[1..];

        let seal_with_wrong_aux = generate_seal(&seed, seal_payload, b"wrong aux data");
        assert_eq!(seal_with_wrong_aux[0], 0);
        let seal_data = &seal_with_wrong_aux[1..];

        let entropy_result = generate_seal(&seed, b"dummy", &[]);
        assert_eq!(entropy_result[0], 0);
        let entropy_data = &entropy_result[1..];

        let result = verify_header_seals(
            pub_key,
            seal_data,
            seal_payload,
            unsealed_header,
            entropy_data,
            entropy_prefix,
        );

        assert_eq!(result[0], 1, "verify_header_seals should fail");
        assert_eq!(result.len(), 65, "result should still be 65 bytes");
        assert_eq!(&result[1..], &[0u8; 64], "output should be zeros on error");
    }

    #[test]
    fn should_fail_verify_header_seals_with_invalid_entropy() {
        let seed = hex::decode("007596986419e027e65499cc87027a236bf4a78b5e8bd7f675759d73e7a9c799")
            .unwrap();
        let seal_payload = b"seal vrf input";
        let unsealed_header = b"unsealed header bytes";
        let entropy_prefix = b"entropy_prefix:";

        let pub_key_result = derive_public_key(&seed);
        assert_eq!(pub_key_result[0], 0);
        let pub_key = &pub_key_result[1..];

        let seal_result = generate_seal(&seed, seal_payload, unsealed_header);
        assert_eq!(seal_result[0], 0);
        let seal_data = &seal_result[1..];

        let entropy_with_wrong_input = generate_seal(&seed, b"completely wrong input", &[]);
        assert_eq!(entropy_with_wrong_input[0], 0);
        let entropy_data = &entropy_with_wrong_input[1..];

        let result = verify_header_seals(
            pub_key,
            seal_data,
            seal_payload,
            unsealed_header,
            entropy_data,
            entropy_prefix,
        );

        assert_eq!(result[0], 1, "verify_header_seals should fail on invalid entropy");
        assert_eq!(result.len(), 65, "result should still be 65 bytes");
        assert_eq!(&result[1..], &[0u8; 64], "output should be zeros on error");
    }
}
