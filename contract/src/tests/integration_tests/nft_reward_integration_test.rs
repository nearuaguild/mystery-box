use near_sdk::{
    serde_json::{json, Map, Value},
    AccountId,
};
use near_workspaces::{
    network::Sandbox, result::ExecutionFinalResult, types::Gas, types::NearToken, Account,
    Contract, Worker,
};

use crate::tests::integration_tests::utils::deploy_nft_contract;

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

    assert!(
        transfer_outcome.is_success(),
        "Transfer failed {:#?}",
        transfer_outcome
    );

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
