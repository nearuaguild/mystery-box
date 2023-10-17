use boxes::MysteryBoxContainer;
use json::{JsonBox, JsonPoolRewards, Pagination};
use near_contract_standards::non_fungible_token::metadata::TokenMetadata;
use near_contract_standards::non_fungible_token::{NonFungibleToken, Token, TokenId};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, LookupSet};
use near_sdk::serde_json::{self, Value};
use near_sdk::{
    env, json_types::U128, log, near_bindgen, require, AccountId, BorshStorageKey, Gas,
    PanicOnDefault, Promise, PromiseOrValue, PromiseResult, ONE_NEAR,
};
use reward_pools::{PendingRewardId, RewardPoolContainer};
use types::{BoxId, BoxRarity, Capacity, PoolId, Reward};

mod boxes;
mod json;
mod reward_pools;
mod types;
mod utils;

const MINIMAL_NEAR_REWARD: u128 = ONE_NEAR / 10; // 0.1N

#[derive(BorshStorageKey, BorshSerialize)]
enum StorageKey {
    NonFungibleToken,
    TokenMetadata,
    Enumeration,
    Approval,
    ///
    RewardsPools,
    AvailablePools,
    ///
    Boxes,
    UserBoxes,
    ///
    TokenToBoxMapping,
    ///
    WhitelistedNftContracts,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    nft: NonFungibleToken,
    next_token_id: u64,
    owner_id: AccountId,
    rewards: RewardPoolContainer,
    boxes: MysteryBoxContainer,
    token_to_box: LookupMap<TokenId, BoxId>,
    whitelisted_nft_contracts: LookupSet<AccountId>,
    // https://github.com/near-ndc/i-am-human
    registry_iah_contract: AccountId,
    issuer_iah_contract: AccountId,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(registry_iah_contract: AccountId, issuer_iah_contract: AccountId) -> Self {
        let owner_id = env::predecessor_account_id();

        Self {
            nft: NonFungibleToken::new(
                StorageKey::NonFungibleToken,
                owner_id.clone(),
                Some(StorageKey::TokenMetadata),
                Some(StorageKey::Enumeration),
                Some(StorageKey::Approval),
            ),
            rewards: RewardPoolContainer::new(StorageKey::RewardsPools, StorageKey::AvailablePools),
            boxes: MysteryBoxContainer::new(StorageKey::Boxes, StorageKey::UserBoxes),
            whitelisted_nft_contracts: LookupSet::new(StorageKey::WhitelistedNftContracts),
            owner_id: owner_id.clone(),
            next_token_id: 1,
            token_to_box: LookupMap::new(StorageKey::TokenToBoxMapping),
            registry_iah_contract,
            issuer_iah_contract,
        }
    }

    pub fn get_registry_iah_contract(&self) -> String {
        self.registry_iah_contract.to_string()
    }

    pub fn get_issuer_iah_contract(&self) -> String {
        self.issuer_iah_contract.to_string()
    }

    fn assert_only_owner(&self) {
        require!(
            env::predecessor_account_id() == self.owner_id,
            "ERR_FORBIDDEN"
        );
    }

    pub fn set_owner(&mut self, new_owner_id: AccountId) {
        // only owner can set another owner
        self.assert_only_owner();

        self.owner_id = new_owner_id;
    }

    pub fn whitelist_nft_contract(&mut self, nft_contract: AccountId) {
        self.assert_only_owner();

        self.whitelisted_nft_contracts.insert(&nft_contract);
    }

