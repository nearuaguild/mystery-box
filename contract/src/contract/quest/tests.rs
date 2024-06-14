#![allow(unused)]
#[cfg(test)]
use near_sdk::json_types::{U128, U64};
use near_sdk::test_utils::VMContextBuilder;
use near_sdk::{testing_env, AccountId, ONE_NEAR};

use std::str::FromStr;

use crate::contract::enums::BoxRarity;
use crate::contract::json_types::json_reward::JsonPoolRewards;
use crate::contract::quest::test_utils::create_quest;
use crate::contract::types::Probability;
use crate::contract::Contract;

fn owner() -> AccountId {
    AccountId::from_str("owner").unwrap()
}

fn user1() -> AccountId {
    AccountId::from_str("user1").unwrap()
}

fn user2() -> AccountId {
    AccountId::from_str("user2").unwrap()
}

fn user3() -> AccountId {
    AccountId::from_str("user3").unwrap()
}

fn nft() -> AccountId {
    AccountId::from_str("nft_contract").unwrap()
}

fn nft2() -> AccountId {
    AccountId::from_str("nft_contract_2").unwrap()
}

fn nft3() -> AccountId {
    AccountId::from_str("nft_contract_3").unwrap()
}

fn setup() -> (Contract, VMContextBuilder) {
    let mut context = VMContextBuilder::new();

    context.predecessor_account_id(owner());
    context.account_balance(50 * ONE_NEAR);

    testing_env!(context.build());

    //let trusted_nft_contracts = vec![nft(), nft2()];

    let contract = Contract::new();

    (contract, context)
}

#[test]
fn test_setup_succeeds() {
    setup();
}

#[test]
fn test_quest_ownership() {
    let (mut contract, mut context) = setup();
    
    let quest = create_quest(&mut contract, &mut context, None);

    let owner_quests = contract.quests_per_owner(owner());
    assert_eq!(owner_quests.len(), 1);

    let first_quest = owner_quests.get(0);
    assert_eq!(first_quest.is_some(), true);
    
    let first_quest_unwrapped = first_quest.unwrap();

    contract.set_owner( first_quest_unwrapped.quest_id, user1());

    let owner_quests = contract.quests_per_owner(owner());
    assert_eq!(owner_quests.len(), 0);

    let new_owner_quests = contract.quests_per_owner(user1());
    assert_eq!(owner_quests.len(), 0);

    let new_owner_first_quest = new_owner_quests.get(0);
    assert_eq!(new_owner_first_quest.is_some(), true);

    let new_owner_first_quest_unwrapped = new_owner_first_quest.unwrap();

    assert_eq!(&new_owner_first_quest_unwrapped.title, &quest.title);
}

#[test]
#[should_panic(expected = "ERR_FORBIDDEN")]
fn test_quest_ownership_panics() {
    let (mut contract, mut context) = setup();
    
    let quest = create_quest(&mut contract, &mut context, None);

    let owner_quests = contract.quests_per_owner(owner());
    assert_eq!(owner_quests.len(), 1);

    let first_quest = owner_quests.get(0);
    assert_eq!(first_quest.is_some(), true);
    
    let first_quest_unwrapped = first_quest.unwrap();

    contract.set_owner( first_quest_unwrapped.quest_id, user1());
    // should panic since the predecessor is "owner" that no more has ownership
    contract.set_owner( first_quest_unwrapped.quest_id, user2());
}

#[test]
#[should_panic(expected = "ERR_FORBIDDEN")]
fn test_set_probability_with_regular_user_panic() {
    let (mut contract, mut context) = setup();

    let quest = create_quest(&mut contract, &mut context, None);

    testing_env!(context.predecessor_account_id(user1()).build());

    contract.set_probability(quest.id, BoxRarity::Epic, Probability::ZERO);
}

#[test]
#[should_panic(expected = "Denominator can't be zero")]
fn test_set_probability_with_zero_denominator_panic() {
    let (mut contract, mut context) = setup();

    let probability = Probability {
        numerator: 5,
        denominator: 0,
    };

    let quest = create_quest(&mut contract, &mut context, None);

    contract.set_probability(quest.id, BoxRarity::Epic, probability);
}

