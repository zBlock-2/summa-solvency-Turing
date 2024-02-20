# cargo fuzz

This repository contains the fuzz targets for the summa codebase. Go to the root of the directory and follow thsi path:
`summa/zk_prover/fuzz` and then run `cargo fuzz --help`. If this does not work then follow the setup.

### SetUp
Go to the following directory `summa/zk_prover/fuzz` and run:
```
cargo install cargo fuzz
```
To test a function, create a new fuzz target:
```
cargo fuzz add <fuzz_target_name>`
```
To check all fuzz targets, run:
```
cargo fuzz list
```
To run fuzz tests on a target, run:
```
cargo fuzz run <fuzz_target_name>`
```

### How to write tests

Refer `fuzz_targets/operation_helpers.rs` 

``` rust
fuzz_target!(|data: &[u8]| {
   // code goes here
}
```
This function generates a random bytes array that can be used to pass as inputs in any function you wish to fuzz.
In order to import a function, write: 
``` rust
use summa_solvency::merkle_sum_tree
```
at the top and write the function by providing the specific path

For example:
``` rust
let username_as_big_uint = merkle_sum_tree::utils::big_intify_username(username);
```