    #[payable]
    pub fn add_near_reward(&mut self, rarity: BoxRarity, amount: U128, capacity: Capacity) {
        self.assert_only_owner();

        assert!(
            MINIMAL_NEAR_REWARD <= amount.into(),
            "The minimal reward in Near tokens is {} yocto",
            MINIMAL_NEAR_REWARD
        );

        let reward_deposit = u128::from(amount) * capacity as u128;

        let storage_used_before = env::storage_usage();

        self.rewards.add_near_pool(rarity, amount.into(), capacity);

        let storage_used_after = env::storage_usage();

        let storage_deposit =
            env::storage_byte_cost() * (storage_used_after - storage_used_before) as u128;

        let total_deposit = storage_deposit + reward_deposit.clone();

        assert!(
            env::attached_deposit() >= total_deposit,
            "Deposited amount must be equal to {} yocto",
            total_deposit
        );
    }

    pub fn get_account_boxes(
        &self,
        account_id: AccountId,
        pagination: Option<Pagination>,
    ) -> Vec<JsonBox> {
        let pagination = pagination.unwrap_or_default();

        pagination.assert_valid();

        self.boxes
            .get_user_boxes(
                account_id,
                pagination.calculate_limit(),
                pagination.calculate_offset(),
            )
            .iter()
            .map(|box_data| box_data.into())
            .collect()
    }

    pub fn get_available_rewards(
        &self,
        rarity: BoxRarity,
        pagination: Option<Pagination>,
    ) -> Vec<JsonPoolRewards> {
        let pagination = pagination.unwrap_or_default();

        pagination.assert_valid();

        self.rewards
            .find_available_reward_pools(
                rarity,
                pagination.calculate_limit(),
                pagination.calculate_offset(),
            )
            .iter()
            .map(|pool| pool.into())
            .collect()
    }

    #[private]
    pub fn on_iah_verification_callback(&self) -> bool {
        // https://docs.rs/near-sdk/latest/near_sdk/env/fn.promise_results_count.html
        assert_eq!(env::promise_results_count(), 1, "ERR_TOO_MANY_RESULTS");

        let iah_result = env::promise_result(0);

        match iah_result {
            PromiseResult::Failed => {
                log!(
                    "Something failed while getting data from IAH registry {}",
                    self.registry_iah_contract.clone()
                );

                false
            }
            PromiseResult::NotReady => {
                log!("Promise for on_iah_verification_callback isn't ready yet");

                false
            }
            PromiseResult::Successful(data) => {
                let deserealize_result = serde_json::from_slice::<Value>(data.as_slice());

                match deserealize_result {
                    Err(_) => {
                        log!("Couldn't deserialize cross-contract results");

                        false
                    }
                    Ok(data) => {
                        let verification_result = data.pointer("/0/1/0");

                        match verification_result {
                            None => {
                                log!(
                                    "Verification data not found in registry {} for issuer {}",
                                    self.registry_iah_contract,
                                    self.issuer_iah_contract
                                );

                                false
                            }
                            Some(_) => true,
                        }
                    }
                }
            }
        }
    }

    fn internal_nft_mint(&mut self, account_id: AccountId, box_rarity: BoxRarity) -> Token {
        let token_id: TokenId = self.next_token_id.clone().to_string();

        self.next_token_id += 1;

        let title = format!("Mystery Box #{}", &token_id);
        let media = box_rarity.to_media_ipfs();
        let extra: String = serde_json::json!({"type": &box_rarity}).to_string();

        let metadata = TokenMetadata {
            title: Some(title),
            description: None,
            copies: Some(1),
            expires_at: None,
            issued_at: None,
            media: Some(media),
            media_hash: None,
            reference: None,
            reference_hash: None,
            starts_at: None,
            updated_at: None,
            extra: Some(extra),
        };

        self.nft
            .internal_mint(token_id.clone(), account_id, Some(metadata))
    }

    // TODO: add nft_mint_many function (ensure <10 boxes are being created at time)

