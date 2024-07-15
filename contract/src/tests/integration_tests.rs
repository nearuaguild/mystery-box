use near_sdk::{serde_json::json, AccountId};
use near_workspaces::{network::Sandbox, result::ExecutionFinalResult, types::NearToken, Contract, Worker};

const NFT_CONTRACT: &[u8] = include_bytes!("./wasms/non_fungible_token.wasm");

#[tokio::test]
async fn test_nft_transfer() -> anyhow::Result<()> {
    let sandbox = near_workspaces::sandbox().await?;
    let contract = deploy_nft_contract(&sandbox).await?;

    let root = sandbox.root_account()?;
    let user_account = root.create_subaccount("user1").transact().await?.unwrap();

    // transfering nft to user
    let transfer_outcome = contract
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

    let outcome: near_sdk::serde_json::Value = contract
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