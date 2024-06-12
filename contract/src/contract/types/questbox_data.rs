use near_sdk::AccountId;

use crate::contract::{enums::{BoxRarity, BoxStatus}, json::JsonBox};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};

use super::{BoxId, QuestId};

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub struct QuestBoxData {
    pub box_id: BoxId,
    pub box_rarity: BoxRarity,
    pub box_status: BoxStatus,
    pub quest_id: QuestId,
    pub owner_id: AccountId,
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