use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};

use super::{BoxId, QuestId};

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub struct QuestBoxData {
    pub box_id: BoxId,
    pub quest_id: QuestId,
}

impl QuestBoxData {
    pub fn new(quest_id: QuestId, box_id: BoxId) -> Self {
        Self {
            quest_id,
            box_id,
        }
    }
}