#[test]
#[should_panic(expected = "Denominator must be bigger than or equal to numerator")]
fn test_set_probability_bigger_than_one_panic() {
    let (mut contract, mut context) = setup();

    let probability = Probability {
        numerator: 5,
        denominator: 2,
    };

    let quest = create_quest(&mut contract, &mut context, None);

    contract.set_probability(quest.id, BoxRarity::Epic, probability);
}

#[test]
#[should_panic(expected = "The minimal reward in Near tokens is 100000000000000000000000 yocto")]
fn test_add_small_near_pool_with_panic() {
    let (mut contract, mut context) = setup();

    let quest = create_quest(&mut contract, &mut context, None);

    contract.add_near_reward(quest.id, BoxRarity::Rare, U128(ONE_NEAR / 20), U64(10));
}

#[test]
fn test_add_near_pool_succeeds() {
    let (mut contract, mut context) = setup();

    let quest = create_quest(&mut contract, &mut context, None);

    const FIRST_POOL_ID:u32 = 0;
    assert_eq!(quest.pools.get(&FIRST_POOL_ID).is_none(), true);
    
    const POOL_RARITY:BoxRarity = BoxRarity::Rare;
    const AMOUNT:U128 = U128(ONE_NEAR);
    const CAPACITY:U64 = U64(3);

    contract.add_near_reward(quest.id, POOL_RARITY, AMOUNT, CAPACITY);

    let quest_modified = contract.quests.get(&quest.id).expect("Quest should exist");

    let pool = quest_modified.pools.get(&FIRST_POOL_ID);

    assert_eq!(pool.is_some(), true);

    let unwrapped_pool = pool.unwrap();
    let json_pool: JsonPoolRewards = unwrapped_pool.clone().into();

    assert_eq!(unwrapped_pool.rarity, POOL_RARITY);

    assert_eq!(
            json_pool,
            JsonPoolRewards::Near {
                amount: AMOUNT,
                available: CAPACITY.into(),
                total: CAPACITY.into()
            }
        );
}

#[test]
fn test_add_big_near_pool() {
    let (mut contract, mut context) = setup();

    let quest = create_quest(&mut contract, &mut context, Some(10005));
    
    const FIRST_POOL_ID:u32 = 0;
    const POOL_RARITY:BoxRarity = BoxRarity::Rare;
    const AMOUNT:U128 = U128(ONE_NEAR);
    const CAPACITY:U64 = U64(10_000);

    contract.add_near_reward(quest.id, POOL_RARITY, AMOUNT, CAPACITY);

    let quest_modified = contract.quests.get(&quest.id).expect("Quest should exist");

    let pool = quest_modified.pools.get(&FIRST_POOL_ID);

    assert_eq!(pool.is_some(), true);

    let unwrapped_pool = pool.unwrap();
    let json_pool: JsonPoolRewards = unwrapped_pool.clone().into();

    assert_eq!(unwrapped_pool.rarity, POOL_RARITY);

    assert_eq!(
            json_pool,
            JsonPoolRewards::Near {
                amount: AMOUNT,
                available: CAPACITY.into(),
                total: CAPACITY.into()
            }
        );

}

#[test]
fn test_add_multiple_near_pools_succeeds() {
    let (mut contract, mut context) = setup();

    let quest = create_quest(&mut contract, &mut context, Some(10));

    contract.add_near_reward(quest.id, BoxRarity::Rare, U128(ONE_NEAR), U64(5));
    contract.add_near_reward(quest.id, BoxRarity::Rare, U128(ONE_NEAR), U64(5));

    const FIRST_POOL_ID:u32 = 0;
    const SECOND_POOL_ID:u32 = 1;

    let quest_modified = contract.quests.get(&quest.id).expect("Quest should exist");

    let pool1 = quest_modified.pools.get(&FIRST_POOL_ID);
    let pool2 = quest_modified.pools.get(&SECOND_POOL_ID);

    assert_eq!(pool1.is_some(), true);
    assert_eq!(pool2.is_some(), true);
}

