#[cfg(test)]
mod tests {
    use crate::{
        RingSize, batch_generate_ring_vrf_impl, compute_ring_commitment, compute_vrf_output_hash,
        derive_public_key_from_seed, deserialize_public_key, generate_ietf_seal,
        verify_header_seals_impl, verify_seal_impl,
    };

    #[test]
    fn should_get_ring_commitment() {
        let keys_str = "ff71c6c03ff88adb5ed52c9681de1629a54e702fc14729f6b50d2f0a76f185b3dee6d555b82024f1ccf8a1e37e60fa60fd40b1958c4bb3006af78647950e1b919326edb21e5541717fde24ec085000b28709847b8aab1ac51f84e94b37ca1b660746846d17469fb2f95ef365efcab9f4e22fa1feb53111c995376be8019981cc151e5c8fe2b9d8a606966a79edd2f9e5db47e83947ce368ccba53bf6ba20a40b2105650944fcd101621fd5bb3124c9fd191d114b7ad936c1d79d734f9f21392e";
        let keys = hex::decode(keys_str).unwrap();

        let public_keys: Vec<_> = keys.chunks(32).map(deserialize_public_key).collect();

        let start = std::time::Instant::now();
        let commitment = compute_ring_commitment(&public_keys, RingSize::Tiny).unwrap();
        let duration1 = start.elapsed();
        println!("First call took: {:?}", duration1);

        let start = std::time::Instant::now();
        let commitment2 = compute_ring_commitment(&public_keys, RingSize::Tiny).unwrap();
        let duration2 = start.elapsed();
        println!("Second call took: {:?}", duration2);

        println!("Difference: {:?}", duration1.saturating_sub(duration2));

        let mut expected = vec![0u8];
        expected.extend_from_slice(&commitment);
        assert_eq!(
            hex::encode(&expected),
            "00af39b7de5fcfb9fb8a46b1645310529ce7d08af7301d9758249da4724ec698eb127f489b58e49ae9ab85027509116962a135fc4d97b66fbbed1d3df88cd7bf5cc6e5d7391d261a4b552246648defcb64ad440d61d69ec61b5473506a48d58e1992e630ae2b14e758ab0960e372172203f4c9a41777dadd529971d7ab9d23ab29fe0e9c85ec450505dde7f5ac038274cf"
        );

        assert_eq!(hex::encode(&commitment), hex::encode(&commitment2));
    }

    #[test]
    fn should_derive_public_key_from_seed() {
        let seed = b"example seed";

        let public_key = derive_public_key_from_seed(seed).unwrap();

        assert_eq!(
            hex::encode(&public_key),
            "a777e887df9b783c6734140cdd95f74615bfd083ec8189c98ef5e3892f1a2ac1"
        );
    }

    #[test]
    fn should_derive_public_key_from_seed2() {
        let seed = hex::decode("007596986419e027e65499cc87027a236bf4a78b5e8bd7f675759d73e7a9c799")
            .unwrap();

        let public_key = derive_public_key_from_seed(&seed).unwrap();

        assert_eq!(
            hex::encode(&public_key),
            "ff71c6c03ff88adb5ed52c9681de1629a54e702fc14729f6b50d2f0a76f185b3"
        );
    }

