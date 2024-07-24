use near_sdk::{
    serde_json::{json, Map, Value},
    AccountId,
};
use near_workspaces::{
    network::Sandbox, result::ExecutionFinalResult, types::Gas, types::NearToken, Account,
    Contract, Worker,
};

use crate::contract::types::BoxRarity;

use crate::tests::integration_tests::utils::{
    claim_box, create_quest, create_user_account, deploy_mystery_box_contract,
    get_quest_per_owner_by_index, get_quest_boxes_per_owner, mint_box, INITIAL_NEAR_PER_ACCOUNT,
    QUEST_OWNER_ACCOUNT_NAME, QUEST_TITLE, USER_1_ACCOUNT_NAME, USER_1_BOX_NEAR_REWARD,
    USER_2_ACCOUNT_NAME, USER_2_BOX_NEAR_REWARD,
};

#[tokio::test]
async fn test_mint_many() -> anyhow::Result<()> {
    //Arrange
    let sandbox = near_workspaces::sandbox().await?;
    let mystery_box_contract = deploy_mystery_box_contract(&sandbox).await?;

    let root = sandbox.root_account()?;

    let quest_owner_account = create_user_account(&root, QUEST_OWNER_ACCOUNT_NAME).await?;
    let user_1_account = create_user_account(&root, USER_1_ACCOUNT_NAME).await?;
    let user_2_account = create_user_account(&root, USER_2_ACCOUNT_NAME).await?;

    const SECOND_QUEST: &str = "second quest";

    create_quest(&mystery_box_contract, &quest_owner_account, QUEST_TITLE).await?;
    create_quest(&mystery_box_contract, &quest_owner_account, SECOND_QUEST).await?;

    let quest = get_quest_per_owner_by_index(&mystery_box_contract, &quest_owner_account, 0).await?;
    let quest_2 = get_quest_per_owner_by_index(&mystery_box_contract, &quest_owner_account, 1).await?;

    let quest_id = quest.get("quest_id").unwrap();
    let quest_2_id = quest_2.get("quest_id").unwrap();

    add_near_reward(
        &mystery_box_contract,
        &quest_owner_account,
        quest_id,
        BoxRarity::Rare,
        NearToken::from_near(USER_1_BOX_NEAR_REWARD),
    )
    .await?;

    add_near_reward(
        &mystery_box_contract,
        &quest_owner_account,
        quest_id,
        BoxRarity::Rare,
        NearToken::from_near(USER_2_BOX_NEAR_REWARD),
    )
    .await?;

    let user_1_boxes =
        get_quest_boxes_per_owner(&mystery_box_contract, &user_1_account, quest_id).await?;
    assert!(user_1_boxes.len() == 0);

    let user_2_boxes =
        get_quest_boxes_per_owner(&mystery_box_contract, &user_2_account, quest_id).await?;
    assert!(user_2_boxes.len() == 0);

    //Act
    mint_box(
        &mystery_box_contract,
        &quest_owner_account,
        vec![user_1_account.id(), user_2_account.id()],
        quest_id,
        BoxRarity::Rare,
    )
    .await?;

    let user_1_boxes =
        get_quest_boxes_per_owner(&mystery_box_contract, &user_1_account, quest_id).await?;

    let user_1_box_1_id = user_1_boxes[0].as_object().unwrap().get("box_id").unwrap();
    claim_box(
        &mystery_box_contract,
        &user_1_account,
        quest_id,
        user_1_box_1_id,
    )
    .await?;

    let user_1_boxes =
        get_quest_boxes_per_owner(&mystery_box_contract, &user_1_account, quest_id).await?;

    let user_1_box_1_status = user_1_boxes[0].as_object().unwrap().get("box_status").unwrap().as_object().unwrap().get("kind").unwrap();

    let user_2_boxes =
        get_quest_boxes_per_owner(&mystery_box_contract, &user_2_account, quest_id).await?;
    let user_2_box_1_id = user_2_boxes[0].as_object().unwrap().get("box_id").unwrap();
    let user_2_box_1_status = user_2_boxes[0].as_object().unwrap().get("box_status").unwrap().as_object().unwrap().get("kind").unwrap();

    //Assert
    assert_ne!(user_1_box_1_id, user_2_box_1_id, "Box ids should differ");
    assert_eq!(user_1_box_1_status, "claimed", "Box should be claimed");
    assert_eq!(user_2_box_1_status, "non_claimed", "Box shouldn't be claimed");

    Ok(())
}

async fn add_near_reward(
    mystery_box_contract: &Contract,
    user_account: &Account,
    quest_id: &Value,
    rarity: BoxRarity,
    amount: NearToken,
) -> anyhow::Result<ExecutionFinalResult> {
    const STORAGE_DEPOSIT: u128 = 5;

    let add_near_reward_outcome = user_account
        .call(mystery_box_contract.id(), "add_near_reward")
        .deposit(amount.saturating_add(NearToken::from_millinear(STORAGE_DEPOSIT)))
        .args_json(json!({
            "quest_id": quest_id,
            "rarity": rarity.to_string(),
            "capacity": "1",
            "amount": amount.as_yoctonear().to_string()
        }))
        .transact()
        .await?;

    assert!(
        add_near_reward_outcome.is_success(),
        "Adding reward failed {:#?}",
        add_near_reward_outcome
    );

    return Ok((add_near_reward_outcome));
}