#[ignore = "clarification needed on nft_on_transfer"]
#[test]
fn test_add_nft_pool_succeeds() {
    let (mut contract, mut context) = setup();

    let mut quest = create_quest(&mut contract, &mut context, Some(10));

    testing_env!(context.predecessor_account_id(nft()).build());

    contract.nft_on_transfer(
        quest.id,
        owner(),
        owner(),
        "some_token".to_string(),
        "rare".to_string(),
    );
}

// #[test]
// fn test_add_multiple_nft_pool_succeeds() {
//     let (mut contract, mut context) = setup();

//     testing_env!(context.build());

//     testing_env!(context.predecessor_account_id(nft()).build());
//     contract.nft_on_transfer(
//         owner(),
//         owner(),
//         "some_token".to_string(),
//         "rare".to_string(),
//     );
//     contract.nft_on_transfer(
//         owner(),
//         owner(),
//         "some_token_2".to_string(),
//         "rare".to_string(),
//     );
// }

// #[should_panic(expected = "ERR_NFT_CONTRACT_NOT_TRUSTED")]
// #[test]
// fn test_add_non_whitelisted_nft_pool_with_panic() {
//     let (mut contract, mut context) = setup();

//     testing_env!(context.predecessor_account_id(nft3()).build());

//     contract.nft_on_transfer(
//         owner(),
//         owner(),
//         "some_token".to_string(),
//         "rare".to_string(),
//     );
// }

#[test]
fn test_available_near_rewards_amount() {
    let (mut contract, mut context) = setup();

    let mut quest = create_quest(&mut contract, &mut context, Some(10));

    contract.add_near_reward(quest.id, BoxRarity::Rare, U128(ONE_NEAR), U64(5));

    let rewards = quest.available_rewards(BoxRarity::Rare, None);

    assert_eq!(rewards.len(), 1);

    contract.add_near_reward(quest.id, BoxRarity::Rare, U128(ONE_NEAR), U64(5));

    let rewards = quest.available_rewards(BoxRarity::Rare, None);

    assert_eq!(rewards.len(), 2);

    contract.add_near_reward(quest.id, BoxRarity::Epic, U128(ONE_NEAR), U64(5));

    let rewards = quest.available_rewards(BoxRarity::Rare, None);
    assert_eq!(rewards.len(), 2);

    let rewards = quest.available_rewards(BoxRarity::Epic, None);
    assert_eq!(rewards.len(), 1);
}

