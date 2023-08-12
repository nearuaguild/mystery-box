use crate::types::BoxType;
use near_sdk::env;

pub fn get_media_for_box_type(box_type: &BoxType) -> Option<String> {
    match box_type {
        // TODO: set actual ipfs link
        BoxType::Common => None,
        BoxType::Rare => None,
        BoxType::Legendary => None,
    }
}

pub fn get_random_number(shift_amount: usize) -> u128 {
    let mut seed = env::random_seed();
    let seed_len = seed.len();

    let mut arr: [u8; 16] = Default::default();

    seed.rotate_left(shift_amount % seed_len);
    arr.copy_from_slice(&seed[..16]);

    u128::from_le_bytes(arr)
}
