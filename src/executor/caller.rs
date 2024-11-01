use alloy::contract::{ContractInstance, Interface};
use alloy::dyn_abi::DynSolValue;
use alloy::json_abi::JsonAbi;
use alloy::primitives::Address;
use alloy::providers::ProviderBuilder;
use alloy::transports::http::reqwest::Url;
use eyre::Result;

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
