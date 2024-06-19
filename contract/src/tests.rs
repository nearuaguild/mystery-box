#![allow(unused)]
#[cfg(test)]
use near_sdk::json_types::{ U128, U64 };
use near_sdk::test_utils::VMContextBuilder;
use near_sdk::{ testing_env, AccountId, ONE_NEAR };
use utils::create_quest;

use std::str::FromStr;

use crate::contract::json::{ JsonBoxStatus, JsonPoolRewards, JsonReward, Pagination };
use crate::contract::quest::Quest;
use crate::contract::types::{ BoxRarity, Probability, Reward };

mod utils;
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

fn testnet_user() -> AccountId {
    AccountId::from_str("test.testnet").unwrap()
}

fn mainnet_user() -> AccountId {
    AccountId::from_str("test.near").unwrap()
}

fn setup(
    attached_deposit: Option<u128>,
    contract_owner_account_id: Option<AccountId>,
    contract_predecessor_account_id: Option<AccountId>
) -> (Contract, VMContextBuilder, Quest) {
    let mut context = VMContextBuilder::new();

    let contract_owner_account_id = contract_owner_account_id.unwrap_or(owner());
    context.current_account_id(contract_owner_account_id.clone());
    //setting predecessor to owner to simulate contract deployment.
    //with contract deployment the predecessor will be equal to current account.
    context.predecessor_account_id(contract_owner_account_id.clone());

    context.account_balance(50 * ONE_NEAR);

    testing_env!(context.build());

    let mut contract = Contract::new();

    //now, after the contract has been deployed we can switch predecessor to whatever our test requires.
    context.predecessor_account_id(contract_predecessor_account_id.unwrap_or(owner()));

    testing_env!(context.build());

    let mut quest = create_quest(&mut contract, &mut context, attached_deposit);

    (contract, context, quest)
}

#[test]
fn test_setup_succeeds() {
    setup(None, None, None);
}

#[test]
fn test_quest_ownership() {
    let (mut contract, mut context, quest) = setup(None, None, None);

    let owner_quests = contract.quests_per_owner(owner());
    assert_eq!(owner_quests.len(), 1);

    let first_quest = owner_quests.get(0);
    assert_eq!(first_quest.is_some(), true);

    let first_quest_unwrapped = first_quest.unwrap();

    contract.set_owner(first_quest_unwrapped.quest_id, user1());

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
    let (mut contract, mut context, quest) = setup(None, None, None);

    let owner_quests = contract.quests_per_owner(owner());
    assert_eq!(owner_quests.len(), 1);

    let first_quest = owner_quests.get(0);
    assert_eq!(first_quest.is_some(), true);

    let first_quest_unwrapped = first_quest.unwrap();

    contract.set_owner(first_quest_unwrapped.quest_id, user1());
    // should panic since the predecessor is "owner" that no more has ownership
    contract.set_owner(first_quest_unwrapped.quest_id, user2());
}

#[test]
#[should_panic(expected = "ERR_FORBIDDEN")]
fn test_set_probability_with_regular_user_panic() {
    let (mut contract, mut context, quest) = setup(None, None, None);

    testing_env!(context.predecessor_account_id(user1()).build());

    contract.set_probability(quest.id, BoxRarity::Epic, Probability::ZERO);
}

#[test]
#[should_panic(expected = "Denominator can't be zero")]
fn test_set_probability_with_zero_denominator_panic() {
    let (mut contract, mut context, quest) = setup(None, None, None);

    let probability = Probability {
        numerator: 5,
        denominator: 0,
    };

    contract.set_probability(quest.id, BoxRarity::Epic, probability);
}

#[test]
#[should_panic(expected = "Denominator must be bigger than or equal to numerator")]
fn test_set_probability_bigger_than_one_panic() {
    let (mut contract, mut context, quest) = setup(None, None, None);

    let probability = Probability {
        numerator: 5,
        denominator: 2,
    };

    contract.set_probability(quest.id, BoxRarity::Epic, probability);
}

