use near_sdk::serde::{Deserialize, Serialize};

use crate::contract::{enums::{BoxRarity, BoxStatus}, types::{BoxId, QuestId}};

use super::json_reward::JsonReward;


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(crate = "near_sdk::serde", tag = "kind", rename_all = "snake_case")]
pub enum JsonBoxStatus {
    Claimed {
        reward: JsonReward,
    },
    NonClaimed,
}

impl Into<JsonBoxStatus> for BoxStatus {
    fn into(self) -> JsonBoxStatus {
        match self {
            BoxStatus::Claimed { reward } =>
                JsonBoxStatus::Claimed {
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
    pub box_rarity: BoxRarity,
    pub box_status: JsonBoxStatus,
    pub ipfs: String,
}

impl JsonBox {
    pub fn new(
        quest_id: QuestId,
        box_id: BoxId,
        box_rarity: BoxRarity,
        box_status: JsonBoxStatus
    ) -> Self {
        Self {
            quest_id,
            box_id,
            box_rarity,
            box_status,
            ipfs: box_rarity.to_media_ipfs(),
        }
    }
}