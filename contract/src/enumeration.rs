use crate::*;

#[near_bindgen]
impl Contract {
    pub fn total_supply(&self) -> U128 {
        U128(self.next_box_id - 1)
    }

    pub fn supply_for_owner(&self, account_id: AccountId) -> U128 {
        let boxes_per_owner_set = self.boxes_per_owner.get(&account_id).unwrap_or_default();

        U128(boxes_per_owner_set.len() as u128)
    }

    pub fn boxes_for_owner(
        &self,
        account_id: AccountId,
        pagination: Option<Pagination>,
    ) -> Vec<JsonBox> {
        let pagination = pagination.unwrap_or_default();

        pagination.assert_valid();

        self.boxes_per_owner
            .get(&account_id)
            .unwrap_or_default()
            .iter()
            .take(pagination.take())
            .skip(pagination.skip())
            .map(|box_id| self.boxes.get(box_id).unwrap().into())
            .collect()
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
}
