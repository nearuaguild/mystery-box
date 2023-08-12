use near_contract_standards::non_fungible_token::metadata::TokenMetadata;
use near_contract_standards::non_fungible_token::{NonFungibleToken, Token, TokenId};
use near_sdk::__private::schemars::Set;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, LookupSet, UnorderedSet};
use near_sdk::serde_json::{self, json, Value};
use near_sdk::{
    env, log, near_bindgen, require, AccountId, BorshStorageKey, Gas, PanicOnDefault, Promise,
    PromiseResult, Timestamp,
};
use types::{Bounds, BoxType, InputReward, Reward, RewardId, RewardToken, State};
use utils::{get_media_for_box_type, get_random_number};

mod types;
mod utils;

pub const TGAS: u64 = 1_000_000_000_000;

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    AvailableRewards,
    AvailableLegendaryRewards,
    AvailableRareRewards,
    AvailableCommonRewards,
    Rewards,
    UnclaimedTokens,
    BoxTypeByToken,
    Participants,
}

#[derive(BorshSerialize, BorshStorageKey)]
enum ExtStorageKey {
    NonFungibleToken,
    TokenMetadata,
    Enumeration,
    Approval,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    tokens: NonFungibleToken,
    owner_id: AccountId,
    rewards: LookupMap<RewardId, Reward>,
    available_rewards: LookupMap<BoxType, UnorderedSet<RewardId>>,
    total_rewards: u32,
    unclaimed_tokens: LookupSet<TokenId>,
    next_reward_id: RewardId,
    box_type_by_token_id: LookupMap<TokenId, BoxType>,
    next_token_id: u32,
    // all time fields are in nanoseconds
    registration_start_time_ns: Option<Timestamp>,
    registration_end_time_ns: Option<Timestamp>,
    claiming_start_time_ns: Option<Timestamp>,
    participants: LookupSet<AccountId>,
    // https://github.com/near-ndc/i-am-human
    registry_iah_contract: AccountId,
    issuer_iah_contract: AccountId,
    default_title: String,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(
        registry_iah_contract: AccountId,
        issuer_iah_contract: AccountId,
        default_title: String,
    ) -> Self {
        let owner_id = env::predecessor_account_id();

        let mut available_rewards = LookupMap::new(StorageKey::AvailableRewards);

        available_rewards.insert(
            &BoxType::Common,
            &UnorderedSet::new(StorageKey::AvailableCommonRewards),
        );
        available_rewards.insert(
            &BoxType::Rare,
            &UnorderedSet::new(StorageKey::AvailableRareRewards),
        );
        available_rewards.insert(
            &BoxType::Legendary,
            &UnorderedSet::new(StorageKey::AvailableLegendaryRewards),
        );

        Self {
            tokens: NonFungibleToken::new(
                ExtStorageKey::NonFungibleToken,
                owner_id.clone(),
                Some(ExtStorageKey::TokenMetadata),
                Some(ExtStorageKey::Enumeration),
                Some(ExtStorageKey::Approval),
            ),
            owner_id: owner_id.clone(),
            rewards: LookupMap::new(StorageKey::Rewards),
            available_rewards,
            unclaimed_tokens: LookupSet::new(StorageKey::UnclaimedTokens),
            total_rewards: 0,
            next_reward_id: 0,
            next_token_id: 1,
            box_type_by_token_id: LookupMap::new(StorageKey::BoxTypeByToken),
            registration_start_time_ns: None,
            registration_end_time_ns: None,
            claiming_start_time_ns: None,
            participants: LookupSet::new(StorageKey::Participants),
            registry_iah_contract,
            issuer_iah_contract,
            default_title,
        }
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

    fn internal_add_reward(
        &mut self,
        box_type: &BoxType,
        token: RewardToken,
        bounds: Bounds,
    ) -> RewardId {
        let reward_id = self.next_reward_id.clone();
        self.next_reward_id += 1;

        let reward = Reward {
            id: reward_id,
            token,
            bounds,
        };

        self.rewards.insert(&reward_id, &reward);

        // it's not gonna fail because we initialized state before inside "new" function
        let mut available_rewards_by_type = self.available_rewards.get(box_type).unwrap();

        available_rewards_by_type.insert(&reward_id);

        self.available_rewards
            .insert(&box_type, &available_rewards_by_type);

        self.total_rewards += 1;

        reward_id
    }

    pub fn add_near_reward(&mut self, box_type: BoxType, reward: InputReward) {
        self.assert_only_owner();

        require!(
            self.internal_state() == State::Created,
            "Too late to add rewards"
        );

        reward.assert_valid();

        let total = reward.supply as u128 * reward.bounds.max;

        // not possible, just double-checking
        require!(total > 0, "Total amount of rewards can't be 0");

        assert_eq!(
            total,
            env::attached_deposit(),
            "Deposited amount must be equal to total rewards amount ({} yocto)",
            total
        );

        for _ in 1..reward.supply {
            self.internal_add_reward(&box_type, RewardToken::Near, reward.bounds.clone());
        }

        // TODO: track amount of tokens that haven't been distributed
    }

    pub fn start_registration(&mut self, when: Option<Timestamp>) {
        self.assert_only_owner();

        // either right now, or provided time in nanoseconds
        let time_ns = when.unwrap_or_else(|| env::block_timestamp());

        require!(
            self.registration_start_time_ns.is_none(),
            "Can't override registration start time"
        );

        self.registration_start_time_ns = Some(time_ns);
    }

    pub fn stop_registration(&mut self, when: Option<Timestamp>) {
        self.assert_only_owner();

        // either right now, or provided time in nanoseconds
        let time_ns = when.unwrap_or_else(|| env::block_timestamp());

        require!(
            self.registration_start_time_ns.is_some(),
            "End registration time can't be set if start time wasn't provided"
        );

        require!(
            self.registration_start_time_ns.unwrap() < time_ns,
            "End registration time must be later than start"
        );

        require!(
            self.registration_end_time_ns.is_none(),
            "Can't override registration end time"
        );

        self.registration_end_time_ns = Some(time_ns);
    }

    pub fn start_claiming(&mut self, when: Option<Timestamp>) {
        self.assert_only_owner();

        require!(
            self.internal_state() == State::RegistrationEnded,
            "Can't start claiming before registration ends"
        );

        // either right now, or provided time in nanoseconds
        let time_ns = when.unwrap_or_else(|| env::block_timestamp());

        require!(
            self.claiming_start_time_ns.is_none(),
            "Can't override claiming start time"
        );

        self.claiming_start_time_ns = Some(time_ns);
    }

    pub fn get_registration_start_time(&self) -> Option<Timestamp> {
        self.registration_start_time_ns
    }

    pub fn get_registration_end_time(&self) -> Option<Timestamp> {
        self.registration_end_time_ns
    }

    pub fn get_claiming_start_time(&self) -> Option<Timestamp> {
        self.claiming_start_time_ns
    }

    pub fn register(&mut self) -> Promise {
        require!(self.internal_state() == State::RegistrationStarted);

        let participant_id = env::predecessor_account_id();

        require!(
            // If "false" is returned, it indicates that the user was previously registered.
            self.participants.insert(&participant_id),
            "You have already completed registration"
        );

        let cross_contract_gas = Gas(5 * TGAS);

        let get_iah_verification_promise = Promise::new(self.registry_iah_contract.clone())
            .function_call(
                "sbt_tokens_by_owner".to_string(),
                json!({
                    "issuer": self.issuer_iah_contract.clone(),
                    "account": participant_id.clone()
                })
                .to_string()
                .into_bytes(),
                0,
                cross_contract_gas,
            );

        let callback_promise = Self::ext(env::current_account_id())
            .with_static_gas(cross_contract_gas)
            .on_register_callback(participant_id.clone());

        get_iah_verification_promise.then(callback_promise)
    }

    #[private]
    pub fn on_register_callback(&mut self, participant_id: AccountId) -> bool {
        // https://docs.rs/near-sdk/latest/near_sdk/env/fn.promise_results_count.html
        assert_eq!(env::promise_results_count(), 1, "ERR_TOO_MANY_RESULTS");

        let iah_result = env::promise_result(0);

        match iah_result {
            PromiseResult::Failed => {
                log!(
                    "Something failed while getting data from IAH registry {}",
                    self.registry_iah_contract.clone()
                );

                self.participants.remove(&participant_id);

                false
            }
            PromiseResult::NotReady => {
                log!("Promise isn't ready yet");

                self.participants.remove(&participant_id);

                false
            }
            PromiseResult::Successful(data) => {
                let deserealize_result = serde_json::from_slice::<Value>(data.as_slice());

                match deserealize_result {
                    Err(_) => {
                        log!("Couldn't deserialize cross-contract results");

                        self.participants.remove(&participant_id);

                        false
                    }
                    Ok(data) => {
                        let verification_result = data.pointer("/0/1/0");

                        match verification_result {
                            None => {
                                log!(
                                    "Verification data about {} not found in registry {} for issuer {}",
                                    participant_id,
                                    self.registry_iah_contract,
                                    self.issuer_iah_contract
                                );

                                self.participants.remove(&participant_id);

                                false
                            }
                            Some(_) => {
                                log!(
                                    "Confirming that user {} is verified in registry {}",
                                    participant_id,
                                    self.registry_iah_contract
                                );

                                true
                            }
                        }
                    }
                }
            }
        }
    }

    fn internal_nft_mint(&mut self, box_type: BoxType, account: AccountId) {
        require!(
            self.participants.contains(&account),
            "Only participants are eligible for mint"
        );

        let token_id = self.next_token_id.clone().to_string();
        self.next_token_id += 1;

        let title = format!("{}: #{}", self.default_title, &token_id);
        let media = get_media_for_box_type(&box_type);

        let metadata = TokenMetadata {
            title: Some(title),
            description: None,
            copies: Some(1),
            expires_at: None,
            issued_at: None,
            media,
            media_hash: None,
            reference: None,
            reference_hash: None,
            starts_at: None,
            updated_at: None,
            extra: None,
        };

        self.tokens
            .internal_mint(token_id.clone(), account, Some(metadata));

        self.unclaimed_tokens.insert(&token_id);
        self.box_type_by_token_id.insert(&token_id, &box_type);
    }

    pub fn nft_mint(&mut self, box_type: BoxType, account: AccountId) {
        self.assert_only_owner();

        require!(self.internal_state() == State::RegistrationEnded);

        // TODO: make sure amount of tokens won't exceed rewards amount for box type
        // let total_tokens = self.next_token_id - 1;
        // require!(total_tokens + 1 <= self.total_rewards, "");

        self.internal_nft_mint(box_type, account);
    }

    pub fn nft_mint_many(&mut self, box_type: BoxType, accounts: Set<AccountId>) {
        self.assert_only_owner();

        require!(self.internal_state() == State::RegistrationEnded);

        require!(accounts.len() >= 1, "At least one account must be provided");
        require!(
            accounts.len() <= 10,
            "The maximum amount of accounts to provide is 10"
        );

        for account in accounts {
            self.internal_nft_mint(box_type.clone(), account);
        }
    }

    pub fn nft_is_claimed(&self, token_id: TokenId) -> bool {
        !self.unclaimed_tokens.contains(&token_id)
    }

    fn internal_state(&self) -> State {
        if self.claiming_start_time_ns.is_some()
            && env::block_timestamp() >= self.claiming_start_time_ns.unwrap()
        {
            return State::ClaimStarted;
        }

        if self.registration_end_time_ns.is_some()
            && env::block_timestamp() >= self.registration_end_time_ns.unwrap()
        {
            return State::RegistrationEnded;
        }

        if self.registration_start_time_ns.is_some()
            && env::block_timestamp() >= self.registration_start_time_ns.unwrap()
        {
            return State::RegistrationStarted;
        }

        return State::Created;
    }

    pub fn claim_reward(&mut self, token_id: TokenId) -> Promise {
        require!(self.internal_state() == State::ClaimStarted);

        let participant_id = env::predecessor_account_id();

        let token_owner = self.tokens.owner_by_id.get(&token_id).unwrap_or_else(|| {
            env::panic_str(format!("The token {} doesn't exist", token_id.clone()).as_str())
        });

        require!(
            token_owner == participant_id,
            format!(
                "The token {} isn't owned by {}",
                token_id.clone(),
                participant_id.clone()
            )
        );

        require!(
            // If "false" is returned, it indicates that the reward was previously claimed
            self.unclaimed_tokens.remove(&token_id),
            format!(
                "The reward for token {} has already been claimed",
                token_id.clone()
            )
        );

        // 'unwrap' is not gonna fail because we previously checked that the token exists, s
        let box_type = self.box_type_by_token_id.get(&token_id).unwrap();

        let mut available_rewards_for_box = self.available_rewards.get(&box_type).unwrap();

        let seed = get_random_number(0) as u64;

        let index = seed % available_rewards_for_box.len();

        let reward_id = available_rewards_for_box.as_vector().get(index).unwrap();

        available_rewards_for_box.remove(&reward_id);
        self.available_rewards
            .insert(&box_type, &available_rewards_for_box);

        let reward = self.rewards.get(&reward_id).unwrap();

        let amount = reward.generate_random_amount_within_bounds(8);

        match reward.token {
            RewardToken::Near => {
                let cross_contract_gas = Gas(5 * TGAS);

                let transfer_promise =
                    Promise::new(participant_id.clone()).transfer(amount.clone());
                let callback_promise = Self::ext(env::current_account_id())
                    .with_static_gas(cross_contract_gas)
                    .on_claim_near_reward_callback(
                        participant_id.clone(),
                        box_type,
                        reward_id,
                        amount.clone(),
                    );

                transfer_promise.then(callback_promise)
            }
        }

        // TODO: randomly determine amount of the reward and transfer
    }

    #[private]
    pub fn on_claim_near_reward_callback(
        &mut self,
        participant_id: AccountId,
        box_type: BoxType,
        reward_id: RewardId,
        amount: u128,
    ) -> bool {
        // https://docs.rs/near-sdk/latest/near_sdk/env/fn.promise_results_count.html
        assert_eq!(env::promise_results_count(), 1, "ERR_TOO_MANY_RESULTS");

        let transfer_result = env::promise_result(0);

        match transfer_result {
            PromiseResult::Failed => {
                log!(
                    "Something failed while transferring {} yocto to {}",
                    amount,
                    participant_id
                );

                // revert state back
                let mut available_rewards_for_box = self.available_rewards.get(&box_type).unwrap();
                available_rewards_for_box.insert(&reward_id);
                self.available_rewards
                    .insert(&box_type, &available_rewards_for_box);

                false
            }
            PromiseResult::NotReady => {
                log!(
                    "Promise isn't ready yet to transfer {} yocto to {}",
                    amount,
                    participant_id
                );

                // revert state back
                let mut available_rewards_for_box = self.available_rewards.get(&box_type).unwrap();
                available_rewards_for_box.insert(&reward_id);
                self.available_rewards
                    .insert(&box_type, &available_rewards_for_box);

                false
            }
            PromiseResult::Successful(_) => {
                log!(
                    "Successfully transferred {} yocto to {} as reward",
                    amount,
                    participant_id
                );

                true
            }
        }
    }
}

near_contract_standards::impl_non_fungible_token_enumeration!(Contract, tokens);
