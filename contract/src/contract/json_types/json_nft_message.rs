use crate::contract::types::{BoxRarity, QuestId};

use near_sdk::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct NftOnTransferMessage {
    pub rarity: BoxRarity,
    pub quest_id: QuestId,
}