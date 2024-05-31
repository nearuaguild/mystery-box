use std::collections::HashSet;

use near_sdk::assert_one_yocto;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::store::{LookupMap, UnorderedSet};
use near_sdk::serde_json::{self, Value};
use near_sdk::{
    env,
    json_types::{U128, U64},
    log, near_bindgen, require, AccountId, BorshStorageKey, Gas, PanicOnDefault, Promise,
    PromiseOrValue, PromiseResult,
};

use crate::callbacks::*;
use crate::internal::*;
use crate::json::*;
pub use crate::pools::*;
pub use crate::types::*;

mod callbacks;
mod enumeration;
mod internal;
mod json;
mod pools;
mod types;

const ONE_NEAR: u128 = 10_u128.pow(24);
const MINIMAL_NEAR_REWARD: u128 = ONE_NEAR / 10; // 0.1N

#[derive(BorshStorageKey, BorshSerialize)]
enum StorageKey {
    Pools,
    PoolsByRarity,
    NftPoolByKey,
    ///
    Boxes,
    BoxesPerOwner,
    ///
    TrustedNftContracts,
    ///
    ProbabilityByRarity,
    ///
    Users,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    owner_id: AccountId,
    next_pool_id: PoolId,
    pools: LookupMap<PoolId, Pool>,
    nft_pool_by_key: LookupMap<String, PoolId>,
    pool_ids_by_rarity: LookupMap<BoxRarity, HashSet<PoolId>>,
    next_box_id: BoxId,
    boxes: LookupMap<BoxId, BoxData>,
    boxes_per_owner: LookupMap<AccountId, HashSet<BoxId>>,
    trusted_nft_contracts: UnorderedSet<AccountId>,
    probability_by_rarity: LookupMap<BoxRarity, Probability>,
    users: UnorderedSet<AccountId>,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(
        default_trusted_nft_contracts: Option<Vec<AccountId>>,
        owner_id: Option<AccountId>,
    ) -> Self {
        let owner_id = owner_id.unwrap_or(env::predecessor_account_id());
        let default_trusted_nft_contracts = default_trusted_nft_contracts.unwrap_or_default();

        let mut trusted_nft_contracts = UnorderedSet::new(StorageKey::TrustedNftContracts);

        default_trusted_nft_contracts
            .iter()
            .for_each(|contract_id| {
                trusted_nft_contracts.insert(contract_id);
            });

        Self {
            next_pool_id: 1,
            pools: LookupMap::new(StorageKey::Pools),
            pool_ids_by_rarity: LookupMap::new(StorageKey::PoolsByRarity),
            nft_pool_by_key: LookupMap::new(StorageKey::NftPoolByKey),
            trusted_nft_contracts,
            owner_id: owner_id.clone(),
            next_box_id: 1,
            boxes: LookupMap::new(StorageKey::Boxes),
            boxes_per_owner: LookupMap::new(StorageKey::BoxesPerOwner),
            probability_by_rarity: LookupMap::new(StorageKey::ProbabilityByRarity),
            users: UnorderedSet::new(StorageKey::Users),
        }
    }

    pub fn set_probability(&mut self, rarity: BoxRarity, probability: Probability) {
        self.assert_only_owner();

        probability.assert_valid();

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

    #[payable]
    pub fn add_near_reward(&mut self, rarity: BoxRarity, amount: U128, capacity: U64) {
        self.assert_only_owner();

        assert!(
            MINIMAL_NEAR_REWARD <= amount.into(),
            "The minimal reward in Near tokens is {} yocto",
            MINIMAL_NEAR_REWARD
        );

        let reward_deposit = u128::from(amount) * capacity.0 as u128;

        let storage_used_before = env::storage_usage();

        self.internal_add_near_pool(rarity, amount.into(), capacity.into());

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

    #[payable]
    pub fn mint_many(&mut self, rarity: BoxRarity, accounts: Vec<AccountId>) -> Vec<BoxId> {
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
    pub fn mint(&mut self, account_id: AccountId, rarity: BoxRarity) -> BoxId {
        self.assert_only_owner();

        let storage_used_before = env::storage_usage();

        let box_data = self.internal_mint(account_id.clone(), rarity.clone());

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

        box_data.id
    }

    #[payable]
    pub fn delete_boxes(&mut self, ids: Vec<BoxId>) {
        self.assert_only_owner();

        ids.iter().for_each(|box_data| {
            let box_data = self.boxes.remove(box_data).unwrap();

            require!(
                box_data.status == BoxStatus::NonClaimed,
                format!("Box {} already claimed", box_data.id)
            );

            let mut boxes_per_owner = self
                .boxes_per_owner
                .get(&box_data.owner_id)
                .unwrap_or_default();

            // should never panic
            require!(boxes_per_owner.remove(&box_data.id));

            self.boxes_per_owner
                .insert(&box_data.owner_id, &boxes_per_owner);
        });
    }

    #[payable]
    pub fn claim(&mut self, box_id: BoxId) -> Promise {
        assert_one_yocto();

        require!(self.boxes.contains_key(&box_id), "ERR_BOX_NOT_FOUND");

        let account_id = env::predecessor_account_id();

        let boxes_for_owner = self.boxes_per_owner.get(&account_id).unwrap_or_default();

        require!(boxes_for_owner.contains(&box_id), "ERR_ONLY_OWNER_CAN_BURN");

        let pool_id = self.internal_claim(box_id);

        create_withdraw_box_reward_promise_with_verification(&account_id, &box_id, &pool_id)
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

        // TODO: add storage management
        self.internal_add_nft_pool(rarity, nft_account_id, token_id);

        // stands for OK response
        PromiseOrValue::Value(false)
    }
}

#[cfg(test)]
mod tests;
