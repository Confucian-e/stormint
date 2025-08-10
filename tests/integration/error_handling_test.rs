use crate::common::{deploy_contract, parse_artifact, TestEnvironment};
use alloy::primitives::utils::parse_ether;
use eyre::Result;
use stormint::account::generate_accounts_internal;
use stormint::distributor::{distribute, DistributeParam};

#[tokio::test]
async fn test_distribute_with_insufficient_balance() -> Result<()> {
    let test_env = TestEnvironment::try_default()?;
    let (provider, url) = (test_env.provider, test_env.url);
    let signer = test_env.signers.first().unwrap().clone();

    // Deploy distributor contract
    let (abi, bytecode) = parse_artifact("contracts/out/Distributor.sol/Distributor.json")?;
    let contract_address = deploy_contract(provider.clone(), bytecode).await?;

    // Generate receiver accounts
    let receivers = generate_accounts_internal("test test test test test test test test test test test junk", 0, 2, false)?;
    
    // Try to distribute more ETH than the signer has
    let excessive_amount = parse_ether("1000000")?; // 1 million ETH
    let params: Vec<DistributeParam> = receivers
        .iter()
        .map(|r| DistributeParam {
            receiver: r.address(),
            amount: excessive_amount,
        })
        .collect();

    // This should fail due to insufficient balance
    let result = distribute(signer, url.clone(), abi, contract_address, params).await;
    assert!(result.is_err());

    Ok(())
}

#[tokio::test]
async fn test_distribute_with_empty_params() -> Result<()> {
    let test_env = TestEnvironment::try_default()?;
    let (provider, url) = (test_env.provider, test_env.url);
    let signer = test_env.signers.first().unwrap().clone();

    // Deploy distributor contract
    let (abi, bytecode) = parse_artifact("contracts/out/Distributor.sol/Distributor.json")?;
    let contract_address = deploy_contract(provider.clone(), bytecode).await?;

    // Try to distribute with empty parameters
    let params: Vec<DistributeParam> = vec![];

    // This should succeed but do nothing
    let result = distribute(signer, url.clone(), abi, contract_address, params).await;
    assert!(result.is_ok());

    Ok(())
}

#[tokio::test] 
async fn test_mint_with_already_minted_account() -> Result<()> {
    let test_env = TestEnvironment::new(Some(2))?;
    let (provider, url, signers) = (test_env.provider, test_env.url, test_env.signers);

    let account = signers[1].clone();
    let (abi, bytecode) = parse_artifact("contracts/out/FreeMint.sol/FreeMint.json")?;
    let contract_address = deploy_contract(provider.clone(), bytecode).await?;

    // First mint should succeed
    let first_mint = stormint::mint::mint_loop(
        vec![account.clone()],
        url.clone(),
        abi.clone(),
        contract_address,
        None,
        None,
        None,
    )
    .await?;

    assert_eq!(first_mint.len(), 1);
    assert!(first_mint[0].result.is_ok());

    // Second mint with same account should fail
    let second_mint = stormint::mint::mint_loop(
        vec![account],
        url.clone(),
        abi.clone(),
        contract_address,
        None,
        None,
        None,
    )
    .await?;

    assert_eq!(second_mint.len(), 1);
    assert!(second_mint[0].result.is_err());

    Ok(())
}