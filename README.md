# Stormint ⚡

[![Rust CI](https://github.com/Confucian-e/stormint/actions/workflows/rust.yml/badge.svg)](https://github.com/Confucian-e/stormint/actions/workflows/rust.yml)
[![Foundry CI](https://github.com/Confucian-e/stormint/actions/workflows/foundry.yml/badge.svg)](https://github.com/Confucian-e/stormint/actions/workflows/foundry.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A **blazing fast**, high-performance Rust CLI tool for multi-account FreeMint token operations on Ethereum. Stormint enables efficient batch generation of accounts, gas distribution, and concurrent token minting with comprehensive error handling and progress tracking.

## 🚀 Features

- **🔐 Multi-Account Generation**: Generate unlimited Ethereum accounts from mnemonic phrases using HD wallet derivation
- **⛽ Automated Gas Distribution**: Batch distribute Ether to multiple accounts using optimized smart contracts
- **🪙 Concurrent Token Minting**: Mint FreeMint tokens across multiple accounts simultaneously with real-time progress tracking
- **🎯 High Performance**: Optimized with Tokio async runtime and Rayon parallelism for maximum throughput
- **🛡️ Comprehensive Testing**: 100% test coverage with 46+ unit, integration, and E2E tests
- **📊 Progress Monitoring**: Real-time progress bars and detailed result reporting
- **🔧 Production Ready**: Optimized release builds (85% smaller) with robust error handling

## 🏗️ Architecture

### Smart Contracts
- **FreeMint Token**: ERC20 token allowing one-time minting of 5M tokens per address (210B max supply)
- **Distributor Contract**: Gas-optimized batch Ether distribution with automatic refund mechanism

### Rust Modules
- **Account**: HD wallet account generation from mnemonic phrases
- **Distributor**: Batch gas distribution operations
- **Executor**: Smart contract transaction execution via Alloy
- **Mint**: Concurrent token minting with result tracking

## 📋 Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable)
- [Foundry](https://getfoundry.sh/) (for smart contract development)
- Git (for cloning and submodules)

## 🚀 Quick Start

### 1. Clone the Repository
```bash
git clone https://github.com/Confucian-e/stormint.git
cd stormint
```

### 2. Initialize Submodules
```bash
git submodule update --init --recursive
```

### 3. Setup Smart Contracts
```bash
cd contracts/
forge soldeer install
forge build --sizes
cd ../
```

### 4. Build the Rust Application
```bash
cargo build --release
```

### 5. Run Tests (Optional)
```bash
cargo test --verbose
```

## 💻 Usage

### Basic Example
```rust
use stormint::{
    account::generate_accounts,
    distributor::{distribute, DistributeParam},
    mint::mint_loop,
};
use alloy::primitives::utils::parse_ether;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let mnemonic = "your mnemonic phrase here";
    let rpc_url = "http://localhost:8545".parse()?;
    
    // Generate 10 accounts
    let accounts = generate_accounts(mnemonic, 0, 10)?;
    println!("Generated {} accounts", accounts.len());
    
    // Mint tokens concurrently
    let results = mint_loop(
        accounts, rpc_url, abi, contract_address,
        None, None, None
    ).await?;
    
    let successful = results.iter().filter(|r| r.result.is_ok()).count();
    println!("Successfully minted {} tokens", successful);
    
    Ok(())
}
```

## 🔧 Development

### Running Tests
```bash
# Run all tests (requires contract compilation)
cargo test --verbose

# Run only Rust tests
cargo test --lib

# Run contract tests
cd contracts/ && forge test -vvv
```

### Code Quality
```bash
# Format code
cargo fmt

# Lint code
cargo clippy --all-targets --all-features

# Check compilation
cargo check --verbose
```

### Smart Contract Development
```bash
cd contracts/

# Compile contracts
forge build --sizes

# Test contracts
forge test -vvv

# Format Solidity code
forge fmt
```

## 📊 Performance

Stormint is optimized for high performance:

- **Concurrent Processing**: Tokio async runtime for I/O operations
- **Parallel Computing**: Rayon for CPU-intensive account generation
- **Memory Optimization**: Arc for shared data and strategic pre-allocation
- **Small Binary**: Release builds are 85% smaller than debug builds
- **Zero-Copy**: Efficient data structures to minimize allocations

## 🛡️ Testing

The project maintains 100% test coverage with:

- **Unit Tests**: Individual module functionality
- **Integration Tests**: Cross-module interactions
- **End-to-End Tests**: Complete workflow testing with local blockchain
- **Property-Based Tests**: Randomized input validation
- **Performance Tests**: Benchmarking critical paths

Run specific test suites:
```bash
cargo test unit::        # Unit tests only
cargo test integration:: # Integration tests only
cargo test e2e::         # E2E tests only
```

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🤝 Contributing

We welcome contributions! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

### Development Guidelines

1. **Code Style**: Run `cargo fmt` and `forge fmt` before committing
2. **Testing**: Ensure all tests pass with `cargo test`
3. **Linting**: Fix all clippy warnings with `cargo clippy`
4. **Documentation**: Update documentation for public APIs
5. **Commit Messages**: Use conventional commits format

## 📞 Support

- **Issues**: [GitHub Issues](https://github.com/Confucian-e/stormint/issues)
- **Discussions**: [GitHub Discussions](https://github.com/Confucian-e/stormint/discussions)
- **Documentation**: Check the code documentation with `cargo doc --open`

---

**⚡ Built with ❤️ using Rust and Foundry**