    #[test]
    fn should_generate_correct_seal() {
        let seed = b"example seed";
        let input = b"example input";
        let aux_data = b"example aux data";
        let seal = generate_ietf_seal(seed, input, aux_data).unwrap();

        assert_eq!(
            hex::encode(&seal),
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

        let id = verify_seal_impl(&pub_key, &seal, input, aux_data).unwrap();

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

        let result = verify_seal_impl(&incorrect_pub_key, &seal, input, aux_data);
        assert!(result.is_err());
    }

    #[test]
    fn should_generate_seal_and_verify_it() {
        let seed = hex::decode("007596986419e027e65499cc87027a236bf4a78b5e8bd7f675759d73e7a9c799")
            .unwrap();
        let input = b"test input data";
        let aux_data = b"test auxiliary data";

        let pub_key = derive_public_key_from_seed(&seed).unwrap();
        let entropy = compute_vrf_output_hash(&seed, input).unwrap();
        let seal = generate_ietf_seal(&seed, input, aux_data).unwrap();
        let verify_result = verify_seal_impl(&pub_key, &seal, input, aux_data).unwrap();

        assert_eq!(entropy, verify_result);
    }

    #[test]
    fn should_verify_header_seals() {
        let seed = hex::decode("007596986419e027e65499cc87027a236bf4a78b5e8bd7f675759d73e7a9c799")
            .unwrap();
        let seal_payload = b"seal vrf input";
        let unsealed_header = b"unsealed header bytes";
        let entropy_prefix = b"entropy_prefix:";

        let pub_key = derive_public_key_from_seed(&seed).unwrap();
        let seal_data = generate_ietf_seal(&seed, seal_payload, unsealed_header).unwrap();
        let seal_output =
            verify_seal_impl(&pub_key, &seal_data, seal_payload, unsealed_header).unwrap();

        let mut entropy_payload = Vec::with_capacity(entropy_prefix.len() + seal_output.len());
        entropy_payload.extend_from_slice(entropy_prefix);
        entropy_payload.extend_from_slice(&seal_output);

        let entropy_data = generate_ietf_seal(&seed, &entropy_payload, &[]).unwrap();

        let (returned_seal, returned_entropy) = verify_header_seals_impl(
            &pub_key,
            &seal_data,
            seal_payload,
            unsealed_header,
            &entropy_data,
            entropy_prefix,
        )
        .unwrap();

        assert_eq!(returned_seal, seal_output);

        let expected_entropy =
            verify_seal_impl(&pub_key, &entropy_data, &entropy_payload, &[]).unwrap();
        assert_eq!(returned_entropy, expected_entropy);
    }

    #[test]
    fn should_fail_verify_header_seals_with_invalid_seal() {
        let seed = hex::decode("007596986419e027e65499cc87027a236bf4a78b5e8bd7f675759d73e7a9c799")
            .unwrap();
        let seal_payload = b"seal vrf input";
        let unsealed_header = b"unsealed header bytes";
        let entropy_prefix = b"entropy_prefix:";

        let pub_key = derive_public_key_from_seed(&seed).unwrap();
        let seal_with_wrong_aux =
            generate_ietf_seal(&seed, seal_payload, b"wrong aux data").unwrap();
        let entropy_data = generate_ietf_seal(&seed, b"dummy", &[]).unwrap();

        let result = verify_header_seals_impl(
            &pub_key,
            &seal_with_wrong_aux,
            seal_payload,
            unsealed_header,
            &entropy_data,
            entropy_prefix,
        );

        assert!(result.is_err());
    }

    #[test]
    fn should_fail_verify_header_seals_with_invalid_entropy() {
        let seed = hex::decode("007596986419e027e65499cc87027a236bf4a78b5e8bd7f675759d73e7a9c799")
            .unwrap();
        let seal_payload = b"seal vrf input";
        let unsealed_header = b"unsealed header bytes";
        let entropy_prefix = b"entropy_prefix:";

        let pub_key = derive_public_key_from_seed(&seed).unwrap();
        let seal_data = generate_ietf_seal(&seed, seal_payload, unsealed_header).unwrap();
        let entropy_with_wrong_input =
            generate_ietf_seal(&seed, b"completely wrong input", &[]).unwrap();

        let result = verify_header_seals_impl(
            &pub_key,
            &seal_data,
            seal_payload,
            unsealed_header,
            &entropy_with_wrong_input,
            entropy_prefix,
        );

        assert!(result.is_err());
    }

    /// Helper: create a ring of `size` keys, returning (seeds, public_keys).
    fn make_ring(size: usize) -> (Vec<Vec<u8>>, Vec<crate::bandersnatch::Public>) {
        let seeds: Vec<Vec<u8>> = (0..size).map(|i| i.to_le_bytes().to_vec()).collect();
        let public_keys: Vec<_> = seeds
            .iter()
            .map(|s| {
                let pk_bytes = derive_public_key_from_seed(s).unwrap();
                deserialize_public_key(&pk_bytes)
            })
            .collect();
        (seeds, public_keys)
    }

    #[test]
    fn should_batch_generate_and_batch_verify() {
        let (seeds, public_keys) = make_ring(RingSize::Tiny.size());
        let prover_index = 1;
        let input_len = 36;
        let num_inputs = 3u32;

        // Build concatenated inputs
        let mut inputs_data = Vec::new();
        for attempt in 0..num_inputs {
            inputs_data.extend_from_slice(&[0xCD; 32]);
            inputs_data.extend_from_slice(&attempt.to_le_bytes());
        }

        let results = batch_generate_ring_vrf_impl(
            &public_keys,
            prover_index,
            &seeds[prover_index],
            &inputs_data,
            input_len,
        );

        assert_eq!(results.len(), num_inputs as usize);

        let commitment_bytes = compute_ring_commitment(&public_keys, RingSize::Tiny).unwrap();

        // Build verify input: signature || vrf_input per item
        let mut verify_data = Vec::new();
        for (i, result) in results.iter().enumerate() {
            let signature = result.as_ref().unwrap();
            verify_data.extend_from_slice(signature);
            verify_data.extend_from_slice(&inputs_data[i * input_len..(i + 1) * input_len]);
        }

        let verify_results = crate::batch_verify_tickets_impl(
            RingSize::Tiny,
            &commitment_bytes,
            &verify_data,
            input_len,
        );

        assert_eq!(verify_results.len(), num_inputs as usize);
        for (i, verify_result) in verify_results.iter().enumerate() {
            let verified_hash = verify_result.as_ref().expect("verification should succeed");
            let vrf_input = &inputs_data[i * input_len..(i + 1) * input_len];
            let expected = compute_vrf_output_hash(&seeds[prover_index], vrf_input).unwrap();
            assert_eq!(*verified_hash, expected);
        }
    }
}
