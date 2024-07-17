use near_sdk::{
    serde_json::{json, Map, Value},
    AccountId,
};
use near_workspaces::{
    network::Sandbox, result::ExecutionFinalResult, types::Gas, types::NearToken, Account,
    Contract, Worker,
};

const MYSTERY_BOX_CONTRACT: &[u8] = include_bytes!("./wasms/mystery_box.wasm");
const NFT_CONTRACT: &[u8] = include_bytes!("./wasms/non_fungible_token.wasm");

pub async fn deploy_mystery_box_contract(sandbox: &Worker<Sandbox>) -> anyhow::Result<(Contract)> {
    let root = sandbox.root_account()?;

    let contract = sandbox.dev_deploy(MYSTERY_BOX_CONTRACT).await?;

    contract.call("new").transact().await?;

    return Ok((contract));
}

pub async fn deploy_nft_contract(sandbox: &Worker<Sandbox>) -> anyhow::Result<(Contract)> {
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