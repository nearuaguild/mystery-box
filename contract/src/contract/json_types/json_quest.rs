use near_sdk::serde::{Deserialize, Serialize};

use crate::contract::types::QuestId;

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct JsonQuest {
    pub quest_id: QuestId,
    pub title: String,
}

impl JsonQuest {
    pub fn new(
        quest_id: QuestId,
        title: String,
    ) -> Self {
        Self {
            quest_id,
            title
        }
    }
}