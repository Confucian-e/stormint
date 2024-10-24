use alloy::signers::local::{coins_bip39::English, MnemonicBuilder, PrivateKeySigner};
use eyre::Result;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use std::sync::Mutex;

/// Generates multiple Ethereum accounts from a single mnemonic phrase.
///
/// # Arguments
///
/// * `mnemonic` - A BIP39 mnemonic phrase string
/// * `start_index` - The starting index for the derivation path
/// * `end_index` - The ending index for the derivation path (exclusive)
///
/// # Returns
///
/// * `Result<Vec<PrivateKeySigner>>` - A vector of private key signers on success
pub fn generate_accounts(
    mnemonic: &str,
    start_index: u32,
    end_index: u32,
) -> Result<Vec<PrivateKeySigner>> {
    let account_count = end_index - start_index;

    // set process bar
    let pb = ProgressBar::new(account_count as u64);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} accounts generation ({percent}%) ETA: {eta_precise}")?
        .progress_chars("=>-"));

    // preallocate a vector and create a structure for storing the results
    let accounts = Mutex::new(Vec::with_capacity(account_count as usize));

    // generate initial builder
    let builder = MnemonicBuilder::<English>::default().phrase(mnemonic);

    // parallel account generation
    (start_index..end_index)
        .into_par_iter()
        .try_for_each(|index| -> Result<()> {
            let wallet = builder.clone().index(index)?.build()?;
            accounts.lock().unwrap().push(wallet);
            pb.inc(1);
            Ok(())
        })?;

    // finish process bar and fetch result
    pb.finish_with_message("Account generation completed successfully!");
    let accounts = accounts.into_inner()?;

    Ok(accounts)
}

#[cfg(test)]
mod tests {
    use super::*;

    const PHRASE: &str = "test test test test test test test test test test test junk";

    #[test]
    fn test_accounts_generation_length() {
        let (start_index, end_index) = (0u32, 9u32);
        let accounts = generate_accounts(PHRASE, start_index, end_index);

        assert!(accounts.is_ok());
        assert_eq!(accounts.unwrap().len() as u32, end_index - start_index);
    }

    #[test]
    fn test_accounts_generation() {
        let (start_index, end_index) = (0u32, 1u32);
        let accounts = generate_accounts(PHRASE, start_index, end_index);

        if let Some(first_account) = accounts.unwrap().first() {
            let address = (*first_account).address();
            assert_eq!(
                address.to_string(),
                "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266"
            );
        }
    }
}
