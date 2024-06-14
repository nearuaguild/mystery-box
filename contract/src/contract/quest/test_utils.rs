use near_sdk::{test_utils::VMContextBuilder, testing_env, ONE_NEAR};

use crate::contract::Contract;

use super::Quest;

pub fn create_quest(contract: &mut Contract, context: &mut VMContextBuilder) -> Quest {
    testing_env!(context.attached_deposit(5 * ONE_NEAR).build());

    let new_quest_title = String::from("first quest");
    let new_quest_id = contract.create_quest(&new_quest_title);

    let quest = contract.quests.get(&new_quest_id).expect("Quest should exist");

    return quest;
}