#[test]
fn test_available_near_rewards_data() {
    let (mut contract, mut context) = setup();

    let mut quest = create_quest(&mut contract, &mut context, Some(10));

    contract.add_near_reward(quest.id, BoxRarity::Rare, U128(ONE_NEAR), U64(5));

    let rewards = quest.available_rewards(BoxRarity::Rare, None);

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

// #[test]
// fn test_available_nft_rewards_amount_for_different_contracts() {
//     let (mut contract, mut context) = setup();

//     testing_env!(context.build());

//     testing_env!(context.predecessor_account_id(nft()).build());
//     contract.nft_on_transfer(
//         owner(),
//         owner(),
//         "some_token".to_string(),
//         "rare".to_string(),
//     );

//     let rewards = contract.available_rewards(BoxRarity::Rare, None);
//     assert_eq!(rewards.len(), 1);

//     testing_env!(context.predecessor_account_id(nft2()).build());
//     contract.nft_on_transfer(
//         owner(),
//         owner(),
//         "another_token".to_string(),
//         "rare".to_string(),
//     );

//     let rewards = contract.available_rewards(BoxRarity::Rare, None);
//     assert_eq!(rewards.len(), 2);
// }

// #[test]
// fn test_available_nft_rewards_amount_for_same_contract() {
//     let (mut contract, mut context) = setup();

//     testing_env!(context.build());

//     testing_env!(context.predecessor_account_id(nft()).build());
//     contract.nft_on_transfer(
//         owner(),
//         owner(),
//         "some_token".to_string(),
//         "rare".to_string(),
//     );

//     let rewards = contract.available_rewards(BoxRarity::Rare, None);
//     assert_eq!(rewards.len(), 1);

//     testing_env!(context.predecessor_account_id(nft()).build());
//     contract.nft_on_transfer(
//         owner(),
//         owner(),
//         "some_token_2".to_string(),
//         "rare".to_string(),
//     );

//     let rewards = contract.available_rewards(BoxRarity::Rare, None);
//     assert_eq!(rewards.len(), 1);
// }

// #[test]
// fn test_available_nft_rewards_amount_for_same_contract_and_different_rarity() {
//     let (mut contract, mut context) = setup();

//     testing_env!(context.build());

//     testing_env!(context.predecessor_account_id(nft()).build());
//     contract.nft_on_transfer(
//         owner(),
//         owner(),
//         "some_token".to_string(),
//         "rare".to_string(),
//     );

//     let rewards = contract.available_rewards(BoxRarity::Rare, None);
//     assert_eq!(rewards.len(), 1);

//     testing_env!(context.predecessor_account_id(nft()).build());
//     contract.nft_on_transfer(
//         owner(),
//         owner(),
//         "some_token_2".to_string(),
//         "epic".to_string(),
//     );

//     let rewards = contract.available_rewards(BoxRarity::Rare, None);
//     assert_eq!(rewards.len(), 1);
//     let rewards = contract.available_rewards(BoxRarity::Epic, None);
//     assert_eq!(rewards.len(), 1);
// }

// #[test]
// fn test_available_nft_rewards_data_for_same_contract() {
//     let (mut contract, mut context) = setup();

//     testing_env!(context.build());

//     testing_env!(context.predecessor_account_id(nft()).build());

//     contract.nft_on_transfer(
//         owner(),
//         owner(),
//         "some_token".to_string(),
//         "rare".to_string(),
//     );
//     contract.nft_on_transfer(
//         owner(),
//         owner(),
//         "some_token_2".to_string(),
//         "rare".to_string(),
//     );

//     let rewards = contract.available_rewards(BoxRarity::Rare, None);

//     let reward = rewards.get(0).unwrap().to_owned();

//     assert_eq!(
//         reward,
//         JsonPoolRewards::NonFungibleToken {
//             contract_id: nft(),
//             token_ids: vec!["some_token".to_string(), "some_token_2".to_string()],
//             total: 2
//         }
//     );
// }

#[test]
fn test_mint_succeeds() {
    let (mut contract, mut context) = setup();

    let mut quest = create_quest(&mut contract, &mut context, Some(10));

    contract.mint(quest.id, user1(), BoxRarity::Rare);
}

#[test]
fn test_delete_boxes_succeeds() {
    let (mut contract, mut context) = setup();

    testing_env!(context.predecessor_account_id(user1()).build());

    let mut quest = create_quest(&mut contract, &mut context, Some(10));

    contract.mint(quest.id, user1(), BoxRarity::Epic);
    contract.mint(quest.id, user1(), BoxRarity::Rare);

    assert_eq!(contract.questboxes_supply_per_owner(user1()), U128(2));

    contract.delete_boxes(quest.id, vec![0]);

    assert_eq!(contract.questboxes_supply_per_owner(user1()), U128(1));

    const FIRST_BOX_ID: u128 = 0;
    assert!(quest.boxes.get(&FIRST_BOX_ID).is_none());
}

// #[test]
// fn test_mint_many_succeeds() {
//     let (mut contract, mut context) = setup();

//     testing_env!(context.attached_deposit(ONE_NEAR).build());

//     contract.mint_many(BoxRarity::Rare, vec![user1(), user2()]);
// }

// #[should_panic]
// #[test]
// fn test_mint_many_without_accounts_panic() {
//     let (mut contract, mut context) = setup();

//     testing_env!(context.attached_deposit(ONE_NEAR).build());

//     contract.mint_many(BoxRarity::Rare, vec![]);
// }

// #[test]
// fn test_total_supply_default() {
//     let (mut contract, mut context) = setup();

//     assert_eq!(contract.total_supply(), U128(0));
// }

// #[test]
// fn test_total_supply_increases() {
//     let (mut contract, mut context) = setup();

//     testing_env!(context.attached_deposit(ONE_NEAR * 2).build());

//     contract.mint(user1(), BoxRarity::Rare);

//     assert_eq!(contract.total_supply(), U128(1));

//     contract.mint(user1(), BoxRarity::Rare);
//     contract.mint(user2(), BoxRarity::Epic);

//     assert_eq!(contract.total_supply(), U128(3));

//     contract.mint_many(BoxRarity::Legendary, vec![user1(), user2()]);

//     assert_eq!(contract.total_supply(), U128(5));
// }

// #[test]
// fn test_supply_for_owner_default() {
//     let (mut contract, mut context) = setup();

//     assert_eq!(contract.supply_for_owner(user1()), U128(0));
// }

// #[test]
// fn test_supply_for_owner_increases() {
//     let (mut contract, mut context) = setup();

//     testing_env!(context.attached_deposit(ONE_NEAR * 2).build());

//     contract.mint(user1(), BoxRarity::Rare);

//     assert_eq!(contract.supply_for_owner(user1()), U128(1));

//     contract.mint(user1(), BoxRarity::Rare);
//     contract.mint(user2(), BoxRarity::Epic);

//     assert_eq!(contract.supply_for_owner(user1()), U128(2));
//     assert_eq!(contract.supply_for_owner(user2()), U128(1));

//     contract.mint_many(BoxRarity::Legendary, vec![user1(), user2(), user1()]);

//     assert_eq!(contract.supply_for_owner(user1()), U128(4));
//     assert_eq!(contract.supply_for_owner(user2()), U128(2));
// }

// #[test]
// fn test_boxes_for_owner_amount() {
//     let (mut contract, mut context) = setup();

//     testing_env!(context.attached_deposit(ONE_NEAR * 2).build());

//     contract.mint(user1(), BoxRarity::Rare);

//     assert_eq!(contract.boxes_for_owner(user1(), None).len(), 1);

//     contract.mint(user1(), BoxRarity::Epic);
//     contract.mint(user1(), BoxRarity::Epic);

//     contract.mint(user1(), BoxRarity::Legendary);

//     assert_eq!(contract.boxes_for_owner(user1(), None).len(), 4);

//     contract.mint(user2(), BoxRarity::Epic);

//     assert_eq!(contract.boxes_for_owner(user1(), None).len(), 4);
//     assert_eq!(contract.boxes_for_owner(user2(), None).len(), 1);
// }

// #[test]
// fn test_boxes_for_owner_status() {
//     let (mut contract, mut context) = setup();

//     testing_env!(context.attached_deposit(ONE_NEAR * 2).build());

//     contract.mint(user1(), BoxRarity::Rare);

//     let boxes = contract.boxes_for_owner(user1(), None);

//     let box_data = boxes.get(0).unwrap().to_owned();

//     assert_eq!(box_data.status, JsonBoxStatus::NonClaimed);
// }

// #[should_panic(expected = "ERR_NO_POOLS_AVAILABLE")]
// #[test]
// fn test_claim_without_pools_with_panic() {
//     let (mut contract, mut context) = setup();

//     testing_env!(context.attached_deposit(ONE_NEAR).build());

//     contract.mint(user1(), BoxRarity::Rare);

//     testing_env!(context
//         .attached_deposit(1)
//         .predecessor_account_id(user1())
//         .build());

//     contract.claim(1);
// }

// #[should_panic(expected = "ERR_ONLY_OWNER_CAN_BURN")]
// #[test]
// fn test_claim_as_another_user_with_panic() {
//     let (mut contract, mut context) = setup();

//     testing_env!(context.attached_deposit(ONE_NEAR).build());

//     contract.mint(user1(), BoxRarity::Rare);

//     testing_env!(context
//         .attached_deposit(1)
//         .predecessor_account_id(user2())
//         .build());

//     contract.claim(1);
// }

// #[should_panic(expected = "ERR_BOX_NOT_FOUND")]
// #[test]
// fn test_claim_non_existing_with_panic() {
//     let (mut contract, mut context) = setup();

//     testing_env!(context.attached_deposit(ONE_NEAR).build());

//     contract.add_near_reward(BoxRarity::Rare, U128(ONE_NEAR / 10), U64(1));
//     contract.mint(user1(), BoxRarity::Rare);

//     testing_env!(context
//         .attached_deposit(1)
//         .predecessor_account_id(user1())
//         .build());

//     contract.claim(500000);
// }

// #[should_panic(expected = "ERR_BOX_ALREADY_CLAIMED")]
// #[test]
// fn test_claim_twice_with_panic() {
//     let (mut contract, mut context) = setup();

//     testing_env!(context.attached_deposit(ONE_NEAR).build());

//     contract.add_near_reward(BoxRarity::Rare, U128(ONE_NEAR / 10), U64(5));
//     contract.mint(user1(), BoxRarity::Rare);

//     testing_env!(context
//         .attached_deposit(1)
//         .predecessor_account_id(user1())
//         .build());

//     contract.claim(1);
//     contract.claim(1);
// }

// #[test]
// fn test_claim_box_status() {
//     let (mut contract, mut context) = setup();

//     testing_env!(context.attached_deposit(3 * ONE_NEAR).build());

//     contract.add_near_reward(BoxRarity::Rare, U128(ONE_NEAR), U64(2));
//     contract.mint(user1(), BoxRarity::Rare);

//     testing_env!(context
//         .attached_deposit(1)
//         .predecessor_account_id(user1())
//         .build());

//     // promises aren't called
//     contract.claim(1);

//     let boxes = contract.boxes_for_owner(user1(), None);

//     let box_data = boxes.get(0).unwrap();

//     assert_eq!(
//         box_data.status,
//         JsonBoxStatus::Claimed {
//             reward: JsonReward::Near {
//                 amount: ONE_NEAR.into()
//             }
//         }
//     );
// }

// #[test]
// fn test_claim_box_with_zero_probability() {
//     let (mut contract, mut context) = setup();

//     testing_env!(context.attached_deposit(3 * ONE_NEAR).build());

//     contract.set_probability(BoxRarity::Rare, Probability::ZERO);
//     contract.add_near_reward(BoxRarity::Rare, U128(ONE_NEAR), U64(2));
//     contract.mint(user1(), BoxRarity::Rare);

//     testing_env!(context
//         .attached_deposit(1)
//         .predecessor_account_id(user1())
//         .build());

//     // promises aren't called
//     contract.claim(1);

//     let boxes = contract.boxes_for_owner(user1(), None);

//     let box_data = boxes.get(0).unwrap();

//     assert_eq!(
//         box_data.status,
//         JsonBoxStatus::Claimed {
//             reward: JsonReward::Nothing
//         }
//     );
// }

// #[test]
// fn test_claim_decreases_reward_availability() {
//     let (mut contract, mut context) = setup();

//     testing_env!(context.attached_deposit(3 * ONE_NEAR).build());

//     contract.add_near_reward(BoxRarity::Rare, U128(ONE_NEAR), U64(2));
//     contract.mint(user1(), BoxRarity::Rare);

//     testing_env!(context
//         .attached_deposit(1)
//         .predecessor_account_id(user1())
//         .build());

//     // promises aren't called
//     contract.claim(1);

//     let rewards = contract.available_rewards(BoxRarity::Rare, None);

//     assert_eq!(rewards.len(), 1);

//     let reward = rewards.get(0).unwrap().to_owned();

//     // availability decreased
//     assert_eq!(
//         reward,
//         JsonPoolRewards::Near {
//             amount: ONE_NEAR.into(),
//             available: 1,
//             total: 2
//         }
//     );
// }

// #[test]
// fn test_claim_empty_reward_availability() {
//     let (mut contract, mut context) = setup();

//     testing_env!(context.attached_deposit(3 * ONE_NEAR).build());

//     contract.add_near_reward(BoxRarity::Rare, U128(ONE_NEAR), U64(1));
//     contract.mint(user1(), BoxRarity::Rare);

//     testing_env!(context
//         .attached_deposit(1)
//         .predecessor_account_id(user1())
//         .build());

//     // promises aren't called
//     contract.claim(1);

//     let rewards = contract.available_rewards(BoxRarity::Rare, None);

//     assert_eq!(rewards.len(), 0);
// }

// #[test]
// fn test_claim_near_reward_succeeds() {
//     let (mut contract, mut context) = setup();

//     testing_env!(context.attached_deposit(3 * ONE_NEAR).build());

//     contract.add_near_reward(BoxRarity::Rare, U128(ONE_NEAR), U64(2));
//     contract.mint(user1(), BoxRarity::Rare);

//     testing_env!(context
//         .attached_deposit(1)
//         .predecessor_account_id(user1())
//         .build());

//     // promises aren't called
//     contract.claim(1);
// }

// #[test]
// fn test_claim_nft_reward_succeeds() {
//     let (mut contract, mut context) = setup();

//     testing_env!(context.attached_deposit(ONE_NEAR).build());

//     contract.mint(user1(), BoxRarity::Rare);

//     // add NFT token as reward
//     testing_env!(context.predecessor_account_id(nft()).build());
//     contract.nft_on_transfer(
//         owner(),
//         owner(),
//         "some_token".to_string(),
//         "rare".to_string(),
//     );

//     testing_env!(context
//         .attached_deposit(1)
//         .predecessor_account_id(user1())
//         .build());

//     contract.claim(1);
// }

// #[test]
// fn test_claim_for_multiple_pools_succeeds() {
//     let (mut contract, mut context) = setup();

//     testing_env!(context.attached_deposit(2 * ONE_NEAR).build());

//     contract.add_near_reward(BoxRarity::Rare, U128(ONE_NEAR), U64(1));
//     contract.add_near_reward(BoxRarity::Rare, U128(ONE_NEAR / 2), U64(2));
//     contract.add_near_reward(BoxRarity::Rare, U128(ONE_NEAR / 4), U64(4));

//     contract.mint(user1(), BoxRarity::Rare);

//     testing_env!(context
//         .attached_deposit(1)
//         .predecessor_account_id(user1())
//         .random_seed([2; 32])
//         .build());

//     contract.claim(1);
// }

// #[should_panic]
// #[test]
// fn test_check_verification_and_claim_callback_by_someone_with_panic() {
//     let (mut contract, mut context) = setup();

//     contract.check_iah_verification_and_claim_callback(user1(), 1, 1);
// }

// #[should_panic]
// #[test]
// fn test_transfer_reward_callback_by_someone_with_panic() {
//     let (mut contract, mut context) = setup();

//     contract.transfer_reward_callback(user1(), 1, 1, Reward::Near { amount: ONE_NEAR });
// }

// #[test]
// fn test_default_trusted_nft_contract_set_succeeds() {
//     let (mut contract, mut context) = setup();

//     contract.trusted_nft_contracts.contains(&nft());
//     contract.trusted_nft_contracts.contains(&nft2());
// }

// #[should_panic(expected = "ERR_FORBIDDEN")]
// #[test]
// fn test_trust_nft_contract_with_regular_user_panic() {
//     let (mut contract, mut context) = setup();

//     testing_env!(context.predecessor_account_id(user1()).build());

//     contract.trust_nft_contract(nft3());
// }

// #[test]
// fn test_trust_nft_contract_succeeds() {
//     let (mut contract, mut context) = setup();

//     contract.trust_nft_contract(nft3());
// }

// #[test]
// fn test_untrust_nft_contract_succeeds() {
//     let (mut contract, mut context) = setup();

//     contract.untrust_nft_contract(nft());
// }

// #[should_panic]
// #[test]
// fn test_trust_nft_contract_with_existed_value_panic() {
//     let (mut contract, mut context) = setup();

//     contract.trust_nft_contract(nft());
// }

// #[should_panic]
// #[test]
// fn test_untrust_nft_contract_with_non_existed_value_panic() {
//     let (mut contract, mut context) = setup();

//     contract.untrust_nft_contract(nft3());
// }

// #[test]
// fn test_users_default() {
//     let (mut contract, mut context) = setup();

//     assert_eq!(contract.users(None).len(), 0);
// }

// #[test]
// fn test_users_increases() {
//     let (mut contract, mut context) = setup();

//     testing_env!(context.attached_deposit(ONE_NEAR).build());

//     contract.mint(user1(), BoxRarity::Rare);

//     assert_eq!(contract.users(None).len(), 1);

//     contract.mint(user1(), BoxRarity::Rare);
//     contract.mint(user2(), BoxRarity::Epic);

//     assert_eq!(contract.users(None).len(), 2);

//     contract.mint_many(BoxRarity::Legendary, vec![user1(), user2(), user3()]);

//     assert_eq!(contract.users(None).len(), 3);
//     assert_eq!(contract.users(None), vec![user1(), user2(), user3()]);
//}
