use crate::executor::execute;
use alloy::{
    dyn_abi::DynSolValue,
    json_abi::JsonAbi,
    primitives::{Address, TxHash, U256},
    signers::local::PrivateKeySigner,
    transports::http::reqwest::Url,
};
use eyre::Result;

/// Parameters for the `distribute` function.
///
/// # Fields
///
/// * `receiver` - The address of the receiver.
/// * `amount` - The amount to be distributed.
#[derive(Debug)]
pub struct DistributeParam {
    pub receiver: Address,
    pub amount: U256,
}

/// Distributes Ether to multiple receivers.
///
/// # Arguments
///
/// * `sender` - The private key signer of the sender.
/// * `rpc_http` - The HTTP URL of the Ethereum RPC endpoint.
/// * `abi` - The JSON ABI of the contract.
/// * `contract_address` - The address of the contract.
/// * `params` - A vector of `DistributeParam` containing receiver addresses and amounts.
///
/// # Returns
///
/// * `Result<TxHash>` - The transaction hash on success.
pub async fn distribute(
    sender: PrivateKeySigner,
    rpc_http: Url,
    abi: JsonAbi,
    contract_address: Address,
    params: Vec<DistributeParam>,
) -> Result<TxHash> {
    let txns = DynSolValue::Array(
        params
            .iter()
            .map(|r| {
                DynSolValue::Tuple(vec![
                    DynSolValue::from(r.receiver),
                    DynSolValue::from(r.amount),
                ])
            })
            .collect(),
    );

    let args = &[txns];

    let value: U256 = params.iter().map(|param| param.amount).sum();

    let tx_hash = execute(
        sender,
        rpc_http,
        abi,
        contract_address,
        "distributeEther",
        args,
        Some(value),
    )
    .await?
    .tx_hash;

    Ok(tx_hash)
}
