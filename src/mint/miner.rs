use alloy::dyn_abi::DynSolValue;
use alloy::json_abi::JsonAbi;
use alloy::primitives::{Address, TxHash, U256};
use alloy::signers::local::PrivateKeySigner;
use alloy::transports::http::reqwest::Url;
use eyre::{Report, Result};

use crate::executor::execute;

#[derive(Debug)]
pub struct MintResult {
    pub signer: Address,
    pub result: Result<TxHash, Report>,
}

impl MintResult {
    // 添加构造函数
    fn new(signer: Address, tx: Result<TxHash, Report>) -> Self {
        Self { signer, result: tx }
    }
}

pub async fn mint_loop(
    signers: Vec<PrivateKeySigner>,
    rpc_http: Url,
    abi: JsonAbi,
    contract_address: Address,
    function_name: Option<&str>,
    args: Option<&[DynSolValue]>,
    value: Option<U256>,
) -> Result<Vec<MintResult>> {
    let mut results: Vec<MintResult> = Vec::with_capacity(signers.len());
    for signer in &signers {
        // 使用 &signers 避免不必要的 clone
        let tx = execute_mint(
            signer.clone(),
            rpc_http.clone(),
            abi.clone(),
            contract_address,
            function_name,
            args,
            value,
        )
        .await;

        results.push(MintResult::new(signer.address(), tx));
    }

    Ok(results)
}

async fn execute_mint(
    signer: PrivateKeySigner,
    rpc_http: Url,
    abi: JsonAbi,
    contract_address: Address,
    function_name: Option<&str>,
    args: Option<&[DynSolValue]>,
    value: Option<U256>,
) -> Result<TxHash> {
    let function_name = function_name.unwrap_or("mint");

    let tx_hash = execute(
        signer,
        rpc_http,
        abi,
        contract_address,
        function_name,
        args.unwrap_or_default(),
        value,
    )
    .await?
    .tx_hash;

    Ok(tx_hash)
}
