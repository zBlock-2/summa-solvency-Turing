#![no_main]
use halo2_proofs::halo2curves::bn256::Fr as Fp;
use libfuzzer_sys::fuzz_target;
use num_bigint::BigUint;
use summa_solvency::merkle_sum_tree;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        // Call the function you wish to fuzz
        let _ = merkle_sum_tree::utils::big_intify_username(&s);
    }
    let b = BigUint::from_bytes_be(data);
    // Call the function you wish to fuzz
    let _ = merkle_sum_tree::utils::big_uint_to_fp(&b);

    let mut fixed_size_array = [0u8; 32];
    for (i, &byte) in data.iter().enumerate().take(32) {
        fixed_size_array[i] = byte;
    }
    let f = Fp::from_bytes(&fixed_size_array);

    if let Some(f) = f.into() {
        // Call the function you wish to fuzz
        let _ = merkle_sum_tree::utils::fp_to_big_uint(f);
    }
});
