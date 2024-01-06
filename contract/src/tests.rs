#![allow(unused)]
#[cfg(test)]
use crate::json::{JsonBoxStatus, JsonPoolRewards, JsonReward};
use crate::types::{BoxRarity, Probability};
use crate::{Contract, Pool, Reward};
use near_sdk::json_types::{U128, U64};
use near_sdk::test_utils::VMContextBuilder;
use near_sdk::{testing_env, AccountId, ONE_NEAR};

use std::str::FromStr;

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

fn nft2() -> AccountId {
    AccountId::from_str("nft_contract_2").unwrap()
}

fn setup() -> (Contract, VMContextBuilder) {
    let mut context = VMContextBuilder::new();

    context.predecessor_account_id(owner());
    context.account_balance(100 * ONE_NEAR);

    testing_env!(context.build());

    let contract = Contract::new();

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
#[should_panic(expected = "ERR_FORBIDDEN")]
fn test_set_probability_with_regular_user_panic() {
    let (mut contract, mut context) = setup();

    testing_env!(context.predecessor_account_id(user1()).build());

    contract.set_probability(BoxRarity::Epic, Probability::ZERO);
}

#[should_panic]
#[test]
fn test_set_probability_with_zero_denominator_panic() {
    let (mut contract, _) = setup();

    let probability = Probability {
        numerator: 5,
        denominator: 0,
    };

    contract.set_probability(BoxRarity::Epic, probability);
}

#[should_panic]
#[test]
fn test_set_probability_bigger_than_one_panic() {
    let (mut contract, _) = setup();

    let probability = Probability {
        numerator: 5,
        denominator: 2,
    };

    contract.set_probability(BoxRarity::Epic, probability);
}

#[test]
#[should_panic]
fn test_add_small_near_pool_with_panic() {
    let (mut contract, _) = setup();

    contract.add_near_reward(BoxRarity::Rare, U128(ONE_NEAR / 20), U64(10));
}

#[test]
fn test_add_near_pool_succeeds() {
    let (mut contract, mut context) = setup();

    testing_env!(context.attached_deposit(5 * ONE_NEAR).build());

    contract.add_near_reward(BoxRarity::Rare, U128(ONE_NEAR), U64(3));
}

#[test]
fn test_add_big_near_pool() {
    let (mut contract, mut context) = setup();

    testing_env!(context.attached_deposit(10005 * ONE_NEAR).build());

    contract.add_near_reward(BoxRarity::Rare, U128(ONE_NEAR), U64(10_000));
}

#[test]
fn test_add_multiple_near_pools_succeeds() {
    let (mut contract, mut context) = setup();

    testing_env!(context.attached_deposit(10 * ONE_NEAR).build());

    contract.add_near_reward(BoxRarity::Rare, U128(ONE_NEAR), U64(5));
    contract.add_near_reward(BoxRarity::Rare, U128(ONE_NEAR), U64(5));
}

#[test]
fn test_add_nft_pool_succeeds() {
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
}

#[test]
fn test_add_multiple_nft_pool_succeeds() {
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
        "rare".to_string(),
    );
}

