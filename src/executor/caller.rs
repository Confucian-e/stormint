use alloy::{
    contract::{ContractInstance, Interface},
    dyn_abi::DynSolValue,
    json_abi::JsonAbi,
    primitives::Address,
    providers::ProviderBuilder,
    transports::http::reqwest::Url,
};
use eyre::Result;

/// Calls a function on an Ethereum smart contract.
///
/// # Arguments
///
/// * `rpc_http` - The HTTP URL of the Ethereum RPC endpoint.
/// * `abi` - The JSON ABI of the contract.
/// * `contract_address` - The address of the contract.
/// * `function_name` - The name of the function to call.
/// * `args` - The arguments to pass to the function.
///
/// # Returns
///
/// * `Result<Vec<DynSolValue>>` - The result of the function call on success.
pub async fn call(
    rpc_http: Url,
    abi: JsonAbi,
    contract_address: Address,
    function_name: &str,
    args: &[DynSolValue],
) -> Result<Vec<DynSolValue>> {
    let provider = ProviderBuilder::new()
        .with_recommended_fillers()
        .on_http(rpc_http);

    let contract = ContractInstance::new(contract_address, provider.clone(), Interface::new(abi));

    let value = contract.function(function_name, args)?.call().await?;

    Ok(value)
}
