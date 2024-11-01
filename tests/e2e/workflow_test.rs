use crate::common::{deploy_contract, get_token_balance, parse_artifact, TestEnvironment};
use alloy::primitives::utils::parse_ether;
use alloy::primitives::U256;
use alloy::providers::Provider;
use eyre::Result;
use stormint::account::generate_accounts;
use stormint::distributor::{distribute, DistributeParam};
use stormint::mint::mint_loop;

const MNEMONIC: &str = "test test test test test test test test test test test junk";
const START_INDEX: u32 = 100;
const END_INDEX: u32 = 200;

/// Test the workflow of the project
/// step 1: set up the node
/// step 2: deploy the contract
/// step 3: generate accounts
/// step 4: distribute ether to the accounts
/// step 5: mint tokens to the accounts
/// step 6: check the balances
#[tokio::test]
async fn test_workflow() -> Result<()> {
    let test_env = TestEnvironment::try_default()?;
    let (provider, url, signers) = (test_env.provider, test_env.url, test_env.signers);

    // generate receiver accounts
    let receivers = generate_accounts(MNEMONIC, START_INDEX, END_INDEX)?;

    // deploy distributor contract
    let (abi, bytecode) = parse_artifact("contracts/out/Distributor.sol/Distributor.json")?;
    let distributor_address = deploy_contract(provider.clone(), bytecode).await?;

    // distribute ether to receiver accounts
    let each_amount = parse_ether("0.001")?;
    let param = receivers
        .iter()
        .map(|r| DistributeParam {
            receiver: r.address(),
            amount: each_amount,
        })
        .collect();

    let sender = signers.first().unwrap().clone();
    let tx_hash = distribute(sender, url.clone(), abi.clone(), distributor_address, param).await?;
    let receipt = provider.get_transaction_receipt(tx_hash).await?.unwrap();
    assert!(receipt.status());

    // deploy mint contract
    let (abi, bytecode) = parse_artifact("contracts/out/FreeMint.sol/FreeMint.json")?;
    let mint_address = deploy_contract(provider.clone(), bytecode).await?;

    // mint tokens to receiver accounts
    let results = mint_loop(
        receivers,
        url.clone(),
        abi.clone(),
        mint_address,
        None,
        None,
        None,
    )
    .await?;

    // check balances
    for result in results {
        let token_balance =
            get_token_balance(url.clone(), abi.clone(), mint_address, result.signer).await?;
        assert!(token_balance > U256::from(0));
    }

    Ok(())
}
