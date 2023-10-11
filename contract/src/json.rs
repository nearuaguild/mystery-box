use near_sdk::{
    require,
    serde::{Deserialize, Serialize},
};

use crate::types::{BoxData, BoxId, BoxRarity, BoxStatus, Capacity, Reward, RewardPool};

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct JsonReward {
    pub reward: Reward,
    pub box_rarity: BoxRarity,
    pub available: Capacity,
}

impl JsonReward {
    pub fn from(reward_pool: &RewardPool) -> Self {
        Self {
            reward: reward_pool.reward.clone(),
            box_rarity: reward_pool.box_rarity.clone(),
            available: reward_pool.available_capacity.clone(),
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct JsonBoxData {
    pub box_id: BoxId,
    pub ipfs: String,
    pub rarity: BoxRarity,
    pub status: BoxStatus,
}

impl JsonBoxData {
    pub fn from(box_data: &BoxData) -> Self {
        Self {
            box_id: box_data.id.clone(),
            ipfs: box_data.rarity.to_media_ipfs(),
            rarity: box_data.rarity.clone(),
            status: box_data.status.clone(),
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Pagination {
    page: u8,
    size: u8,
}

impl Default for Pagination {
    fn default() -> Self {
        Self { page: 1, size: 10 }
    }
}

impl Pagination {
    pub fn assert_valid(&self) {
        require!(
            self.size <= 10,
            "A single page can't contain more than 10 elements"
        )
    }

    pub fn calculate_limit(&self) -> usize {
        self.size.into()
    }

    pub fn calculate_offset(&self) -> usize {
        (self.size * (self.page - 1)).into()
    }
}
