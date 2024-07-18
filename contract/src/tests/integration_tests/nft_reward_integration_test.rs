use std::fmt::format;

use near_sdk::{
    serde_json::{json, Map, Value},
    AccountId,
};
use near_workspaces::{
    network::Sandbox, result::ExecutionFinalResult, types::Gas, types::NearToken, Account,
    Contract, Worker,
};

use crate::contract::types::BoxRarity;

use crate::tests::integration_tests::utils::deploy_mystery_box_contract;
use crate::tests::integration_tests::utils::{
    claim_box, create_quest, create_user_account, deploy_nft_contract, get_first_quests_per_owner,
    get_quest_boxes_per_owner, mint_box, INITIAL_NEAR_PER_ACCOUNT, NFT_1_MEDIA, NFT_1_TOKEN_ID,
    NFT_2_MEDIA, NFT_2_TOKEN_ID, QUEST_OWNER_ACCOUNT_NAME, QUEST_TITLE, USER_1_ACCOUNT_NAME,
    USER_2_ACCOUNT_NAME,
};

#[tokio::test]
async fn test_nft_reward_flow() -> anyhow::Result<()> {
    //Arrange
    let sandbox = near_workspaces::sandbox().await?;
    let mystery_box_contract = deploy_mystery_box_contract(&sandbox).await?;

    let root = sandbox.root_account()?;
    let quest_owner_account = create_user_account(&root, QUEST_OWNER_ACCOUNT_NAME).await?;
    let user_1_account = create_user_account(&root, USER_1_ACCOUNT_NAME).await?;
    let user_2_account = create_user_account(&root, USER_2_ACCOUNT_NAME).await?;

    let nft_contract = deploy_nft_contract(&sandbox, &quest_owner_account).await?;

    let nft_tokens = get_nft_tokens(&nft_contract, &quest_owner_account).await?;

    //By default 2 NFT's are added to quest owner account
    assert!(nft_tokens.len() == 2, "Tokens amount incorrect");

    create_quest(&mystery_box_contract, &quest_owner_account, QUEST_TITLE).await?;

    let quest = get_first_quests_per_owner(&mystery_box_contract, &quest_owner_account).await?;

    let quest_id = quest.get("quest_id").unwrap();

    trust_nft_contract(&mystery_box_contract, &nft_contract).await;

    add_nft_reward(
        &mystery_box_contract,
        &nft_contract,
        &quest_owner_account,
        quest_id,
        BoxRarity::Rare,
        NFT_1_TOKEN_ID,
    )
    .await?;

    add_nft_reward(
        &mystery_box_contract,
        &nft_contract,
        &quest_owner_account,
        quest_id,
        BoxRarity::Epic,
        NFT_2_TOKEN_ID,
    )
    .await?;

    //Checking that user1 and user2 have 0 boxes
    let nft_tokens = get_nft_tokens(&nft_contract, &quest_owner_account).await?;
    assert!(nft_tokens.len() == 0, "Tokens amount incorrect");

    let user_1_boxes =
        get_quest_boxes_per_owner(&mystery_box_contract, &user_1_account, quest_id).await?;
    assert!(user_1_boxes.len() == 0);

    let user_2_boxes =
        get_quest_boxes_per_owner(&mystery_box_contract, &user_2_account, quest_id).await?;
    assert!(user_2_boxes.len() == 0);

    mint_box(
        &mystery_box_contract,
        &quest_owner_account,
        &user_1_account,
        quest_id,
        BoxRarity::Rare,
    )
    .await?;

    mint_box(
        &mystery_box_contract,
        &quest_owner_account,
        &user_2_account,
        quest_id,
        BoxRarity::Epic,
    )
    .await?;

    //Checking that user1 and user2 have 1 box each
    let user_1_boxes =
        get_quest_boxes_per_owner(&mystery_box_contract, &user_1_account, quest_id).await?;
    assert!(user_1_boxes.len() == 1);

    let user_2_boxes =
        get_quest_boxes_per_owner(&mystery_box_contract, &user_2_account, quest_id).await?;
    assert!(user_2_boxes.len() == 1);

    //Checking that user1 and user2 have 0 NFT tokens before claim
    let user_1_nft_tokens = get_nft_tokens(&nft_contract, &user_1_account).await?;
    assert!(user_1_nft_tokens.len() == 0, "Tokens amount incorrect");

    let user_2_nft_tokens = get_nft_tokens(&nft_contract, &user_2_account).await?;
    assert!(user_2_nft_tokens.len() == 0, "Tokens amount incorrect");

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

    //Checking that user1 and user2 have 1 NFT token each after claim. Validating that user1 received token_1 and user2 received token_2
    let user_1_nft_tokens = get_nft_tokens(&nft_contract, &user_1_account).await?;
    assert!(user_1_nft_tokens.len() == 1, "Tokens amount incorrect");

    let user_1_nft_token = user_1_nft_tokens[0].as_object().unwrap();
    assert!(user_1_nft_token.get("token_id").unwrap() == NFT_1_TOKEN_ID);

    let nft_1_metadata = user_1_nft_token
        .get("metadata")
        .unwrap()
        .as_object()
        .unwrap();
    assert!(nft_1_metadata.get("media").unwrap() == NFT_1_MEDIA);

    let user_2_nft_tokens = get_nft_tokens(&nft_contract, &user_2_account).await?;
    assert!(user_2_nft_tokens.len() == 1, "Tokens amount incorrect");

    let user_2_nft_token = user_2_nft_tokens[0].as_object().unwrap();
    assert!(user_1_nft_token.get("token_id").unwrap() == NFT_1_TOKEN_ID);

    let nft_2_metadata = user_2_nft_token
        .get("metadata")
        .unwrap()
        .as_object()
        .unwrap();
    assert!(nft_2_metadata.get("media").unwrap() == NFT_2_MEDIA);

    Ok(())
}

async fn get_nft_tokens(
    nft_contract: &Contract,
    user_account: &Account,
) -> anyhow::Result<(Vec<Value>)> {
    let outcome: Value = user_account
        .call(nft_contract.id(), "nft_tokens_for_owner")
        .args_json(json!({
            "account_id": user_account.id(),
        }))
        .view()
        .await?
        .json()?;

    let nfts = outcome.as_array().unwrap();

    return Ok((nfts.clone()));
}

async fn trust_nft_contract(
    mystery_box_contract: &Contract,
    nft_contract: &Contract,
) -> anyhow::Result<()> {
    let add_nft_reward_outcome = mystery_box_contract
        .call("trust_nft_contract")
        .args_json(json!({
            "contract_id": nft_contract.id(),
        }))
        .transact()
        .await?;

    return Ok(());
}

async fn add_nft_reward(
    mystery_box_contract: &Contract,
    nft_contract: &Contract,
    user_account: &Account,
    quest_id: &Value,
    rarity: BoxRarity,
    token_id: &str,
) -> anyhow::Result<ExecutionFinalResult> {
    const STORAGE_DEPOSIT: u128 = 5;

    let add_nft_reward_outcome = user_account
        .call(nft_contract.id(), "nft_transfer_call")
        .max_gas()
        .deposit(NearToken::from_yoctonear(1))
        .args_json(json!({
            "receiver_id": mystery_box_contract.id(),
            "msg": format!("{{\"quest_id\": {}, \"rarity\": \"{}\" }}", quest_id, rarity.to_string()),
            "token_id": token_id.to_string()
        }))
        .transact()
        .await?;

    assert!(
        add_nft_reward_outcome.is_success(),
        "Adding nft reward failed {:#?}",
        add_nft_reward_outcome
    );

    return Ok((add_nft_reward_outcome));
}
