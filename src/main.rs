use std::collections::HashMap;
use std::time::Instant;

use num_bigint::{BigUint, RandomBits};
use rand::Rng;

fn key_space(exp: u32) -> BigUint {
    BigUint::from(2_u32).pow(exp)
}

fn generate_key(length: u64) -> BigUint {
    let mut rng = rand::thread_rng();
    let key: BigUint = rng.sample(RandomBits::new(length));
    key
}

fn find_same_key(length: u64, key: &BigUint) {
    let mut found_key = BigUint::default();
    let start = Instant::now();
    while key != &found_key {
        found_key = generate_key(length);
    }
    println!(
        "For {} bits, key has been found in {:?}ms",
        length,
        start.elapsed().as_millis()
    );
}

fn main() {
    let lengths: Vec<u32> = vec![8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096];
    let mut initial_keys: HashMap<u32, BigUint> = HashMap::new();
    for length in lengths.iter() {
        println!(
            "Key space for {} bits is {}",
            length,
            key_space(length.clone())
        );
        let key = generate_key(length.clone() as u64);
        initial_keys.insert(length.clone(), key.clone());
        println!(
            "Initial key for {length} bits is {key}\nHex 0x{key:x}\n",
            length = length,
            key = key,
        );
    }
    for length in lengths {
        find_same_key(length as u64, initial_keys.get(&length).unwrap());
    }
}
