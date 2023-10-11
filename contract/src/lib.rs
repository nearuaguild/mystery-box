use json::JsonBoxData;
use json::JsonReward;
use json::Pagination;
use mystery_box::MysteryBoxContainer;
use near_contract_standards::non_fungible_token::metadata::TokenMetadata;
use near_contract_standards::non_fungible_token::{NonFungibleToken, Token, TokenId};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, LookupSet};
use near_sdk::serde_json::{self, Value};
use near_sdk::{
    env, log, near_bindgen, require, AccountId, Balance, BorshStorageKey, Gas, PanicOnDefault,
    Promise, PromiseOrValue, PromiseResult,
};
use reward_pool::RewardPoolContainer;
use types::{BoxId, BoxRarity, Capacity, Reward, RewardPool, RewardPoolId};

mod json;
mod mystery_box;
mod reward_pool;
mod types;
mod utils;

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
            "Only owner is capable of calling this function"
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
    pub fn add_near_reward(&mut self, box_rarity: BoxRarity, amount: Balance, capacity: Capacity) {
        self.assert_only_owner();

        let reward_deposit = amount * capacity as u128;

        assert!(
            env::attached_deposit() > reward_deposit,
            "Deposited amount must be bigger than added rewards - {} yocto",
            reward_deposit
        );

        let storage_used_before = env::storage_usage();

        let reward = Reward::Near { amount };
        self.rewards.add_reward_pool(reward, box_rarity, capacity);

        let storage_used_after = env::storage_usage();

        let storage_deposit =
            env::storage_byte_cost() * (storage_used_after - storage_used_before) as u128;

        let total_deposit = storage_deposit + reward_deposit.clone();

        assert!(
            env::attached_deposit() == total_deposit,
            "Deposited amount must be equal to {} yocto",
            total_deposit
        );
    }

    pub fn get_account_boxes(
        &self,
        account_id: AccountId,
        pagination: Option<Pagination>,
    ) -> Vec<JsonBoxData> {
        let pagination = pagination.unwrap_or_default();

        pagination.assert_valid();

        self.boxes
            .get_user_boxes(
                account_id,
                pagination.calculate_limit(),
                pagination.calculate_offset(),
            )
            .iter()
            .map(|box_data| JsonBoxData::from(box_data))
            .collect()
    }

    pub fn get_available_rewards_by_box_rarity(
        &self,
        rarity: BoxRarity,
        pagination: Option<Pagination>,
    ) -> Vec<JsonReward> {
        let pagination = pagination.unwrap_or_default();

        pagination.assert_valid();

        self.rewards
            .find_available_reward_pools(
                rarity,
                pagination.calculate_limit(),
                pagination.calculate_offset(),
            )
            .iter()
            .map(|reward_pool| JsonReward::from(reward_pool))
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

        assert_eq!(
            storage_deposit,
            env::attached_deposit(),
            "Deposited amount must be equal to {} yocto",
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
        reward_pool_id: &RewardPoolId,
    ) -> Promise {
        Promise::new(env::current_account_id()).function_call(
            "on_transfer_reward_callback".to_string(),
            serde_json::json!({
                "account_id": account_id,
                "box_id": box_id,
                "reward_pool_id": reward_pool_id,
            })
            .to_string()
            .into_bytes(),
            0,
            Gas::ONE_TERA * 10,
        )
    }

    fn create_transfer_reward_promise(
        &self,
        reward_pool: &RewardPool,
        receiver_id: &AccountId,
    ) -> Promise {
        match reward_pool.reward.clone() {
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

    pub fn nft_burn(&mut self, token_id: TokenId) -> Promise {
        let account_id = env::predecessor_account_id();

        let owner_id = self.nft.owner_by_id.get(&token_id).expect(
            format!(
                "The token {} doesn't exist or has been already burnt",
                token_id.clone()
            )
            .as_str(),
        );

        require!(
            owner_id == account_id,
            "Only token owner can burn his tokens"
        );

        let box_id = self
            .token_to_box
            .remove(&token_id)
            .expect("Unexpected error, missing token_to_box relation");
        self.internal_nft_burn(token_id, account_id.clone());

        let box_rarity = self.boxes.get_box_rarity(&box_id);

        let reward_pool = self.rewards.find_random_available_reward_pool(box_rarity);
        self.rewards.decrement_availability(reward_pool.id);

        self.boxes
            .claim_box(box_id.clone(), reward_pool.reward.clone());

        let transfer_promise = self.create_transfer_reward_promise(&reward_pool, &account_id);
        let callback_promise =
            self.create_on_transfer_reward_callback_promise(&account_id, &box_id, &reward_pool.id);

        transfer_promise.then(callback_promise)
    }

    #[private]
    pub fn on_transfer_reward_callback(
        &mut self,
        account_id: AccountId,
        box_id: BoxId,
        reward_pool_id: RewardPoolId,
    ) -> bool {
        // https://docs.rs/near-sdk/latest/near_sdk/env/fn.promise_results_count.html
        require!(env::promise_results_count() == 1, "ERR_TOO_MANY_RESULTS");

        let transfer_result = env::promise_result(0);

        let box_rarity = self.boxes.get_box_rarity(&box_id);

        match transfer_result {
            PromiseResult::Failed => {
                log!(
                    "Something failed while transferring box {} reward of {:?} rarity to {}",
                    box_id,
                    box_rarity,
                    account_id
                );

                self.rewards.increment_availability(reward_pool_id);
                let token = self.internal_nft_mint(account_id, box_rarity);
                self.token_to_box.insert(&token.token_id, &box_id);
                self.boxes.revert_claim_box(box_id, token.token_id);

                false
            }
            PromiseResult::NotReady => {
                log!(
                    "Promise isn't ready yet to transfer box {} reward of {:?} rarity to {}",
                    box_id,
                    box_rarity,
                    account_id
                );

                self.rewards.increment_availability(reward_pool_id);
                let token = self.internal_nft_mint(account_id, box_rarity);
                self.token_to_box.insert(&token.token_id, &box_id);
                self.boxes.revert_claim_box(box_id, token.token_id);

                false
            }
            PromiseResult::Successful(_) => {
                log!(
                    "Successfully transferred box {} reward of {:?} rarity to {}",
                    box_id,
                    box_rarity,
                    account_id
                );

                true
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
        assert!(
            self.whitelisted_nft_contracts.contains(&nft_account_id),
            "The NFT contract {} isn't whitelisted to be stored as reward",
            nft_account_id
        );

        require!(
            self.owner_id == previous_owner_id,
            "Only owner is able to transfer NFT as reward"
        );

        let result = serde_json::from_value::<BoxRarity>(Value::String(msg));

        require!(result.is_ok(), "Can't parse BoxRarity value from msg");

        let box_rarity = result.unwrap();

        // TODO: add storage management
        let reward = Reward::NonFungibleToken {
            contract_id: nft_account_id,
            token_id: token_id,
        };
        self.rewards.add_reward_pool(reward, box_rarity, 1);

        // stands for OK response
        PromiseOrValue::Value(false)
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
