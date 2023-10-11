use near_sdk::env;

pub fn get_random_number(shift_amount: usize) -> u64 {
    let mut seed = env::random_seed();
    let seed_len = seed.len();

    let mut arr: [u8; 8] = Default::default();

    seed.rotate_left(shift_amount % seed_len);
    arr.copy_from_slice(&seed[..8]);

    u64::from_le_bytes(arr)
}
