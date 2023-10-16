use near_contract_standards::non_fungible_token::TokenId;
use near_sdk::{
    json_types::U128,
    require,
    serde::{Deserialize, Serialize},
    AccountId,
};

use crate::{
    reward_pools::Pool,
    types::{BoxData, BoxId, BoxRarity, BoxStatus, Capacity, Reward},
};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(crate = "near_sdk::serde", tag = "kind", rename_all = "snake_case")]
pub enum JsonPoolRewards {
    Near {
        amount: U128,
        capacity: Capacity,
    },
    NonFungibleToken {
        contract_id: AccountId,
        token_id: Vec<TokenId>,
    },
}

impl Into<JsonPoolRewards> for &Pool {
    fn into(self) -> JsonPoolRewards {
        match self {
            Pool::Near(pool) => JsonPoolRewards::Near {
                amount: pool.amount.to_owned().into(),
                capacity: pool.available.clone(),
            },
            Pool::NonFungibleToken(pool) => JsonPoolRewards::NonFungibleToken {
                contract_id: pool.contract_id.clone(),
                token_id: pool.available_tokens.clone(),
            },
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(crate = "near_sdk::serde", tag = "kind", rename_all = "snake_case")]
pub enum JsonReward {
    Near {
        amount: U128,
    },
    NonFungibleToken {
        contract_id: AccountId,
        token_id: TokenId,
    },
    Nothing,
}

impl Into<JsonReward> for Reward {
    fn into(self) -> JsonReward {
        match self {
            Reward::Near { amount } => JsonReward::Near {
                amount: amount.into(),
            },
            Reward::NonFungibleToken {
                contract_id,
                token_id,
            } => JsonReward::NonFungibleToken {
                contract_id,
                token_id,
            },
        }
    }
}
impl Into<JsonReward> for Option<Reward> {
    fn into(self) -> JsonReward {
        match self {
            Option::None => JsonReward::Nothing,
            Option::Some(reward) => reward.into(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(crate = "near_sdk::serde", tag = "kind", rename_all = "snake_case")]
pub enum JsonBoxStatus {
    Claimed { reward: JsonReward },
    NonClaimed { token_id: TokenId },
}

impl Into<JsonBoxStatus> for BoxStatus {
    fn into(self) -> JsonBoxStatus {
        match self {
            BoxStatus::Claimed { reward } => JsonBoxStatus::Claimed {
                reward: reward.into(),
            },
            BoxStatus::NonClaimed { token_id } => JsonBoxStatus::NonClaimed { token_id },
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct JsonBox {
    pub id: BoxId,
    pub ipfs: String,
    pub rarity: BoxRarity,
    pub status: JsonBoxStatus,
}

impl Into<JsonBox> for &BoxData {
    fn into(self) -> JsonBox {
        JsonBox {
            id: self.id.clone(),
            ipfs: self.ipfs(),
            rarity: self.rarity.clone(),
            status: self.status.to_owned().into(),
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