#[test]
#[should_panic(expected = "The minimal reward in Near tokens is 100000000000000000000000 yocto")]
fn test_add_small_near_pool_with_panic() {
    let (mut contract, mut context, quest) = setup(None, None, None);

    contract.add_near_reward(quest.id, BoxRarity::Rare, U128(ONE_NEAR / 20), U64(10));
}

#[test]
fn test_add_near_pool_succeeds() {
    let (mut contract, mut context, quest) = setup(None, None, None);

    const FIRST_POOL_ID: u32 = 0;
    assert_eq!(quest.pools.get(&FIRST_POOL_ID).is_none(), true);

    const POOL_RARITY: BoxRarity = BoxRarity::Rare;
    const AMOUNT: U128 = U128(ONE_NEAR);
    const CAPACITY: U64 = U64(3);

    contract.add_near_reward(quest.id, POOL_RARITY, AMOUNT, CAPACITY);

    let quest_modified = contract.quests.get(&quest.id).expect("Quest should exist");

    let pool = quest_modified.pools.get(&FIRST_POOL_ID);

    assert_eq!(pool.is_some(), true);

    let unwrapped_pool = pool.unwrap();
    let json_pool: JsonPoolRewards = unwrapped_pool.clone().into();

    assert_eq!(unwrapped_pool.rarity, POOL_RARITY);

    assert_eq!(json_pool, JsonPoolRewards::Near {
        amount: AMOUNT,
        available: CAPACITY.into(),
        total: CAPACITY.into(),
    });
}

#[test]
fn test_add_big_near_pool() {
    let (mut contract, mut context, quest) = setup(Some(10005), None, None);

    const FIRST_POOL_ID: u32 = 0;
    const POOL_RARITY: BoxRarity = BoxRarity::Rare;
    const AMOUNT: U128 = U128(ONE_NEAR);
    const CAPACITY: U64 = U64(10_000);

    contract.add_near_reward(quest.id, POOL_RARITY, AMOUNT, CAPACITY);

    let quest_modified = contract.quests.get(&quest.id).expect("Quest should exist");

    let pool = quest_modified.pools.get(&FIRST_POOL_ID);

    assert_eq!(pool.is_some(), true);

    let unwrapped_pool = pool.unwrap();
    let json_pool: JsonPoolRewards = unwrapped_pool.clone().into();

    assert_eq!(unwrapped_pool.rarity, POOL_RARITY);

    assert_eq!(json_pool, JsonPoolRewards::Near {
        amount: AMOUNT,
        available: CAPACITY.into(),
        total: CAPACITY.into(),
    });
}

#[test]
fn test_add_multiple_near_pools_succeeds() {
    let (mut contract, mut context, quest) = setup(Some(10), None, None);

    contract.add_near_reward(quest.id, BoxRarity::Rare, U128(ONE_NEAR), U64(5));
    contract.add_near_reward(quest.id, BoxRarity::Rare, U128(ONE_NEAR), U64(5));

    const FIRST_POOL_ID: u32 = 0;
    const SECOND_POOL_ID: u32 = 1;

    let quest_modified = contract.quests.get(&quest.id).expect("Quest should exist");

    let pool1 = quest_modified.pools.get(&FIRST_POOL_ID);
    let pool2 = quest_modified.pools.get(&SECOND_POOL_ID);

    assert_eq!(pool1.is_some(), true);
    assert_eq!(pool2.is_some(), true);
}

#[test]
fn test_add_nft_pool_succeeds() {
    let (mut contract, mut context, quest) = setup(Some(10), None, None);

    contract.trust_nft_contract(nft().clone());
    
    testing_env!(context.predecessor_account_id(nft()).build());

    contract.nft_on_transfer(
        nft(),
        owner(),
        "some_token".to_string(),
        String::from(
            r#"
        {
            "quest_id": 0,
            "rarity": "rare"
        }
        "#
        )
    );
}

