use crate::common::{deploy_contract, parse_artifact, TestEnvironment};
use alloy::primitives::utils::parse_ether;
use alloy::providers::Provider;
use eyre::Result;
use stormint::account::generate_accounts;
use stormint::distributor::{distribute, DistributeParam};

const ARTIFACT_PATH: &str = "contracts/out/Distributor.sol/Distributor.json";
const MNEMONIC: &str = "test test test test test test test test test test test junk";
const START_INDEX: u32 = 100;
const END_INDEX: u32 = 200;

#[tokio::test]
async fn test_distribute() -> Result<()> {
    let test_env = TestEnvironment::try_default()?;
    let (provider, url) = (test_env.provider, test_env.url);
    let signer = test_env.signers.first().unwrap().clone();

    let (abi, bytecode) = parse_artifact(ARTIFACT_PATH)?;

    let contract_address = deploy_contract(provider.clone(), bytecode).await?;

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
