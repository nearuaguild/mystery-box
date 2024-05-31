use std::collections::HashSet;

use near_sdk::borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::store::LookupMap;
use near_sdk::json_types::U128;
use near_sdk::serde::Serialize;
use near_sdk::serde_json::{self};
use near_sdk::{
    env, log, near_bindgen, require, AccountId, NearToken, BorshStorageKey, Gas, PanicOnDefault,
    Promise, PromiseResult,
};

const CONTRACT: &[u8] = include_bytes!("../../contract/res/mystery_box.wasm");

#[derive(BorshStorageKey, BorshSerialize)]
enum StorageKey {
    Contracts,
    ContractsForOwner,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq, Serialize)]
#[serde(crate = "near_sdk::serde", rename_all = "snake_case")]
pub struct Metadata {
    pub title: String,
    pub contract_id: AccountId,
}

#[derive(Serialize)]
#[serde(crate = "near_sdk::serde", rename_all = "snake_case")]
pub struct ContractNewArguments {
    owner_id: AccountId,
    default_trusted_nft_contracts: Vec<AccountId>,
}

enum Network {
    Mainnet,
    Testnet,
}

impl From<AccountId> for Network {
    fn from(account_id: AccountId) -> Self {
        if account_id.to_string().ends_with(".near") {
            return Network::Mainnet;
        }

        return Network::Testnet;
    }
}

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

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    contracts: LookupMap<AccountId, Metadata>,
    contracts_for_owner: LookupMap<AccountId, HashSet<AccountId>>,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new() -> Self {
        Self {
            contracts: LookupMap::new(StorageKey::Contracts),
            contracts_for_owner: LookupMap::new(StorageKey::ContractsForOwner),
        }
    }

    pub fn contract_metadata(&self, contract_id: AccountId) -> Option<Metadata> {
        self.contracts.get(&contract_id)
    }

    pub fn contracts_for_owner(&self, account_id: AccountId) -> Vec<Metadata> {
        self.contracts_for_owner
            .get(&account_id)
            .unwrap_or_default()
            .iter()
            .map(|contract_id| self.contracts.get(contract_id))
            .flatten()
            .collect()
    }

    pub fn contract_byte_cost(&self) -> U128 {
        U128(self.internal_contract_byte_cost())
    }

    fn internal_contract_byte_cost(&self) -> NearToken {
        let contract_bytes = CONTRACT.len() as u128;

        env::storage_byte_cost() * contract_bytes
    }

    fn internal_register_contract(
        &mut self,
        owner_id: AccountId,
        contract_id: AccountId,
        title: String,
    ) {
        let metadata = Metadata {
            contract_id: contract_id.clone(),
            title: title.clone(),
        };

        require!(
            self.contracts.insert(&contract_id, &metadata).is_none(),
            "ERR_CONTRACT_ALREADY_EXIST"
        );

        let mut owner_contracts = self.contracts_for_owner.get(&owner_id).unwrap_or_default();
        owner_contracts.insert(contract_id.clone());

        self.contracts_for_owner.insert(&owner_id, &owner_contracts);
    }

    fn internal_remove_contract(&mut self, owner_id: AccountId, contract_id: AccountId) {
        self.contracts.remove(&contract_id);

        let mut owner_contracts = self.contracts_for_owner.get(&owner_id).unwrap_or_default();
        owner_contracts.remove(&contract_id);

        self.contracts_for_owner.insert(&owner_id, &owner_contracts);
    }

    #[payable]
    pub fn deploy_mystery_box_contract(&mut self, alias: String, title: String) -> Promise {
        // Assert the sub-account is valid
        let current_account_id = env::current_account_id().to_string();
        let contract_id: AccountId = format!("{alias}.{current_account_id}").parse().unwrap();
        require!(
            env::is_valid_account_id(contract_id.as_bytes()),
            "ERR_INVALID_SUBACCOUNT_ALIAS"
        );

        let owner_id = env::predecessor_account_id();

        self.internal_register_contract(owner_id.clone(), contract_id.clone(), title);

        let attached_deposit = env::attached_deposit();
        let contract_deposit = self.internal_contract_byte_cost();

        assert!(
            attached_deposit >= contract_deposit,
            "Deposited amount must be bigger than {contract_deposit} yocto, you attached {attached_deposit} yocto",
        );

        let args = serde_json::to_vec(&ContractNewArguments {
            owner_id: owner_id.clone(),
            default_trusted_nft_contracts: get_trusted_nft_contracts(),
        })
        .unwrap();
        let deployment_promise = Promise::new(contract_id.clone())
            .create_account()
            .transfer(attached_deposit)
            .deploy_contract(CONTRACT.to_vec())
            .function_call("new".to_owned(), args, 0, Gas::ONE_TERA * 5);

        let callback_promise = Contract::ext(env::current_account_id())
            .deploy_mystery_box_contract_callback(
                contract_id.to_owned(),
                attached_deposit.to_owned(),
                owner_id.to_owned(),
            );

        deployment_promise.then(callback_promise)
    }

    #[private]
    pub fn deploy_mystery_box_contract_callback(
        &mut self,
        contract_id: AccountId,
        deposited_amount: NearToken,
        owner_id: AccountId,
    ) -> Option<AccountId> {
        // https://docs.rs/near-sdk/latest/near_sdk/env/fn.promise_results_count.html
        require!(env::promise_results_count() == 1, "ERR_TOO_MANY_RESULTS");

        let deployment_result = env::promise_result(0);

        match deployment_result {
            PromiseResult::Successful(_) => {
                log!(format!(
                    "Successfully created {contract_id} and put Mystery Contract on it"
                ));

                Some(contract_id)
            }
            _ => {
                log!(format!(
                    "Error creating {contract_id}, reverting state and returning {deposited_amount} yocto to {owner_id}"
                ));
                Promise::new(owner_id.clone()).transfer(deposited_amount);

                self.internal_remove_contract(owner_id.clone(), contract_id.clone());

                None
            }
        }
    }
}