#[test]
fn test_add_multiple_nft_pool_succeeds() {
    let (mut contract, mut context, quest) = setup(None, None, None);
    
    contract.trust_nft_contract(nft().clone());
    contract.trust_nft_contract(nft2().clone());

    testing_env!(context.predecessor_account_id(nft()).build());
    
    contract.nft_on_transfer(
        nft(),
        owner(),
        "some_token".to_string(),
        String::from(
            r#"
        {
            "quest_id": 0,
            "rarity": "rare"
        }
        "#
        )
    );
    contract.nft_on_transfer(
        owner(),
        owner(),
        "some_token_2".to_string(),
        String::from(
            r#"
        {
            "quest_id": 0,
            "rarity": "rare"
        }
        "#
        )
    );
}

#[test]
#[should_panic(expected = "ERR_NFT_CONTRACT_NOT_TRUSTED")]
fn test_add_non_whitelisted_nft_pool_with_panic() {
    let (mut contract, mut context, quest) = setup(None, None, Some(nft3()));

    contract.nft_on_transfer(
        owner(),
        owner(),
        "some_token".to_string(),
        String::from(
            r#"
        {
            "quest_id": 0,
            "rarity": "rare"
        }
        "#
        )
    );
}

#[test]
fn test_available_near_rewards_amount() {
    let (mut contract, mut context, quest) = setup(Some(10), None, None);

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
    let (mut contract, mut context, quest) = setup(Some(10), None, None);

    contract.add_near_reward(quest.id, BoxRarity::Rare, U128(ONE_NEAR), U64(5));

    let rewards = quest.available_rewards(BoxRarity::Rare, None);

    let reward = rewards.get(0).unwrap().to_owned();

    assert_eq!(reward, JsonPoolRewards::Near {
        amount: U128(ONE_NEAR),
        available: 5,
        total: 5,
    });
}

#[test]
fn test_available_nft_rewards_amount_for_different_contracts() {
    let (mut contract, mut context, quest) = setup(None, None, None);

    contract.trust_nft_contract(nft().clone());
    contract.trust_nft_contract(nft2().clone());
	
    testing_env!(context.predecessor_account_id(nft()).build());

    contract.nft_on_transfer(
        nft(),
        owner(),
        "some_token".to_string(),
        String::from(
            r#"
        {
            "quest_id": 0,
            "rarity": "rare"
        }
        "#
        )
    );

    let rewards = contract.available_rewards(quest.id, BoxRarity::Rare, None);
    assert_eq!(rewards.len(), 1);

    testing_env!(context.predecessor_account_id(nft2()).build());
    contract.nft_on_transfer(
        nft2(),
        owner(),
        "another_token".to_string(),
        String::from(
            r#"
        {
            "quest_id": 0,
            "rarity": "rare"
        }
        "#
        )
    );

    let rewards = contract.available_rewards(quest.id, BoxRarity::Rare, None);
    assert_eq!(rewards.len(), 2);
}

#[test]
fn test_available_nft_rewards_amount_for_same_contract() {
    let (mut contract, mut context, quest) = setup(None, None, None);

    contract.trust_nft_contract(nft().clone());
	
    testing_env!(context.predecessor_account_id(nft()).build());

    contract.nft_on_transfer(
        nft(),
        owner(),
        "some_token".to_string(),
        String::from(
            r#"
        {
            "quest_id": 0,
            "rarity": "rare"
        }
        "#
        )
    );

    let rewards = contract.available_rewards(quest.id, BoxRarity::Rare, None);
    assert_eq!(rewards.len(), 1);

    testing_env!(context.predecessor_account_id(nft()).build());

    contract.nft_on_transfer(
        nft(),
        owner(),
        "some_token_2".to_string(),
        String::from(
            r#"
        {
            "quest_id": 0,
            "rarity": "rare"
        }
        "#
        )
    );

    let rewards = contract.available_rewards(quest.id, BoxRarity::Rare, None);
    assert_eq!(rewards.len(), 1);
}

