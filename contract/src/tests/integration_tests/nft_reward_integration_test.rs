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
    deploy_nft_contract, NFT_1_MEDIA, NFT_1_TOKEN_ID, NFT_2_MEDIA, NFT_2_TOKEN_ID,
};

const QUEST_OWNER_ACCOUNT_NAME: &str = "quest_owner";
const USER_1_ACCOUNT_NAME: &str = "user1";
const USER_2_ACCOUNT_NAME: &str = "user2";

const INITIAL_NEAR_PER_ACCOUNT: u128 = 10;

const USER_1_BOX_NEAR_REWARD: u128 = 1;
const USER_2_BOX_NEAR_REWARD: u128 = 2;

const QUEST_TITLE: &str = "my first quest";

#[tokio::test]
async fn test_nft_reward_flow() -> anyhow::Result<()> {
    let sandbox = near_workspaces::sandbox().await?;
    let mystery_box_contract = deploy_mystery_box_contract(&sandbox).await?;

    let root = sandbox.root_account()?;
    let quest_owner_account = create_user_account(&root, QUEST_OWNER_ACCOUNT_NAME).await?;
    let user_1_account = create_user_account(&root, USER_1_ACCOUNT_NAME).await?;
    let user_2_account = create_user_account(&root, USER_2_ACCOUNT_NAME).await?;

    let nft_contract = deploy_nft_contract(&sandbox, &quest_owner_account).await?;

    let nft_tokens = get_nft_tokens(&nft_contract, &quest_owner_account).await?;

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

    let user_1_boxes =
        get_quest_boxes_per_owner(&mystery_box_contract, &user_1_account, quest_id).await?;
    assert!(user_1_boxes.len() == 1);

    let user_2_boxes =
        get_quest_boxes_per_owner(&mystery_box_contract, &user_2_account, quest_id).await?;
    assert!(user_2_boxes.len() == 1);

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

    let user_1_nft_tokens = get_nft_tokens(&nft_contract, &user_1_account).await?;
    assert!(user_1_nft_tokens.len() == 1, "Tokens amount incorrect");

    let user_1_nft_token = user_1_nft_tokens[0].as_object().unwrap();
    assert!(user_1_nft_token.get("token_id").unwrap() == NFT_1_TOKEN_ID);
    
    let nft_1_metadata = user_1_nft_token.get("metadata").unwrap().as_object().unwrap();
    assert!(nft_1_metadata.get("media").unwrap() == NFT_1_MEDIA);
    

    
    let user_2_nft_tokens = get_nft_tokens(&nft_contract, &user_2_account).await?;
    assert!(user_2_nft_tokens.len() == 1, "Tokens amount incorrect");

    let user_2_nft_token = user_2_nft_tokens[0].as_object().unwrap();
    assert!(user_1_nft_token.get("token_id").unwrap() == NFT_1_TOKEN_ID);
    
    let nft_2_metadata = user_2_nft_token.get("metadata").unwrap().as_object().unwrap();
    assert!(nft_2_metadata.get("media").unwrap() == NFT_2_MEDIA);

    Ok(())
}

