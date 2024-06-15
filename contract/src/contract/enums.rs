use near_sdk::borsh::{self, BorshSerialize};
use near_sdk::{AccountId, BorshStorageKey};


pub enum Network {
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

#[derive(BorshStorageKey, BorshSerialize)]
pub enum StorageKey {
    Pools,
    PoolsByRarity,
    NftPoolByKey,
    Boxes,
    BoxesPerOwner,
    TrustedNftContracts,
    ProbabilityByRarity,
    Users,
    Quests,
    QuestsPerOwner,
    QuestBoxesPerOwner,
    QuestIdsPerOwner,
}