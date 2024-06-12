use std::collections::HashSet;

use near_sdk::collections::{LookupMap, UnorderedSet};
use near_sdk::json_types::U128;
use near_sdk::{
    assert_one_yocto, env, require, AccountId, PanicOnDefault, Promise, PromiseOrValue
};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};

use crate::contract::callbacks::create_withdraw_box_reward_promise_with_verification;
use crate::contract::enums::BoxStatus;
use crate::contract::trusted_contracts::get_trusted_nft_contracts;

use crate::contract::types::{BoxId, PoolId, Probability};

use near_sdk::serde_json::{self, Value};

use super::enums::{BoxRarity, StorageKey};
use super::pools::Pool;
use super::types::questbox_data::QuestBoxData;
use super::types::{QuestId, TokenId};

#[derive(BorshSerialize, BorshDeserialize, PanicOnDefault)]
pub struct Quest {
    pub id: QuestId,
    pub title: String,
    pub owner_id: AccountId,
    pub next_pool_id: PoolId,
    pub pools: LookupMap<PoolId, Pool>,
    pub nft_pool_by_key: LookupMap<String, PoolId>,
    pub pool_ids_by_rarity: LookupMap<BoxRarity, HashSet<PoolId>>,
    pub next_box_id: BoxId,
    pub boxes: LookupMap<BoxId, QuestBoxData>,
    pub trusted_nft_contracts: UnorderedSet<AccountId>,
    pub probability_by_rarity: LookupMap<BoxRarity, Probability>,
    pub users: UnorderedSet<AccountId>,
}

impl Quest {
    pub fn new(
        title: &String,
        owner_id: &AccountId,
    ) -> Self {
        let default_trusted_nft_contracts = get_trusted_nft_contracts();

        let mut trusted_nft_contracts = UnorderedSet::new(StorageKey::TrustedNftContracts);

        default_trusted_nft_contracts
            .iter()
            .for_each(|contract_id| {
                trusted_nft_contracts.insert(contract_id);
            });

        Self {
            id: 0,
            title: title.to_string(),
            next_pool_id: 1,
            pools: LookupMap::new(StorageKey::Pools),
            pool_ids_by_rarity: LookupMap::new(StorageKey::PoolsByRarity),
            nft_pool_by_key: LookupMap::new(StorageKey::NftPoolByKey),
            trusted_nft_contracts,
            owner_id: owner_id.clone(),
            next_box_id: 1,
            boxes: LookupMap::new(StorageKey::Boxes),
            //boxes_per_owner: LookupMap::new(StorageKey::BoxesPerOwner),
            probability_by_rarity: LookupMap::new(StorageKey::ProbabilityByRarity),
            users: UnorderedSet::new(StorageKey::Users),
        }
    }

    fn assert_only_owner(&self) {
        require!(
            env::predecessor_account_id() == self.owner_id,
            "ERR_FORBIDDEN"
        );
    }

    pub fn add_near_reward(&mut self, rarity: BoxRarity, amount: U128, capacity: u64) {
        self.assert_only_owner();

        let pool_id = self.next_pool_id.clone();

        self.next_pool_id += 1;

        let pool = Pool::create_near_pool(pool_id, rarity, amount.into(), capacity);

        self.pools.insert(&pool.id, &pool);

        let mut pool_ids = self.pool_ids_by_rarity.get(&rarity).unwrap_or_default();
        pool_ids.insert(pool_id.clone());
        self.pool_ids_by_rarity.insert(&rarity, &pool_ids);
    }

    pub fn set_probability(&mut self, rarity: BoxRarity, probability: Probability) {
        probability.assert_valid();

        self.assert_only_owner();
    
        self.probability_by_rarity.insert(&rarity, &probability);
    }

    pub fn set_owner(&mut self, new_owner_id: AccountId) {
        // only owner can set another owner
        self.assert_only_owner();        

        self.owner_id = new_owner_id;
    }

    pub fn trust_nft_contract(&mut self, contract_id: AccountId) {
        self.assert_only_owner();        

        require!(
            self.trusted_nft_contracts.insert(&contract_id),
            "Provided contract is already in the set"
        );
    }

    pub fn untrust_nft_contract(&mut self, contract_id: AccountId) {
        self.assert_only_owner();
        
        require!(
            self.trusted_nft_contracts.remove(&contract_id),
            "Provided contract wasn't trusted before"
        );
    }

    pub fn mint_many(&mut self, rarity: BoxRarity, accounts: Vec<AccountId>) -> Vec<BoxId> {
        self.assert_only_owner();

        let box_ids = accounts
            .iter()
            .map(|account_id| {
                let box_data = self.internal_mint(account_id.clone(), rarity.clone());

                box_data.box_id
            })
            .collect::<Vec<BoxId>>();

        return box_ids
    }

    pub fn mint(&mut self, account_id: AccountId, rarity: BoxRarity) -> QuestBoxData {
        self.assert_only_owner();

        let box_data = self.internal_mint(account_id.clone(), rarity.clone());

        return box_data
    }

    pub fn delete_boxes(&mut self, ids: &Vec<BoxId>) {
        self.assert_only_owner();

        ids.iter().for_each(|box_data| {
            let box_data = self.boxes.remove(box_data).unwrap();

            require!(
                box_data.box_status == BoxStatus::NonClaimed,
                format!("Box {} already claimed", box_data.box_id)
            );
        });
    }

    pub fn claim(&mut self, box_id: BoxId) -> Promise {
        assert_one_yocto();

        require!(self.boxes.contains_key(&box_id), "ERR_BOX_NOT_FOUND");

        let account_id = env::predecessor_account_id();

        let pool_id = self.internal_claim(box_id);

        create_withdraw_box_reward_promise_with_verification(&account_id, self.id, &box_id, &pool_id)
    }

    pub fn nft_on_transfer(
        &mut self,
        #[allow(unused_variables)] sender_id: AccountId,
        previous_owner_id: AccountId,
        token_id: TokenId,
        msg: String,
    ) -> PromiseOrValue<bool> {
        let nft_account_id = env::predecessor_account_id();

        // we're required to ensure that the predecessor account is whitelisted, since the function is public
        require!(
            self.trusted_nft_contracts.contains(&nft_account_id),
            "ERR_NFT_CONTRACT_NOT_TRUSTED",
        );

        require!(self.owner_id == previous_owner_id, "ERR_FORBIDDEN");

        let rarity =
            serde_json::from_value::<BoxRarity>(Value::String(msg)).expect("ERR_PARSE_MSG");

        self.internal_add_nft_pool(rarity, nft_account_id, token_id);

        // stands for OK response
        PromiseOrValue::Value(false)
    }

    fn internal_mint(&mut self, owner_id: AccountId, rarity: BoxRarity) -> QuestBoxData {
        let box_id = self.next_box_id.clone();

        self.next_box_id += 1;

        let box_data = QuestBoxData::new(self.id, box_id, rarity, owner_id);

        self.boxes.insert(&box_data.box_id, &box_data);

        self.users.insert(&box_data.owner_id);

        box_data
    }
}

// #[cfg(test)]
// mod tests;
