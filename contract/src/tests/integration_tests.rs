use near_sdk::{serde_json::json, AccountId};
use near_workspaces::{network::Sandbox, result::ExecutionFinalResult, types::NearToken, Contract, Worker};

const NFT_CONTRACT: &[u8] = include_bytes!("./wasms/non_fungible_token.wasm");
const MYSTERY_BOX_CONTRACT: &[u8] = include_bytes!("./wasms/mystery_box.wasm");

#[tokio::test]
async fn test_nft_transfer() -> anyhow::Result<()> {
    let sandbox = near_workspaces::sandbox().await?;
    let nft_contract = deploy_nft_contract(&sandbox).await?;

    let root = sandbox.root_account()?;
    let user_account = root.create_subaccount("user1").transact().await?.unwrap();

    // transfering nft to user
    let transfer_outcome = nft_contract
        .call("nft_transfer")
        .deposit(NearToken::from_yoctonear(1))
        .args_json(json!({
            "token_id": "1",
            "receiver_id": user_account.id(),
            "memo": "transfer ownership"
        }))
        .transact()
        .await?;

    assert!(transfer_outcome.is_success(), "Transfer failed {:#?}", transfer_outcome);

    let outcome: near_sdk::serde_json::Value = nft_contract
        .call("nft_tokens_for_owner")
        .args_json(json!({
            "account_id": user_account.id(),
        }))
        .view()
        .await?
        .json()?;

    let nft_tokens = outcome.as_array().unwrap();

    assert!(nft_tokens.len() == 1);

    let nft_token = nft_tokens[0].as_object().unwrap();

    assert!(nft_token.get("token_id").unwrap() == "1");

    println!("new_default_meta outcome: {:#?}", outcome.as_array());

    Ok(())
}

async fn deploy_nft_contract(sandbox: &Worker<Sandbox>) -> anyhow::Result<(Contract)> {
    let root = sandbox.root_account()?;
    
    // deploying nft contract
    let contract = sandbox.dev_deploy(NFT_CONTRACT).await?;

    // initializing nft contract
    contract
        .call("new_default_meta")
        .args_json(json!({
            "owner_id": contract.id(),
        }))
        .transact()
        .await?;

    // minting nft
    contract
        .call("nft_mint")
        .deposit(NearToken::from_near(1))
        .args_json(json!({
            "token_id": "1",
            "receiver_id": contract.id(),
            "token_metadata": {
                "title": "Mystery box",
                "description": "Mystery box",
                "media": "https://i.etsystatic.com/26469675/r/il/641d1e/2834595676/il_570xN.2834595676_paod.jpg", 
                "copies": 50
            }
        }))
        .transact()
        .await?;

    return Ok((contract));
}

#[tokio::test]
async fn test_mystery_box() -> anyhow::Result<()> {
    let sandbox = near_workspaces::sandbox().await?;
    let mystery_box_contract = deploy_mystery_box_contract(&sandbox).await?;

    let root = sandbox.root_account()?;
    let user_account = root.create_subaccount("user1")
        .initial_balance(NearToken::from_near(10))
        .transact()
        .await?
        .unwrap();

    let title = "My first quest";

    let create_quest_outcome = user_account
        .call(mystery_box_contract.id(), "create_quest")
        .deposit(NearToken::from_millinear(10))
        .args_json(json!({
            "title": title
        }))
        .transact()
        .await?;

    assert!(create_quest_outcome.is_success(), "Quest creation failed {:#?}", create_quest_outcome);

    let outcome: near_sdk::serde_json::Value = mystery_box_contract
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
    
    assert!(quest.get("title").unwrap() == title);

    let quest_id = quest.get("quest_id").unwrap();

    let add_near_reward_outcome = user_account
        .call(mystery_box_contract.id(), "add_near_reward")
        .deposit(NearToken::from_millinear(1005))
        .args_json(json!({
            "quest_id": quest_id,
            "rarity": "rare",
            "capacity": "1",
            "amount": "1000000000000000000000000"
        }))
        .transact()
        .await?;

    assert!(add_near_reward_outcome.is_success(), "Adding reward failed {:#?}", add_near_reward_outcome);

    let add_second_near_reward_outcome = user_account
        .call(mystery_box_contract.id(), "add_near_reward")
        .deposit(NearToken::from_millinear(2005))
        .args_json(json!({
            "quest_id": 0,
            "rarity": "epic",
            "capacity": "1",
            "amount": "2000000000000000000000000"
        }))
        .transact()
        .await?;

    assert!(add_second_near_reward_outcome.is_success(), "Adding second reward failed {:#?}", add_second_near_reward_outcome);

    assert!(user_account.view_account().await?.balance < NearToken::from_near(7));
    assert!(user_account.view_account().await?.balance > NearToken::from_near(6));

    Ok(())
}

async fn deploy_mystery_box_contract(sandbox: &Worker<Sandbox>) -> anyhow::Result<(Contract)> {
    let root = sandbox.root_account()?;
    
    let contract = sandbox.dev_deploy(MYSTERY_BOX_CONTRACT).await?;

    contract
        .call("new")
        .transact()
        .await?;

    return Ok((contract));
}