#[test]
fn test_available_nft_rewards_amount_for_same_contract_and_different_rarity() {
    let (mut contract, mut context, quest) = setup(None, None, None);

    contract.trust_nft_contract(nft().clone());
        
    testing_env!(context.predecessor_account_id(nft()).build());

    contract.nft_on_transfer(
        nft(),
        owner(),
        "some_token".to_string(),
        String::from(
            r#"
        {
            "quest_id": 0,
            "rarity": "rare"
        }
        "#
        )
    );

    let rewards = contract.available_rewards(quest.id, BoxRarity::Rare, None);
    assert_eq!(rewards.len(), 1);

    testing_env!(context.predecessor_account_id(nft()).build());
    contract.nft_on_transfer(
        nft(),
        owner(),
        "some_token_2".to_string(),
        String::from(
            r#"
        {
            "quest_id": 0,
            "rarity": "epic"
        }
        "#
        )
    );

    let rewards = contract.available_rewards(quest.id, BoxRarity::Rare, None);
    assert_eq!(rewards.len(), 1);
    let rewards = contract.available_rewards(quest.id, BoxRarity::Epic, None);
    assert_eq!(rewards.len(), 1);
}

#[test]
fn test_available_nft_rewards_data_for_same_contract() {
    let (mut contract, mut context, quest) = setup(None, None, None);

    contract.trust_nft_contract(nft().clone());
	
    testing_env!(context.predecessor_account_id(nft()).build());

    contract.nft_on_transfer(
        nft(),
        owner(),
        "some_token".to_string(),
        format!(
            r#"
        {{
            "quest_id": {},
            "rarity": "rare"
        }}
        "#,
            quest.id
        )
    );
    contract.nft_on_transfer(
        nft(),
        owner(),
        "some_token_2".to_string(),
        format!(
            r#"
        {{
            "quest_id": {},
            "rarity": "rare"
        }}
        "#,
            quest.id
        )
    );

    let rewards = contract.available_rewards(quest.id, BoxRarity::Rare, None);

    let reward = rewards.get(0).unwrap().to_owned();

    assert_eq!(reward, JsonPoolRewards::NonFungibleToken {
        contract_id: nft(),
        token_ids: vec!["some_token".to_string(), "some_token_2".to_string()],
        total: 2,
    });
}

#[test]
fn test_mint_succeeds() {
    let (mut contract, mut context, quest) = setup(Some(10), None, None);

    contract.mint(quest.id, user1(), BoxRarity::Rare);
}

#[test]
fn test_delete_boxes_succeeds() {
    let (mut contract, mut context, _) = setup(None, None, Some(user1()));

    let mut quest = create_quest(&mut contract, &mut context, Some(10));

    let box_id = contract.mint(quest.id, user1(), BoxRarity::Epic);
    contract.mint(quest.id, user1(), BoxRarity::Rare);

    assert_eq!(contract.questboxes_supply_per_owner(user1()), U128(2));

    contract.delete_boxes(quest.id, vec![0]);

    assert_eq!(contract.questboxes_supply_per_owner(user1()), U128(1));

    assert!(quest.boxes.get(&box_id).is_none());
}

#[test]
fn test_mint_many_succeeds() {
    let (mut contract, mut context, quest) = setup(Some(10), None, None);

    contract.mint_many(quest.id, BoxRarity::Rare, vec![user1(), user2()]);
}

#[test]
#[should_panic(expected = "Accounts can't be empty")]
fn test_mint_many_without_accounts_panic() {
    let (mut contract, mut context, quest) = setup(Some(10), None, None);

    contract.mint_many(quest.id, BoxRarity::Rare, vec![]);
}

#[test]
fn test_total_supply_default() {
    let (mut contract, mut context, quest) = setup(Some(10), None, None);

    assert_eq!(contract.questboxes_total_supply(quest.id), U128(0));
}

#[test]
fn test_total_supply_increases() {
    let (mut contract, mut context, quest) = setup(Some(10), None, None);

    contract.mint(quest.id, user1(), BoxRarity::Rare);

    assert_eq!(contract.questboxes_total_supply(quest.id), U128(1));

    contract.mint(quest.id, user1(), BoxRarity::Rare);
    contract.mint(quest.id, user2(), BoxRarity::Epic);

    assert_eq!(contract.questboxes_total_supply(quest.id), U128(3));

    contract.mint_many(quest.id, BoxRarity::Legendary, vec![user1(), user2()]);

    assert_eq!(contract.questboxes_total_supply(quest.id), U128(5));
}

