
use near_sdk::borsh::{ self, BorshDeserialize, BorshSerialize };
use near_sdk::serde::{ Deserialize, Serialize };
use near_sdk::AccountId;

use super::json::JsonBox;
use super::types::{BoxId, BoxRarity, QuestId, BoxStatus};

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub struct QuestBox {
    pub box_id: BoxId,
    pub box_rarity: BoxRarity,
    pub box_status: BoxStatus,
    pub quest_id: QuestId,
    pub owner_id: AccountId,
}

impl QuestBox {
    pub fn new(quest_id: QuestId, box_id: BoxId, rarity: BoxRarity, box_owner_id: AccountId) -> Self {
        Self {
            quest_id,
            box_id,
            box_rarity: rarity,
            box_status: BoxStatus::NonClaimed,
            owner_id: box_owner_id
        }
    }

    pub fn ipfs(&self) -> String {
        self.box_rarity.to_media_ipfs()
    }
}

impl From<QuestBox> for JsonBox {
    fn from(value: QuestBox) -> Self {
        Self {
            quest_id: value.quest_id,
            box_id: value.box_id,
            ipfs: value.ipfs(),
            box_rarity: value.box_rarity,
            box_status: value.box_status.into(),
        }
    }
}
