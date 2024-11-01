use crate::common::TestProvider;
use alloy::network::TransactionBuilder;
use alloy::primitives::Address;
use alloy::providers::Provider;
use alloy::rpc::types::TransactionRequest;
use eyre::Result;

/// Deploys a contract using the provided provider and bytecode.
///
/// # Arguments
///
/// * `provider` - The provider to use for sending the transaction.
/// * `bytecode` - The bytecode of the contract to deploy.
///
/// # Returns
///
/// The address of the deployed contract.
///
/// # Errors
///
/// This function will return an error if:
/// - The transaction cannot be sent.
/// - The transaction receipt cannot be retrieved.
/// - The contract address is not found in the receipt.
pub async fn deploy_contract(provider: TestProvider, bytecode: Vec<u8>) -> Result<Address> {
    let deploy_tx = TransactionRequest::default().with_deploy_code(bytecode);
    let builder = provider.send_transaction(deploy_tx).await?;

    let tx_receipt = builder.get_receipt().await?;
    let contract_address = tx_receipt.contract_address.unwrap();

    Ok(contract_address)
}
