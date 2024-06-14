use near_sdk::{json_types::U128, serde::{Deserialize, Serialize}, AccountId};

use crate::contract::types::{Capacity, Reward, TokenId};


#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(crate = "near_sdk::serde", tag = "kind", rename_all = "snake_case")]
pub enum JsonPoolRewards {
    Near {
        amount: U128,
        available: Capacity,
        total: Capacity,
    },
    NonFungibleToken {
        contract_id: AccountId,
        token_ids: Vec<TokenId>,
        total: Capacity,
    },
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

impl From<Reward> for JsonReward {
    fn from(value: Reward) -> Self {
        match value {
            Reward::Near { amount } =>
                Self::Near {
                    amount: amount.into(),
                },
            Reward::NonFungibleToken { contract_id, token_id } =>
                Self::NonFungibleToken {
                    contract_id,
                    token_id,
                },
        }
    }
}

impl From<Option<Reward>> for JsonReward {
    fn from(value: Option<Reward>) -> Self {
        match value {
            Option::None => Self::Nothing,
            Option::Some(reward) => reward.into(),
        }
    }
}