use std::collections::HashSet;

use near_sdk::collections::{LookupMap, UnorderedSet};
use near_sdk::json_types::{U128, U64};
use near_sdk::{
    env, require, AccountId, PanicOnDefault
};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};

use crate::contract::trusted_contracts::get_trusted_nft_contracts;

use crate::contract::types::{BoxData, BoxId, PoolId, Probability};

use super::enums::{BoxRarity, StorageKey};
use super::pools::Pool;
use super::types::QuestId;

#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Quest {
    pub id: QuestId,
    pub title: String,
    pub owner_id: AccountId,
    pub next_pool_id: PoolId,
    pub pools: LookupMap<PoolId, Pool>,
    pub nft_pool_by_key: LookupMap<String, PoolId>,
    pub pool_ids_by_rarity: LookupMap<BoxRarity, HashSet<PoolId>>,
    pub next_box_id: BoxId,
    pub boxes: LookupMap<BoxId, BoxData>,
    //pub boxes_per_owner: LookupMap<AccountId, HashSet<BoxId>>,
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

    pub fn add_near_reward(&mut self, rarity: BoxRarity, amount: U128, capacity: U64) {
        let pool_id = self.next_pool_id.clone();

        self.next_pool_id += 1;

        let pool = Pool::create_near_pool(pool_id, rarity, amount.into(), capacity);

        self.pools.insert(&pool.id, &pool);

        let mut pool_ids = self.pool_ids_by_rarity.get(&rarity).unwrap_or_default();
        pool_ids.insert(pool_id.clone());
        self.pool_ids_by_rarity.insert(&rarity, &pool_ids);
    }

    pub fn internal_mint(&mut self, owner_id: AccountId, rarity: BoxRarity) -> BoxData {
        let box_id = self.next_box_id.clone();

        self.next_box_id += 1;

        let box_data = BoxData::new(box_id, rarity, owner_id);

        self.boxes.insert(&box_data.id, &box_data);

        //TODO. Handle boxes_per_owner on higher level
        // let mut boxes_per_owner = self
        //     .boxes_per_owner
        //     .get(&box_data.owner_id)
        //     .unwrap_or_default();

        // // should never panic
        // require!(boxes_per_owner.insert(box_data.id));

        // self.boxes_per_owner
        //     .insert(&box_data.owner_id, &boxes_per_owner);
        self.users.insert(&box_data.owner_id);

        box_data
    }
}

// #[cfg(test)]
// mod tests;
