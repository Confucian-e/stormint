use alloy::{
    contract::{ContractInstance, Interface},
    dyn_abi::DynSolValue,
    json_abi::JsonAbi,
    network::Ethereum,
    primitives::{Address, TxHash, U256},
    providers::ProviderBuilder,
    signers::local::PrivateKeySigner,
    transports::http::reqwest::Url,
};
use eyre::Result;

/// Result of a smart contract transaction execution.
///
/// Contains the caller's address and the resulting transaction hash,
/// providing complete tracking of transaction execution.
///
/// # Fields
///
/// * `caller` - The Ethereum address that initiated the transaction
/// * `tx_hash` - The unique transaction hash returned by the network
///
/// # Examples
///
/// ```rust
/// use stormint::executor::Execution;
/// use alloy::primitives::{address, TxHash, B256};
///
/// let execution = Execution {
///     caller: address!("d8dA6BF26964aF9D7eEd9e03E53415D37aA96045"),
///     tx_hash: TxHash::from(B256::ZERO),
/// };
/// ```
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

/// Executes a state-changing function on an Ethereum smart contract.
///
/// This function submits a transaction that modifies blockchain state.
/// It waits for the transaction to be mined and confirmed before returning.
/// Use [`call`](crate::executor::call) for read-only operations instead.
///
/// # Arguments
///
/// * `account` - Wallet signer that will pay gas and sign the transaction
/// * `rpc_http` - Ethereum RPC endpoint URL for transaction submission
/// * `abi` - JSON ABI definition of the target contract
/// * `contract_address` - Address of the deployed smart contract
/// * `function_name` - Name of the contract function to call
/// * `args` - Function arguments as dynamic Solidity values
/// * `value` - Optional Ether amount to send (for payable functions)
///
/// # Returns
///
/// Returns `Ok(Execution)` with caller address and transaction hash on success,
/// or an error if the transaction fails, reverts, or times out.
///
/// # Examples
///
/// ```rust,no_run
/// use stormint::executor::execute;
/// use alloy::dyn_abi::DynSolValue;
/// use alloy::primitives::{utils::parse_ether, U256};
///
/// # async fn example() -> eyre::Result<()> {
/// # let wallet = todo!(); // PrivateKeySigner
/// # let rpc_url: alloy::transports::http::reqwest::Url = "http://localhost:8545".parse()?;
/// # let contract_abi = alloy::json_abi::JsonAbi::new();
/// # let contract_address: alloy::primitives::Address = "0x0000000000000000000000000000000000000000".parse()?;
/// # let recipient_address: alloy::primitives::Address = "0x0000000000000000000000000000000000000000".parse()?;
/// // Execute a simple mint function
/// // let execution = execute(
/// //     wallet,
/// //     rpc_url,
/// //     contract_abi,
/// //     contract_address,
/// //     "mint",
/// //     &[], // No arguments
/// //     None, // No ETH value
/// // ).await?;
/// //
/// // println!("Transaction: {:?}", execution.tx_hash);
///
/// // Execute a payable function with arguments
/// let args = vec![DynSolValue::Address(recipient_address)];
/// // let execution = execute(
/// //     wallet,
/// //     rpc_url,
/// //     contract_abi,
/// //     contract_address,
/// //     "mintTo",
/// //     &args,
/// //     Some(parse_ether("0.01")?), // Send 0.01 ETH
/// // ).await?;
/// # Ok(())
/// # }
/// ```
///
/// # Transaction Lifecycle
///
/// 1. Function encodes arguments using the provided ABI
/// 2. Transaction is signed with the account's private key
/// 3. Transaction is submitted to the network via RPC
/// 4. Function waits for transaction to be mined and confirmed
/// 5. Returns execution result with transaction hash
///
/// # Gas Handling
///
/// - Gas limit is estimated automatically by the provider
/// - Gas price follows network recommendations
/// - Failed transactions (reverts) will still consume gas
///
/// # Errors
///
/// This function will return an error if:
/// - The account has insufficient balance for gas + value
/// - The function name doesn't exist in the ABI
/// - Function arguments don't match the ABI specification
/// - The contract reverts the transaction
/// - Network connection fails or times out
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
    let provider = ProviderBuilder::new()
        .wallet(account)
        .connect_http(rpc_http);

    let contract: ContractInstance<_, Ethereum> =
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

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::primitives::{address, TxHash};
    
    #[test]
    fn test_execution_new() {
        let caller = address!("d8dA6BF26964aF9D7eEd9e03E53415D37aA96045");
        let tx_hash = TxHash::default();
        
        let execution = Execution::new(caller, tx_hash);
        
        assert_eq!(execution.caller, caller);
        assert_eq!(execution.tx_hash, tx_hash);
    }
    
    #[test]
    fn test_execution_debug() {
        let caller = address!("d8dA6BF26964aF9D7eEd9e03E53415D37aA96045");
        let tx_hash = TxHash::default();
        
        let execution = Execution::new(caller, tx_hash);
        let debug_str = format!("{:?}", execution);
        
        assert!(debug_str.contains("Execution"));
        assert!(debug_str.contains("caller"));
        assert!(debug_str.contains("tx_hash"));
    }
    
    #[test]
    fn test_execution_fields_access() {
        let caller = address!("d8dA6BF26964aF9D7eEd9e03E53415D37aA96045");
        let tx_hash = TxHash::default();
        
        let execution = Execution { caller, tx_hash };
        
        // Test direct field access
        assert_eq!(execution.caller, caller);
        assert_eq!(execution.tx_hash, tx_hash);
    }
}