    #[payable]
    pub fn nft_mint(&mut self, account_id: AccountId, rarity: BoxRarity) {
        self.assert_only_owner();

        let storage_used_before = env::storage_usage();

        let token = self.internal_nft_mint(account_id.clone(), rarity.clone());
        let box_data =
            self.boxes
                .add_box(rarity.clone(), account_id.clone(), token.token_id.clone());
        self.token_to_box.insert(&token.token_id, &box_data.id);

        let storage_used_after = env::storage_usage();

        let storage_deposit =
            env::storage_byte_cost() * (storage_used_after - storage_used_before) as u128;

        assert!(
            env::attached_deposit() >= storage_deposit,
            "Deposited amount must be bigger than {} yocto",
            storage_deposit
        );
    }

    fn internal_nft_burn(&mut self, token_id: TokenId, account_id: AccountId) {
        if let Some(next_approval_id_by_id) = &mut self.nft.next_approval_id_by_id {
            next_approval_id_by_id.remove(&token_id);
        }

        if let Some(approvals_by_id) = &mut self.nft.approvals_by_id {
            approvals_by_id.remove(&token_id);
        }

        if let Some(tokens_per_owner) = &mut self.nft.tokens_per_owner {
            let mut token_ids = tokens_per_owner.get(&account_id).unwrap();
            token_ids.remove(&token_id);
            tokens_per_owner.insert(&account_id, &token_ids);
        }

        if let Some(token_metadata_by_id) = &mut self.nft.token_metadata_by_id {
            token_metadata_by_id.remove(&token_id);
        }

        self.nft.owner_by_id.remove(&token_id);
    }

    fn create_on_transfer_reward_callback_promise(
        &self,
        account_id: &AccountId,
        box_id: &BoxId,
        pool_id: &PoolId,
        pending_reward_id: &PendingRewardId,
    ) -> Promise {
        Promise::new(env::current_account_id()).function_call(
            "on_transfer_reward_callback".to_string(),
            serde_json::json!({
                "account_id": account_id,
                "box_id": box_id,
                "pool_id": pool_id,
                "pending_reward_id": pending_reward_id,
            })
            .to_string()
            .into_bytes(),
            0,
            Gas::ONE_TERA * 10,
        )
    }

    fn create_transfer_reward_promise(&self, reward: Reward, receiver_id: &AccountId) -> Promise {
        match reward {
            Reward::Near { amount } => Promise::new(receiver_id.clone()).transfer(amount),
            Reward::NonFungibleToken {
                contract_id,
                token_id,
            } => Promise::new(contract_id).function_call(
                "nft_transfer".to_string(),
                serde_json::json!({
                    "token_id": token_id.clone(),
                    "receiver_id": receiver_id.clone()
                })
                .to_string()
                .into_bytes(),
                0,
                Gas::ONE_TERA * 5,
            ),
        }
    }

    // TODO: return Option<JsonReward> instead of bool
    pub fn nft_burn(&mut self, token_id: TokenId) -> PromiseOrValue<bool> {
        let account_id = env::predecessor_account_id();

        let owner_id = self
            .nft
            .owner_by_id
            .get(&token_id)
            .expect("ERR_TOKEN_NOT_FOUND");

        require!(owner_id == account_id, "ERR_ONLY_OWNER_CAN_BURN");

        // this should never panic
        let box_id = self.token_to_box.remove(&token_id).unwrap();
        self.internal_nft_burn(token_id, account_id.clone());

        let rarity = self.boxes.get_box_rarity(&box_id);

        let pending_reward = self.rewards.take_random_reward(rarity);

        match pending_reward {
            Option::None => {
                self.boxes.claim_box(box_id.clone(), None);

                PromiseOrValue::Value(true)
            }
            Option::Some(pending_reward) => {
                let reward = pending_reward.reward;

                self.boxes.claim_box(box_id.clone(), Some(reward.clone()));

                let transfer_promise =
                    self.create_transfer_reward_promise(reward.clone(), &account_id);
                let callback_promise = self.create_on_transfer_reward_callback_promise(
                    &account_id,
                    &box_id,
                    &pending_reward.pool_id,
                    &pending_reward.id,
                );

                let promise = transfer_promise.then(callback_promise);
                PromiseOrValue::Promise(promise)
            }
        }
    }

