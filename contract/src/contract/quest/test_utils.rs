use near_sdk::{test_utils::VMContextBuilder, testing_env, ONE_NEAR};

use crate::contract::Contract;

use super::Quest;


pub fn create_quest(contract: &mut Contract, context: &mut VMContextBuilder, attached_deposit: Option<u128>) -> Quest {
    
    let mut deposit = 5;
    if attached_deposit.is_some(){
        let unwrapped_deposit = attached_deposit.unwrap();
        assert!(unwrapped_deposit > 0, "Deposit should be bigger than 0");
        deposit = unwrapped_deposit;
    }
    
    testing_env!(context.attached_deposit(deposit * ONE_NEAR).build());

    let new_quest_title = String::from("first quest");
    let new_quest_id = contract.create_quest(&new_quest_title);

    let quest = contract.quests.get(&new_quest_id).expect("Quest should exist");

    return quest;
}