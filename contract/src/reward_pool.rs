use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::IntoStorageKey;
use near_sdk::{
    collections::{LookupMap, UnorderedSet},
    require,
};

use crate::{
    types::{BoxRarity, Capacity, Reward, RewardPool, RewardPoolId},
    utils::get_random_number,
};

#[derive(BorshDeserialize, BorshSerialize)]
pub struct RewardPoolContainer {
    next_reward_pool_id: RewardPoolId,
    reward_pools: LookupMap<RewardPoolId, RewardPool>,
    available_pools: UnorderedSet<RewardPoolId>,
}

impl RewardPoolContainer {
    pub fn new<Q, R>(pools_prefix: Q, available_pools_prefix: R) -> Self
    where
        Q: IntoStorageKey,
        R: IntoStorageKey,
    {
        Self {
            next_reward_pool_id: 1,
            reward_pools: LookupMap::new(pools_prefix),
            available_pools: UnorderedSet::new(available_pools_prefix),
        }
    }

    pub fn add_reward_pool(
        &mut self,
        reward: Reward,
        box_rarity: BoxRarity,
        capacity: Capacity,
    ) -> RewardPool {
        let reward_pool_id = self.next_reward_pool_id.clone();
        self.next_reward_pool_id += 1;

        let reward_pool = RewardPool::from(reward_pool_id, reward, capacity, box_rarity);

        reward_pool.assert_valid();

        self.reward_pools.insert(&reward_pool.id, &reward_pool);
        self.available_pools.insert(&reward_pool.id);

        reward_pool
    }

    pub fn find_available_reward_pools(
        &self,
        box_rarity: BoxRarity,
        take: usize,
        skip: usize,
    ) -> Vec<RewardPool> {
        self.available_pools
            .iter()
            .map(|reward_pool_id| self.reward_pools.get(&reward_pool_id))
            .flatten()
            // find only appropriate BoxRarity and with available rewards
            .filter(|reward_pool| reward_pool.box_rarity == box_rarity && !reward_pool.is_empty())
            // limit amount of groups to not exceed Gas limitations
            .take(take)
            .skip(skip)
            .collect::<Vec<RewardPool>>()
    }

    pub fn find_random_available_reward_pool(&self, box_rarity: BoxRarity) -> RewardPool {
        let available_reward_pools = self.find_available_reward_pools(box_rarity, 5, 0);

        require!(
            !available_reward_pools.is_empty(),
            "No available reward pools found"
        );

        let total_availability: Capacity = available_reward_pools
            .iter()
            .map(|reward_pool| reward_pool.available_capacity)
            .sum();

        let random_number = get_random_number(0);

        // bring to range [0, total_availability - 1]
        let random_in_range = random_number % total_availability;

        let mut last: u64 = 0;

        let mut reward_pool_iterator = available_reward_pools.iter();

        loop {
            let next_reward_pool = reward_pool_iterator.next();

            require!(
                next_reward_pool.is_some(),
                "Reward pools are over before a random one had been found"
            );

            let reward_pool = next_reward_pool.unwrap();

            if random_in_range <= reward_pool.available_capacity + last - 1 {
                break reward_pool.clone().to_owned();
            }

            last = reward_pool.available_capacity.clone();
        }
    }

    pub fn decrement_availability(&mut self, reward_pool_id: RewardPoolId) {
        let mut reward_pool = self
            .reward_pools
            .get(&reward_pool_id)
            .expect("Reward RewardPoolContainer isn't found");

        reward_pool.decrement_availability();

        if reward_pool.is_empty() {
            self.available_pools.remove(&reward_pool.id);
        }

        self.reward_pools.insert(&reward_pool.id, &reward_pool);
    }

    pub fn increment_availability(&mut self, reward_pool_id: RewardPoolId) {
        let mut reward_pool = self
            .reward_pools
            .get(&reward_pool_id)
            .expect("Reward RewardPoolContainer isn't found");

        reward_pool.increment_availability();

        self.available_pools.insert(&reward_pool_id);
        self.reward_pools.insert(&reward_pool.id, &reward_pool);
    }
}
