# solΦ: Privacy-Preserving Advertising Platform
![1](https://github.com/user-attachments/assets/00468590-236a-4d69-98d5-c2ce8fa4cb81)

<div align="center">
  
  [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
  [![Rust](https://img.shields.io/badge/Rust-1.55+-orange.svg)](https://www.rust-lang.org/)
  [![Solana](https://img.shields.io/badge/Solana-1.7+-blue.svg)](https://solana.com/)
  [![Light Protocol](https://img.shields.io/badge/Light%20Protocol-Integrated-brightgreen.svg)](https://lightprotocol.com/)
</div>

## Table of Contents

- [Introduction](#-introduction)
- [Key Features](#-key-features)
- [System Architecture](#-system-architecture)
- [Blockchain Integrations](#-blockchain-integrations)
- [ZK-Compression and Privacy](#-zk-compression-and-privacy)
- [Light SDK Integration](#-light-sdk-integration)
- [On-Chain Operations](#-on-chain-operations)
- [Advanced NLP and Advertising Algorithms](#-advanced-nlp-and-advertising-algorithms)
- [Installation](#-installation)
- [Usage](#-usage)
- [Contributing](#-contributing)
- [License](#-license)

## Introduction

solΦ offers a user-friendly advertising tool that turns adverts into a source of income for users. Users earn by watching adverts and using advertising platforms, while advertisers can reach their target audience directly. This creates an ecosystem that delivers value for both parties. At the same time, advertisers can never read users' data. Privacy and profit are ensured by zk-compression.

### Demo

🎥 [DEMO VIDEO](..)

### Problem
You can see the biggest disadvantages of traditional advertisement applications or protocols:
![Problem](https://github.com/user-attachments/assets/aeccf05c-e811-48cf-bed7-40babab28a86)

### Solution
solΦ is an private advertising platform where users earn revenue by watching ads, advertisers can reach their target audience directly, and privacy is guaranteed with zk-compression technology:
![Solution](https://github.com/user-attachments/assets/1aa71461-8808-4c50-96fa-bc49e1b2a92d)


## Key Features

![Features](https://github.com/user-attachments/assets/abe87fc2-a515-4ec4-a433-64902698868d)

| Feature | Description | Technology Stack |
|---------|-------------|-------------------|
| Multi-Chain Support | Seamless integration with Solana and future blockchain networks | Solana SDK, Wormhole Protocol |
| ZK-Compression | Privacy-preserving data compression | Custom ZK-SNARK implementation |
| On-Chain Analytics | Decentralized and transparent ad performance metrics | Light Protocol, Solana Programs |
| AI-Powered Matching | Intelligent ad-to-user pairing | gize.tech NLP algorithms (future integration) |
| User Rewards | "Blinks" token system for engagement | SPL Token, Solana Token Program |
| Cross-Chain Interoperability | Fluid asset and data transfer between supported chains | Wormhole Bridge |


## 🏗 System Architecture

solΦ's architecture is designed for scalability, security, and interoperability:
<img width="800" alt="arch" src="https://github.com/user-attachments/assets/0dc5e7a1-afb0-4946-98b1-50dfaff22462">



## Blockchain Integrations

### Solana Integration

solΦ leverages Solana's high-throughput, low-latency blockchain for core functionalities:

- **Account Management**: Utilizes Solana's account model for user wallets and data storage.
- **Transaction Processing**: Employs Solana's parallel transaction processing for real-time ad interactions.
- **Token Operations**: Implements SPL Token standard for "Blinks" reward distribution.

### Light Protocol Integration

Light Protocol enhances solΦ's privacy and scalability features:

- **ZK-Compressed State**: Utilizes Light's merkle tree structure for efficient state management.
- **Privacy-Preserving Transactions**: Implements Light's zero-knowledge proofs for confidential ad interactions.
- **Scalable Analytics**: Leverages Light's compression techniques for on-chain analytics storage.

## ZK-Compression and Privacy

solΦ's ZK-compression algorithm is at the heart of our privacy-preserving technology:

1. **Data Minimization**: Reduces the amount of data stored on-chain without losing essential information.
2. **Zero-Knowledge Proofs**: Allows verification of ad interactions without revealing user-specific data.
3. **Homomorphic Encryption**: Enables computations on encrypted data for secure analytics.

```rust
use light_sdk::compressed_account::{CompressedAccount, CompressedAccountData};

fn zk_compress(data: &str) -> Result<CompressedAccount, Error> {
    // ZK-compression implementation
    // ...
}
```

##  Light SDK Integration
![light-protocol](https://github.com/user-attachments/assets/2e9644cb-a7b8-4108-99a2-a569ca074ad2)

solΦ extensively utilizes the Light SDK for on-chain operations:

```rust
use light_sdk::{
    compressed_account::{CompressedAccount, CompressedAccountData},
    merkle_context::MerkleContext,
    proof::CompressedProof,
    constants::PROGRAM_ID_LIGHT_TOKEN,
};

// Example of creating a compressed account
let compressed_account = CompressedAccount {
    owner: program_id,
    lamports: 0,
    address: None,
    data: Some(CompressedAccountData {
        discriminator: [0; 8],
        data: zk_compressed_data,
        data_hash: hash_data(&zk_compressed_data),
    }),
};
```

## On-Chain Operations

solΦ's on-chain components are designed for efficiency and privacy:

1. **Ad Serving**: On-chain programs match compressed user profiles with ad inventory.
2. **Analytics Storage**: Aggregated campaign performance data is stored in compressed format.
3. **Reward Distribution**: Automated "Blinks" token transfers based on user engagement.

Example of an on-chain instruction:

```rust
#[derive(Accounts)]
pub struct ServeAd<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub ad_account: Account<'info, CompressedAdAccount>,
    pub system_program: Program<'info, System>,
}

pub fn serve_ad(ctx: Context<ServeAd>, user_profile: Vec<u8>) -> Result<()> {
    // Ad serving logic using ZK-compressed user profile
    // ...
}
```

## 🧠 Advanced NLP and Advertising Algorithms

In future iterations, solΦ will incorporate cutting-edge NLP technologies in collaboration with gize.tech:
![132906091](https://github.com/user-attachments/assets/292a9784-3f83-4f96-9df1-908e190c5ffd)


- **Privacy-Preserving NLP**: Utilizing zero-knowledge proofs for language processing without exposing raw text data.
- **Federated Learning**: Implementing decentralized machine learning models for improved ad targeting.
- **Semantic Analysis**: Employing advanced NLP techniques for nuanced understanding of user interests and ad content.

These advancements will be implemented with a continued focus on user privacy and data minimization.

## 🛠 Installation

Detailed installation instructions...

## 🖱 Usage

Step-by-step guide on how to use solΦ...

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guidelines](CONTRIBUTING.md) for more details.

## 📄 License

solΦ is released under the MIT License. See the [LICENSE](LICENSE) file for more details.

---

<div align="center">
  <sub>Built with ❤️ by the solΦ Team</sub>
</div>
