#### To Implement

- [ ] Implement Storage management properly

####

- use unsigned integer for quest id
- 

# Init contract
`near contract call-function as-transaction erratic-stew.testnet new json-args {} prepaid-gas '100.0 Tgas' attached-deposit '0 NEAR' sign-as erratic-stew.testnet network-config testnet sign-with-keychain send`

# Read quests_per_owner:
`near contract call-function as-read-only erratic-stew.testnet quests_per_owner json-args '{"account_id":"volodymyr_matseliukh1.testnet"}' network-config testnet now`

# Read quests_per_owner:
`near contract call-function as-read-only erratic-stew.testnet questboxes_supply_per_owner json-args '{"account_id":"volodymyr_matseliukh1.testnet"}' network-config testnet now`

# Read questboxes_per_owner:
`near contract call-function as-read-only erratic-stew.testnet questboxes_per_owner json-args '{"account_id":"volodymyr_matseliukh1.testnet"}' network-config testnet now`

# Call create_quest
`near contract call-function as-transaction erratic-stew.testnet create_quest json-args '{"title": "My first quest"}' prepaid-gas '100.0 Tgas' attached-deposit '0.01 NEAR' sign-as volodymyr_matseliukh1.testnet network-config testnet sign-with-keychain send`

