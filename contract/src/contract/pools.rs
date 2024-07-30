use std::collections::BTreeSet;
use contract::json::JsonPoolRewards;
use contract::types::{Capacity, PoolId, Reward, TokenId};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::require;
use near_sdk::{AccountId, Balance};

use crate::*;

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct Pool {
    pub id: PoolId,
    pub rarity: BoxRarity,
    kind: PoolKind,
}

impl Pool {
    pub fn create_near_pool(
        id: PoolId,
        rarity: BoxRarity,
        amount: Balance,
        capacity: Capacity,
    ) -> Self {
        let near_pool = NearPoolKind::new(amount, capacity);

        Self {
            id,
            rarity,
            kind: PoolKind::Near(near_pool),
        }
    }

    pub fn create_nft_pool(id: PoolId, rarity: BoxRarity, contract_id: AccountId) -> Self {
        let nft_pool = NonFungibleTokenPoolKind::new(contract_id);

        Self {
            id,
            rarity,
            kind: PoolKind::NonFungibleToken(nft_pool),
        }
    }

    pub fn is_empty(&self) -> bool {
        match self.kind {
            PoolKind::Near(ref pool) => pool.available == 0,
            PoolKind::NonFungibleToken(ref pool) => pool.available_tokens.len() == 0,
        }
    }

    pub fn add_nft_token(&mut self, token_id: TokenId) {
        match self.kind {
            PoolKind::Near(_) => unreachable!(),
            PoolKind::NonFungibleToken(ref mut pool) => pool.add_token(token_id),
        };
    }

    pub fn availability(&self) -> u64 {
        match self.kind {
            PoolKind::Near(ref pool) => pool.available,
            PoolKind::NonFungibleToken(ref pool) => pool.available_tokens.len() as u64,
        }
    }

    pub fn take_reward_from_pool(&mut self) -> Reward {
        match self.kind {
            PoolKind::Near(ref mut pool) => {
                let amount = pool.decrease_available().unwrap();

                Reward::Near { amount }
            }
            PoolKind::NonFungibleToken(ref mut pool) => {
                let token_id = pool.take_first_token().unwrap();

                Reward::NonFungibleToken {
                    contract_id: pool.contract_id.clone(),
                    token_id,
                }
            }
        }
    }

    pub fn put_reward_to_pool(&mut self, reward: Reward) {
        match reward {
            Reward::Near { amount: _ } => match self.kind {
                PoolKind::Near(ref mut pool) => pool.increase_available(),
                _ => unreachable!(),
            },
            Reward::NonFungibleToken {
                contract_id: _,
                token_id,
            } => match self.kind {
                PoolKind::NonFungibleToken(ref mut pool) => pool.put_token_back(token_id),
                _ => unreachable!(),
            },
        };
    }
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
enum PoolKind {
    Near(NearPoolKind),
    NonFungibleToken(NonFungibleTokenPoolKind),
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct NearPoolKind {
    pub amount: Balance,
    pub capacity: Capacity,
    pub available: Capacity,
}

impl NearPoolKind {
    pub fn new(amount: Balance, capacity: Capacity) -> Self {
        Self {
            amount,
            capacity,
            available: capacity,
        }
    }

    pub fn decrease_available(&mut self) -> Option<Balance> {
        require!(self.available > 0, "ERR_POOL_NOT_AVAILABLE");

        self.available -= 1;

        Some(self.amount.clone())
    }

    pub fn increase_available(&mut self) {
        self.available += 1;
    }
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct NonFungibleTokenPoolKind {
    pub contract_id: AccountId,
    pub tokens: BTreeSet<TokenId>,
    pub available_tokens: BTreeSet<TokenId>,
}

impl NonFungibleTokenPoolKind {
    pub fn new(contract_id: AccountId) -> Self {
        Self {
            contract_id,
            tokens: BTreeSet::new(),
            available_tokens: BTreeSet::new(),
        }
    }

    pub fn add_token(&mut self, token_id: TokenId) {
        // should never panic
        require!(self.tokens.insert(token_id.clone()), format!("Token add problem {}", token_id.clone()));
        require!(self.available_tokens.insert(token_id.clone()), format!("Available tokens add problem {}", token_id.clone()));
    }

    pub fn take_first_token(&mut self) -> Option<TokenId> {
        require!(self.available_tokens.len() > 0, "ERR_POOL_NOT_AVAILABLE");

        self.available_tokens.pop_first()
    }

    pub fn put_token_back(&mut self, token_id: TokenId) {
        // should never panic
        require!(self.available_tokens.insert(token_id));
    }
}

impl Into<JsonPoolRewards> for Pool {
    fn into(self) -> JsonPoolRewards {
        match self.kind {
            PoolKind::Near(ref pool) => JsonPoolRewards::Near {
                amount: pool.amount.to_owned().into(),
                available: pool.available.clone(),
                total: pool.capacity.clone(),
            },
            PoolKind::NonFungibleToken(ref pool) => JsonPoolRewards::NonFungibleToken {
                contract_id: pool.contract_id.clone(),
                token_ids: pool.available_tokens.clone().into_iter().collect(),
                total: pool.tokens.len() as u64,
            },
        }
    }
}
