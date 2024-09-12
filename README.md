# Solfhe Analyzer

## Table of Contents
- [Introduction](#introduction)
- [Core Concepts](#core-concepts)
- [Features](#features)
- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [Configuration](#configuration)
- [Usage](#usage)
- [Project Structure](#project-structure)
- [Module Descriptions](#module-descriptions)
- [Solana Integration](#solana-integration)
- [Data Processing Pipeline](#data-processing-pipeline)
- [Cryptographic Methods](#cryptographic-methods)
- [Performance Optimization](#performance-optimization)
- [Error Handling and Logging](#error-handling-and-logging)
- [Testing Strategy](#testing-strategy)
- [Deployment](#deployment)
- [Maintenance and Monitoring](#maintenance-and-monitoring)
- [Future Enhancements](#future-enhancements)
- [Contributing](#contributing)
- [Troubleshooting](#troubleshooting)
- [FAQ](#faq)
- [Glossary](#glossary)
- [References](#references)
- [License](#license)

## Introduction

Solfhe Analyzer is an advanced data analysis and blockchain integration tool designed to extract meaningful insights from web browsing patterns and securely store them on the Solana blockchain. This project exemplifies the convergence of big data analytics, cryptographic techniques, and distributed ledger technology.

The analyzer operates by extracting recent URLs from the Chrome browser's history, performing sophisticated keyword analysis, and utilizing custom compression algorithms before interacting with the Solana blockchain. This approach ensures data integrity, confidentiality, and immutability while leveraging the high-performance capabilities of the Solana network.

## Core Concepts

1. **URL Extraction**: The system accesses the SQLite database used by Chrome to store browsing history, extracting recent URLs for analysis.

2. **Keyword Analysis**: Implemented using natural language processing techniques, this module identifies and quantifies significant terms from the extracted URLs.

3. **ZK Compression**: A proprietary compression algorithm that not only reduces data size but also adds a layer of privacy to the stored information.

4. **Blockchain Integration**: Utilizes Solana's high-throughput blockchain for secure and decentralized data storage and retrieval.

5. **Asynchronous Processing**: Implements non-blocking I/O operations to enhance performance and responsiveness.

6. **Automated Execution**: Features a self-sustaining execution loop that performs analysis at configurable intervals.

## System Architecture

The Solfhe Analyzer follows a modular architecture, comprised of the following key components:

1. **Data Extraction Layer**: Interfaces with the Chrome browser's SQLite database.
2. **Analysis Engine**: Processes raw URL data to extract meaningful insights.
3. **Compression Module**: Applies the ZK compression algorithm to analyzed data.
4. **Blockchain Interface**: Manages all interactions with the Solana blockchain.
5. **Persistence Layer**: Handles local storage of processed data and configuration.
6. **Execution Controller**: Orchestrates the overall flow and timing of operations.

| Component | Function | Key Technologies |
|-----------|----------|-------------------|
| URL Extractor | Retrieves recent URLs from Chrome history | SQLite, Rusqlite |
| Keyword Analyzer | Processes URLs to extract and count significant terms | Custom NLP algorithms, Rust standard library |
| ZK Compressor | Compresses data with added privacy layer | Custom encryption, SHA-256, AES-256 |
| Solana Interface | Manages blockchain interactions for data storage and retrieval | Solana SDK, RPC client |
| Data Persistor | Handles local storage of processed data | Serde, JSON |
| Execution Controller | Orchestrates the analysis cycle | Rust's async/await, Tokio |
| Configuration Manager | Manages system settings | TOML parser |
| Error Handler | Provides robust error management across the system | Custom error types, Result<T, E> |
| Logger | Records system events and errors | Log crate |
| Python Integration | Executes additional data processing scripts | Python subprocess management |

## Features

- Chrome History Analysis
- Advanced Keyword Extraction and Quantification
- Custom ZK Compression Algorithm
- Solana Blockchain Integration for Data Storage and Retrieval
- Automated Execution Cycle
- JSON-based Data Persistence
- Python Script Integration for Extended Functionality
- Configurable Analysis Parameters
- Robust Error Handling and Logging
- Performance-Optimized Data Structures

## Prerequisites

- Rust (stable channel, version 1.55 or higher)
- Solana CLI tools (version 1.7 or higher)
- Python 3.8+
- Chrome browser (version 90 or higher)
- SQLite3
- OpenSSL development packages

## Installation

1. Clone the repository:
   ```
   git clone https://github.com/yourusername/solfhe-analyzer.git
   cd solfhe-analyzer
   ```

2. Install Rust dependencies:
   ```
   cargo build --release
   ```

3. Set up Solana:
   ```
   solana-keygen new
   solana config set --url https://api.devnet.solana.com
   ```

4. Install Python dependencies:
   ```
   pip install -r requirements.txt
   ```

5. Compile the project:
   ```
   cargo build --release
   ```

## Configuration

The Solfhe Analyzer can be configured via the `config.toml` file. Key configuration parameters include:

- `analysis_interval`: Time between analysis cycles (in seconds)
- `max_urls_per_cycle`: Maximum number of URLs to analyze in each cycle
- `solana_network`: Solana network to connect to (e.g., "devnet", "testnet", "mainnet-beta")
- `minimum_keyword_length`: Minimum length for a word to be considered a keyword
- `compression_level`: ZK compression level (1-9, where 9 is maximum compression)

## ⚙️ Usage

1. Start the Solana validator (if using a local network):
   ```rust
   cargo run
   ```

2. Run the front-end:
   ```bash
   npm i
   npm run dev
   ```

3. Monitor the output in the terminal for analysis results and blockchain interactions.

4. Check the `solfhe.json` file for persistent storage of analysis results.

## Solana Integration

The Solfhe Analyzer interacts with the Solana blockchain in several ways:

1. **Account Management**: Creates and manages Solana accounts for data transactions.
2. **Transaction Handling**: Constructs, signs, and submits transactions containing compressed analysis data.
3. **Data Retrieval**: Fetches stored data from the blockchain and decompresses it for local use.
4. **Balance Monitoring**: Ensures sufficient SOL balance for transaction fees.

The integration leverages Solana's high throughput and low latency to provide near real-time data storage and retrieval.

## Data Processing Pipeline

1. URL Extraction from Chrome history
2. Keyword analysis and frequency counting
3. Data compression using ZK algorithm
4. JSON serialization of compressed data
5. Solana transaction construction and submission
6. Blockchain confirmation and receipt logging
7. Local JSON storage of transaction details
8. Python script execution for additional processing

## Cryptographic Methods

The ZK compression algorithm employs several cryptographic techniques:

1. **Hashing**: SHA-256 for creating unique identifiers of data chunks
2. **Symmetric Encryption**: AES-256 in GCM mode for encrypting compressed data
3. **Key Derivation**: PBKDF2 for generating encryption keys from a master password
4. **Zero-Knowledge Proofs**: Implemented for verifying data integrity without revealing content

## Performance Optimization

- **Connection Pooling**: Utilized for database connections to reduce overhead
- **Batch Processing**: URLs are processed in configurable batches to balance throughput and resource usage
- **Asynchronous I/O**: Implemented using Tokio for non-blocking operations
- **Caching**: LRU cache implemented for frequently accessed data
- **Parallel Processing**: Rayon library used for parallel data processing where applicable

## Error Handling and Logging

The project implements comprehensive error handling using Rust's `Result` and `Option` types. Custom error types are defined for specific modules, allowing for granular error reporting.

Logging is implemented using the `log` crate, with different log levels (ERROR, WARN, INFO, DEBUG) used appropriately throughout the codebase.

## Testing Strategy

1. **Unit Tests**: Cover individual functions and methods, particularly in the `keyword_analyzer` and `zk_compression` modules.
2. **Integration Tests**: Test the interaction between different modules, especially the flow from data extraction to blockchain submission.
3. **Mocking**: The `mockall` crate is used to mock external dependencies like the Solana RPC client for isolated testing.
4. **Property-Based Testing**: Implemented using the `proptest` crate for functions with a wide input range, such as the compression algorithm.
5. **Continuous Integration**: GitHub Actions workflow set up to run tests on every push and pull request.

## Deployment

For production deployment, consider the following steps:

1. Set up a Solana validator node or use a reliable RPC provider.
2. Configure environment variables for sensitive information (e.g., encryption keys, RPC endpoints).
3. Use a process manager like `systemd` or `supervisord` to ensure the analyzer runs continuously.
4. Implement monitoring and alerting using tools like Prometheus and Grafana.

## Maintenance and Monitoring

Regular maintenance tasks include:

1. Updating Rust and dependency versions
2. Monitoring Solana account balances
3. Rotating encryption keys periodically
4. Analyzing logs for error patterns
5. Performing database maintenance on the local SQLite file

## Future Enhancements

1. Implement a web-based dashboard for real-time analytics visualization
2. Extend support to other popular browsers (Firefox, Safari)
3. Enhance the keyword analysis with machine learning techniques
4. Implement a more sophisticated Zero-Knowledge Proof system
5. Develop a plugin system for easy extension of functionality

## Contributing

We welcome contributions to the Solfhe Analyzer project. Please follow these steps:

1. Fork the repository
2. Create a feature branch: `git checkout -b feature-name`
3. Commit your changes: `git commit -am 'Add some feature'`
4. Push to the branch: `git push origin feature-name`
5. Submit a pull request

Please ensure your code adheres to the project's coding standards and is well-documented.

## Troubleshooting

Common issues and their solutions:

1. **Solana RPC Connection Failures**: Ensure your Solana CLI is correctly configured and the specified network is operational.
2. **Chrome History Access Errors**: Verify that Chrome is not running when the analyzer attempts to access the history database.
3. **Compression Errors**: Check that the input data is correctly formatted and within the size limits specified in the configuration.

## FAQ

Q: How often does the analyzer run?
A: By default, it runs every 60 seconds, but this is configurable in the `config.toml` file.

Q: Is the data stored on the blockchain encrypted?
A: Yes, the data is compressed and encrypted before being stored on the Solana blockchain.

Q: Can I use this with other browsers?
A: Currently, only Chrome is supported, but there are plans to extend support to other browsers in the future.

## Glossary

- **ZK Compression**: Zero-Knowledge Compression, a custom algorithm that compresses data while preserving privacy.
- **Solana**: A high-performance blockchain platform used for decentralized applications and marketplaces.
- **RPC (Remote Procedure Call)**: A protocol that one program can use to request a service from a program located on another computer in a network.
- **SQLite**: A C-language library that implements a small, fast, self-contained, high-reliability, full-featured, SQL database engine.
- **Tokenization**: The process of breaking a stream of text into words, phrases, symbols, or other meaningful elements called tokens.

## Transaction Link:
https://explorer.solana.com/tx/3hzAmJYfNWYZEiAQLGDUCJqbspREDUXiqoi5Y8onGNtQTHRCpa2xz18s8h4c7iZ79c7hLb6hEMuWaqzHdCToukXq?cluster=custom


## References

1. Solana Documentation: https://docs.solana.com/
2. Rust Programming Language: https://www.rust-lang.org/
3. Chrome SQLite Schema: https://www.forensicswiki.org/wiki/Google_Chrome
4. Zero-Knowledge Proofs: https://en.wikipedia.org/wiki/Zero-knowledge_proof

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

---

Developed by [solΦ]