    #[private]
    pub fn on_transfer_reward_callback(
        &mut self,
        account_id: AccountId,
        box_id: BoxId,
        pool_id: PoolId,
        pending_reward_id: PendingRewardId,
    ) -> bool {
        // https://docs.rs/near-sdk/latest/near_sdk/env/fn.promise_results_count.html
        require!(env::promise_results_count() == 1, "ERR_TOO_MANY_RESULTS");

        let transfer_result = env::promise_result(0);

        let rarity = self.boxes.get_box_rarity(&box_id);

        match transfer_result {
            PromiseResult::Successful(_) => {
                log!(
                    "Successfully transferred box {} reward of {:?} rarity to {}",
                    box_id,
                    rarity,
                    account_id
                );

                self.rewards
                    .confirm_pending_reward(pool_id, pending_reward_id);

                true
            }
            _ => {
                log!(
                    "Something failed while transferring box {} reward of {:?} rarity to {}",
                    box_id,
                    rarity,
                    account_id
                );

                self.rewards
                    .return_pending_reward(pool_id, pending_reward_id);
                let token = self.internal_nft_mint(account_id, rarity);
                self.token_to_box.insert(&token.token_id, &box_id);
                self.boxes.revert_claim_box(box_id, token.token_id);

                false
            }
        }
    }