async fn create_user_account(root: &Account, account_name: &str) -> anyhow::Result<(Account)> {
    let user_account = root
        .create_subaccount(account_name)
        .initial_balance(NearToken::from_near(INITIAL_NEAR_PER_ACCOUNT))
        .transact()
        .await?
        .unwrap();

    return Ok((user_account));
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

async fn create_quest(
    mystery_box_contract: &Contract,
    user_account: &Account,
    quest_title: &str,
) -> anyhow::Result<(ExecutionFinalResult)> {
    let create_quest_outcome = user_account
        .call(mystery_box_contract.id(), "create_quest")
        .deposit(NearToken::from_millinear(10))
        .args_json(json!({
            "title": quest_title
        }))
        .transact()
        .await?;

    assert!(
        create_quest_outcome.is_success(),
        "Quest creation failed {:#?}",
        create_quest_outcome
    );

    return Ok((create_quest_outcome));
}

async fn get_first_quests_per_owner(
    mystery_box_contract: &Contract,
    user_account: &Account,
) -> anyhow::Result<(Map<String, Value>)> {
    let outcome: Value = mystery_box_contract
        .call("quests_per_owner")
        .args_json(json!({
            "account_id": user_account.id(),
        }))
        .view()
        .await?
        .json()?;

    let quests = outcome.as_array().unwrap();

    assert!(quests.len() == 1);

    let quest = quests[0].as_object().unwrap();

    assert!(quest.get("title").unwrap() == QUEST_TITLE);

    return Ok((quest.clone()));
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

async fn mint_box(
    mystery_box_contract: &Contract,
    quest_owner: &Account,
    user_account: &Account,
    quest_id: &Value,
    rarity: BoxRarity,
) -> anyhow::Result<ExecutionFinalResult> {
    const STORAGE_DEPOSIT: u128 = 10;

    let mint_box_outcome = quest_owner
        .call(mystery_box_contract.id(), "mint_many")
        .deposit(NearToken::from_millinear(STORAGE_DEPOSIT))
        .args_json(json!({
            "quest_id": quest_id,
            "rarity": rarity.to_string(),
            "accounts": [
                user_account.id()
            ]
        }))
        .transact()
        .await?;

    assert!(
        mint_box_outcome.is_success(),
        "Minting box failed {:#?}",
        mint_box_outcome
    );

    return Ok((mint_box_outcome));
}

async fn get_quest_boxes_per_owner(
    mystery_box_contract: &Contract,
    user_account: &Account,
    quest_id: &Value,
) -> anyhow::Result<Vec<Value>> {
    let outcome: Value = mystery_box_contract
        .call("questboxes_per_owner")
        .args_json(json!({
            "account_id": user_account.id(),
        }))
        .view()
        .await?
        .json()?;

    let quest_boxes_ids = outcome.as_array().unwrap();

    return Ok((quest_boxes_ids.clone()));
}

async fn claim_box(
    mystery_box_contract: &Contract,
    user_account: &Account,
    quest_id: &Value,
    box_id: &Value,
) -> anyhow::Result<ExecutionFinalResult> {
    let claim_box_outcome = user_account
        .call(mystery_box_contract.id(), "claim")
        .gas(Gas::from_tgas(300))
        .deposit(NearToken::from_yoctonear(1))
        .args_json(json!({
            "quest_id": quest_id,
            "box_id": box_id
        }))
        .transact()
        .await?;

    assert!(
        claim_box_outcome.is_success(),
        "Claiming box failed {:#?}",
        claim_box_outcome
    );

    return Ok((claim_box_outcome));
}

// #[tokio::test]
// async fn test_nft_transfer() -> anyhow::Result<()> {
//     let sandbox = near_workspaces::sandbox().await?;
//     let nft_contract = deploy_nft_contract(&sandbox).await?;

//     let root = sandbox.root_account()?;
//     let user_account = root.create_subaccount("user1").transact().await?.unwrap();

//     // transfering nft to user
//     let transfer_outcome = nft_contract
//         .call("nft_transfer")
//         .deposit(NearToken::from_yoctonear(1))
//         .args_json(json!({
//             "token_id": "1",
//             "receiver_id": user_account.id(),
//             "memo": "transfer ownership"
//         }))
//         .transact()
//         .await?;

//     assert!(
//         transfer_outcome.is_success(),
//         "Transfer failed {:#?}",
//         transfer_outcome
//     );

//     let outcome: near_sdk::serde_json::Value = nft_contract
//         .call("nft_tokens_for_owner")
//         .args_json(json!({
//             "account_id": user_account.id(),
//         }))
//         .view()
//         .await?
//         .json()?;

//     let nft_tokens = outcome.as_array().unwrap();

//     assert!(nft_tokens.len() == 1);

//     let nft_token = nft_tokens[0].as_object().unwrap();

//     assert!(nft_token.get("token_id").unwrap() == "1");

//     println!("new_default_meta outcome: {:#?}", outcome.as_array());

//     Ok(())
// }
