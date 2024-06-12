use near_sdk::{
    json_types::U128,
    require,
    serde::{Deserialize, Serialize},
    AccountId,
};

use super::{enums::{BoxRarity, BoxStatus}, types::{questbox_data::QuestBoxData, BoxId, Capacity, QuestId, Reward, TokenId}};

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
            Reward::Near { amount } => Self::Near {
                amount: amount.into(),
            },
            Reward::NonFungibleToken {
                contract_id,
                token_id,
            } => Self::NonFungibleToken {
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(crate = "near_sdk::serde", tag = "kind", rename_all = "snake_case")]
pub enum JsonBoxStatus {
    Claimed { reward: JsonReward },
    NonClaimed,
}

impl Into<JsonBoxStatus> for BoxStatus {
    fn into(self) -> JsonBoxStatus {
        match self {
            BoxStatus::Claimed { reward } => JsonBoxStatus::Claimed {
                reward: reward.into(),
            },
            BoxStatus::NonClaimed => JsonBoxStatus::NonClaimed,
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct JsonBox {
    pub quest_id: QuestId,
    pub box_id: BoxId,
    pub ipfs: String,
    pub rarity: BoxRarity,
    pub status: JsonBoxStatus,
}

impl Into<JsonBox> for &QuestBoxData {
    fn into(self) -> JsonBox {
        JsonBox {
            quest_id: self.quest_id.clone(),
            box_id: self.box_id.clone(),
            ipfs: self.ipfs(),
            rarity: self.box_rarity.clone(),
            status: self.box_status.to_owned().into(),
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
            self.size <= 50,
            "A single page can't contain more than 50 elements"
        )
    }

    pub fn take(&self) -> usize {
        self.size.into()
    }

    pub fn skip(&self) -> usize {
        (self.size * (self.page - 1)).into()
    }
}
