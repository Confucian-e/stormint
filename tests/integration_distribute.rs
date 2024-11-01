use alloy::{
    network::{EthereumWallet, TransactionBuilder},
    primitives::utils::parse_ether,
    providers::{Provider, ProviderBuilder},
    rpc::types::TransactionRequest,
    signers::local::PrivateKeySigner,
};
use alloy_node_bindings::Anvil;
use eyre::Result;

use stormint::account::generate_accounts;
use stormint::distributor::{distribute, DistributeParam};

mod common;
use common::get_artifact;

const ARTIFACT_PATH: &str = "contracts/out/Distributor.sol/Distributor.json";
const MNEMONIC: &str = "test test test test test test test test test test test junk";
const START_INDEX: u32 = 100;
const END_INDEX: u32 = 200;

#[tokio::test]
async fn test_distribute() -> Result<()> {
    let anvil = Anvil::default().try_spawn()?;
    let signer: PrivateKeySigner = anvil.keys()[0].clone().into();
    let wallet = EthereumWallet::new(signer.clone());
    let url = anvil.endpoint_url();

    let provider = ProviderBuilder::new()
        .with_recommended_fillers()
        .wallet(wallet)
        .on_http(url.clone());

    let (abi, bytecode) = get_artifact(ARTIFACT_PATH)?;

    let deploy_tx = TransactionRequest::default().with_deploy_code(bytecode);

    let deploy_tx_hash = provider.send_transaction(deploy_tx).await?.watch().await?;
    let deploy_tx_receipt = provider
        .get_transaction_receipt(deploy_tx_hash)
        .await?
        .unwrap();
    let contract_address = deploy_tx_receipt.contract_address.unwrap();

    // generate receiver accounts
    let receivers = generate_accounts(MNEMONIC, START_INDEX, END_INDEX)?;
    let each_amount = parse_ether("0.001")?;
    let params: Vec<DistributeParam> = receivers
        .iter()
        .map(|r| DistributeParam {
            receiver: r.address(),
            amount: each_amount,
        })
        .collect();

    // distribute ether to receiver accounts
    let distribute_tx = distribute(signer, url.clone(), abi, contract_address, params).await?;

    // check distribute transaction
    let distribute_receipt = provider
        .get_transaction_receipt(distribute_tx)
        .await?
        .unwrap();
    assert!(distribute_receipt.status());

    // check balances
    for receiver in receivers {
        let balance = provider.get_balance(receiver.address()).await?;
        assert_eq!(balance, each_amount);
    }

    Ok(())
}
