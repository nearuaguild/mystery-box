
use crate::contract::json::JsonBox;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{require, AccountId, Balance};

use super::enums::{BoxRarity, BoxStatus};

pub type TokenId = String;

pub type BoxId = u128;

pub type QuestTitle = String;


#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub struct QuestBoxData {
    pub box_id: BoxId,
    pub box_rarity: BoxRarity,
    pub box_status: BoxStatus,
    pub quest_id: QuestId,
    pub owner_id: AccountId,
}

pub type QuestId = u64;
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

impl QuestBoxData {
    pub fn new(quest_id: QuestId, box_id: BoxId, rarity: BoxRarity, owner_id: AccountId) -> Self {
        Self {
            quest_id,
            box_id,
            box_rarity: rarity,
            owner_id,
            box_status: BoxStatus::NonClaimed,
        }
    }

    pub fn ipfs(&self) -> String {
        self.box_rarity.to_media_ipfs()
    }
}

impl From<QuestBoxData> for JsonBox {
    fn from(value: QuestBoxData) -> Self {
        Self {
            quest_id: value.quest_id,
            box_id: value.box_id,
            ipfs: value.ipfs(),
            rarity: value.box_rarity,
            status: value.box_status.into(),
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde", rename_all = "snake_case")]
pub struct Probability {
    pub numerator: u8,
    pub denominator: u8,
}

impl Probability {
    pub const ZERO: Probability = Probability {
        numerator: 0,
        denominator: 1,
    };

    pub const ONE: Probability = Probability {
        numerator: 1,
        denominator: 1,
    };

    pub fn assert_valid(&self) {
        require!(self.denominator != 0, "Denominator can't be zero");

        require!(
            self.denominator >= self.numerator,
            "Denominator must be bigger than or equal to numerator"
        );
    }

    pub fn calculate_threshold(&self) -> u8 {
        ((u8::MAX as u16) * (self.numerator as u16) / (self.denominator as u16)) as u8
    }
}

#[cfg(test)]
mod tests {
    use crate::contract::types::Probability;

    #[test]
    fn test_probability_threshold() {
        let probability = Probability::ONE;

        assert_eq!(probability.calculate_threshold(), 255);

        let probability = Probability::ZERO;

        assert_eq!(probability.calculate_threshold(), 0);

        let probability = Probability {
            numerator: 234,
            denominator: 255,
        };

        assert_eq!(probability.calculate_threshold(), 234);

        let probability = Probability {
            numerator: 2,
            denominator: 3,
        };

        assert_eq!(probability.calculate_threshold(), 170);

        let probability = Probability {
            numerator: 1,
            denominator: 2,
        };

        assert_eq!(probability.calculate_threshold(), 127);
    }
}
