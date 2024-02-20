#![no_main]
use halo2_proofs::halo2curves::bn256::Fr as Fp;
use libfuzzer_sys::fuzz_target;
use num_bigint::BigUint;
use summa_solvency::merkle_sum_tree;

const N_CURRENCIES: usize = 2;
const N_BYTES: usize = 8;

fuzz_target!(|data: &[u8]| {
    // make sure an empty slice doesn't cause a panic
    if data.len() > 0 {
        // get a random number of entries from the first byte
        let num_entries = data[0] as usize;
        // make sure number of entries is not zero
        if num_entries != 0 {
            // create a vector of entries
            let mut entries = Vec::new();
            for i in 0..num_entries {
                // create a new random data slicefor each entry
                let mut curr_data = Vec::new();
                for j in 0..data.len() {
                    curr_data.push((data[j].wrapping_mul((i + 1) as u8)));
                }
                let data_slice: &[u8] = &curr_data;
                // use random data to make an entry
                let mut username = "0";
                if let Ok(s) = std::str::from_utf8(data_slice) {
                    username = s;
                };
                let (data_1, data_2) = data_slice.split_at(data_slice.len() / 2);
                let balances = [
                    BigUint::from_bytes_be(data_1),
                    BigUint::from_bytes_be(data_2),
                ];
                let entry = merkle_sum_tree::Entry::new(username.to_string(), balances).unwrap();
                // push the entry to the vector
                entries.push(entry);
            }
            // call the function with random entries to see if it panics
            let _ = merkle_sum_tree::utils::build_leaves_from_entries(&entries);
        }
    }
});
