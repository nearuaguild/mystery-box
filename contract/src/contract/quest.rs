use std::collections::HashSet;

use near_sdk::collections::{ LookupMap, UnorderedSet };
use near_sdk::json_types::{ U128, U64 };
use near_sdk::{
    assert_one_yocto,
    env,
    require,
    AccountId,
    PanicOnDefault,
    Promise,
    PromiseOrValue,
};
use near_sdk::borsh::{ self, BorshDeserialize, BorshSerialize };

use crate::contract::callbacks::create_withdraw_box_reward_promise_with_verification;

use crate::contract::types::{ BoxId, BoxStatus, PoolId, Probability };

use super::enums::StorageKey;
use super::pools::Pool;
use super::questbox::QuestBox;
use super::types::{ BoxRarity, QuestId, TokenId };

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
    pub boxes: LookupMap<BoxId, QuestBox>,
    pub probability_by_rarity: LookupMap<BoxRarity, Probability>,
    pub users: UnorderedSet<AccountId>,
}

impl Quest {
    pub fn new(id: QuestId, title: &String, owner_id: &AccountId) -> Self {
        // nested prefixes should be unique according to this: https://docs.near.org/sdk/rust/contract-structure/nesting
        let quest_hash = env::sha256_array(&id.to_be_bytes());

        Self {
            id,
            title: title.to_string(),
            next_pool_id: 0,
            pools: LookupMap::new(StorageKey::Pools { quest_hash }),
            pool_ids_by_rarity: LookupMap::new(StorageKey::PoolsByRarity { quest_hash }),
            nft_pool_by_key: LookupMap::new(StorageKey::NftPoolByKey { quest_hash }),
            owner_id: owner_id.clone(),
            next_box_id: 0,
            boxes: LookupMap::new(StorageKey::Boxes { quest_hash }),
            probability_by_rarity: LookupMap::new(StorageKey::ProbabilityByRarity { quest_hash }),
            users: UnorderedSet::new(StorageKey::Users { quest_hash }),
        }
    }

    fn assert_only_owner(&self) {
        require!(env::predecessor_account_id() == self.owner_id, "ERR_FORBIDDEN");
    }

    pub fn add_near_reward(&mut self, rarity: BoxRarity, amount: U128, capacity: U64) {
        self.assert_only_owner();

        let pool_id = self.next_pool_id.clone();

        self.next_pool_id += 1;

        let pool = Pool::create_near_pool(pool_id, rarity, amount.into(), capacity.into());

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

    pub fn set_owner(&mut self, new_owner_id: &AccountId) {
        // only owner can set another owner
        self.assert_only_owner();

        self.owner_id = new_owner_id.clone();
    }

    pub fn mint(&mut self, box_owner_id: AccountId, rarity: BoxRarity) -> QuestBox {
        self.assert_only_owner();

        let box_data = self.internal_mint(box_owner_id.clone(), rarity.clone());

        return box_data;
    }

    pub fn delete_boxes(&mut self, ids: &Vec<BoxId>) {
        self.assert_only_owner();

        ids.iter().for_each(|box_id| {
            assert!(self.boxes.get(&box_id).is_some(), "Box {} doesn't exist", &box_id);

            let box_data = self.boxes.remove(box_id).unwrap();

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

        create_withdraw_box_reward_promise_with_verification(
            &account_id,
            self.id,
            &box_id,
            &pool_id
        )
    }

    pub fn nft_on_transfer(
        &mut self,
        #[allow(unused_variables)] sender_id: AccountId,
        previous_owner_id: AccountId,
        token_id: TokenId,
        box_rarity: BoxRarity
    ) -> PromiseOrValue<bool> {
        let nft_account_id = env::predecessor_account_id();

        //there is no point in sending nft to itself
        require!(self.owner_id == previous_owner_id, "ERR_FORBIDDEN");

        self.internal_add_nft_pool(box_rarity, nft_account_id, token_id);

        // stands for OK response
        PromiseOrValue::Value(false)
    }

    fn internal_mint(&mut self, box_owner_id: AccountId, rarity: BoxRarity) -> QuestBox {
        let box_id = self.next_box_id.clone();

        self.next_box_id += 1;

        let box_data = QuestBox::new(self.id, box_id, rarity, box_owner_id);

        self.boxes.insert(&box_data.box_id, &box_data);

        self.users.insert(&box_data.owner_id);

        box_data
    }
}
