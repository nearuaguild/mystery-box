use crate::*;

pub(crate) fn create_withdraw_box_reward_promise_with_verification(
    account_id: &AccountId,
    box_id: &BoxId,
    pool_id: &PoolId,
) -> Promise {
    let get_iah_verification_promise = Promise::new(get_registry_iah_contract()).function_call(
        "sbt_tokens_by_owner".to_string(),
        serde_json::json!({
            "issuer": get_issuer_iah_contract(),
            "account": account_id.clone()
        })
        .to_string()
        .into_bytes(),
        0,
        Gas::ONE_TERA * 5,
    );

    let on_iah_verification_callback_promise = Contract::ext(env::current_account_id())
        .with_static_gas(Gas::ONE_TERA * 10)
        .check_iah_verification_and_claim_callback(
            account_id.to_owned(),
            box_id.to_owned(),
            pool_id.to_owned(),
        );

    get_iah_verification_promise.then(on_iah_verification_callback_promise)
}

pub(crate) fn create_withdraw_box_reward_promise(
    receiver_id: &AccountId,
    box_id: &BoxId,
    pool_id: &PoolId,
    reward: &Reward,
) -> Promise {
    let transfer_promise = create_transfer_reward_promise(receiver_id, reward);
    let on_transfer_promise =
        create_transfer_reward_callback_promise(receiver_id, box_id, pool_id, reward);

    transfer_promise.then(on_transfer_promise)
}

fn create_transfer_reward_callback_promise(
    account_id: &AccountId,
    box_id: &BoxId,
    pool_id: &PoolId,
    reward: &Reward,
) -> Promise {
    Contract::ext(env::current_account_id()).transfer_reward_callback(
        account_id.to_owned(),
        box_id.to_owned(),
        pool_id.to_owned(),
        reward.to_owned(),
    )
}

fn create_transfer_reward_promise(receiver_id: &AccountId, reward: &Reward) -> Promise {
    match reward {
        Reward::Near { amount } => Promise::new(receiver_id.clone()).transfer(amount.to_owned()),
        Reward::NonFungibleToken {
            contract_id,
            token_id,
        } => Promise::new(contract_id.to_owned()).function_call(
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

#[near_bindgen]
impl Contract {
    #[private]
    pub fn check_iah_verification_and_claim_callback(
        &mut self,
        receiver_id: AccountId,
        box_id: BoxId,
        pool_id: PoolId,
    ) -> PromiseOrValue<Option<JsonReward>> {
        // https://docs.rs/near-sdk/latest/near_sdk/env/fn.promise_results_count.html
        assert_eq!(env::promise_results_count(), 1, "ERR_TOO_MANY_RESULTS");

        let iah_result = env::promise_result(0);

        let is_verified = match iah_result {
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
                                    get_registry_iah_contract(),
                                    get_issuer_iah_contract()
                                );

                                false
                            }
                            Some(_) => true,
                        }
                    }
                }
            }
            _ => {
                log!(
                    "Something failed while getting data from IAH registry {}",
                    get_registry_iah_contract()
                );

                false
            }
        };

        if !is_verified {
            self.internal_undo_claim(box_id, pool_id);

            return PromiseOrValue::Value(None);
        };

        let box_data = self.boxes.get(&box_id).expect("ERR_BOX_NOT_FOUND");

        let reward = match box_data.status {
            BoxStatus::NonClaimed => unreachable!(),
            BoxStatus::Claimed { reward } => reward,
        };

        match reward {
            Option::None => PromiseOrValue::Value(Some(reward.into())),
            Option::Some(reward) => PromiseOrValue::Promise(create_withdraw_box_reward_promise(
                &receiver_id,
                &box_id,
                &pool_id,
                &reward,
            )),
        }
    }

    #[private]
    pub fn transfer_reward_callback(
        &mut self,
        account_id: AccountId,
        box_id: BoxId,
        pool_id: PoolId,
        reward: Reward,
    ) -> Option<JsonReward> {
        // https://docs.rs/near-sdk/latest/near_sdk/env/fn.promise_results_count.html
        require!(env::promise_results_count() == 1, "ERR_TOO_MANY_RESULTS");

        let transfer_result = env::promise_result(0);

        match transfer_result {
            PromiseResult::Successful(_) => {
                log!(
                    "Successfully transferred box {} reward to {}",
                    box_id,
                    account_id
                );

                Some(reward.into())
            }
            _ => {
                log!(
                    "Something failed while transferring box {} reward to {}",
                    box_id,
                    account_id
                );

                self.internal_undo_claim(box_id, pool_id);

                None
            }
        }
    }
}
