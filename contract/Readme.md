#### To Implement

- [ ] Implement Storage management properly

####

- use unsigned integer for quest id
-

# Init contract

`near contract call-function as-transaction succinct-slave.testnet new json-args {} prepaid-gas '100.0 Tgas' attached-deposit '0 NEAR' sign-as succinct-slave.testnet network-config testnet sign-with-keychain send`

# Read quests_per_owner:

`near contract call-function as-read-only succinct-slave.testnet quests_per_owner json-args '{"account_id":"volodymyr_matseliukh1.testnet"}' network-config testnet now`

# Read questboxes_supply_per_owner:

`near contract call-function as-read-only succinct-slave.testnet questboxes_supply_per_owner json-args '{"account_id":"volodymyr_matseliukh1.testnet"}' network-config testnet now`

# Read questboxes_per_owner:

`near contract call-function as-read-only succinct-slave.testnet questboxes_per_owner json-args '{"account_id":"volodymyr_matseliukh1.testnet"}' network-config testnet now`

`near contract call-function as-read-only succinct-slave.testnet questboxes_per_owner json-args '{"account_id":"denbite.testnet"}' network-config testnet now`

# Call create_quest

`near contract call-function as-transaction succinct-slave.testnet create_quest json-args '{"title": "My first quest"}' prepaid-gas '100.0 Tgas' attached-deposit '0.01 NEAR' sign-as volodymyr_matseliukh1.testnet network-config testnet sign-with-keychain send`

# Call delete_quest

`near contract call-function as-transaction succinct-slave.testnet delete_quest json-args '{"quest_id":1}' prepaid-gas '100.0 Tgas' attached-deposit '0 NEAR' sign-as volodymyr_matseliukh1.testnet network-config testnet sign-with-keychain send`

# Read users

`near contract call-function as-read-only succinct-slave.testnet get_users json-args '{"quest_id":0, "pagination": {"page": 0, "size": 40}}' network-config testnet now`

# Read get_trusted_nft_contracts

`near contract call-function as-read-only succinct-slave.testnet get_trusted_nft_contracts json-args '{}' network-config testnet now`

# Call claim

`near contract call-function as-transaction succinct-slave.testnet claim json-args '{"quest_id":1, "box_id": 0}' prepaid-gas '100.0 Tgas' attached-deposit '1 yoctoNEAR' sign-as volodymyr_matseliukh1.testnet network-config testnet sign-with-keychain send`
