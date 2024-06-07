use near_sdk::{env, AccountId};

use crate::contract::enums::Network;

pub fn get_trusted_nft_contracts() -> Vec<AccountId> {
    let network = Network::from(env::current_account_id());

    match network {
        Network::Testnet => vec![
            "nft.helpua.testnet".parse().unwrap(),
            "nft2.helpua.testnet".parse().unwrap(),
            "paras-token-v2.testnet".parse().unwrap(),
            "nearkingdoms.testnet".parse().unwrap(),
        ],
        Network::Mainnet => vec![
            "x.paras.near".parse().unwrap(),
            "nft.herewallet.near".parse().unwrap(),
            "tinkerunion_nft.enleap.near".parse().unwrap(),
            "secretskelliessociety.near".parse().unwrap(),
            "near-punks.near".parse().unwrap(),
            "asac.near".parse().unwrap(),
            "ff.nekotoken.near".parse().unwrap(),
            "spin-nft-contract.near".parse().unwrap(),
            "mrbrownproject.near".parse().unwrap(),
            "nft.thedons.near".parse().unwrap(),
        ],
    }
}