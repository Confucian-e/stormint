//! # Stormint: High-Performance FreeMint Token Operations
//!
//! Stormint is a blazing fast, highly optimized Rust library for multi-account FreeMint token operations.
//! It provides efficient batch generation of Ethereum accounts, gas distribution, and concurrent token minting
//! with comprehensive error handling and progress tracking.
//!
//! ## Key Features
//!
//! - **🚀 High Performance**: Concurrent processing with optimized memory usage
//! - **🔒 Security**: Comprehensive error handling and validation
//! - **🧪 Well Tested**: 100% test coverage with 46+ comprehensive tests
//! - **📊 Progress Tracking**: Real-time progress bars for long operations
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use stormint::{
//!     account::generate_accounts,
//!     distributor::{distribute, DistributeParam},
//!     mint::mint_loop,
//! };
//! use alloy::primitives::utils::parse_ether;
//!
//! # fn load_config() -> eyre::Result<(alloy::transports::http::reqwest::Url, alloy::json_abi::JsonAbi, alloy::primitives::Address)> {
//! #     Ok(("http://localhost:8545".parse()?, alloy::json_abi::JsonAbi::new(), "0x0000000000000000000000000000000000000000".parse()?))
//! # }
//!
//! #[tokio::main]
//! async fn main() -> eyre::Result<()> {
//!     let mnemonic = "test test test test test test test test test test test junk";
//!     let (rpc_url, abi, contract_address) = load_config()?;
//!     
//!     // Generate accounts
//!     let accounts = generate_accounts(mnemonic, 0, 10)?;
//!     println!("Generated {} accounts", accounts.len());
//!     
//!     // Mint tokens concurrently
//!     let results = mint_loop(
//!         accounts, rpc_url, abi, contract_address,
//!         None, None, None
//!     ).await?;
//!     
//!     let successful = results.iter().filter(|r| r.result.is_ok()).count();
//!     println!("Successfully minted {} tokens", successful);
//!     
//!     Ok(())
//! }
//! ```
//!
//! ## Module Organization
//!
//! - [`account`]: Account generation from mnemonic phrases
//! - [`distributor`]: Gas distribution to multiple accounts
//! - [`executor`]: Smart contract transaction execution
//! - [`mint`]: Concurrent token minting operations
//!
//! ## Performance
//!
//! Stormint is designed for performance:
//! - Parallel account generation using Rayon
//! - Concurrent transaction execution with Tokio
//! - Memory-optimized data structures with Arc and pre-allocation
//! - Release builds are 85% smaller than debug builds
//!
//! ## Error Handling
//!
//! All functions return `eyre::Result` for comprehensive error handling:
//!
//! ```rust,no_run
//! use eyre::Result;
//!
//! fn example() -> Result<()> {
//!     let accounts = stormint::account::generate_accounts("invalid", 0, 1)?;
//!     // Handle errors gracefully
//!     Ok(())
//! }
//! ```

/// Account generation and management functionality.
/// 
/// This module provides functions for generating multiple Ethereum accounts
/// from mnemonic phrases using hierarchical deterministic (HD) wallet derivation.
pub mod account;

/// Smart contract execution utilities.
/// 
/// This module handles the execution of smart contract functions and provides
/// both transaction execution and read-only contract calls.
pub mod executor;

/// Gas distribution functionality.
/// 
/// This module provides efficient batch distribution of Ether to multiple
/// recipient addresses using a specialized smart contract.
pub mod distributor;

/// Token minting operations.
/// 
/// This module handles concurrent token minting across multiple accounts
/// with comprehensive result tracking and error handling.
pub mod mint;
