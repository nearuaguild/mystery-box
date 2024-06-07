use near_sdk::{assert_one_yocto, env, json_types::{U128, U64}, near_bindgen, require, AccountId, Promise, PromiseOrValue, ONE_NEAR};

use super::{enums::BoxRarity, quest::Quest, types::{BoxId, Probability, TokenId}};
use crate::contract::{Contract, ContractExt};

use near_sdk::serde_json::{self, Value};

const MINIMAL_NEAR_REWARD: u128 = ONE_NEAR / 10; // 0.1N

impl Contract {

    // pub fn set_probability(&mut self, title: &String, rarity: BoxRarity, probability: Probability) {
    //     let mut quest = self.get_quest_by_title(&title);

    //     self.assert_only_owner();

    //     probability.assert_valid();

    //     quest.probability_by_rarity.insert(&rarity, &probability);
    // }

    // pub fn set_owner(&mut self, title: &String, new_owner_id: AccountId) {
    //     let mut quest = self.get_quest_by_title(&title);

    //     // only owner can set another owner
    //     self.assert_only_owner();

    //     quest.owner_id = new_owner_id;
    // }

    // pub fn trust_nft_contract(&mut self, title: &String, contract_id: AccountId) {
    //     let mut quest = self.get_quest_by_title(&title);

    //     self.assert_only_owner();

    //     require!(
    //         quest.trusted_nft_contracts.insert(&contract_id),
    //         "Provided contract is already in the set"
    //     );
    // }

    // pub fn untrust_nft_contract(&mut self, title: &String, contract_id: AccountId) {
    //     let mut quest = self.get_quest_by_title(&title);
        
    //     self.assert_only_owner();

    //     require!(
    //         quest.trusted_nft_contracts.remove(&contract_id),
    //         "Provided contract wasn't trusted before"
    //     );
    // }

    //#[payable]
    pub fn add_near_reward(&mut self, quest: Quest, rarity: BoxRarity, amount: U128, capacity: U64) {
        self.internal_add_near_pool(rarity, amount.into(), capacity.into());
    }

    // #[payable]
    // pub fn mint_many(&mut self, rarity: BoxRarity, accounts: Vec<AccountId>) -> Vec<BoxId> {
    //     self.assert_only_owner();

    //     require!(accounts.len() != 0, "accounts can't be empty");

    //     let storage_used_before = env::storage_usage();

    //     let box_ids = accounts
    //         .iter()
    //         .map(|account_id| {
    //             let box_data = self.internal_mint(account_id.clone(), rarity.clone());

    //             box_data.id
    //         })
    //         .collect::<Vec<BoxId>>();

    //     let storage_used_after = env::storage_usage();

    //     let storage_deposit =
    //         env::storage_byte_cost() * (storage_used_after - storage_used_before) as u128;

    //     assert!(
    //         env::attached_deposit() >= storage_deposit,
    //         "Deposited amount must be bigger than {} yocto, you attached {} yocto",
    //         storage_deposit,
    //         env::attached_deposit()
    //     );

    //     let refund = env::attached_deposit() - storage_deposit;
    //     if refund > 1 {
    //         Promise::new(env::predecessor_account_id()).transfer(refund);
    //     }

    //     box_ids
    // }

    // #[payable]
    // pub fn mint(&mut self, account_id: AccountId, rarity: BoxRarity) -> BoxId {
    //     self.assert_only_owner();

    //     let storage_used_before = env::storage_usage();

    //     let box_data = self.internal_mint(account_id.clone(), rarity.clone());

    //     let storage_used_after = env::storage_usage();

    //     let storage_deposit =
    //         env::storage_byte_cost() * (storage_used_after - storage_used_before) as u128;

    //     assert!(
    //         env::attached_deposit() >= storage_deposit,
    //         "Deposited amount must be bigger than {} yocto",
    //         storage_deposit
    //     );

    //     let refund = env::attached_deposit() - storage_deposit;
    //     if refund > 1 {
    //         Promise::new(env::predecessor_account_id()).transfer(refund);
    //     }

    //     box_data.id
    // }

    // #[payable]
    // pub fn delete_boxes(&mut self, ids: Vec<BoxId>) {
    //     self.assert_only_owner();

    //     ids.iter().for_each(|box_data| {
    //         let box_data = self.boxes.remove(box_data).unwrap();

    //         require!(
    //             box_data.status == BoxStatus::NonClaimed,
    //             format!("Box {} already claimed", box_data.id)
    //         );

    //         let mut boxes_per_owner = self
    //             .boxes_per_owner
    //             .get(&box_data.owner_id)
    //             .unwrap_or_default();

    //         // should never panic
    //         require!(boxes_per_owner.remove(&box_data.id));

    //         self.boxes_per_owner
    //             .insert(&box_data.owner_id, &boxes_per_owner);
    //     });
    // }

    // #[payable]
    // pub fn claim(&mut self, box_id: BoxId) -> Promise {
    //     assert_one_yocto();

    //     require!(self.boxes.contains_key(&box_id), "ERR_BOX_NOT_FOUND");

    //     let account_id = env::predecessor_account_id();

    //     let boxes_for_owner = self.boxes_per_owner.get(&account_id).unwrap_or_default();

    //     require!(boxes_for_owner.contains(&box_id), "ERR_ONLY_OWNER_CAN_BURN");

    //     let pool_id = self.internal_claim(box_id);

    //     create_withdraw_box_reward_promise_with_verification(&account_id, &box_id, &pool_id)
    // }

    // pub fn nft_on_transfer(
    //     &mut self,
    //     #[allow(unused_variables)] sender_id: AccountId,
    //     previous_owner_id: AccountId,
    //     token_id: TokenId,
    //     msg: String,
    // ) -> PromiseOrValue<bool> {
    //     let nft_account_id = env::predecessor_account_id();

    //     // we're required to ensure that the predecessor account is whitelisted, since the function is public
    //     require!(
    //         self.trusted_nft_contracts.contains(&nft_account_id),
    //         "ERR_NFT_CONTRACT_NOT_TRUSTED",
    //     );

    //     require!(self.owner_id == previous_owner_id, "ERR_FORBIDDEN");

    //     let rarity =
    //         serde_json::from_value::<BoxRarity>(Value::String(msg)).expect("ERR_PARSE_MSG");

    //     // TODO: add storage management
    //     self.internal_add_nft_pool(rarity, nft_account_id, token_id);

    //     // stands for OK response
    //     PromiseOrValue::Value(false)
    // }
}