    pub fn nft_on_transfer(
        &mut self,
        _sender_id: AccountId,
        previous_owner_id: AccountId,
        token_id: TokenId,
        msg: String,
    ) -> PromiseOrValue<bool> {
        let nft_account_id = env::predecessor_account_id();

        // we're required to ensure that the predecessor account is whitelisted, since the function is public
        require!(
            self.whitelisted_nft_contracts.contains(&nft_account_id),
            "ERR_PREDECESSOR_NOT_WHITELISTED",
        );

        require!(self.owner_id == previous_owner_id, "ERR_FORBIDDEN");

        let rarity =
            serde_json::from_value::<BoxRarity>(Value::String(msg)).expect("ERR_PARSE_MSG");

        // TODO: add storage management
        self.rewards.add_nft_pool(rarity, nft_account_id, token_id);

        // stands for OK response
        PromiseOrValue::Value(false)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use near_sdk::json_types::U128;
    use near_sdk::test_utils::VMContextBuilder;

    use near_sdk::{testing_env, AccountId, ONE_NEAR};

    use crate::json::{JsonBoxStatus, JsonPoolRewards, JsonReward};
    use crate::types::BoxRarity;
    use crate::Contract;

    fn owner() -> AccountId {
        AccountId::from_str("owner").unwrap()
    }

    fn user1() -> AccountId {
        AccountId::from_str("user1").unwrap()
    }

    fn user2() -> AccountId {
        AccountId::from_str("user2").unwrap()
    }

    fn nft() -> AccountId {
        AccountId::from_str("nft_contract").unwrap()
    }

    fn setup() -> (Contract, VMContextBuilder) {
        let mut context = VMContextBuilder::new();

        context.predecessor_account_id(owner());

        testing_env!(context.build());

        let contract = Contract::new(
            AccountId::from_str("414").unwrap(),
            AccountId::from_str("4142").unwrap(),
        );

        (contract, context)
    }

    #[test]
    fn test_setup_succeeds() {
        setup();
    }

    #[test]
    #[should_panic(expected = "ERR_FORBIDDEN")]
    fn test_ownership() {
        let (mut contract, _) = setup();

        assert_eq!(contract.owner_id, owner());

        contract.set_owner(user1());

        assert_eq!(contract.owner_id, user1());

        // should panic since the predecessor is "owner" that no more has ownership
        contract.set_owner(user2());
    }

    #[test]
    #[should_panic]
    fn test_add_small_near_pool_with_panic() {
        let (mut contract, _) = setup();

        contract.add_near_reward(BoxRarity::Rare, U128(ONE_NEAR / 10 - 1), 10);
    }

    #[test]
    fn test_add_near_pool() {
        let (mut contract, mut context) = setup();

        testing_env!(context.attached_deposit(50 * ONE_NEAR * 2).build());

        contract.add_near_reward(BoxRarity::Rare, U128(ONE_NEAR), 50);

        let rewards = contract.get_available_rewards(BoxRarity::Rare, None);

        assert_eq!(rewards.len(), 1);

        let reward = rewards.get(0).unwrap().to_owned();

        assert_eq!(
            reward,
            JsonPoolRewards::Near {
                amount: U128(ONE_NEAR),
                available: 50
            }
        );
    }

    #[should_panic(expected = "ERR_NEAR_POOL_ALREADY_EXIST")]
    #[test]
    fn test_add_near_pools_with_similar_ids_with_panic() {
        let (mut contract, mut context) = setup();

        testing_env!(context.attached_deposit(10 * ONE_NEAR).build());

        contract.add_near_reward(BoxRarity::Rare, U128(ONE_NEAR), 5);
        contract.add_near_reward(BoxRarity::Rare, U128(ONE_NEAR), 5);
    }

    #[test]
    fn test_add_few_near_pools_in_one_block() {
        let (mut contract, mut context) = setup();

        testing_env!(context
            .block_timestamp(0)
            .attached_deposit(10 * ONE_NEAR)
            .build());

        contract.add_near_reward(BoxRarity::Rare, U128(ONE_NEAR), 5);

        testing_env!(context.block_timestamp(1).build());
        contract.add_near_reward(BoxRarity::Rare, U128(ONE_NEAR), 5);

        testing_env!(context.block_timestamp(2).build());
        contract.add_near_reward(BoxRarity::Rare, U128(ONE_NEAR), 5);

        testing_env!(context.block_timestamp(3).build());
        contract.add_near_reward(BoxRarity::Rare, U128(ONE_NEAR), 5);

        let rewards = contract.get_available_rewards(BoxRarity::Rare, None);

        assert_eq!(rewards.len(), 4);
    }

    #[test]
    fn test_add_nft_pool() {
        let (mut contract, mut context) = setup();

        testing_env!(context.build());

        contract.whitelist_nft_contract(nft());

        testing_env!(context.predecessor_account_id(nft()).build());

        contract.nft_on_transfer(
            owner(),
            owner(),
            "some_token".to_string(),
            "rare".to_string(),
        );

        let rewards = contract.get_available_rewards(BoxRarity::Rare, None);

        assert_eq!(rewards.len(), 1);

        let reward = rewards.get(0).unwrap().to_owned();

        assert_eq!(
            reward,
            JsonPoolRewards::NonFungibleToken {
                contract_id: nft(),
                token_ids: vec!["some_token".to_string()]
            }
        );
    }

    #[should_panic(expected = "ERR_PREDECESSOR_NOT_WHITELISTED")]
    #[test]
    fn test_add_nft_pool_without_whitelist_with_panic() {
        let (mut contract, mut context) = setup();

        testing_env!(context.predecessor_account_id(nft()).build());

        contract.nft_on_transfer(
            owner(),
            owner(),
            "some_token".to_string(),
            "rare".to_string(),
        );
    }

    #[test]
    fn test_add_nft_pool_of_different_rarity() {
        let (mut contract, mut context) = setup();

        testing_env!(context.build());

        contract.whitelist_nft_contract(nft());

        testing_env!(context.predecessor_account_id(nft()).build());

        contract.nft_on_transfer(
            owner(),
            owner(),
            "some_token".to_string(),
            "rare".to_string(),
        );
        contract.nft_on_transfer(
            owner(),
            owner(),
            "some_token_2".to_string(),
            "epic".to_string(),
        );
        contract.nft_on_transfer(
            owner(),
            owner(),
            "some_token_3".to_string(),
            "rare".to_string(),
        );

        let rare_rewards = contract.get_available_rewards(BoxRarity::Rare, None);
        assert_eq!(rare_rewards.len(), 1);
        let epic_rewards = contract.get_available_rewards(BoxRarity::Epic, None);
        assert_eq!(epic_rewards.len(), 1);

        let rare_reward = rare_rewards.get(0).unwrap().to_owned();

        assert_eq!(
            rare_reward,
            JsonPoolRewards::NonFungibleToken {
                contract_id: nft(),
                token_ids: vec!["some_token".to_string(), "some_token_3".to_string()]
            }
        );

        let epic_reward = epic_rewards.get(0).unwrap().to_owned();

        assert_eq!(
            epic_reward,
            JsonPoolRewards::NonFungibleToken {
                contract_id: nft(),
                token_ids: vec!["some_token_2".to_string()]
            }
        );
    }

    #[test]
    fn test_mint_box() {
        let (mut contract, mut context) = setup();

        testing_env!(context.attached_deposit(50 * ONE_NEAR * 2).build());

        contract.nft_mint(user1(), BoxRarity::Rare);

        let boxes = contract.get_account_boxes(user1(), None);

        assert_eq!(boxes.len(), 1);

        let box_data = boxes.get(0).unwrap().to_owned();

        assert_eq!(
            box_data.status,
            JsonBoxStatus::NonClaimed {
                token_id: "1".to_string()
            }
        );
    }

    #[should_panic(expected = "ERR_NO_POOLS_AVAILABLE")]
    #[test]
    fn test_burn_box_no_pools_with_panic() {
        let (mut contract, mut context) = setup();

        testing_env!(context.attached_deposit(50 * ONE_NEAR * 2).build());

        contract.nft_mint(user1(), BoxRarity::Rare);

        testing_env!(context
            .attached_deposit(0)
            .predecessor_account_id(user1())
            .build());

        contract.nft_burn("1".to_string());
    }

    #[should_panic(expected = "ERR_ONLY_OWNER_CAN_BURN")]
    #[test]
    fn test_burn_box_as_another_user_with_panic() {
        let (mut contract, mut context) = setup();

        testing_env!(context.attached_deposit(ONE_NEAR).build());

        contract.nft_mint(user1(), BoxRarity::Rare);

        testing_env!(context
            .attached_deposit(0)
            .predecessor_account_id(user2())
            .build());

        contract.nft_burn("1".to_string());
    }

    #[should_panic(expected = "ERR_TOKEN_NOT_FOUND")]
    #[test]
    fn test_burn_box_twice_with_panic() {
        let (mut contract, mut context) = setup();

        testing_env!(context.attached_deposit(5 * ONE_NEAR * 2).build());

        contract.add_near_reward(BoxRarity::Rare, U128(ONE_NEAR), 5);
        contract.nft_mint(user1(), BoxRarity::Rare);

        testing_env!(context
            .attached_deposit(0)
            .predecessor_account_id(user1())
            .build());

        contract.nft_burn("1".to_string());
        contract.nft_burn("1".to_string());
    }

    #[test]
    fn test_burn_box_with_reward() {
        let (mut contract, mut context) = setup();

        testing_env!(context.attached_deposit(50 * ONE_NEAR * 2).build());

        contract.add_near_reward(BoxRarity::Rare, U128(ONE_NEAR), 5);
        contract.nft_mint(user1(), BoxRarity::Rare);

        testing_env!(context
            .attached_deposit(0)
            .predecessor_account_id(user1())
            .build());

        // promises aren't called
        contract.nft_burn("1".to_string());

        let rewards = contract.get_available_rewards(BoxRarity::Rare, None);

        assert_eq!(rewards.len(), 1);

        let reward = rewards.get(0).unwrap().to_owned();

        // availability decreased
        assert_eq!(
            reward,
            JsonPoolRewards::Near {
                amount: U128(ONE_NEAR),
                available: 4
            }
        );
    }

    #[test]
    fn test_burn_box_with_nft_reward() {
        let (mut contract, mut context) = setup();

        testing_env!(context.attached_deposit(ONE_NEAR).build());

        contract.whitelist_nft_contract(nft());
        contract.nft_mint(user1(), BoxRarity::Rare);

        testing_env!(context.predecessor_account_id(nft()).build());

        contract.nft_on_transfer(
            owner(),
            owner(),
            "some_token".to_string(),
            "rare".to_string(),
        );

        testing_env!(context
            .attached_deposit(0)
            .predecessor_account_id(user1())
            .build());

        contract.nft_burn("1".to_string());

        let boxes = contract.get_account_boxes(user1(), None);

        assert_eq!(boxes.len(), 1);

        let box_data = boxes.get(0).unwrap().to_owned();

        assert_eq!(
            box_data.status,
            JsonBoxStatus::Claimed {
                reward: JsonReward::NonFungibleToken {
                    contract_id: nft(),
                    token_id: "some_token".to_string()
                }
            }
        );

        let rewards = contract.get_available_rewards(BoxRarity::Rare, None);

        // pool became empty
        assert_eq!(rewards.len(), 0);
    }
}

// near_contract_standards::impl_non_fungible_token_enumeration!(Contract, nft);

// let get_iah_verification_promise = Promise::new(self.registry_iah_contract.clone())
// .function_call(
//     "sbt_tokens_by_owner".to_string(),
//     serde_json::json!({
//         "issuer": self.issuer_iah_contract.clone(),
//         "account": account_id.clone()
//     })
//     .to_string()
//     .into_bytes(),
//     0,
//     Gas::ONE_TERA * 5,
// );

// let verification_callback_promise = Self::ext(env::current_account_id())
// .with_static_gas(Gas::ONE_TERA * 5)
// .on_iah_verification_callback();

// let callback_promise = Self::ext(env::current_account_id())
// .with_static_gas(Gas::ONE_TERA * 5)
// .on_register_callback(account_id.clone());

// get_iah_verification_promise
// .then(verification_callback_promise)
// .then(callback_promise)

// #[private]
// pub fn on_register_callback(&mut self, participant_id: AccountId) -> bool {
//     // https://docs.rs/near-sdk/latest/near_sdk/env/fn.promise_results_count.html
//     assert_eq!(env::promise_results_count(), 1, "ERR_TOO_MANY_RESULTS");

//     let verified_result = env::promise_result(0);

//     match verified_result {
//         PromiseResult::Failed => {
//             log!("Something failed while validating verification status",);

//             self.participants.remove(&participant_id);

//             false
//         }
//         PromiseResult::NotReady => {
//             log!("Promise for on_register_callback isn't ready yet");

//             self.participants.remove(&participant_id);

//             false
//         }
//         PromiseResult::Successful(data) => {
//             let result = bool::try_from_slice(data.as_slice());

//             if result.is_err() {
//                 log!("Couldn't unwrap result from successful promise");

//                 self.participants.remove(&participant_id);

//                 return false;
//             }

//             let verified = result.unwrap();

//             if verified {
//                 log!(
//                     "Participant {} is verified and was successfully registered",
//                     participant_id,
//                 );

//                 return true;
//             } else {
//                 self.participants.remove(&participant_id);

//                 log!(
//                     "Participant {} is not verified, reverting registration status",
//                     participant_id
//                 );

//                 return false;
//             }
//         }
//     }
// }

// for _ in participant.mintable_box_raritys.iter().take(5) {
//     let box_rarity = self.participants.use_first_mintable_box(&account_id);

//     self.internal_nft_mint(account_id.clone(), box_rarity.clone());
// }