#[test]
fn test_supply_for_owner_default() {
    let (mut contract, mut context, quest) = setup(Some(10), None, None);

    assert_eq!(contract.questboxes_supply_per_owner(user1()), U128(0));
}

#[test]
fn test_supply_for_owner_increases() {
    let (mut contract, mut context, quest) = setup(Some(10), None, None);

    contract.mint(quest.id, user1(), BoxRarity::Rare);

    assert_eq!(contract.questboxes_supply_per_owner(user1()), U128(1));

    contract.mint(quest.id, user1(), BoxRarity::Rare);
    contract.mint(quest.id, user2(), BoxRarity::Epic);

    assert_eq!(contract.questboxes_supply_per_owner(user1()), U128(2));
    assert_eq!(contract.questboxes_supply_per_owner(user2()), U128(1));

    contract.mint_many(quest.id, BoxRarity::Legendary, vec![user1(), user2(), user1()]);

    assert_eq!(contract.questboxes_supply_per_owner(user1()), U128(4));
    assert_eq!(contract.questboxes_supply_per_owner(user2()), U128(2));
}

#[test]
fn test_boxes_for_owner_amount() {
    let (mut contract, mut context, quest) = setup(None, None, None);

    testing_env!(context.attached_deposit(ONE_NEAR * 2).build());

    contract.mint(quest.id, user1(), BoxRarity::Rare);

    assert_eq!(contract.questboxes_per_owner(user1(), None).len(), 1);

    contract.mint(quest.id, user1(), BoxRarity::Epic);
    contract.mint(quest.id, user1(), BoxRarity::Epic);

    contract.mint(quest.id, user1(), BoxRarity::Legendary);

    assert_eq!(contract.questboxes_per_owner(user1(), None).len(), 4);

    contract.mint(quest.id, user2(), BoxRarity::Epic);

    assert_eq!(contract.questboxes_per_owner(user1(), None).len(), 4);
    assert_eq!(contract.questboxes_per_owner(user2(), None).len(), 1);
}

#[test]
fn test_boxes_for_owner_status() {
    let (mut contract, mut context, quest) = setup(None, None, None);

    contract.mint(quest.id, user1(), BoxRarity::Rare);

    let boxes = contract.questboxes_per_owner(user1(), None);

    let box_data = boxes.get(0).unwrap().to_owned();

    assert_eq!(box_data.box_status, JsonBoxStatus::NonClaimed);
}

#[test]
fn test_boxes_for_quest_for_owner_amount() {
    let (mut contract, mut context, quest) = setup(None, None, None);

    let new_quest_id = contract.create_quest(&"new quest".to_string());

    contract.mint(quest.id, user1(), BoxRarity::Rare);

    assert_eq!(contract.questboxes_for_quest_per_owner(quest.id, user1(), None).len(), 1);

    contract.mint(new_quest_id, user1(), BoxRarity::Epic);
    contract.mint(new_quest_id, user1(), BoxRarity::Epic);

    contract.mint(quest.id, user1(), BoxRarity::Legendary);

    assert_eq!(contract.questboxes_for_quest_per_owner(new_quest_id, user1(), None).len(), 2);
    assert_eq!(contract.questboxes_for_quest_per_owner(quest.id, user1(), None).len(), 2);
}

#[test]
#[should_panic(expected = "ERR_NO_POOLS_AVAILABLE")]
fn test_claim_without_pools_with_panic() {
    let (mut contract, mut context, quest) = setup(None, None, None);

    let box_id = contract.mint(quest.id, user1(), BoxRarity::Rare);

    testing_env!(context.attached_deposit(1).predecessor_account_id(user1()).build());

    contract.claim(quest.id, box_id);
}

