use std::collections::HashSet;

use enums::{BoxRarity, StorageKey};
use near_sdk::{env, require, Promise, PromiseOrValue, ONE_NEAR};
use near_sdk::json_types::{U128, U64};
use near_sdk::{collections::LookupMap, near_bindgen, AccountId, PanicOnDefault};

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use types::{BoxId, Probability, QuestId, QuestTitle, TokenId};

use crate::contract::quest::Quest;

pub mod trusted_contracts;
pub mod quest;
pub mod callbacks;
pub mod enumeration;
pub mod internal;
pub mod json;
pub mod pools;
pub mod types;
pub mod enums;

pub mod quest_contract_methods;

const MINIMAL_NEAR_REWARD: u128 = ONE_NEAR / 10; // 0.1N

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    quests: LookupMap<QuestId, Quest>,
    quests_for_owner: LookupMap<AccountId, HashSet<QuestId>>,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new() -> Self {
        Self {
            quests: LookupMap::new(StorageKey::Quests),
            quests_for_owner: LookupMap::new(StorageKey::QuestsForOwner),
        }
    }

    pub fn contracts_for_owner(&self, account_id: AccountId) -> Vec<Quest> {
        self.quests_for_owner
            .get(&account_id)
            .unwrap_or_default()
            .iter()
            .map(|quest_id| self.quests.get(quest_id))
            .flatten()
            .collect()
    }

    #[payable]
    pub fn add_near_reward(&mut self, quest_id: QuestId, rarity: BoxRarity, amount: U128, capacity: U64) {
        self.assert_only_owner();

        assert!(
            MINIMAL_NEAR_REWARD <= amount.into(),
            "The minimal reward in Near tokens is {} yocto",
            MINIMAL_NEAR_REWARD
        );

        let reward_deposit = u128::from(amount) * capacity.0 as u128;

        let storage_used_before = env::storage_usage();

        let mut quest = self.quests.get(&quest_id).unwrap();

        quest.add_near_reward(rarity, amount, capacity);

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
        let mut quest = self.quests.get(&quest_id).unwrap();

        quest.set_probability(rarity, probability);
    }

    pub fn set_owner(&mut self, quest_id: QuestId, new_owner_id: AccountId) {
        let mut quest = self.quests.get(&quest_id).unwrap();

        quest.set_owner(new_owner_id);
    }

    pub fn trust_nft_contract(&mut self, quest_id: QuestId, contract_id: AccountId) {
        let mut quest = self.quests.get(&quest_id).unwrap();

        quest.trust_nft_contract(contract_id);
    }

    pub fn untrust_nft_contract(&mut self, quest_id: QuestId, contract_id: AccountId) {
        self.assert_only_owner();
        
        let mut quest = self.quests.get(&quest_id).unwrap();
        
        require!(
            quest.trusted_nft_contracts.remove(&contract_id),
            "Provided contract wasn't trusted before"
        );
    }

    #[payable]
    pub fn mint_many(&mut self, quest_id: QuestId, rarity: BoxRarity, accounts: Vec<AccountId>) -> Vec<BoxId> {
        self.assert_only_owner();

        require!(accounts.len() != 0, "accounts can't be empty");

        let storage_used_before = env::storage_usage();

        let box_ids = accounts
            .iter()
            .map(|account_id| {
                let box_data = self.internal_mint(account_id.clone(), rarity.clone());

                box_data.id
            })
            .collect::<Vec<BoxId>>();

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
    pub fn mint(&mut self, quest_id: QuestId, account_id: AccountId, rarity: BoxRarity) -> BoxId {
        return 0;
    }

    #[payable]
    pub fn delete_boxes(&mut self, quest_id: QuestId, ids: Vec<BoxId>) {
    }

    #[payable]
    pub fn claim(&mut self, quest_id: QuestId, box_id: BoxId) -> Promise {
    }

    pub fn nft_on_transfer(
            &mut self,
            #[allow(unused_variables)] sender_id: AccountId,
            previous_owner_id: AccountId,
            token_id: TokenId,
            msg: String,
        ) -> PromiseOrValue<bool> {
    }

    // pub fn contract_byte_cost(&self) -> U128 {
    //     U128(self.internal_contract_byte_cost())
    // }

    // fn internal_contract_byte_cost(&self) -> Balance {
    //     //let contract_bytes = CONTRACT.len() as u128;

    //     //TODO: calculate contract size more precisely
    //     let contract_bytes = 1024u128;
    //     env::storage_byte_cost() * contract_bytes
    // }

    // fn internal_register_contract(
    //     &mut self,
    //     owner_id: AccountId,
    //     contract_id: AccountId,
    //     title: String,
    // ) {
    //     let metadata = Quest::new(&title, &owner_id);

    //     require!(
    //         self.contracts.insert(&contract_id, &metadata).is_none(),
    //         "ERR_CONTRACT_ALREADY_EXIST"
    //     );

    //     let mut owner_contracts = self.contracts_for_owner.get(&owner_id).unwrap_or_default();
    //     owner_contracts.insert(contract_id.clone());

    //     self.contracts_for_owner.insert(&owner_id, &owner_contracts);
    // }

    // fn internal_remove_contract(&mut self, owner_id: AccountId, contract_id: AccountId) {
    //     self.contracts.remove(&contract_id);

    //     let mut owner_contracts = self.contracts_for_owner.get(&owner_id).unwrap_or_default();
    //     owner_contracts.remove(&contract_id);

    //     self.contracts_for_owner.insert(&owner_id, &owner_contracts);
    // }

    // #[payable]
    // pub fn deploy_mystery_box_contract(&mut self, alias: String, title: String) -> Promise {
    //     // Assert the sub-account is valid
    //     let current_account_id = env::current_account_id().to_string();
    //     let contract_id: AccountId = format!("{alias}.{current_account_id}").parse().unwrap();
    //     require!(
    //         env::is_valid_account_id(contract_id.as_bytes()),
    //         "ERR_INVALID_SUBACCOUNT_ALIAS"
    //     );

    //     let owner_id = env::predecessor_account_id();

    //     self.internal_register_contract(owner_id.clone(), contract_id.clone(), title);

    //     let attached_deposit = env::attached_deposit();
    //     let contract_deposit = self.internal_contract_byte_cost();

    //     assert!(
    //         attached_deposit >= contract_deposit,
    //         "Deposited amount must be bigger than {contract_deposit} yocto, you attached {attached_deposit} yocto",
    //     );

    //     let args = serde_json::to_vec(&ContractNewArguments {
    //         owner_id: owner_id.clone(),
    //         default_trusted_nft_contracts: get_trusted_nft_contracts(),
    //     })
    //     .unwrap();
    //     let deployment_promise = Promise::new(contract_id.clone())
    //         .create_account()
    //         .transfer(attached_deposit)
    //         .deploy_contract(CONTRACT.to_vec())
    //         .function_call("new".to_owned(), args, 0, Gas::ONE_TERA * 5);

    //     let callback_promise = Contract::ext(env::current_account_id())
    //         .deploy_mystery_box_contract_callback(
    //             contract_id.to_owned(),
    //             attached_deposit.to_owned(),
    //             owner_id.to_owned(),
    //         );

    //     deployment_promise.then(callback_promise)
    // }

    // #[private]
    // pub fn deploy_mystery_box_contract_callback(
    //     &mut self,
    //     contract_id: AccountId,
    //     deposited_amount: Balance,
    //     owner_id: AccountId,
    // ) -> Option<AccountId> {
    //     // https://docs.rs/near-sdk/latest/near_sdk/env/fn.promise_results_count.html
    //     require!(env::promise_results_count() == 1, "ERR_TOO_MANY_RESULTS");

    //     let deployment_result = env::promise_result(0);

    //     match deployment_result {
    //         PromiseResult::Successful(_) => {
    //             log!(format!(
    //                 "Successfully created {contract_id} and put Mystery Contract on it"
    //             ));

    //             Some(contract_id)
    //         }
    //         _ => {
    //             log!(format!(
    //                 "Error creating {contract_id}, reverting state and returning {deposited_amount} yocto to {owner_id}"
    //             ));
    //             Promise::new(owner_id.clone()).transfer(deposited_amount);

    //             self.internal_remove_contract(owner_id.clone(), contract_id.clone());

    //             None
    //         }
    //     }
    // }    
}