#[should_panic(expected = "ERR_NOT_WHITELISTED")]
#[test]
fn test_add_non_whitelisted_nft_pool_with_panic() {
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
fn test_available_near_rewards_amount() {
    let (mut contract, mut context) = setup();

    testing_env!(context.attached_deposit(10 * ONE_NEAR).build());

    contract.add_near_reward(BoxRarity::Rare, U128(ONE_NEAR), U64(5));

    let rewards = contract.available_rewards(BoxRarity::Rare, None);

    assert_eq!(rewards.len(), 1);

    contract.add_near_reward(BoxRarity::Rare, U128(ONE_NEAR), U64(5));

    let rewards = contract.available_rewards(BoxRarity::Rare, None);

    assert_eq!(rewards.len(), 2);

    contract.add_near_reward(BoxRarity::Epic, U128(ONE_NEAR), U64(5));

    let rewards = contract.available_rewards(BoxRarity::Rare, None);
    assert_eq!(rewards.len(), 2);

    let rewards = contract.available_rewards(BoxRarity::Epic, None);
    assert_eq!(rewards.len(), 1);
}

#[test]
fn test_available_near_rewards_data() {
    let (mut contract, mut context) = setup();

    testing_env!(context.attached_deposit(10 * ONE_NEAR).build());

    contract.add_near_reward(BoxRarity::Rare, U128(ONE_NEAR), U64(5));

    let rewards = contract.available_rewards(BoxRarity::Rare, None);

    let reward = rewards.get(0).unwrap().to_owned();

    assert_eq!(
        reward,
        JsonPoolRewards::Near {
            amount: U128(ONE_NEAR),
            available: 5,
            total: 5
        }
    );
}

#[test]
fn test_available_nft_rewards_amount_for_different_contracts() {
    let (mut contract, mut context) = setup();

    testing_env!(context.build());

    contract.whitelist_nft_contract(nft());
    contract.whitelist_nft_contract(nft2());

    testing_env!(context.predecessor_account_id(nft()).build());
    contract.nft_on_transfer(
        owner(),
        owner(),
        "some_token".to_string(),
        "rare".to_string(),
    );

    let rewards = contract.available_rewards(BoxRarity::Rare, None);
    assert_eq!(rewards.len(), 1);

    testing_env!(context.predecessor_account_id(nft2()).build());
    contract.nft_on_transfer(
        owner(),
        owner(),
        "another_token".to_string(),
        "rare".to_string(),
    );

    let rewards = contract.available_rewards(BoxRarity::Rare, None);
    assert_eq!(rewards.len(), 2);
}

#[test]
fn test_available_nft_rewards_amount_for_same_contract() {
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

    let rewards = contract.available_rewards(BoxRarity::Rare, None);
    assert_eq!(rewards.len(), 1);

    testing_env!(context.predecessor_account_id(nft()).build());
    contract.nft_on_transfer(
        owner(),
        owner(),
        "some_token_2".to_string(),
        "rare".to_string(),
    );

    let rewards = contract.available_rewards(BoxRarity::Rare, None);
    assert_eq!(rewards.len(), 1);
}

#[test]
fn test_available_nft_rewards_amount_for_same_contract_and_different_rarity() {
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

    let rewards = contract.available_rewards(BoxRarity::Rare, None);
    assert_eq!(rewards.len(), 1);

    testing_env!(context.predecessor_account_id(nft()).build());
    contract.nft_on_transfer(
        owner(),
        owner(),
        "some_token_2".to_string(),
        "epic".to_string(),
    );

    let rewards = contract.available_rewards(BoxRarity::Rare, None);
    assert_eq!(rewards.len(), 1);
    let rewards = contract.available_rewards(BoxRarity::Epic, None);
    assert_eq!(rewards.len(), 1);
}

#[test]
fn test_available_nft_rewards_data_for_same_contract() {
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
        "rare".to_string(),
    );

    let rewards = contract.available_rewards(BoxRarity::Rare, None);

    let reward = rewards.get(0).unwrap().to_owned();

    assert_eq!(
        reward,
        JsonPoolRewards::NonFungibleToken {
            contract_id: nft(),
            token_ids: vec!["some_token".to_string(), "some_token_2".to_string()],
            total: 2
        }
    );
}

#[test]
fn test_mint_succeeds() {
    let (mut contract, mut context) = setup();

    testing_env!(context.attached_deposit(ONE_NEAR).build());

    contract.mint(user1(), BoxRarity::Rare);
}

#[test]
fn test_delete_boxes_succeeds() {
    let (mut contract, mut context) = setup();

    testing_env!(context.attached_deposit(ONE_NEAR).build());

    contract.mint(user1(), BoxRarity::Epic);
    contract.mint(user1(), BoxRarity::Rare);

    assert_eq!(contract.supply_for_owner(user1()), U128(2));

    contract.delete_boxes(vec![1]);

    assert_eq!(contract.supply_for_owner(user1()), U128(1));
    assert!(contract.boxes.get(&1).is_none());
}

#[test]
fn test_mint_many_succeeds() {
    let (mut contract, mut context) = setup();

    testing_env!(context.attached_deposit(ONE_NEAR).build());

    contract.mint_many(BoxRarity::Rare, vec![user1(), user2()]);
}

#[should_panic]
#[test]
fn test_mint_many_without_accounts_panic() {
    let (mut contract, mut context) = setup();

    testing_env!(context.attached_deposit(ONE_NEAR).build());

    contract.mint_many(BoxRarity::Rare, vec![]);
}

#[test]
fn test_total_supply_default() {
    let (mut contract, mut context) = setup();

    assert_eq!(contract.total_supply(), U128(0));
}

#[test]
fn test_total_supply_increases() {
    let (mut contract, mut context) = setup();

    testing_env!(context.attached_deposit(ONE_NEAR * 2).build());

    contract.mint(user1(), BoxRarity::Rare);

    assert_eq!(contract.total_supply(), U128(1));

    contract.mint(user1(), BoxRarity::Rare);
    contract.mint(user2(), BoxRarity::Epic);

    assert_eq!(contract.total_supply(), U128(3));

    contract.mint_many(BoxRarity::Legendary, vec![user1(), user2()]);

    assert_eq!(contract.total_supply(), U128(5));
}

#[test]
fn test_supply_for_owner_default() {
    let (mut contract, mut context) = setup();

    assert_eq!(contract.supply_for_owner(user1()), U128(0));
}

