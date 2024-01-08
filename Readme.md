<!-- PROJECT LOGO -->
<br />
<div align="center">
  <h2 align="center">Near Protocol | Mystery Box</h2>

  <p align="center">
    Randomly distribute NEAR & NFT rewards among users
    <br />
    <br />
    <a href="https://github.com/nearuaguild"> Explore other projects</a>
    ·
    <a href="https://github.com/nearuaguild/near-web4-contracts/issues">Report a bug</a>
  </p>
</div>

<!-- ABOUT THE PROJECT -->

## About The Project

Mystery Boxes, a fascinating and dynamic concept, are a popular mechanism used to distribute rewards randomly among users in various domains, including online gaming, eCommerce, and marketing campaigns. These intriguing boxes offer participants an element of chance, suspense, and surprise, creating an engaging and thrilling experience.

### Built With

- [![Rust][rust]][rust-url]
- [near-sdk-rs (v4.0.0)](https://github.com/near/near-sdk-rs)
- [Blockchain Operating System](https://docs.near.org/bos)

---

<!-- GETTING STARTED -->

## Getting Started

💡 _Before starting, please ensure that you have all the necessary installations completed_

- [Rust](https://doc.rust-lang.org/cargo/getting-started/installation.html)
- [Cargo](https://github.com/rust-lang/cargo#compiling-from-source)
- [BOS CLI](https://github.com/bos-cli-rs/bos-cli-rs)
- [Git](https://git-scm.com/book/en/v2/Getting-Started-Installing-Git/)

## Smart Contract

### Build

1. Clone the repo

```sh
 git clone https://github.com/nearuaguild/mystery-box.git
```

2. Open project folder

```sh
cd mystery-box/contract
```

3. Compile the project

```sh
./build.sh
```

### Deploy to testnet

```sh
near dev-deploy --wasmFile res/mystery_box.wasm
```

### Initialize

```sh
near call mystery_box.testnet new '{}' --accountId some_account.testnet
```

### Add NEAR reward

```sh
near call mystery_box.testnet add_near_reward '{"rarity": "epic", "amount": "100000000000000000000000", "capacity": "5"}' --accountId some_account.testnet --depositYocto 186000000000000000000000
```

### Add NFT reward

```sh
near call mystery_box.testnet trust_nft_contract '{"contract_id": "some_nft_contract.testnet"}' --accountId some_account.testnet
near call some_nft_contract.testnet nft_transfer_call '{"token_id": "5", "receiver_id": "mystery_box.testnet", "msg": "epic"}' --accountId some_account.testnet --depositYocto 1
```

### Mint box

```sh
near call mystery_box.testnet mint '{"account_id": "another_account.testnet", "rarity": "legendary"}' --accountId denbite.testnet --depositYocto 1980000000000000000000
```

### Claim box

```sh
near call mystery_box.testnet claim '{"box_id": 1}' --accountId another_account.testnet --depositYocto 1
```

## BOS Widget

Open project folder

```sh
cd mystery-box/widget
```

### Preparation

Update `widget_owner_id` property in each component with the address you want it deploy to

### Deploy

```sh
bos components deploy
```

### Post Deploy

Make sure to include the `contract_id` (the address you deployed the Smart Contract to in the steps above) query parameter in the URL for the widget to function correctly after deploying it

---

## Developed by

![Guild cover][cover]

**Near Ukraine Guild** is a fast-growing guild based in Ukraine, aimed at providing high-quality educational content and assistance to grow a strong community of developers/entrepreneurs/enthusiasts within the Near Protocol ecosystem

## Community Validator Node

![Community Validator cover][validator]

Our validator has been active for a few months now, and the funds it generates are being put towards sponsoring community activities

Join us now to stake and earn 10% APY

**Click below to get started 👇**

<a href="https://bit.ly/43GSKhs" target="_blank">
<img src="https://img.shields.io/badge/stake-red?style=for-the-badge"  height="48" />
</a>

## Socials

[![Twitter][twitter]][twitter-url]
[![Youtube][youtube]][youtube-url]
[![Telegram Chat][telegram-chat]][telegram-chat-url]
[![Telegram Channel][telegram-channel]][telegram-channel-url]
[![Medium][medium]][medium-url]
[![Github][github]][github-url]

<!-- Images -->

[cover]: https://github.com/nearuaguild/.github/blob/main/images/cover.png
[validator]: https://github.com/nearuaguild/.github/blob/main/images/validator.png

<!-- Socials -->

[twitter]: https://img.shields.io/badge/news-1DA1F2?style=for-the-badge&logo=twitter&logoColor=white
[youtube]: https://img.shields.io/badge/broadcasting-282828?style=for-the-badge&logo=youtube&logoColor=ff0000
[medium]: https://img.shields.io/badge/articles-202020?style=for-the-badge&logo=medium&logoColor=ffffff
[telegram-chat]: https://img.shields.io/badge/chat-229ED9?style=for-the-badge&logo=telegram&logoColor=white
[telegram-channel]: https://img.shields.io/badge/channel-229ED9?style=for-the-badge&logo=telegram&logoColor=white
[github]: https://img.shields.io/badge/code-000000?style=for-the-badge&logo=github&logoColor=ffffff
[twitter-url]: https://twitter.com/nearuaguild
[youtube-url]: https://www.youtube.com/@nearprotocolukraineguild4064
[medium-url]: https://medium.com/near-protocol-ua
[telegram-chat-url]: https://t.me/nearprotocolua
[telegram-channel-url]: https://t.me/nearprotocoluachannel
[github-url]: https://github.com/nearuaguild

<!-- CTA -->

[stake]: https://img.shields.io/badge/stake-yellow?style=for-the-badge
[stake-url]: https://bit.ly/43GSKhs

<!-- LICENSE -->

## License

Based on [LittleLink](https://littlelink.io/)

See `LICENSE.txt` for more information

<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->

<!-- Built with -->

[rust]: https://img.shields.io/badge/rust-000000?style=for-the-badge&logo=rust&logoColor=white
[rust-url]: https://www.rust-lang.org/
[javascript]: https://img.shields.io/badge/javascript-000000?style=for-the-badge&logo=javascript&logoColor=F7E018
[javascript-url]: https://developer.mozilla.org/en-US/docs/Web/JavaScript
[assemblyscript]: https://img.shields.io/badge/assembly%20script-1B7ACE?style=for-the-badge&logo=assemblyscript&logoColor=white
[assemblyscript-url]: https://www.assemblyscript.org/
[littlelink]: https://img.shields.io/badge/LittleLink-1D84FF?style=for-the-badge
[littlelink-url]: https://littlelink.io/
