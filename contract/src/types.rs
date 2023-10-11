use near_contract_standards::non_fungible_token::TokenId;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{require, AccountId, Balance};

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(crate = "near_sdk::serde", rename_all = "snake_case")]
pub enum BoxRarity {
    Rare,
    Epic,
    Legendary,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(crate = "near_sdk::serde", tag = "kind", rename_all = "snake_case")]
pub enum BoxStatus {
    Claimed { reward: Reward },
    NonClaimed { token_id: TokenId },
}

pub type BoxId = u64;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct BoxData {
    pub id: BoxId,
    pub rarity: BoxRarity,
    pub status: BoxStatus,
    pub owner_id: AccountId,
}

pub type RewardPoolId = u16;
pub type Capacity = u64;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(crate = "near_sdk::serde", tag = "kind")]
pub enum Reward {
    Near {
        amount: Balance,
    },
    NonFungibleToken {
        contract_id: AccountId,
        token_id: TokenId,
    },
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub struct RewardPool {
    pub id: RewardPoolId,
    pub reward: Reward,
    pub box_rarity: BoxRarity,
    pub capacity: Capacity,
    pub available_capacity: Capacity,
}

/// Implementation
impl BoxRarity {
    pub fn to_media_ipfs(&self) -> String {
        match *self {
            BoxRarity::Rare => {
                String::from("bafkreibwmkcer2kp3kv67cydzhzzvzki7hdph5f4w7jeiep2r4s5dp7eb4")
            }
            BoxRarity::Epic => {
                String::from("bafkreick7sjo4uzdy3sznvqjuafcds6f5p37apkggvvwkctptdy3qu2vbi")
            }
            BoxRarity::Legendary => {
                String::from("bafkreigdv4mnfrndcob64wrwbqoqce257v7bvtxp2flnyqg2onukpssyoq")
            }
        }
    }
}

impl BoxData {
    pub fn from(id: BoxId, rarity: BoxRarity, owner_id: AccountId, token_id: TokenId) -> Self {
        Self {
            id,
            rarity,
            owner_id,
            status: BoxStatus::NonClaimed { token_id },
        }
    }

    pub fn ipfs(&self) -> String {
        self.rarity.to_media_ipfs()
    }

    pub fn claim(&mut self, reward: Reward) {
        self.status = BoxStatus::Claimed { reward };
    }

    pub fn revert_claim(&mut self, token_id: TokenId) {
        self.status = BoxStatus::NonClaimed { token_id };
    }
}

impl RewardPool {
    pub fn from(
        id: RewardPoolId,
        reward: Reward,
        capacity: Capacity,
        box_rarity: BoxRarity,
    ) -> Self {
        Self {
            id,
            reward,
            capacity,
            available_capacity: capacity,
            box_rarity,
        }
    }

    pub fn assert_valid(&self) {
        require!(self.capacity > 0, "Maximum reward capacity can't be zero");
    }

    pub fn increment_availability(&mut self) {
        self.available_capacity += 1;
    }

    pub fn decrement_availability(&mut self) {
        self.available_capacity -= 1;
    }

    pub fn is_empty(&self) -> bool {
        self.available_capacity == 0
    }
}
