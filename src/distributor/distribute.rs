use alloy::dyn_abi::DynSolValue;
use alloy::json_abi::JsonAbi;
use alloy::primitives::{Address, TxHash, U256};
use alloy::signers::local::PrivateKeySigner;
use alloy::transports::http::reqwest::Url;
use eyre::Result;

use crate::executor::execute;

#[derive(Debug)]
pub struct DistributeParam {
    pub receiver: Address,
    pub amount: U256,
}

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
