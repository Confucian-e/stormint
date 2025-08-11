# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Overview

Stormint is a high-performance Rust CLI tool for multi-account FreeMint token operations on Ethereum. The project combines Rust backend logic with Solidity smart contracts for efficient batch token minting and gas distribution operations.

## Development Commands

### Rust Development
- **Build**: `cargo build --release` (optimized production build)
- **Test**: `cargo test` (runs all unit, integration, and E2E tests)
- **Format**: `cargo fmt` (format code)
- **Lint**: `cargo clippy --all-targets --all-features` (lint with warnings as errors via RUSTFLAGS="-Dwarnings")
- **Check**: `cargo check --verbose` (fast compilation check)

### Smart Contract Development
Navigate to `contracts/` directory first:
- **Install dependencies**: `forge soldeer install` (installs Soldeer dependencies)
- **Build contracts**: `forge build --sizes` (compile with size reporting)
- **Test contracts**: `forge test -vvv` (run tests with maximum verbosity)
- **Format contracts**: `forge fmt --check` (check Solidity formatting)
- **Update dependencies**: `forge update` (update git submodules)

### Combined Development Workflow
For full testing that includes contract compilation:
```bash
cd contracts/
forge soldeer install && forge update && forge build --sizes
cd ../
cargo test --verbose
```

## Architecture

The project follows a modular Rust library architecture with the following core modules:

### Core Rust Modules (`src/`)
- **`account/`**: HD wallet account generation from mnemonic phrases using hierarchical deterministic derivation
- **`distributor/`**: Batch Ether distribution to multiple addresses using the Distributor smart contract
- **`executor/`**: Smart contract transaction execution and read-only contract calls via Alloy
- **`mint/`**: Concurrent token minting operations with comprehensive result tracking

### Smart Contracts (`contracts/src/`)
- **`FreeMint.sol`**: ERC20 token with one-time minting capability per address (5M tokens per mint, 210B max supply)
- **`Distributor.sol`**: Gas-optimized batch Ether distribution contract with automatic refund mechanism

### Key Dependencies
- **Alloy**: Ethereum interaction (providers, signers, contracts, JSON-ABI)
- **Tokio**: Async runtime with multi-threaded support
- **Rayon**: Data parallelism for account generation
- **OpenZeppelin**: Smart contract security standards (v5.1.0)
- **Foundry**: Smart contract development toolchain

### Performance Optimizations
- Concurrent processing using Tokio for I/O-bound operations
- Parallel processing using Rayon for CPU-bound tasks
- Memory optimization with Arc for shared data and pre-allocation
- Release profile optimized for size (85% smaller than debug builds) with LTO and panic=abort

### Testing Structure (`tests/`)
- **Unit tests**: Individual module functionality (`tests/unit/`)
- **Integration tests**: Cross-module interactions (`tests/integration/`)
- **E2E tests**: Full workflow testing with local blockchain (`tests/e2e/`)
- **Common utilities**: Shared test helpers and mock data (`tests/common/`)

## Development Workflow

1. **Smart Contract Changes**: Always run contract tests first with `forge test -vvv` in the `contracts/` directory
2. **Rust Changes**: Run `cargo clippy` before committing (CI enforces zero warnings)
3. **Before Committing**: Run both `cargo fmt --check` and `forge fmt --check`
4. **Testing**: The test suite requires compiled smart contracts, so ensure contracts are built before running Rust tests

## CI/CD Integration

The project uses GitHub Actions with two parallel workflows:
- **Rust CI**: Triggered by changes to Rust code (`src/`, `tests/`, `Cargo.*`)
- **Foundry CI**: Triggered by changes to smart contracts (`contracts/**`)

Both workflows enforce formatting, linting, building, and comprehensive testing before allowing merges to main.