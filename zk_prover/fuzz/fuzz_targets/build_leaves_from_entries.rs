#![no_main]
use halo2_proofs::halo2curves::bn256::Fr as Fp;
use libfuzzer_sys::fuzz_target;
use num_bigint::BigUint;
use summa_solvency::merkle_sum_tree;

const N_CURRENCIES: usize = 2;
const N_BYTES: usize = 8;

fuzz_target!(|data: &[u8]| {
    let username = std::str::from_utf8(data).unwrap();
    let username_as_big_uint = merkle_sum_tree::utils::big_intify_username(username);
    let mut balances = [BigUint::from(0u32); N_CURRENCIES];
    if (data.len() > 2) {
        balances[0] = BigUint::from_bytes_be(&data[0]);
        balances[1] = BigUint::from_bytes_be(&data[1]);
    }
});
