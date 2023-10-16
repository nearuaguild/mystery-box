use near_contract_standards::non_fungible_token::TokenId;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::env::panic_str;
use near_sdk::{collections::LookupMap, require};
use near_sdk::{env, log, AccountId, Balance, IntoStorageKey};

use crate::types::{BoxRarity, Capacity, PoolId, Reward};
use crate::utils::get_random_number;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct RewardPoolContainer {
    pools: LookupMap<PoolId, Pool>,
    pool_ids_by_rarity: LookupMap<BoxRarity, Vec<PoolId>>,
}

impl RewardPoolContainer {
    pub fn new<Q, R>(pools_prefix: Q, pool_ids_by_rarity_prefix: R) -> Self
    where
        Q: IntoStorageKey,
        R: IntoStorageKey,
    {
        let pools = LookupMap::new(pools_prefix);
        let pool_ids_by_rarity = LookupMap::new(pool_ids_by_rarity_prefix);

        Self {
            pools,
            pool_ids_by_rarity,
        }
    }

    pub fn add_near_pool(&mut self, rarity: BoxRarity, amount: Balance, capacity: Capacity) {
        let pool = Pool::create_near_pool(amount, capacity);

        let pool_id = pool.id();

        self.pools.insert(&pool_id, &pool);

        let mut pool_ids = self.pool_ids_by_rarity.get(&rarity).unwrap_or_default();
        pool_ids.push(pool_id);
        self.pool_ids_by_rarity.insert(&rarity, &pool_ids);
    }

    pub fn add_nft_pool(&mut self, rarity: BoxRarity, contract_id: AccountId, token_id: TokenId) {
        let mut pool = Pool::create_nft_pool(contract_id, rarity);

        pool.add_token(token_id);

        let pool_id = pool.id();

        self.pools.insert(&pool_id, &pool);

        let mut pool_ids = self.pool_ids_by_rarity.get(&rarity).unwrap_or_default();
        pool_ids.push(pool_id);
        self.pool_ids_by_rarity.insert(&rarity, &pool_ids);
    }

    pub fn find_available_reward_pools(
        &self,
        rarity: BoxRarity,
        take: usize,
        skip: usize,
    ) -> Vec<Pool> {
        self.pool_ids_by_rarity
            .get(&rarity)
            .unwrap_or_default()
            .iter()
            .map(|pool_id| self.pools.get(&pool_id))
            .flatten()
            // find only appropriate BoxRarity and with available rewards
            .filter(|pool| !pool.is_empty())
            // limit amount of pools to not exceed Gas limitations
            .take(take)
            .skip(skip)
            .collect()
    }

    pub fn take_random_reward(&mut self, rarity: BoxRarity) -> Option<PendingReward> {
        // limit amount of pools to ensure we're not exceeding Gas
        let pools = self.find_available_reward_pools(rarity, 10, 0);

        if pools.len() == 0 {
            env::panic_str("ERR_NO_POOLS_AVAILABLE");
        }

        let total_available: Capacity = pools.iter().map(|pool| pool.available()).sum();

        let random_number = get_random_number(0);

        // bring to range [0, total_available - 1]
        let random_in_range = random_number % total_available;

        let mut last: u64 = 0;

        let mut pool_iterator = pools.iter();

        let mut random_pool = loop {
            let next_pool = pool_iterator.next();

            // should never panic (Reward pools are over before a random one had been found)
            require!(next_pool.is_some());

            let pool = next_pool.unwrap().to_owned();

            if random_in_range <= pool.available() + last - 1 {
                break pool;
            }

            last = pool.available().clone();
        };

        let index = random_in_range as usize - last as usize;
        let pending_reward = random_pool.take_reward_from_pool(index);

        // state has changed
        if pending_reward.is_some() {
            let pool_id = random_pool.id();
            self.pools.insert(&pool_id, &random_pool);
        }

        pending_reward
    }

    pub fn return_pending_reward(&mut self, pool_id: PoolId, pending_reward_id: PendingRewardId) {
        let mut pool = self.pools.get(&pool_id).expect("ERR_POOL_NOT_FOUND");

        pool.return_pending_reward(pending_reward_id);

        self.pools.insert(&pool_id, &pool);
    }

    pub fn confirm_pending_reward(&mut self, pool_id: PoolId, pending_reward_id: PendingRewardId) {
        let mut pool = self.pools.get(&pool_id).expect("ERR_POOL_NOT_FOUND");

        pool.confirm_pending_reward(pending_reward_id);

        self.pools.insert(&pool_id, &pool);
    }
}

