use alloy::dyn_abi::DynSolValue;
use alloy::json_abi::JsonAbi;
use alloy::primitives::{Address, U256};
use alloy::transports::http::reqwest::Url;
use eyre::Result;
use stormint::executor::call;

/// Asynchronously retrieves the token balance of a specified account.
///
/// # Arguments
///
/// * `url` - The URL of the Ethereum node.
/// * `abi` - The ABI of the contract.
/// * `contract_address` - The address of the contract.
/// * `account` - The address of the account to query the balance for.
///
/// # Returns
///
/// A `Result` containing the token balance as a `U256` value, or an error if the call fails.
pub async fn get_token_balance(
    url: Url,
    abi: JsonAbi,
    contract_address: Address,
    account: Address,
) -> Result<U256> {
    let balance = call(
        url,
        abi,
        contract_address,
        "balanceOf",
        &[DynSolValue::from(account)],
    )
    .await?;

    let balance = match balance.first() {
        Some(DynSolValue::Uint(balance, 256)) => *balance,
        _ => U256::default(),
    };

    Ok(balance)
}
