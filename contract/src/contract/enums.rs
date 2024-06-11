use std::fmt::{Display, Formatter, Result};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{AccountId, BorshStorageKey};

use super::types::Reward;

#[derive(
    BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Debug, PartialEq, Copy,
)]
#[serde(crate = "near_sdk::serde", rename_all = "snake_case")]
pub enum BoxRarity {
    Rare,
    Epic,
    Legendary,
}

impl Display for BoxRarity {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            BoxRarity::Rare => write!(f, "rare"),
            BoxRarity::Epic => write!(f, "epic"),
            BoxRarity::Legendary => write!(f, "legendary"),
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
pub enum BoxStatus {
    Claimed { reward: Option<Reward> },
    NonClaimed,
}

pub enum Network {
    Mainnet,
    Testnet,
}

impl From<AccountId> for Network {
    fn from(account_id: AccountId) -> Self {
        if account_id.to_string().ends_with(".near") {
            return Network::Mainnet;
        }

        return Network::Testnet;
    }
}

#[derive(BorshStorageKey, BorshSerialize)]
pub enum StorageKey {
    Pools,
    PoolsByRarity,
    NftPoolByKey,
    Boxes,
    BoxesPerOwner,
    TrustedNftContracts,
    ProbabilityByRarity,
    Users,
    Quests,
    QuestsPerOwner,
    QuestBoxesPerOwner,
}