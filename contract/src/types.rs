use std::fmt::{Display, Formatter, Result};

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{AccountId, Balance};

use crate::*;

pub type TokenId = String;

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

pub type BoxId = u128;

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct BoxData {
    pub id: BoxId,
    pub rarity: BoxRarity,
    pub status: BoxStatus,
    pub owner_id: AccountId,
}

pub type PoolId = u32;
pub type Capacity = u64;

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde", rename_all = "snake_case")]
pub enum Reward {
    Near {
        amount: Balance,
    },
    NonFungibleToken {
        contract_id: AccountId,
        token_id: TokenId,
    },
}

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
    pub fn new(id: BoxId, rarity: BoxRarity, owner_id: AccountId) -> Self {
        Self {
            id,
            rarity,
            owner_id,
            status: BoxStatus::NonClaimed,
        }
    }

    pub fn ipfs(&self) -> String {
        self.rarity.to_media_ipfs()
    }
}

impl From<BoxData> for JsonBox {
    fn from(value: BoxData) -> Self {
        Self {
            id: value.id,
            ipfs: value.ipfs(),
            rarity: value.rarity,
            status: value.status.into(),
        }
    }
}
