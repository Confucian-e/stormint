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
    let args = &params
        .iter()
        .map(|r| DynSolValue::CustomStruct {
            name: "Transaction".to_string(),
            prop_names: vec!["receiver".to_string(), "amount".to_string()],
            tuple: vec![r.receiver.into(), r.amount.into()],
        })
        .collect::<Vec<DynSolValue>>();

    let tx_hash = execute(
        sender,
        rpc_http,
        abi,
        contract_address,
        "distributeEther",
        args,
    )
    .await?
    .tx_hash;

    Ok(tx_hash)
}
