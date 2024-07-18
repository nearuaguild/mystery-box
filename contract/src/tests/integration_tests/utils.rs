use anyhow::Ok;
use near_sdk::{
    serde_json::{json, Map, Value},
    AccountId,
};
use near_workspaces::{
    network::Sandbox, result::ExecutionFinalResult, types::Gas, types::NearToken, Account,
    Contract, Worker,
};

use crate::contract::types::BoxRarity;

const MYSTERY_BOX_CONTRACT: &[u8] = include_bytes!("./wasms/mystery_box.wasm");
const NFT_CONTRACT: &[u8] = include_bytes!("./wasms/non_fungible_token.wasm");

pub const NFT_1_MEDIA: &str = "https://100_NEAR_NFT.com";
pub const NFT_1_TOKEN_ID: &str = "1";

pub const NFT_2_MEDIA: &str = "https://200_NEAR_NFT.com";
pub const NFT_2_TOKEN_ID: &str = "2";

pub const QUEST_OWNER_ACCOUNT_NAME: &str = "quest_owner";
pub const USER_1_ACCOUNT_NAME: &str = "user1";
pub const USER_2_ACCOUNT_NAME: &str = "user2";

pub const INITIAL_NEAR_PER_ACCOUNT: u128 = 10;

pub const USER_1_BOX_NEAR_REWARD: u128 = 1;
pub const USER_2_BOX_NEAR_REWARD: u128 = 2;

pub const QUEST_TITLE: &str = "my first quest";

pub async fn deploy_mystery_box_contract(sandbox: &Worker<Sandbox>) -> anyhow::Result<(Contract)> {
    let root = sandbox.root_account()?;

    let contract = sandbox.dev_deploy(MYSTERY_BOX_CONTRACT).await?;

    contract.call("new").transact().await?;

    return Ok((contract));
}

pub async fn deploy_nft_contract(
    sandbox: &Worker<Sandbox>,
    quest_owner_account: &Account,
) -> anyhow::Result<(Contract)> {
    let root = sandbox.root_account()?;

    let nft_contract = sandbox.dev_deploy(NFT_CONTRACT).await?;

    // initializing nft contract
    nft_contract
        .call("new_default_meta")
        .args_json(json!({
            "owner_id": nft_contract.id(),
        }))
        .transact()
        .await?;

    mint_nft(
        &nft_contract,
        quest_owner_account,
        NFT_1_MEDIA,
        NFT_1_TOKEN_ID,
    )
    .await?;

    mint_nft(
        &nft_contract,
        quest_owner_account,
        NFT_2_MEDIA,
        NFT_2_TOKEN_ID,
    )
    .await?;

    return Ok((nft_contract));
}

async fn mint_nft(
    nft_contract: &Contract,
    quest_owner_account: &Account,
    media: &str,
    token_id: &str,
) -> anyhow::Result<()> {
    // minting nft
    nft_contract
        .call("nft_mint")
        .deposit(NearToken::from_near(1))
        .args_json(json!({
            "token_id": token_id,
            "receiver_id": quest_owner_account.id(),
            "token_metadata": {
                "title": "Mystery box",
                "description": "Mystery box",
                "media": media,
                "copies": 50
            }
        }))
        .transact()
        .await?;

    return Ok(());
}

pub async fn create_user_account(root: &Account, account_name: &str) -> anyhow::Result<(Account)> {
    let user_account = root
        .create_subaccount(account_name)
        .initial_balance(NearToken::from_near(INITIAL_NEAR_PER_ACCOUNT))
        .transact()
        .await?
        .unwrap();

    return Ok((user_account));
}

pub async fn create_quest(
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

pub async fn get_first_quests_per_owner(
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

pub async fn mint_box(
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

pub async fn get_quest_boxes_per_owner(
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

pub async fn claim_box(
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