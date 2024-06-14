use enums::{BoxRarity, StorageKey};

use json_types::json_box::JsonBox;
use json_types::json_pagination::Pagination;
use json_types::json_quest::JsonQuest;
use near_sdk::collections::{UnorderedSet, Vector};
use near_sdk::{env, require, Promise, PromiseOrValue, ONE_NEAR};
use near_sdk::json_types::{U128, U64};
use near_sdk::{collections::LookupMap, near_bindgen, AccountId, PanicOnDefault};

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use questbox::QuestBox;
use trusted_contracts::get_trusted_nft_contracts;
use types::questbox_data::QuestBoxData;
use types::{BoxId, Probability, QuestId, TokenId};

use crate::contract::quest::Quest;

pub mod trusted_contracts;
pub mod quest;
pub mod questbox;
pub mod callbacks;
pub mod enumeration;
pub mod internal;
pub mod pools;
pub mod types;
pub mod enums;
pub mod json_types;


const MINIMAL_NEAR_REWARD: u128 = ONE_NEAR / 10; // 0.1N

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    quests: LookupMap<QuestId, Quest>,
    quests_per_owner: LookupMap<AccountId, UnorderedSet<QuestId>>,
    questboxes_per_owner: LookupMap<AccountId, Vector<QuestBoxData>>,
    next_quest_id: QuestId,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new() -> Self {
        Self {
            quests: LookupMap::new(StorageKey::Quests),
            quests_per_owner: LookupMap::new(StorageKey::QuestsPerOwner),
            questboxes_per_owner: LookupMap::new(StorageKey::QuestBoxesPerOwner),
            next_quest_id: 0,
        }
    }

    pub fn quests_per_owner(&self, account_id: AccountId) -> Vec<JsonQuest> {
        let quests_ids = self.quests_per_owner
            .get(&account_id);

        let mut result_vec = Vec::new();

        if !quests_ids.is_some() {
            return result_vec;
        }

        quests_ids
            .unwrap()
            .iter()
            .for_each(|quest_id| {
                let quest = self.quests.get(&quest_id);

                if quest.is_some() {
                    let quest = quest.unwrap();

                    result_vec.push(
                        JsonQuest::new(quest.id, quest.title)
                    );
                }
            });

        return result_vec;
    }

    pub fn questboxes_supply_per_owner(&self, account_id: AccountId) -> u64 {
        let quest_boxes = self.questboxes_per_owner
            .get(&account_id);

        if quest_boxes.is_some()
        {
            return quest_boxes.unwrap().len();
        }

        return 0;
    }

    pub fn questboxes_per_owner(
        &self,
        account_id: AccountId,
        pagination: Option<Pagination>,
    ) -> Vec<JsonBox> {
        let pagination = pagination.unwrap_or_default();

        let questboxes_per_owner = self.questboxes_per_owner
            .get(&account_id);

        if !questboxes_per_owner.is_some() {
            return Vec::new();
        }

        return questboxes_per_owner
            .unwrap()
            .iter()
            .take(pagination.take())
            .skip(pagination.skip())
            .filter_map(|quest_box| 
            {
                let quest = self.quests.get(&quest_box.quest_id);
                
                if !quest.is_some() {
                    return None;
                }

                let box_object = quest.unwrap().boxes.get(&quest_box.box_id);
                
                if !box_object.is_some() {
                    return None;
                }

                let box_object_unwrapped = box_object.unwrap();

                let box_json = JsonBox::from(box_object_unwrapped);

                return Some(box_json);
            })
            .collect();
    }

    #[payable]
    pub fn create_quest(&mut self, title: &String) {
        assert!(
            !title.is_empty(),
            "Title should be specified"
        );

        let account_id = env::predecessor_account_id();
        let storage_used_before = env::storage_usage();

        let default_trusted_nft_contracts = get_trusted_nft_contracts();
        let quest = Quest::new(self.next_quest_id, &title, &account_id, default_trusted_nft_contracts);
        self.next_quest_id += 1;
        
        self.quests.insert(&quest.id, &quest);
        self.insert_quest_into_quests_per_owner(quest);

        let storage_used_after = env::storage_usage();

        let storage_deposit =
            env::storage_byte_cost() * (storage_used_after - storage_used_before) as u128;

        assert!(
            env::attached_deposit() >= storage_deposit,
            "Deposited amount must be equal to {} yocto",
            storage_deposit
        );

        let refund = env::attached_deposit() - storage_deposit;
        if refund > 1 {
            Promise::new(env::predecessor_account_id()).transfer(refund);
        }
    }

    fn insert_quest_into_quests_per_owner(&mut self, quest: Quest){
        let account_id = env::predecessor_account_id();

        let quests_per_owner_unwrapped = self.quests_per_owner.get(&account_id);
        let mut quests_per_owner = UnorderedSet::new(StorageKey::QuestIdsPerOwner);

        if quests_per_owner_unwrapped.is_some() {
            quests_per_owner = quests_per_owner_unwrapped.unwrap();
        }

        quests_per_owner.insert(&quest.id);
        self.quests_per_owner.insert(&account_id, &quests_per_owner);
    }

    pub fn delete_quest(&mut self, quest_id: QuestId){
        assert!(
            quest_id != 0,
            "Title should be specified"
        );

        assert!(
            self.quests.get(&quest_id).is_some(),
            "Quest not found"
        );
        
        self.quests.remove(&quest_id);
        self.delete_quest_from_quests_per_owner(quest_id);
    }

    fn delete_quest_from_quests_per_owner(&mut self, quest_id: QuestId){
        let account_id = env::predecessor_account_id();

        let quests_per_owner_unwrapped = self.quests_per_owner.get(&account_id);

        if !quests_per_owner_unwrapped.is_some() {
            return;
        }

        //quests_per_owner_unwrapped.unwrap().re
        // use HashSet remove method

        let mut quests_per_owner_retained = UnorderedSet::new(StorageKey::QuestIdsPerOwner);
        
        quests_per_owner_unwrapped
            .unwrap()
            .iter()
            //.collect::<Vec<u64>>();
            .for_each(|quest_id_inner| {
                if quest_id_inner != quest_id {
                    quests_per_owner_retained.insert(&quest_id_inner);
                }
            });

        
        self.quests_per_owner.insert(&account_id, &quests_per_owner_retained);
    }
    
    #[payable]
    pub fn add_near_reward(&mut self, quest_id: QuestId, rarity: BoxRarity, amount: U128, capacity: U64) {
        assert!(
            MINIMAL_NEAR_REWARD <= amount.into(),
            "The minimal reward in Near tokens is {} yocto",
            MINIMAL_NEAR_REWARD
        );

        let reward_deposit = u128::from(amount) * capacity.0 as u128;

        let storage_used_before = env::storage_usage();

        let mut quest = self.quests.get(&quest_id).expect(&format!("Quest with id {} wasn't found", quest_id.clone()));

        quest.add_near_reward(rarity, amount, capacity.into());

        self.quests.insert(&quest_id, &quest);

        let storage_used_after = env::storage_usage();

        let storage_deposit =
            env::storage_byte_cost() * (storage_used_after - storage_used_before) as u128;

        let total_deposit = storage_deposit + reward_deposit.clone();

        assert!(
            env::attached_deposit() >= total_deposit,
            "Deposited amount must be equal to {} yocto",
            total_deposit
        );

        let refund = env::attached_deposit() - total_deposit;
        if refund > 1 {
            Promise::new(env::predecessor_account_id()).transfer(refund);
        }
    }

    pub fn set_probability(&mut self, quest_id: QuestId, rarity: BoxRarity, probability: Probability) {
        let mut quest = self.quests.get(&quest_id).expect(&format!("Quest with id {} wasn't found", quest_id.clone()));

        quest.set_probability(rarity, probability);
    }

    pub fn set_owner(&mut self, quest_id: QuestId, new_owner_id: AccountId) {
        let mut quest = self.quests.get(&quest_id).expect(&format!("Quest with id {} wasn't found", quest_id.clone()));

        quest.set_owner(&new_owner_id);
        
        self.quests.insert(&quest_id, &quest);

        //remove from old owner
        let current_owner_id = env::predecessor_account_id();
        let owner_quests = self.quests_per_owner.get(&current_owner_id);

        if !owner_quests.is_some(){
            return;
        }

        let mut owners_quests_unwrapped = owner_quests.unwrap();

        owners_quests_unwrapped.remove(&quest_id);

        self.quests_per_owner.insert(&current_owner_id, &owners_quests_unwrapped);

        //add to new owner
        let new_owner_quests_unwrapped = self.quests_per_owner.get(&new_owner_id);
        
        let mut quests_per_owner = UnorderedSet::new(StorageKey::QuestIdsPerOwner);

         if new_owner_quests_unwrapped.is_some() {
            quests_per_owner = new_owner_quests_unwrapped.unwrap();
        }

        quests_per_owner.insert(&quest_id);
        self.quests_per_owner.insert(&new_owner_id, &quests_per_owner);

    }

    pub fn trust_nft_contract(&mut self, quest_id: QuestId, contract_id: AccountId) {
        let mut quest = self.quests.get(&quest_id).expect(&format!("Quest with id {} wasn't found", quest_id.clone()));

        quest.trust_nft_contract(contract_id);
    }

    pub fn untrust_nft_contract(&mut self, quest_id: QuestId, contract_id: AccountId) {
        let mut quest = self.quests.get(&quest_id).expect(&format!("Quest with id {} wasn't found", quest_id.clone()));
        
        quest.untrust_nft_contract(contract_id);
    }

    #[payable]
    pub fn mint_many(&mut self, quest_id: QuestId, rarity: BoxRarity, accounts: Vec<AccountId>) -> Vec<BoxId> {
        require!(accounts.len() != 0, "accounts can't be empty");
        let mut quest = self.quests.get(&quest_id).expect(&format!("Quest with id {} wasn't found", quest_id.clone()));

        let storage_used_before = env::storage_usage();

        let box_ids = quest.mint_many(rarity, accounts);

        let storage_used_after = env::storage_usage();

        let storage_deposit =
            env::storage_byte_cost() * (storage_used_after - storage_used_before) as u128;

        assert!(
            env::attached_deposit() >= storage_deposit,
            "Deposited amount must be bigger than {} yocto, you attached {} yocto",
            storage_deposit,
            env::attached_deposit()
        );

        let refund = env::attached_deposit() - storage_deposit;
        if refund > 1 {
            Promise::new(env::predecessor_account_id()).transfer(refund);
        }

        box_ids
    }

    #[payable]
    pub fn mint(&mut self, quest_id: QuestId, box_owner_id: AccountId, rarity: BoxRarity) -> BoxId {
        let mut quest = self.quests.get(&quest_id).expect(&format!("Quest with id {} wasn't found", quest_id.clone()));
        
        let storage_used_before = env::storage_usage();

        let questbox = quest.mint(box_owner_id, rarity);
        self.mint_boxes_per_owner(&questbox);

        let storage_used_after = env::storage_usage();

        let storage_deposit =
            env::storage_byte_cost() * (storage_used_after - storage_used_before) as u128;

        assert!(
            env::attached_deposit() >= storage_deposit,
            "Deposited amount must be bigger than {} yocto",
            storage_deposit
        );

        let refund = env::attached_deposit() - storage_deposit;
        if refund > 1 {
            Promise::new(env::predecessor_account_id()).transfer(refund);
        }

        return questbox.box_id
    }

    fn mint_boxes_per_owner(&mut self, questbox: &QuestBox) {
        let mut boxes_per_owner = self
            .questboxes_per_owner
            .get(&questbox.owner_id)
            .unwrap();

        boxes_per_owner.push(&QuestBoxData::new(questbox.quest_id, questbox.box_id));

        self.questboxes_per_owner
            .insert(&questbox.owner_id, &boxes_per_owner);
    }

    #[payable]
    pub fn delete_boxes(&mut self, quest_id: QuestId, ids: Vec<BoxId>) {
        let mut quest = self.quests.get(&quest_id).expect(&format!("Quest with id {} wasn't found", quest_id.clone()));

        let account_id = env::predecessor_account_id();

        quest.delete_boxes(&ids);

        let owners_questboxes = self.questboxes_per_owner.get(&account_id).unwrap();

        let mut retained_questboxes = Vector::new(StorageKey::QuestBoxesPerOwner);

        owners_questboxes
            .iter()
            .for_each(|quest_box| {
                let is_quest_matches = quest_box.quest_id == quest_id;
                let is_box_in_removal_list = ids.contains(&quest_box.box_id);

                let is_item_to_remove = is_quest_matches && is_box_in_removal_list;

                if !is_item_to_remove {
                    retained_questboxes.push(&quest_box);
                }
            });

        self.questboxes_per_owner
            .insert(&account_id, &retained_questboxes);


    }

    #[payable]
    pub fn claim(&mut self, quest_id: QuestId, box_id: BoxId) -> Promise {
        let account_id = env::predecessor_account_id();

        let questboxes_per_owner = self.questboxes_per_owner.get(&account_id).unwrap();

        require!(questboxes_per_owner
            .iter()
            .find(|quest_box| quest_box.quest_id == quest_id && quest_box.box_id == box_id)
            .is_some(), "ERR_ONLY_OWNER_CAN_BURN");

        let mut quest = self.quests.get(&quest_id).expect(&format!("Quest with id {} wasn't found", quest_id.clone()));

        return quest.claim(box_id);
    }

    #[payable]
    pub fn nft_on_transfer(
            &mut self,
            quest_id: QuestId,
            #[allow(unused_variables)] sender_id: AccountId,
            previous_owner_id: AccountId,
            token_id: TokenId,
            msg: String,
        ) -> PromiseOrValue<bool> {
        let mut quest = self.quests.get(&quest_id).expect(&format!("Quest with id {} wasn't found", quest_id.clone()));

        let storage_used_before = env::storage_usage();
        let result = quest.nft_on_transfer(sender_id, previous_owner_id, token_id, msg);
        let storage_used_after = env::storage_usage();

        let storage_deposit = env::storage_byte_cost() * (storage_used_after - storage_used_before) as u128;

        assert!(
            env::attached_deposit() >= storage_deposit,
            "Deposited amount must be bigger than {} yocto",
            storage_deposit
        );

        let refund = env::attached_deposit() - storage_deposit;
        if refund > 1 {
            Promise::new(env::predecessor_account_id()).transfer(refund);
        }

        return result;
    }
}