pub trait RewardPool {
    fn id(&self) -> PoolId;
    fn is_empty(&self) -> bool;
    fn available(&self) -> Capacity;
    fn capacity(&self) -> Capacity;
    fn take_reward_from_pool(&mut self, index: usize) -> Option<PendingReward>;
    fn return_pending_reward(&mut self, id: PendingRewardId);
    fn confirm_pending_reward(&mut self, id: PendingRewardId);
}

pub type PendingRewardId = u32;

#[derive(BorshDeserialize, BorshSerialize, Clone)]
pub struct PendingReward {
    pub id: PendingRewardId,
    pub pool_id: PoolId,
    pub reward: Reward,
}

/// NFT

#[derive(BorshDeserialize, BorshSerialize, Clone)]
struct PendingNonFungibleTokenReward {
    pub id: PendingRewardId,
    pub token_id: TokenId,
}

#[derive(BorshDeserialize, BorshSerialize, Clone)]
pub struct NonFungibleTokenPool {
    pub contract_id: AccountId,
    pub tokens: Vec<TokenId>,
    pub rarity: BoxRarity,
    pub available_tokens: Vec<TokenId>,
    pending_rewards: Vec<PendingNonFungibleTokenReward>,
    next_pending_reward_id: PendingRewardId,
}

impl NonFungibleTokenPool {
    pub fn new(contract_id: AccountId, rarity: BoxRarity) -> Self {
        Self {
            contract_id,
            tokens: Vec::new(),
            rarity,
            available_tokens: Vec::new(),
            next_pending_reward_id: 0,
            pending_rewards: Vec::new(),
        }
    }

    pub fn compose_id(contract_id: AccountId, rarity: BoxRarity) -> PoolId {
        let rarity = rarity.to_string().to_owned();
        let elements = vec!["nft", contract_id.as_str(), rarity.as_str()];
        elements.join(":")
    }

    fn create_pending_reward(&mut self, token_id: TokenId) -> PendingReward {
        let id = self.next_pending_reward_id.clone();
        let pool_id = self.id();

        self.next_pending_reward_id += 1;

        PendingReward {
            id,
            pool_id,
            reward: Reward::NonFungibleToken {
                contract_id: self.contract_id.clone(),
                token_id,
            },
        }
    }

    pub fn add_token(&mut self, token_id: TokenId) {
        self.tokens.push(token_id.clone());
        self.available_tokens.push(token_id.clone());
    }
}

impl RewardPool for NonFungibleTokenPool {
    fn id(&self) -> PoolId {
        Self::compose_id(self.contract_id.clone(), self.rarity)
    }

    fn available(&self) -> Capacity {
        self.available_tokens.len() as Capacity
    }

    fn capacity(&self) -> Capacity {
        self.tokens.len() as Capacity
    }

    fn is_empty(&self) -> bool {
        self.available() == 0
    }

    fn take_reward_from_pool(&mut self, index: usize) -> Option<PendingReward> {
        if index as Capacity >= self.available() {
            return None;
        }

        // it also decreases availability
        let available_token = self.available_tokens.remove(index);

        let pending_reward = self.create_pending_reward(available_token.clone());

        self.pending_rewards.push(PendingNonFungibleTokenReward {
            id: pending_reward.id,
            token_id: available_token.clone(),
        });

        Some(pending_reward)
    }

    fn return_pending_reward(&mut self, id: PendingRewardId) {
        let index = self
            .pending_rewards
            .iter()
            .position(|pr| pr.id == id)
            .expect("ERR_PENDING_REWARD_NOT_FOUND");

        let pending_reward = self.pending_rewards.remove(index);

        // should never panic
        require!(self.tokens.contains(&pending_reward.token_id));

        self.available_tokens.push(pending_reward.token_id);
    }

    fn confirm_pending_reward(&mut self, id: PendingRewardId) {
        let index = self
            .pending_rewards
            .iter()
            .position(|pr| pr.id == id)
            .expect("ERR_PENDING_REWARD_NOT_FOUND");

        self.pending_rewards.remove(index);
    }
}

/// NEAR
#[derive(BorshDeserialize, BorshSerialize, Clone)]
struct PendingNearReward {
    pub id: PendingRewardId,
    pub amount: Balance,
}

#[derive(BorshDeserialize, BorshSerialize, Clone)]
pub struct NearPool {
    nonce: u64,
    pub amount: Balance,
    pub capacity: Capacity,
    pub available: Capacity,
    pending_rewards: Vec<PendingNearReward>,
    next_pending_reward_id: PendingRewardId,
}

