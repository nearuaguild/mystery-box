use near_sdk::AccountId;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};

use super::QuestId;

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub struct QuestData {
    pub title: String,
    pub quest_id: QuestId,
    pub owner_id: AccountId,
}

impl QuestData {
    pub fn new(quest_id: QuestId, title: String, owner_id: AccountId) -> Self {
        Self {
            quest_id,
            title,
            owner_id,
        }
    }
}