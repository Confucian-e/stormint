use crate::executor::execute;
use alloy::{
    dyn_abi::DynSolValue,
    json_abi::JsonAbi,
    primitives::{Address, TxHash, U256},
    signers::local::PrivateKeySigner,
    transports::http::reqwest::Url,
};
use eyre::{Report, Result};

/// Represents the result of a mint operation.
///
/// # Fields
///
/// * `signer` - The address of the signer who performed the mint operation.
/// * `result` - The result of the mint operation, containing either the transaction hash on success or an error report on failure.
#[derive(Debug)]
pub struct MintResult {
    pub signer: Address,
    pub result: Result<TxHash, Report>,
}

impl MintResult {
    /// Creates a new `MintResult` instance.
    ///
    /// # Arguments
    ///
    /// * `signer` - The address of the signer who performed the mint operation.
    /// * `tx` - The result of the mint operation, containing either the transaction hash on success or an error report on failure.
    ///
    /// # Returns
    ///
    /// * `Self` - A new `MintResult` instance.
    fn new(signer: Address, tx: Result<TxHash, Report>) -> Self {
        Self { signer, result: tx }
    }
}

/// Mints tokens in a loop for multiple signers.
///
/// # Arguments
///
/// * `signers` - A vector of private key signers who will perform the mint operations.
/// * `rpc_http` - The HTTP URL of the Ethereum RPC endpoint.
/// * `abi` - The JSON ABI of the contract.
/// * `contract_address` - The address of the contract.
/// * `function_name` - The name of the function to execute (optional, defaults to "mint").
/// * `args` - The arguments to pass to the function (optional).
/// * `value` - The amount of Ether to send with the transaction (optional).
///
/// # Returns
///
/// * `Result<Vec<MintResult>>` - A vector of `MintResult` containing the results of the mint operations.
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
        // Use &signers to avoid unnecessary cloning
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

/// Executes a mint operation on an Ethereum smart contract.
///
/// # Arguments
///
/// * `signer` - The private key signer of the account executing the transaction.
/// * `rpc_http` - The HTTP URL of the Ethereum RPC endpoint.
/// * `abi` - The JSON ABI of the contract.
/// * `contract_address` - The address of the contract.
/// * `function_name` - The name of the function to execute (optional, defaults to "mint").
/// * `args` - The arguments to pass to the function (optional).
/// * `value` - The amount of Ether to send with the transaction (optional).
///
/// # Returns
///
/// * `Result<TxHash>` - The transaction hash of the executed transaction on success.
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
