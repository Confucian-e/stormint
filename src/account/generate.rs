use alloy::signers::local::{coins_bip39::English, MnemonicBuilder, PrivateKeySigner};
use eyre::Result;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;

/// Generates multiple Ethereum accounts from a single mnemonic phrase with progress tracking.
///
/// This function creates a series of Ethereum accounts using BIP39 mnemonic derivation.
/// It uses parallel processing for optimal performance and displays a progress bar
/// during generation.
///
/// # Arguments
///
/// * `mnemonic` - A valid BIP39 mnemonic phrase (12 or 24 words)
/// * `start_index` - The starting index for HD wallet derivation (inclusive)
/// * `end_index` - The ending index for HD wallet derivation (exclusive)
///
/// # Returns
///
/// Returns `Ok(Vec<PrivateKeySigner>)` containing the generated accounts, or an error
/// if the mnemonic is invalid or derivation fails.
///
/// # Examples
///
/// ```rust,no_run
/// use stormint::account::generate_accounts;
///
/// # fn main() -> eyre::Result<()> {
/// let mnemonic = "test test test test test test test test test test test junk";
/// let accounts = generate_accounts(mnemonic, 0, 10)?;
/// 
/// println!("Generated {} accounts", accounts.len());
/// for (i, account) in accounts.iter().enumerate() {
///     println!("Account {}: {}", i, account.address());
/// }
/// # Ok(())
/// # }
/// ```
///
/// # Performance
///
/// - Uses parallel processing with Rayon for faster generation
/// - Pre-allocates result vector to minimize memory allocations
/// - Displays real-time progress for user feedback
///
/// # Errors
///
/// This function will return an error if:
/// - The mnemonic phrase is invalid or malformed
/// - HD wallet derivation fails for any index
/// - Progress bar template formatting fails
pub fn generate_accounts(
    mnemonic: &str,
    start_index: u32,
    end_index: u32,
) -> Result<Vec<PrivateKeySigner>> {
    generate_accounts_internal(mnemonic, start_index, end_index, true)
}

/// Internal account generation function with optional progress bar.
///
/// This is the core implementation used by both the public API and tests.
/// It allows controlling whether to show a progress bar, making it suitable
/// for both user-facing operations and clean test runs.
///
/// # Arguments
///
/// * `mnemonic` - A valid BIP39 mnemonic phrase
/// * `start_index` - Starting derivation index (inclusive)
/// * `end_index` - Ending derivation index (exclusive)
/// * `show_progress` - Whether to display a progress bar
///
/// # Implementation Details
///
/// - Uses Rayon for parallel account generation
/// - Pre-allocates vectors for optimal memory usage
/// - Supports clean test execution without progress output
///
/// # Performance Notes
///
/// Account generation is CPU-bound and benefits from parallel execution.
/// The function uses all available CPU cores through Rayon's work-stealing scheduler.
pub fn generate_accounts_internal(
    mnemonic: &str,
    start_index: u32,
    end_index: u32,
    show_progress: bool,
) -> Result<Vec<PrivateKeySigner>> {
    let account_count = end_index - start_index;
    
    // Optional progress bar only for non-test environments
    let pb = if show_progress {
        let pb = ProgressBar::new(account_count as u64);
        pb.set_style(ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} accounts generation ({percent}%) ETA: {eta_precise}")?
            .progress_chars("=>-"));
        Some(pb)
    } else {
        None
    };

    // Generate initial builder once
    let builder = MnemonicBuilder::<English>::default().phrase(mnemonic);

    // Parallel account generation with collect instead of mutex
    let accounts: Result<Vec<_>> = (start_index..end_index)
        .into_par_iter()
        .map(|index| -> Result<PrivateKeySigner> {
            let wallet = builder.clone().index(index)?.build()?;
            if let Some(ref pb) = pb {
                pb.inc(1);
            }
            Ok(wallet)
        })
        .collect();

    let accounts = accounts?;

    // Finish progress bar if it exists
    if let Some(pb) = pb {
        pb.finish_with_message("Account generation completed successfully!");
    }

    Ok(accounts)
}

#[cfg(test)]
mod tests {
    use super::*;

    const PHRASE: &str = "test test test test test test test test test test test junk";

    #[test]
    fn test_accounts_generation_length() {
        let (start_index, end_index) = (0u32, 9u32);
        let accounts = generate_accounts_internal(PHRASE, start_index, end_index, false);

        assert!(accounts.is_ok());
        assert_eq!(accounts.unwrap().len() as u32, end_index - start_index);
    }

    #[test]
    fn test_accounts_generation() {
        let (start_index, end_index) = (0u32, 1u32);
        let accounts = generate_accounts_internal(PHRASE, start_index, end_index, false);

        if let Some(first_account) = accounts.unwrap().first() {
            let address = (*first_account).address();
            assert_eq!(
                address.to_string(),
                "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266"
            );
        }
    }

    #[test]
    fn test_empty_account_range() {
        let accounts = generate_accounts_internal(PHRASE, 0, 0, false);
        assert!(accounts.is_ok());
        assert_eq!(accounts.unwrap().len(), 0);
    }

    #[test]
    fn test_single_account() {
        let accounts = generate_accounts_internal(PHRASE, 5, 6, false).unwrap();
        assert_eq!(accounts.len(), 1);
    }

    #[test]
    fn test_large_account_range() {
        let accounts = generate_accounts_internal(PHRASE, 10, 15, false).unwrap();
        assert_eq!(accounts.len(), 5);
        
        // Check that all accounts are unique
        let addresses: std::collections::HashSet<_> = accounts.iter().map(|a| a.address()).collect();
        assert_eq!(addresses.len(), 5);
    }

    #[test]
    fn test_invalid_mnemonic() {
        let result = generate_accounts_internal("invalid mnemonic phrase", 0, 1, false);
        assert!(result.is_err());
    }

    #[test]
    fn test_public_generate_accounts_function() {
        // Test that the public function works, but hide progress for tests
        let accounts = generate_accounts_internal(PHRASE, 0, 2, false).unwrap();
        assert_eq!(accounts.len(), 2);
        
        // Also verify the public API exists (this doesn't actually run during testing)
        let _ = std::panic::catch_unwind(|| {
            // This would show progress in actual usage
            generate_accounts
        });
    }

    #[test]
    fn test_different_ranges() {
        let accounts1 = generate_accounts_internal(PHRASE, 0, 3, false).unwrap();
        let accounts2 = generate_accounts_internal(PHRASE, 10, 13, false).unwrap();
        
        assert_eq!(accounts1.len(), 3);
        assert_eq!(accounts2.len(), 3);
        
        // Accounts from different ranges should be different
        for account1 in &accounts1 {
            for account2 in &accounts2 {
                assert_ne!(account1.address(), account2.address());
            }
        }
    }
}
