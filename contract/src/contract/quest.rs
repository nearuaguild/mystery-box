use std::collections::HashSet;

use near_sdk::collections::{LookupMap, UnorderedSet};
use near_sdk::json_types::{U128, U64};
use near_sdk::{
    AccountId, PanicOnDefault,
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
   // pub boxes_per_owner: LookupMap<AccountId, HashSet<BoxId>>,
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

    pub fn add_near_reward(&mut self, rarity: BoxRarity, amount: U128, capacity: U64) {
        self.next_pool_id += 1
        //self.internal_add_near_pool(rarity, amount.into(), capacity.into());
    }
}

// #[cfg(test)]
// mod tests;