#[test]
#[should_panic(expected = "NO_BOXES_TO_CLAIM")]
fn test_claim_as_a_user_with_zero_boxes_with_panic() {
    let (mut contract, mut context, quest) = setup(None, None, None);

    let box_id = contract.mint(quest.id, user1(), BoxRarity::Rare);

    testing_env!(context.attached_deposit(1).predecessor_account_id(user2()).build());

    contract.claim(quest.id, box_id);
}

#[test]
#[should_panic(expected = "ERR_BOX_NOT_FOUND")]
fn test_claim_as_another_user_with_panic() {
    let (mut contract, mut context, quest) = setup(None, None, None);

    let box_1_id = contract.mint(quest.id, user1(), BoxRarity::Rare);
    let box_2_id = contract.mint(quest.id, user2(), BoxRarity::Rare);

    testing_env!(context.attached_deposit(1).predecessor_account_id(user2()).build());

    //claiming box_1_id by 'user2' will fail because 'user2' is not the owner of box_1_id
    contract.claim(quest.id, box_1_id);
}

#[should_panic(expected = "ERR_BOX_NOT_FOUND")]
#[test]
fn test_claim_non_existing_with_panic() {
    let (mut contract, mut context, quest) = setup(None, None, None);

    contract.add_near_reward(quest.id, BoxRarity::Rare, U128(ONE_NEAR / 10), U64(1));
    let box_id = contract.mint(quest.id, user1(), BoxRarity::Rare);

    testing_env!(context.attached_deposit(1).predecessor_account_id(user1()).build());

    const NON_EXISTING_BOX_ID: u128 = 5000;
    assert!(box_id != NON_EXISTING_BOX_ID, "Box id's can't be equal");

    contract.claim(quest.id, NON_EXISTING_BOX_ID);
}

//TODO. Create a test with kyc verification = false
#[test]
#[should_panic(expected = "ERR_BOX_ALREADY_CLAIMED")]
fn test_claim_twice_with_panic() {
    let (mut contract, mut context, quest) = setup(None, None, None);

    contract.add_near_reward(quest.id, BoxRarity::Rare, U128(ONE_NEAR / 10), U64(5));
    let box_id = contract.mint(quest.id, user1(), BoxRarity::Rare);

    testing_env!(context.attached_deposit(1).predecessor_account_id(user1()).build());

    contract.claim(quest.id, box_id);
    contract.claim(quest.id, box_id);
}

#[test]
fn test_claim_box_status() {
    let (mut contract, mut context, quest) = setup(None, None, None);

    contract.add_near_reward(quest.id, BoxRarity::Rare, U128(ONE_NEAR), U64(2));
    let box_id = contract.mint(quest.id, user1(), BoxRarity::Rare);

    testing_env!(context.attached_deposit(1).predecessor_account_id(user1()).build());

    // promises aren't called
    contract.claim(quest.id, box_id);

    let boxes = contract.questboxes_per_owner(user1(), None);

    let box_data = boxes.get(0).unwrap();

    assert_eq!(box_data.box_status, JsonBoxStatus::Claimed {
        reward: JsonReward::Near {
            amount: ONE_NEAR.into(),
        },
    });
}

#[test]
fn test_claim_box_with_zero_probability() {
    let (mut contract, mut context, quest) = setup(None, None, None);

    contract.set_probability(quest.id, BoxRarity::Rare, Probability::ZERO);
    contract.add_near_reward(quest.id, BoxRarity::Rare, U128(ONE_NEAR), U64(2));
    let box_id = contract.mint(quest.id, user1(), BoxRarity::Rare);

    testing_env!(context.attached_deposit(1).predecessor_account_id(user1()).build());

    // promises aren't called
    contract.claim(quest.id, box_id);

    let boxes = contract.questboxes_per_owner(user1(), None);

    let box_data = boxes.get(0).unwrap();

    assert_eq!(box_data.box_status, JsonBoxStatus::Claimed {
        reward: JsonReward::Nothing,
    });
}

