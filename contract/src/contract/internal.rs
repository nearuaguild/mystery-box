use crate::contract::enums::BoxStatus;
use crate::contract::pools::Pool;
use crate::contract::types::{BoxData, BoxId, Capacity, PoolId, Probability, TokenId};
use near_sdk::{env, require, AccountId};

use std::str::FromStr;

use super::enums::{BoxRarity, Network};
use super::Contract;

fn get_random_number(shift_amount: usize) -> u64 {
    let mut seed = env::random_seed();
    let seed_len = seed.len();

    let mut arr: [u8; 8] = Default::default();

    seed.rotate_left(shift_amount % seed_len);
    arr.copy_from_slice(&seed[..8]);

    u64::from_le_bytes(arr)
}

pub(crate) fn get_registry_iah_contract() -> AccountId {
    let network = Network::from(env::current_account_id());

    match network {
        Network::Mainnet => AccountId::from_str("registry.i-am-human.near").unwrap(),
        Network::Testnet => AccountId::from_str("registry-v2.i-am-human.testnet").unwrap(),
    }
}

pub(crate) fn get_issuer_iah_contract() -> AccountId {
    let network = Network::from(env::current_account_id());

    match network {
        Network::Mainnet => AccountId::from_str("fractal.i-am-human.near").unwrap(),
        Network::Testnet => AccountId::from_str("fractal-v2.i-am-human.testnet").unwrap(),
    }
}

impl Contract {
    pub(crate) fn internal_add_near_pool(
        &mut self,
        rarity: BoxRarity,
        amount: u128,
        capacity: Capacity,
    ) {
        let pool_id = self.next_pool_id.clone();

        self.next_pool_id += 1;

        let pool = Pool::create_near_pool(pool_id, rarity, amount, capacity);

        self.pools.insert(&pool.id, &pool);

        let mut pool_ids = self.pool_ids_by_rarity.get(&rarity).unwrap_or_default();
        pool_ids.insert(pool_id.clone());
        self.pool_ids_by_rarity.insert(&rarity, &pool_ids);
    }

    pub(crate) fn internal_add_nft_pool(
        &mut self,
        rarity: BoxRarity,
        contract_id: AccountId,
        token_id: TokenId,
    ) {
        // to ensure tokens within the contract and rarity will be in the same pool
        let key = vec![
            contract_id.to_owned().to_string(),
            rarity.to_owned().to_string(),
        ]
        .join(":");

        let pool = match self.nft_pool_by_key.get(&key) {
            Option::None => {
                let pool_id = self.next_pool_id.clone();

                self.next_pool_id += 1;

                let mut pool = Pool::create_nft_pool(pool_id, rarity, contract_id);
                pool.add_nft_token(token_id);

                pool
            }
            Option::Some(pool_id) => {
                let mut pool = self.pools.get(&pool_id).expect("ERR_POOL_NOT_FOUND");
                pool.add_nft_token(token_id);

                pool
            }
        };

        self.pools.insert(&pool.id, &pool);

        let mut pool_ids = self.pool_ids_by_rarity.get(&rarity).unwrap_or_default();
        pool_ids.insert(pool.id.clone());
        self.pool_ids_by_rarity.insert(&rarity, &pool_ids);

        self.nft_pool_by_key.insert(&key, &pool.id);
    }

    pub(crate) fn internal_mint(&mut self, owner_id: AccountId, rarity: BoxRarity) -> BoxData {
        let box_id = self.next_box_id.clone();

        self.next_box_id += 1;

        let box_data = BoxData::new(box_id, rarity, owner_id);

        self.boxes.insert(&box_data.id, &box_data);

        let mut boxes_per_owner = self
            .boxes_per_owner
            .get(&box_data.owner_id)
            .unwrap_or_default();

        // should never panic
        require!(boxes_per_owner.insert(box_data.id));

        self.boxes_per_owner
            .insert(&box_data.owner_id, &boxes_per_owner);
        self.users.insert(&box_data.owner_id);

        box_data
    }

    pub(crate) fn internal_claim(&mut self, box_id: BoxId) -> PoolId {
        let mut box_data = self.boxes.get(&box_id).expect("ERR_BOX_NOT_FOUND");

        require!(
            box_data.status == BoxStatus::NonClaimed,
            "ERR_BOX_ALREADY_CLAIMED"
        );

        // take reward of some rarity
        let available_pools = self
            .pool_ids_by_rarity
            .get(&box_data.rarity)
            .unwrap_or_default()
            .iter()
            .filter_map(|pool_id| {
                let pool = self.pools.get(pool_id).unwrap();

                (!pool.is_empty()).then_some(pool)
            })
            .collect::<Vec<Pool>>();

        require!(available_pools.len() > 0, "ERR_NO_POOLS_AVAILABLE");

        let total_available: Capacity =
            available_pools.iter().map(|pool| pool.availability()).sum();

        let random_number = get_random_number(0);

        // bring to range [0, total_available - 1]
        let random_in_range = random_number % total_available;

        let mut last: u64 = 0;
        let mut pool_iterator = available_pools.iter();

        let mut random_pool = loop {
            let next_pool = pool_iterator.next();

            // should never panic (Reward pools are over before a random one had been found)
            require!(next_pool.is_some(), "ERR_LOGIC");

            let pool = next_pool.unwrap().to_owned();

            if random_in_range <= pool.availability() + last - 1 {
                break pool;
            }

            last += pool.availability().clone();
        };

        let probability = self
            .probability_by_rarity
            .get(&box_data.rarity)
            .unwrap_or(Probability::ONE);

        let threshold = probability.calculate_threshold();

        let random = random_number as u8;

        let is_rewarded = threshold == u8::MAX || (threshold != u8::MIN && random < threshold);

        match is_rewarded {
            true => {
                let reward = random_pool.take_reward_from_pool();

                box_data.status = BoxStatus::Claimed {
                    reward: Some(reward),
                };

                self.pools.insert(&random_pool.id, &random_pool);
            }
            false => {
                box_data.status = BoxStatus::Claimed { reward: None };
            }
        }

        self.boxes.insert(&box_data.id, &box_data);

        random_pool.id
    }

    pub(crate) fn internal_undo_claim(&mut self, box_id: BoxId, pool_id: PoolId) {
        let mut box_data = self.boxes.get(&box_id).expect("ERR_BOX_NOT_FOUND");
        let mut pool = self.pools.get(&pool_id).expect("ERR_POOL_NOT_FOUND");

        let reward_or_nothing = match box_data.status {
            BoxStatus::NonClaimed => unreachable!(),
            BoxStatus::Claimed { reward } => reward.to_owned(),
        };

        box_data.status = BoxStatus::NonClaimed;
        self.boxes.insert(&box_data.id, &box_data);

        if let Some(reward) = reward_or_nothing {
            pool.put_reward_to_pool(reward);
            self.pools.insert(&pool.id, &pool);
        }
    }

    pub(crate) fn assert_only_owner(&self) {
        require!(
            env::predecessor_account_id() == self.owner_id,
            "ERR_FORBIDDEN"
        );
    }
}
