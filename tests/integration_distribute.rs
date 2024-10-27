use alloy::network::TransactionBuilder;
use alloy::primitives::utils::parse_ether;
use alloy::primitives::{Address, U256};
use alloy::providers::{Provider, ProviderBuilder};
use alloy::rpc::types::TransactionRequest;
use eyre::Result;

mod common;
use common::get_distributor_artifact;

#[tokio::test]
async fn test_integration_distribute() -> Result<()> {
    let (_abi, bytecode) = get_distributor_artifact()?;
    // let (mnemonic, start_index, end_index) = get_account_config()?;

    // setup anvil node
    let _provider = ProviderBuilder::new()
        .with_recommended_fillers()
        .on_anvil_with_wallet();

    // deploy distributor contract
    println!("Bytecode: {:?}", bytecode);

    // TODO: Deploy failed because of out of gas, need fix this
    // let deploy_tx = TransactionRequest::default().with_deploy_code(bytecode);
    // let deploy_tx_hash = provider.send_transaction(deploy_tx).await?.watch().await?;
    // let deploy_receipt = provider.get_transaction_receipt(deploy_tx_hash).await?.unwrap();
    // let contract_address = deploy_receipt.contract_address.unwrap();
    // println!("Distributor contract deployed at: {:?}", contract_address);

    // // generate receiver accounts
    // let receivers = generate_accounts(&mnemonic, start_index, end_index)?;
    // let each_amount = parse_ether("0.001")?;
    // let _params: Vec<DistributeParam> = receivers
    //     .iter()
    //     .map(|r| DistributeParam {
    //         receiver: r.address(),
    //         amount: each_amount,
    //     })
    //     .collect();

    Ok(())
}

#[tokio::test]
async fn test_transfer() -> Result<()> {
    let provider = ProviderBuilder::new()
        .with_recommended_fillers()
        .on_anvil_with_wallet();

    let receiver = Address::random();

    let balance = provider.get_balance(receiver).await?;

    let value = parse_ether("1")?;
    let tx = TransactionRequest::default()
        .with_to(receiver)
        .with_value(U256::from(value));

    let _tx_hash = provider.send_transaction(tx)
        .await?
        .watch()
        .await?;

    let balance_change = provider.get_balance(receiver).await? - balance;
    assert_eq!(balance_change, value);

    Ok(())
}
