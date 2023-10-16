use std::collections::HashSet;

use crate::types::{BoxData, BoxId, BoxRarity, Reward};
use near_contract_standards::non_fungible_token::TokenId;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::{AccountId, IntoStorageKey};

#[derive(BorshDeserialize, BorshSerialize)]
pub struct MysteryBoxContainer {
    next_box_id: BoxId,
    boxes: LookupMap<BoxId, BoxData>,
    user_boxes: LookupMap<AccountId, HashSet<BoxId>>,
}

impl MysteryBoxContainer {
    pub fn new<Q, R>(boxes_prefix: Q, user_boxes_prefix: R) -> Self
    where
        Q: IntoStorageKey,
        R: IntoStorageKey,
    {
        Self {
            next_box_id: 1,
            boxes: LookupMap::new(boxes_prefix),
            user_boxes: LookupMap::new(user_boxes_prefix),
        }
    }

    pub fn add_box(
        &mut self,
        rarity: BoxRarity,
        owner_id: AccountId,
        token_id: TokenId,
    ) -> BoxData {
        let box_id = self.next_box_id.clone();

        self.next_box_id += 1;

        let box_data = BoxData::from(box_id, rarity, owner_id, token_id);

        self.boxes.insert(&box_data.id, &box_data);

        let mut existing_user_boxes = self.user_boxes.get(&box_data.owner_id).unwrap_or_default();
        existing_user_boxes.insert(box_data.id);
        self.user_boxes
            .insert(&box_data.owner_id, &existing_user_boxes);

        box_data
    }

    fn get_box(&self, box_id: &BoxId) -> BoxData {
        self.boxes
            .get(box_id)
            .expect(format!("Box {} wasn't found", box_id).as_str())
    }

    pub fn get_box_rarity(&self, box_id: &BoxId) -> BoxRarity {
        self.get_box(box_id).rarity
    }

    pub fn claim_box(&mut self, box_id: BoxId, reward: Option<Reward>) {
        let mut box_data = self.get_box(&box_id);

        box_data.claim(reward);

        self.boxes.insert(&box_data.id, &box_data);
    }

    pub fn revert_claim_box(&mut self, box_id: BoxId, token_id: TokenId) {
        let mut box_data = self.get_box(&box_id);

        box_data.revert_claim(token_id);

        self.boxes.insert(&box_data.id, &box_data);
    }

    pub fn get_user_boxes(&self, owner_id: AccountId, take: usize, skip: usize) -> Vec<BoxData> {
        self.user_boxes
            .get(&owner_id)
            .unwrap_or_default()
            .iter()
            .skip(skip)
            .take(take)
            .map(|box_id| self.get_box(box_id))
            .collect()
    }
}
