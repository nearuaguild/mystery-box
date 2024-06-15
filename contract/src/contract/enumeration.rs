use crate::contract::quest::Quest;
use near_sdk::json_types::U128;
use near_sdk::AccountId;

use super::json_types::json_pagination::Pagination;
use super::json_types::json_reward::JsonPoolRewards;
use super::types::BoxRarity;

impl Quest {
    pub fn total_supply(&self) -> U128 {
        U128(self.next_box_id - 1)
    }

    pub fn available_rewards(
        &self,
        rarity: BoxRarity,
        pagination: Option<Pagination>,
    ) -> Vec<JsonPoolRewards> {
        let pagination = pagination.unwrap_or_default();

        pagination.assert_valid();

        self.pool_ids_by_rarity
            .get(&rarity)
            .unwrap_or_default()
            .iter()
            .map(|pool_id| self.pools.get(pool_id))
            .flatten()
            .filter(|pool| !pool.is_empty())
            .take(pagination.take())
            .skip(pagination.skip())
            .map(|pool| pool.into())
            .collect()
    }

    pub fn rewards(
        &self,
        rarity: BoxRarity,
        pagination: Option<Pagination>,
    ) -> Vec<JsonPoolRewards> {
        let pagination = pagination.unwrap_or_default();

        pagination.assert_valid();

        self.pool_ids_by_rarity
            .get(&rarity)
            .unwrap_or_default()
            .iter()
            .map(|pool_id| self.pools.get(pool_id))
            .flatten()
            .take(pagination.take())
            .skip(pagination.skip())
            .map(|pool| pool.into())
            .collect()
    }

    pub fn trusted_nft_contracts(&self) -> Vec<AccountId> {
        self.trusted_nft_contracts.to_vec()
    }

    pub fn users(&self, pagination: Option<Pagination>) -> Vec<AccountId> {
        let pagination = pagination.unwrap_or_default();

        pagination.assert_valid();

        self.users
            .iter()
            .take(pagination.take())
            .skip(pagination.skip())
            .collect()
    }
}
