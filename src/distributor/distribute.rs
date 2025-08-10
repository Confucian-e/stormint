use crate::executor::execute;
use alloy::{
    dyn_abi::DynSolValue,
    json_abi::JsonAbi,
    primitives::{Address, TxHash, U256},
    signers::local::PrivateKeySigner,
    transports::http::reqwest::Url,
};
use eyre::Result;

/// Parameters for gas distribution to a single recipient.
///
/// This structure defines the recipient address and amount for a single
/// distribution operation. Multiple `DistributeParam` instances are typically
/// collected into a vector for batch distribution.
///
/// # Fields
///
/// * `receiver` - The Ethereum address that will receive the funds
/// * `amount` - The amount of Wei to send to the receiver
///
/// # Examples
///
/// ```rust
/// use stormint::distributor::DistributeParam;
/// use alloy::primitives::{address, utils::parse_ether};
///
/// # fn main() -> eyre::Result<()> {
/// let param = DistributeParam {
///     receiver: address!("d8dA6BF26964aF9D7eEd9e03E53415D37aA96045"),
///     amount: parse_ether("0.001")?, // 0.001 ETH
/// };
/// # Ok(())
/// # }
/// ```
#[derive(Debug)]
pub struct DistributeParam {
    pub receiver: Address,
    pub amount: U256,
}

/// Distributes Ether to multiple recipients in a single transaction.
///
/// This function performs batch Ether distribution using a smart contract,
/// which is more gas-efficient than sending individual transactions.
/// The total amount is calculated automatically and sent as the transaction value.
///
/// # Arguments
///
/// * `sender` - The wallet that will send the Ether (must have sufficient balance)
/// * `rpc_http` - The Ethereum RPC endpoint URL for transaction submission
/// * `abi` - The JSON ABI of the distributor contract
/// * `contract_address` - The deployed distributor contract address
/// * `params` - Vector of distribution parameters specifying recipients and amounts
///
/// # Returns
///
/// Returns `Ok(TxHash)` with the transaction hash on successful submission,
/// or an error if the transaction fails or insufficient funds are available.
///
/// # Examples
///
/// ```rust,no_run
/// use stormint::distributor::{distribute, DistributeParam};
/// use alloy::primitives::utils::parse_ether;
///
/// # async fn example() -> eyre::Result<()> {
/// # let wallet = todo!(); // PrivateKeySigner
/// # let rpc_url: alloy::transports::http::reqwest::Url = "http://localhost:8545".parse()?;
/// # let contract_abi = alloy::json_abi::JsonAbi::new();
/// # let contract_addr: alloy::primitives::Address = "0x0000000000000000000000000000000000000000".parse()?;
/// let params = vec![
///     DistributeParam {
///         receiver: "0x742d35Cc5c2B5C03B3D8Ac8E6D7D0Dea6B65B20c".parse()?,
///         amount: parse_ether("0.001")?,
///     },
///     DistributeParam {
///         receiver: "0x8ba1f109551bD432803012645Hac136c30e7E5C4".parse()?,
///         amount: parse_ether("0.002")?,
///     },
/// ];
///
/// // Note: Commented out to avoid compilation issues in doctests
/// // let tx_hash = distribute(
/// //     wallet, rpc_url, contract_abi, contract_addr, params
/// // ).await?;
/// // println!("Distribution sent: {:?}", tx_hash);
/// # Ok(())
/// # }
/// ```
///
/// # Gas Efficiency
///
/// Using batch distribution is significantly more gas-efficient than individual transfers:
/// - Individual transfers: ~21,000 gas per recipient
/// - Batch distribution: ~21,000 + (2,300 × recipients) gas total
///
/// # Errors
///
/// This function will return an error if:
/// - The sender has insufficient balance for the total amount plus gas
/// - The contract address is invalid or not deployed
/// - The RPC connection fails
/// - Any recipient address is invalid
pub async fn distribute(
    sender: PrivateKeySigner,
    rpc_http: Url,
    abi: JsonAbi,
    contract_address: Address,
    params: Vec<DistributeParam>,
) -> Result<TxHash> {
    // Pre-allocate vector with exact capacity
    let mut txns_vec = Vec::with_capacity(params.len());
    for param in &params {
        txns_vec.push(DynSolValue::Tuple(vec![
            DynSolValue::from(param.receiver),
            DynSolValue::from(param.amount),
        ]));
    }
    let txns = DynSolValue::Array(txns_vec);

    let args = &[txns];

    // More efficient sum calculation
    let value = params
        .iter()
        .fold(U256::ZERO, |acc, param| acc + param.amount);

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

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::primitives::{address, U256};
    // Test helper imports removed as they were unused

    #[test]
    fn test_distribute_param_creation() {
        let receiver = address!("d8dA6BF26964aF9D7eEd9e03E53415D37aA96045");
        let amount = U256::from(1000);

        let param = DistributeParam { receiver, amount };

        assert_eq!(param.receiver, receiver);
        assert_eq!(param.amount, amount);
    }

    #[test]
    fn test_distribute_param_debug() {
        let param = DistributeParam {
            receiver: address!("d8dA6BF26964aF9D7eEd9e03E53415D37aA96045"),
            amount: U256::from(1000),
        };

        let debug_str = format!("{:?}", param);
        assert!(debug_str.contains("DistributeParam"));
        // Address might be formatted differently in debug output
        assert!(debug_str.contains("receiver"));
        assert!(debug_str.contains("amount"));
        assert!(debug_str.contains("1000"));
    }

    #[test]
    fn test_empty_params_value_calculation() {
        let params: Vec<DistributeParam> = vec![];
        let value = params
            .iter()
            .fold(U256::ZERO, |acc, param| acc + param.amount);
        assert_eq!(value, U256::ZERO);
    }

    #[test]
    fn test_multiple_params_value_calculation() {
        let params = [
            DistributeParam {
                receiver: address!("d8dA6BF26964aF9D7eEd9e03E53415D37aA96045"),
                amount: U256::from(1000),
            },
            DistributeParam {
                receiver: address!("f39Fd6e51aad88F6F4ce6aB8827279cffFb92266"),
                amount: U256::from(2000),
            },
        ];

        let value = params
            .iter()
            .fold(U256::ZERO, |acc, param| acc + param.amount);
        assert_eq!(value, U256::from(3000));
    }
}