#[test]
fn test_supply_for_owner_increases() {
    let (mut contract, mut context) = setup();

    testing_env!(context.attached_deposit(ONE_NEAR * 2).build());

    contract.mint(user1(), BoxRarity::Rare);

    assert_eq!(contract.supply_for_owner(user1()), U128(1));

    contract.mint(user1(), BoxRarity::Rare);
    contract.mint(user2(), BoxRarity::Epic);

    assert_eq!(contract.supply_for_owner(user1()), U128(2));
    assert_eq!(contract.supply_for_owner(user2()), U128(1));

    contract.mint_many(BoxRarity::Legendary, vec![user1(), user2(), user1()]);

    assert_eq!(contract.supply_for_owner(user1()), U128(4));
    assert_eq!(contract.supply_for_owner(user2()), U128(2));
}

#[test]
fn test_boxes_for_owner_amount() {
    let (mut contract, mut context) = setup();

    testing_env!(context.attached_deposit(ONE_NEAR * 2).build());

    contract.mint(user1(), BoxRarity::Rare);

    assert_eq!(contract.boxes_for_owner(user1(), None).len(), 1);

    contract.mint(user1(), BoxRarity::Epic);
    contract.mint(user1(), BoxRarity::Epic);

    contract.mint(user1(), BoxRarity::Legendary);

    assert_eq!(contract.boxes_for_owner(user1(), None).len(), 4);

    contract.mint(user2(), BoxRarity::Epic);

    assert_eq!(contract.boxes_for_owner(user1(), None).len(), 4);
    assert_eq!(contract.boxes_for_owner(user2(), None).len(), 1);
}

#[test]
fn test_boxes_for_owner_status() {
    let (mut contract, mut context) = setup();

    testing_env!(context.attached_deposit(ONE_NEAR * 2).build());

    contract.mint(user1(), BoxRarity::Rare);

    let boxes = contract.boxes_for_owner(user1(), None);

    let box_data = boxes.get(0).unwrap().to_owned();

    assert_eq!(box_data.status, JsonBoxStatus::NonClaimed);
}

#[should_panic(expected = "ERR_NO_POOLS_AVAILABLE")]
#[test]
fn test_claim_without_pools_with_panic() {
    let (mut contract, mut context) = setup();

    testing_env!(context.attached_deposit(ONE_NEAR).build());

    contract.mint(user1(), BoxRarity::Rare);

    testing_env!(context
        .attached_deposit(1)
        .predecessor_account_id(user1())
        .build());

    contract.claim(1);
}

#[should_panic(expected = "ERR_ONLY_OWNER_CAN_BURN")]
#[test]
fn test_claim_as_another_user_with_panic() {
    let (mut contract, mut context) = setup();

    testing_env!(context.attached_deposit(ONE_NEAR).build());

    contract.mint(user1(), BoxRarity::Rare);

    testing_env!(context
        .attached_deposit(1)
        .predecessor_account_id(user2())
        .build());

    contract.claim(1);
}

#[should_panic(expected = "ERR_BOX_NOT_FOUND")]
#[test]
fn test_claim_non_existing_with_panic() {
    let (mut contract, mut context) = setup();

    testing_env!(context.attached_deposit(ONE_NEAR).build());

    contract.add_near_reward(BoxRarity::Rare, U128(ONE_NEAR / 10), U64(1));
    contract.mint(user1(), BoxRarity::Rare);

    testing_env!(context
        .attached_deposit(1)
        .predecessor_account_id(user1())
        .build());

    contract.claim(500000);
}

#[should_panic(expected = "ERR_BOX_ALREADY_CLAIMED")]
#[test]
fn test_claim_twice_with_panic() {
    let (mut contract, mut context) = setup();

    testing_env!(context.attached_deposit(ONE_NEAR).build());

    contract.add_near_reward(BoxRarity::Rare, U128(ONE_NEAR / 10), U64(5));
    contract.mint(user1(), BoxRarity::Rare);

    testing_env!(context
        .attached_deposit(1)
        .predecessor_account_id(user1())
        .build());

    contract.claim(1);
    contract.claim(1);
}

#[test]
fn test_claim_box_status() {
    let (mut contract, mut context) = setup();

    testing_env!(context.attached_deposit(3 * ONE_NEAR).build());

    contract.add_near_reward(BoxRarity::Rare, U128(ONE_NEAR), U64(2));
    contract.mint(user1(), BoxRarity::Rare);

    testing_env!(context
        .attached_deposit(1)
        .predecessor_account_id(user1())
        .build());

    // promises aren't called
    contract.claim(1);

    let boxes = contract.boxes_for_owner(user1(), None);

    let box_data = boxes.get(0).unwrap();

    assert_eq!(
        box_data.status,
        JsonBoxStatus::Claimed {
            reward: JsonReward::Near {
                amount: ONE_NEAR.into()
            }
        }
    );
}