#[test]
fn test_claim_decreases_reward_availability() {
    let (mut contract, mut context, quest) = setup(None, None, None);

    contract.add_near_reward(quest.id, BoxRarity::Rare, U128(ONE_NEAR), U64(2));
    let box_id = contract.mint(quest.id, user1(), BoxRarity::Rare);

    testing_env!(context.attached_deposit(1).predecessor_account_id(user1()).build());

    // promises aren't called
    contract.claim(quest.id, box_id);

    let rewards = contract.available_rewards(quest.id, BoxRarity::Rare, None);

    assert_eq!(rewards.len(), 1);

    let reward = rewards.get(0).unwrap().to_owned();

    // availability decreased
    assert_eq!(reward, JsonPoolRewards::Near {
        amount: ONE_NEAR.into(),
        available: 1,
        total: 2,
    });
}

#[test]
fn test_claim_empty_reward_availability() {
    let (mut contract, mut context, quest) = setup(None, None, None);

    testing_env!(context.attached_deposit(3 * ONE_NEAR).build());

    contract.add_near_reward(quest.id, BoxRarity::Rare, U128(ONE_NEAR), U64(1));
    let box_id = contract.mint(quest.id, user1(), BoxRarity::Rare);

    testing_env!(context.attached_deposit(1).predecessor_account_id(user1()).build());

    // promises aren't called
    contract.claim(quest.id, box_id);

    let rewards = contract.available_rewards(quest.id, BoxRarity::Rare, None);

    assert_eq!(rewards.len(), 0);
}

#[test]
fn test_claim_near_reward_succeeds() {
    let (mut contract, mut context, quest) = setup(None, None, None);

    contract.add_near_reward(quest.id, BoxRarity::Rare, U128(ONE_NEAR), U64(2));
    let box_id = contract.mint(quest.id, user1(), BoxRarity::Rare);

    testing_env!(context.attached_deposit(1).predecessor_account_id(user1()).build());

    // promises aren't called
    contract.claim(quest.id, box_id);
}

#[test]
#[should_panic(expected = "Error parsing message")]
fn test_claim_nft_reward_message_parse_panics() {
    let (mut contract, mut context, quest) = setup(None, None, None);

    let box_id = contract.mint(quest.id, user1(), BoxRarity::Rare);

    contract.trust_nft_contract(nft().clone());
        
    testing_env!(context.predecessor_account_id(nft()).build());

    // add NFT token as reward
    contract.nft_on_transfer(
        nft(),
        owner(),
        "some_token".to_string(),
        String::from("quest_id: 0")
    );

    testing_env!(context.attached_deposit(1).predecessor_account_id(user1()).build());

    contract.claim(quest.id, box_id);
}

#[test]
fn test_claim_nft_reward_succeeds() {
    let (mut contract, mut context, quest) = setup(None, None, Some(user1()));

    //only contract owner can perform trust
    testing_env!(context.predecessor_account_id(owner()).build());
    contract.trust_nft_contract(nft().clone());

    testing_env!(context.predecessor_account_id(user1()).build());
    let box_id = contract.mint(quest.id, user1(), BoxRarity::Rare);
    
    testing_env!(context.predecessor_account_id(nft()).build());
    // add NFT token as reward
    contract.nft_on_transfer(
        nft(),
        user1(),
        "some_token".to_string(),
        String::from(
            r#"
        {
            "quest_id": 0,
            "rarity": "rare"
        }
        "#
        )
    );

    testing_env!(context.attached_deposit(1).predecessor_account_id(user1()).build());

    contract.claim(quest.id, box_id);
}

#[test]
fn test_claim_for_multiple_pools_succeeds() {
    let (mut contract, mut context, quest) = setup(None, None, None);

    contract.add_near_reward(quest.id, BoxRarity::Rare, U128(ONE_NEAR), U64(1));
    contract.add_near_reward(quest.id, BoxRarity::Rare, U128(ONE_NEAR / 2), U64(2));
    contract.add_near_reward(quest.id, BoxRarity::Rare, U128(ONE_NEAR / 4), U64(4));

    let box_id = contract.mint(quest.id, user1(), BoxRarity::Rare);

    testing_env!(
        context
            .attached_deposit(1)
            .predecessor_account_id(user1())
            .random_seed([2; 32])
            .build()
    );

    contract.claim(quest.id, box_id);
}

