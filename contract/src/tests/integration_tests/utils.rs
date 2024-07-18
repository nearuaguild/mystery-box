use anyhow::Ok;
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