#[test]
fn test_claim_box_with_zero_probability() {
    let (mut contract, mut context) = setup();

    testing_env!(context.attached_deposit(3 * ONE_NEAR).build());

    contract.set_probability(BoxRarity::Rare, Probability::ZERO);
    contract.add_near_reward(BoxRarity::Rare, U128(ONE_NEAR), U64(2));
    contract.mint(user1(), BoxRarity::Rare);

    testing_env!(context
        .attached_deposit(1)
        .predecessor_account_id(user1())
        .build());

    // promises aren't called
    contract.claim(1);

    let boxes = contract.boxes_for_owner(user1(), None);

    let box_data = boxes.get(0).unwrap();

    assert_eq!(
        box_data.status,
        JsonBoxStatus::Claimed {
            reward: JsonReward::Nothing
        }
    );
}

#[test]
fn test_claim_decreases_reward_availability() {
    let (mut contract, mut context) = setup();

    testing_env!(context.attached_deposit(3 * ONE_NEAR).build());

    contract.add_near_reward(BoxRarity::Rare, U128(ONE_NEAR), U64(2));
    contract.mint(user1(), BoxRarity::Rare);

    testing_env!(context
        .attached_deposit(1)
        .predecessor_account_id(user1())
        .build());

    // promises aren't called
    contract.claim(1);

    let rewards = contract.available_rewards(BoxRarity::Rare, None);

    assert_eq!(rewards.len(), 1);

    let reward = rewards.get(0).unwrap().to_owned();

    // availability decreased
    assert_eq!(
        reward,
        JsonPoolRewards::Near {
            amount: ONE_NEAR.into(),
            available: 1,
            total: 2
        }
    );
}

#[test]
fn test_claim_empty_reward_availability() {
    let (mut contract, mut context) = setup();

    testing_env!(context.attached_deposit(3 * ONE_NEAR).build());

    contract.add_near_reward(BoxRarity::Rare, U128(ONE_NEAR), U64(1));
    contract.mint(user1(), BoxRarity::Rare);

    testing_env!(context
        .attached_deposit(1)
        .predecessor_account_id(user1())
        .build());

    // promises aren't called
    contract.claim(1);

    let rewards = contract.available_rewards(BoxRarity::Rare, None);

    assert_eq!(rewards.len(), 0);
}

#[test]
fn test_claim_near_reward_succeeds() {
    let (mut contract, mut context) = setup();

    testing_env!(context.attached_deposit(3 * ONE_NEAR).build());

    contract.add_near_reward(BoxRarity::Rare, U128(ONE_NEAR), U64(2));
    contract.mint(user1(), BoxRarity::Rare);

    testing_env!(context
        .attached_deposit(1)
        .predecessor_account_id(user1())
        .build());

    // promises aren't called
    contract.claim(1);
}

#[test]
fn test_claim_nft_reward_succeeds() {
    let (mut contract, mut context) = setup();

    testing_env!(context.attached_deposit(ONE_NEAR).build());

    contract.whitelist_nft_contract(nft());
    contract.mint(user1(), BoxRarity::Rare);

    // add NFT token as reward
    testing_env!(context.predecessor_account_id(nft()).build());
    contract.nft_on_transfer(
        owner(),
        owner(),
        "some_token".to_string(),
        "rare".to_string(),
    );

    testing_env!(context
        .attached_deposit(1)
        .predecessor_account_id(user1())
        .build());

    contract.claim(1);
}

#[test]
fn test_claim_for_multiple_pools_succeeds() {
    let (mut contract, mut context) = setup();

    testing_env!(context.attached_deposit(50 * ONE_NEAR).build());

    contract.add_near_reward(BoxRarity::Rare, U128(ONE_NEAR), U64(10));
    contract.add_near_reward(BoxRarity::Rare, U128(ONE_NEAR / 2), U64(10));
    contract.add_near_reward(BoxRarity::Rare, U128(ONE_NEAR / 4), U64(12));

    contract.mint(user1(), BoxRarity::Rare);

    testing_env!(context
        .attached_deposit(1)
        .predecessor_account_id(user1())
        .random_seed([2; 32])
        .build());

    contract.claim(1);
}

#[should_panic]
#[test]
fn test_check_verification_and_claim_callback_by_someone_with_panic() {
    let (mut contract, mut context) = setup();

    contract.check_iah_verification_and_claim_callback(user1(), 1, 1);
}

#[should_panic]
#[test]
fn test_transfer_reward_callback_by_someone_with_panic() {
    let (mut contract, mut context) = setup();

    contract.transfer_reward_callback(user1(), 1, 1, Reward::Near { amount: ONE_NEAR });
}
