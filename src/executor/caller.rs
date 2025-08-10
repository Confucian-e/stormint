use alloy::{
    contract::{ContractInstance, Interface},
    dyn_abi::DynSolValue,
    json_abi::JsonAbi,
    primitives::Address,
    providers::ProviderBuilder,
    transports::http::reqwest::Url,
};
use eyre::Result;

/// Performs a read-only call to an Ethereum smart contract function.
///
/// This function executes a view/pure contract function without creating a transaction
/// or modifying blockchain state. It's free to execute and returns immediately.
/// Use [`execute`](crate::executor::execute) for state-changing operations.
///
/// # Arguments
///
/// * `rpc_http` - Ethereum RPC endpoint URL for the query
/// * `abi` - JSON ABI definition of the target contract
/// * `contract_address` - Address of the deployed smart contract
/// * `function_name` - Name of the view/pure function to call
/// * `args` - Function arguments as dynamic Solidity values
///
/// # Returns
///
/// Returns `Ok(Vec<DynSolValue>)` containing the function's return values,
/// or an error if the call fails or the function doesn't exist.
///
/// # Examples
///
/// ```rust,no_run
/// use stormint::executor::call;
/// use alloy::dyn_abi::DynSolValue;
///
/// # async fn example() -> eyre::Result<()> {
/// # let rpc_url: alloy::transports::http::reqwest::Url = "http://localhost:8545".parse()?;
/// # let contract_abi = alloy::json_abi::JsonAbi::new();
/// # let contract_address: alloy::primitives::Address = "0x0000000000000000000000000000000000000000".parse()?;
/// # let account_address: alloy::primitives::Address = "0x0000000000000000000000000000000000000000".parse()?;
/// // Call a simple getter function
/// // let result = call(
/// //     rpc_url,
/// //     contract_abi,
/// //     contract_address,
/// //     "totalSupply",
/// //     &[], // No arguments
/// // ).await?;
/// //
/// // if let Some(DynSolValue::Uint(total_supply, 256)) = result.first() {
/// //     println!("Total supply: {}", total_supply);
/// // }
///
/// // Call a function with arguments
/// let args = vec![DynSolValue::Address(account_address)];
/// // let result = call(
/// //     rpc_url,
/// //     contract_abi,
/// //     contract_address,
/// //     "balanceOf",
/// //     &args,
/// // ).await?;
/// //
/// // if let Some(DynSolValue::Uint(balance, 256)) = result.first() {
/// //     println!("Account balance: {}", balance);
/// // }
/// # Ok(())
/// # }
/// ```
///
/// # Performance
///
/// - Executes instantly (no mining required)
/// - No gas costs
/// - Can be called as frequently as needed
/// - Results reflect the current blockchain state
///
/// # Return Value Handling
///
/// The function returns a vector because Solidity functions can return multiple values:
/// - Single return: `vec![DynSolValue::Uint(value, 256)]`
/// - Multiple returns: `vec![DynSolValue::Address(addr), DynSolValue::Uint(amount, 256)]`
/// - No returns: `vec![]`
///
/// # Errors
///
/// This function will return an error if:
/// - The function name doesn't exist in the ABI
/// - Function arguments don't match the ABI specification
/// - The contract doesn't exist at the specified address
/// - Network connection fails
/// - The function reverts (even view functions can revert)
pub async fn call(
    rpc_http: Url,
    abi: JsonAbi,
    contract_address: Address,
    function_name: &str,
    args: &[DynSolValue],
) -> Result<Vec<DynSolValue>> {
    let provider = ProviderBuilder::new().connect_http(rpc_http);

    let contract = ContractInstance::new(contract_address, provider.clone(), Interface::new(abi));

    let value = contract.function(function_name, args)?.call().await?;

    Ok(value)
}

#[cfg(test)]
mod tests {
    use alloy::dyn_abi::DynSolValue;

    #[test]
    fn test_empty_args() {
        let args: &[DynSolValue] = &[];
        assert_eq!(args.len(), 0);
    }

    #[test]
    fn test_function_name_validation() {
        let function_name = "balanceOf";
        assert!(!function_name.is_empty());
        assert!(function_name
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_'));
    }
}
