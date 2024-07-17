use near_sdk::borsh::{self, BorshSerialize};
use near_sdk::{AccountId, BorshStorageKey, CryptoHash};

#[derive(Debug)]
pub enum Network {
    Mainnet = 1,
    Testnet = 2,
}

impl From<AccountId> for Network {
    fn from(account_id: AccountId) -> Self {
        if account_id.to_string().ends_with(".testnet") {
            return Network::Testnet;
        }

        return Network::Mainnet;
    }
}

#[derive(BorshStorageKey, BorshSerialize)]
pub enum StorageKey {
    Pools { quest_hash: CryptoHash },
    PoolsByRarity { quest_hash: CryptoHash },
    NftPoolByKey { quest_hash: CryptoHash },
    Boxes { quest_hash: CryptoHash },
    BoxesPerOwner { quest_hash: CryptoHash },
    ProbabilityByRarity { quest_hash: CryptoHash },
    Users { quest_hash: CryptoHash },
    Quests,
    QuestsPerOwner,
    QuestBoxesPerOwner,
    QuestBoxesData { account_hash: CryptoHash },
    QuestIdsPerOwner { account_hash: CryptoHash },
    TrustedNftContracts,
}
