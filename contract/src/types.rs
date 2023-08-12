use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::require;
use near_sdk::serde::{Deserialize, Serialize};

use crate::utils::get_random_number;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(crate = "near_sdk::serde", tag = "box_type", rename_all = "snake_case")]
pub enum BoxType {
    Legendary,
    Rare,
    Common,
}

pub type RewardId = u32;

#[derive(BorshDeserialize, BorshSerialize, Clone, Copy)]
pub struct Reward {
    pub id: RewardId,
    pub token: RewardToken,
    pub bounds: Bounds,
}

impl Reward {
    pub fn generate_random_amount_within_bounds(&self, shift_amount: usize) -> u128 {
        let seed = get_random_number(shift_amount);

        let range = self.bounds.max - self.bounds.min + 1;

        // Convert the random seed to a value [0, max - min]
        let random_in_range = seed % range;

        self.bounds.min + random_in_range
    }
}

#[derive(
    BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Copy, Debug, PartialEq,
)]
#[serde(crate = "near_sdk::serde", tag = "reward_token")]
pub enum RewardToken {
    Near,
    // Fungible { contract: AccountId },
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Copy)]
#[serde(crate = "near_sdk::serde")]
pub struct Bounds {
    pub min: u128,
    pub max: u128,
}

impl Bounds {
    pub fn assert_valid(&self) {
        require!(self.min > 0, "Min bound can't be zero");
        require!(self.max > 0, "Max bound can't be zero");

        require!(self.min < self.max, "Min bound can't be bigger than max");
    }
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Copy)]
#[serde(crate = "near_sdk::serde")]
pub struct InputReward {
    pub bounds: Bounds,
    pub supply: u32,
}

impl InputReward {
    pub fn assert_valid(&self) {
        self.bounds.assert_valid();

        // supply can be uint from [1, 20] range
        require!(self.supply >= 1, "Supply must be an uint in range [1,20]");
        require!(self.supply <= 20, "Supply must be an uint in range [1,20]");
    }
}

#[derive(
    BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Copy, Debug, PartialEq,
)]
#[serde(crate = "near_sdk::serde")]
pub enum State {
    Created,
    RegistrationStarted,
    RegistrationEnded,
    ClaimStarted,
}
