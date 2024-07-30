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
    get_quest_boxes_per_owner, get_quest_per_owner_by_index, mint_box, INITIAL_NEAR_PER_ACCOUNT,
    QUEST_OWNER_ACCOUNT_NAME, QUEST_TITLE, USER_1_ACCOUNT_NAME, USER_1_BOX_NEAR_REWARD,
    USER_2_ACCOUNT_NAME, USER_2_BOX_NEAR_REWARD,
};

#[tokio::test]
async fn test_near_reward_flow() -> anyhow::Result<()> {
    //Arrange
    let sandbox = near_workspaces::sandbox().await?;
    let mystery_box_contract = deploy_mystery_box_contract(&sandbox).await?;

    let root = sandbox.root_account()?;

    let quest_owner_account = create_user_account(&root, QUEST_OWNER_ACCOUNT_NAME).await?;
    let user_1_account = create_user_account(&root, USER_1_ACCOUNT_NAME).await?;
    let user_2_account = create_user_account(&root, USER_2_ACCOUNT_NAME).await?;

    create_quest(&mystery_box_contract, &quest_owner_account, QUEST_TITLE).await?;

    let quest = get_quest_per_owner_by_index(&mystery_box_contract, &quest_owner_account, 0).await?;

    let quest_id = quest.get("quest_id").unwrap();

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
        BoxRarity::Epic,
        NearToken::from_near(USER_2_BOX_NEAR_REWARD),
    )
    .await?;

    let expected_quest_owner_balance = NearToken::from_near(
        INITIAL_NEAR_PER_ACCOUNT - USER_1_BOX_NEAR_REWARD - USER_2_BOX_NEAR_REWARD,
    );

    //Assuming GAS fees will not be higher than 1 Near.
    let minimal_expected_quest_owner_balance = NearToken::from_near(
        INITIAL_NEAR_PER_ACCOUNT - USER_1_BOX_NEAR_REWARD - USER_2_BOX_NEAR_REWARD - 1,
    );

    //Checking that quest owner balance is deducted by created rewards
    assert!(quest_owner_account.view_account().await?.balance < expected_quest_owner_balance);
    assert!(
        quest_owner_account.view_account().await?.balance > minimal_expected_quest_owner_balance
    );

    let user_1_boxes =
        get_quest_boxes_per_owner(&mystery_box_contract, &user_1_account, quest_id).await?;
    assert!(user_1_boxes.len() == 0);

    let user_2_boxes =
        get_quest_boxes_per_owner(&mystery_box_contract, &user_2_account, quest_id).await?;
    assert!(user_2_boxes.len() == 0);

    mint_box(
        &mystery_box_contract,
        &quest_owner_account,
        vec![user_1_account.id()],
        quest_id,
        BoxRarity::Rare,
    )
    .await?;

    mint_box(
        &mystery_box_contract,
        &quest_owner_account,
        vec![user_2_account.id()],
        quest_id,
        BoxRarity::Epic,
    )
    .await?;

    let user_1_boxes =
        get_quest_boxes_per_owner(&mystery_box_contract, &user_1_account, quest_id).await?;
    assert!(user_1_boxes.len() == 1);

    let user_2_boxes =
        get_quest_boxes_per_owner(&mystery_box_contract, &user_2_account, quest_id).await?;
    assert!(user_2_boxes.len() == 1);

    let user_1_balance = user_1_account.view_account().await?.balance;
    let user_2_balance = user_2_account.view_account().await?.balance;

    let user_1_box_id = user_1_boxes[0].as_object().unwrap().get("box_id").unwrap();
    claim_box(
        &mystery_box_contract,
        &user_1_account,
        quest_id,
        user_1_box_id,
    )
    .await?;

    let user_2_box_id = user_2_boxes[0].as_object().unwrap().get("box_id").unwrap();
    claim_box(
        &mystery_box_contract,
        &user_2_account,
        quest_id,
        user_2_box_id,
    )
    .await?;

    //Checking user's balance after claim. It should be increased by the amount which was in the boxes
    let user_1_balance_after_claim = user_1_account.view_account().await?.balance;
    let user_2_balance_after_claim = user_2_account.view_account().await?.balance;

    let user_1_expected_balance = user_1_balance
        .saturating_add(NearToken::from_near(USER_1_BOX_NEAR_REWARD))
        .saturating_sub(NearToken::from_yoctonear(1));

    let user_2_expected_balance = user_2_balance
        .saturating_add(NearToken::from_near(USER_2_BOX_NEAR_REWARD))
        .saturating_sub(NearToken::from_yoctonear(1));

    assert!(user_1_balance_after_claim.as_near() >= user_1_expected_balance.as_near());

    assert!(user_2_balance_after_claim.as_near() >= user_2_expected_balance.as_near());

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