impl NearPool {
    pub fn new(amount: Balance, capacity: Capacity) -> Self {
        Self {
            amount,
            capacity,
            available: capacity,
            next_pending_reward_id: 0,
            pending_rewards: Vec::new(),
            nonce: env::block_timestamp_ms(),
        }
    }

    pub fn compose_id(nonce: u64) -> PoolId {
        let nonce = nonce.to_string();
        let elements = vec!["near", nonce.as_str()];
        elements.join(":")
    }

    fn create_pending_reward(&mut self, amount: Balance) -> PendingReward {
        let id = self.next_pending_reward_id.clone();
        let pool_id = self.id();

        self.next_pending_reward_id += 1;

        PendingReward {
            id,
            pool_id,
            reward: Reward::Near { amount },
        }
    }
}

impl RewardPool for NearPool {
    fn id(&self) -> PoolId {
        Self::compose_id(self.nonce)
    }

    fn available(&self) -> Capacity {
        self.available
    }

    fn capacity(&self) -> Capacity {
        self.capacity
    }

    fn is_empty(&self) -> bool {
        self.available == 0
    }

    fn take_reward_from_pool(&mut self, index: usize) -> Option<PendingReward> {
        if index as Capacity >= self.available() {
            return None;
        }

        let pending_reward = self.create_pending_reward(self.amount);

        self.available -= 1;

        self.pending_rewards.push(PendingNearReward {
            id: pending_reward.id,
            amount: self.amount,
        });

        Some(pending_reward)
    }

    fn return_pending_reward(&mut self, id: PendingRewardId) {
        let index = self
            .pending_rewards
            .iter()
            .position(|pr| pr.id == id)
            .expect("ERR_PENDING_REWARD_NOT_FOUND");

        let pending_reward = self.pending_rewards.remove(index);

        // should never panic
        require!(self.amount == pending_reward.amount);

        self.available += 1;
    }

    fn confirm_pending_reward(&mut self, id: PendingRewardId) {
        let index = self
            .pending_rewards
            .iter()
            .position(|pr| pr.id == id)
            .expect("ERR_PENDING_REWARD_NOT_FOUND");

        self.pending_rewards.remove(index);
    }
}

#[derive(BorshDeserialize, BorshSerialize, Clone)]
pub enum Pool {
    Near(NearPool),
    NonFungibleToken(NonFungibleTokenPool),
}

impl Pool {
    fn get_actual_pool(&self) -> Box<dyn RewardPool> {
        match self.to_owned() {
            Self::Near(pool) => Box::new(pool),
            Self::NonFungibleToken(pool) => Box::new(pool),
        }
    }

    pub fn create_near_pool(amount: Balance, capacity: Capacity) -> Self {
        Self::Near(NearPool::new(amount, capacity))
    }

    pub fn create_nft_pool(contract_id: AccountId, rarity: BoxRarity) -> Self {
        Self::NonFungibleToken(NonFungibleTokenPool::new(contract_id, rarity))
    }

    pub fn add_token(&mut self, token_id: TokenId) {
        match self.to_owned() {
            // this should never panic
            Self::Near(_) => panic_str("ERR_INAPPROPRIATE_POOL"),
            Self::NonFungibleToken(mut pool) => pool.add_token(token_id),
        };
    }
}

impl RewardPool for Pool {
    fn id(&self) -> PoolId {
        self.get_actual_pool().id()
    }

    fn is_empty(&self) -> bool {
        self.get_actual_pool().is_empty()
    }

    fn available(&self) -> Capacity {
        self.get_actual_pool().available()
    }

    fn capacity(&self) -> Capacity {
        self.get_actual_pool().capacity()
    }

    fn take_reward_from_pool(&mut self, index: usize) -> Option<PendingReward> {
        match self {
            Pool::Near(ref mut pool) => pool.take_reward_from_pool(index),
            Pool::NonFungibleToken(ref mut pool) => pool.take_reward_from_pool(index),
        }
    }

    fn return_pending_reward(&mut self, id: PendingRewardId) {
        match self {
            Pool::Near(ref mut pool) => pool.return_pending_reward(id),
            Pool::NonFungibleToken(ref mut pool) => pool.return_pending_reward(id),
        }
    }

    fn confirm_pending_reward(&mut self, id: PendingRewardId) {
        match self {
            Pool::Near(ref mut pool) => pool.confirm_pending_reward(id),
            Pool::NonFungibleToken(ref mut pool) => pool.confirm_pending_reward(id),
        }
    }
}
