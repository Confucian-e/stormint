use alloy::{
    contract::{ContractInstance, Interface},
    dyn_abi::DynSolValue,
    json_abi::JsonAbi,
    network::{Ethereum, EthereumWallet},
    primitives::{Address, TxHash, U256},
    providers::ProviderBuilder,
    signers::local::PrivateKeySigner,
    transports::http::{reqwest::Url, Client, Http},
};
use eyre::Result;

/// Represents the result of a contract execution.
///
/// # Fields
///
/// * `caller` - The address of the caller.
/// * `tx_hash` - The transaction hash of the executed transaction.
#[derive(Debug)]
pub struct Execution {
    pub caller: Address,
    pub tx_hash: TxHash,
}

impl Execution {
    /// Creates a new `Execution` instance.
    ///
    /// # Arguments
    ///
    /// * `caller` - The address of the caller.
    /// * `tx_hash` - The transaction hash of the executed transaction.
    ///
    /// # Returns
    ///
    /// * `Self` - A new `Execution` instance.
    fn new(caller: Address, tx_hash: TxHash) -> Self {
        Self { caller, tx_hash }
    }
}

/// Executes a function on an Ethereum smart contract.
///
/// # Arguments
///
/// * `account` - The private key signer of the account executing the transaction.
/// * `rpc_http` - The HTTP URL of the Ethereum RPC endpoint.
/// * `abi` - The JSON ABI of the contract.
/// * `contract_address` - The address of the contract.
/// * `function_name` - The name of the function to execute.
/// * `args` - The arguments to pass to the function.
/// * `value` - The amount of Ether to send with the transaction (optional).
///
/// # Returns
///
/// * `Result<Execution>` - The result of the contract execution, containing the caller's address and the transaction hash.
pub async fn execute(
    account: PrivateKeySigner,
    rpc_http: Url,
    abi: JsonAbi,
    contract_address: Address,
    function_name: &str,
    args: &[DynSolValue],
    value: Option<U256>,
) -> Result<Execution> {
    let caller = account.address();
    let wallet = EthereumWallet::new(account);
    let provider = ProviderBuilder::new()
        .with_recommended_fillers()
        .wallet(wallet)
        .on_http(rpc_http);

    let contract: ContractInstance<Http<Client>, _, Ethereum> =
        ContractInstance::new(contract_address, provider.clone(), Interface::new(abi));

    let tx_hash = contract
        .function(function_name, args)?
        .value(value.unwrap_or_default())
        .send()
        .await?
        .watch()
        .await?;

    Ok(Execution::new(caller, tx_hash))
}