#[test]
#[should_panic(expected = "ERR_TOO_MANY_RESULTS")]
fn test_check_verification_and_claim_callback_by_someone_with_panic() {
    let (mut contract, mut context, quest) = setup(None, None, None);

    contract.check_iah_verification_and_claim_callback(quest.id, user1(), 0, 0);
}

#[test]
#[should_panic(expected = "ERR_TOO_MANY_RESULTS")]
fn test_transfer_reward_callback_by_someone_with_panic() {
    let (mut contract, mut context, quest) = setup(None, None, None);

    contract.transfer_reward_callback(quest.id, user1(), 0, 0, Reward::Near { amount: ONE_NEAR });
}

#[test]
fn test_default_trusted_nft_contract_set_succeeds() {
    let (mut contract, mut context, quest) = setup(None, None, None);

    contract.trusted_nft_contracts.contains(&nft());
    contract.trusted_nft_contracts.contains(&nft2());
}

#[test]
#[should_panic(expected = "ERR_FORBIDDEN")]
fn test_trust_nft_contract_with_regular_user_panic() {
    let (mut contract, mut context, quest) = setup(None, None, Some(user1()));

    contract.trust_nft_contract(nft3());
}

#[test]
fn test_trust_nft_contract_succeeds() {
    let (mut contract, mut context, quest) = setup(None, None, None);

    contract.trust_nft_contract(nft3());
}

#[test]
fn test_untrust_nft_contract_succeeds() {
    let (mut contract, mut context, quest) = setup(None, None, None);

    contract.trust_nft_contract(nft().clone());

    contract.untrust_nft_contract(nft());
}

#[test]
#[should_panic(expected = "Provided contract is already in the set")]
fn test_trust_nft_contract_with_existed_value_panic() {
    let (mut contract, mut context, quest) = setup(None, None, None);

    contract.trust_nft_contract(nft().clone());

    contract.trust_nft_contract(nft());
}

#[test]
#[should_panic(expected = "Provided contract wasn't trusted before")]
fn test_untrust_nft_contract_with_non_existed_value_panic() {
    let (mut contract, mut context, quest) = setup(None, None, None);

    contract.untrust_nft_contract(nft3());
}

#[test]
fn test_users_default() {
    let (mut contract, mut context, quest) = setup(None, None, None);

    let pagination = Some(Pagination::new(1, 40));

    assert_eq!(contract.get_users(quest.id, pagination).len(), 0);
}

#[test]
fn test_users_increases() {
    let (mut contract, mut context, quest) = setup(None, None, None);

    testing_env!(context.attached_deposit(ONE_NEAR).build());

    contract.mint(quest.id, user1(), BoxRarity::Rare);

    let pagination = Some(Pagination::new(1, 40));

    assert_eq!(contract.get_users(quest.id, pagination.clone()).len(), 1);

    contract.mint(quest.id, user1(), BoxRarity::Rare);
    contract.mint(quest.id, user2(), BoxRarity::Epic);

    assert_eq!(contract.get_users(quest.id, pagination.clone()).len(), 2);

    contract.mint_many(quest.id, BoxRarity::Legendary, vec![user1(), user2(), user3()]);

    assert_eq!(contract.get_users(quest.id, pagination.clone()).len(), 3);
    assert_eq!(contract.get_users(quest.id, pagination.clone()), vec![user1(), user2(), user3()]);
}

#[test]
fn test_testnet_default_nft_contracts() {
    let (mut contract, mut context, quest) = setup(
        None,
        Some(testnet_user()),
        Some(testnet_user())
    );

    assert_eq!(contract.get_trusted_nft_contracts().len(), 5);
}

#[test]
fn test_mainnet_default_nft_contracts() {
    let (mut contract, mut context, quest) = setup(
        None,
        Some(mainnet_user()),
        Some(mainnet_user())
    );

    assert_eq!(contract.get_trusted_nft_contracts().len(), 10);
}

