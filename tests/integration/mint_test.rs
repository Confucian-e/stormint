use crate::common::{deploy_contract, get_token_balance, parse_artifact, TestEnvironment};
use alloy::dyn_abi::DynSolValue;
use alloy::json_abi::JsonAbi;
use alloy::primitives::{Address, U256};
use alloy::transports::http::reqwest::Url;
use eyre::Result;
use stormint::executor::call;
use stormint::mint::mint_loop;

const ARTIFACT_PATH: &str = "contracts/out/FreeMint.sol/FreeMint.json";

#[tokio::test]
async fn test_mint() -> Result<()> {
    let test_env = TestEnvironment::new(Some(3))?;
    let (provider, url, signers) = (test_env.provider, test_env.url, test_env.signers);

    let (alice, bob) = (signers[1].clone(), signers[2].clone());

    let (abi, bytecode) = parse_artifact(ARTIFACT_PATH)?;

    let contract_address = deploy_contract(provider.clone(), bytecode).await?;

    let accounts = vec![alice, bob];
    let results = mint_loop(
        accounts,
        url.clone(),
        abi.clone(),
        contract_address,
        None,
        None,
        None,
    )
    .await?;

    let mint_amount = get_mint_amount(url.clone(), abi.clone(), contract_address).await?;
    // check balance
    for result in results {
        let balance =
            get_token_balance(url.clone(), abi.clone(), contract_address, result.signer).await?;
        assert_eq!(balance, mint_amount);
    }

    Ok(())
}

async fn get_mint_amount(url: Url, abi: JsonAbi, contract_address: Address) -> Result<U256> {
    let mint_amount = call(url, abi, contract_address, "MINT_AMOUNT", &[]).await?;

    let mint_amount = match mint_amount.first() {
        Some(DynSolValue::Uint(mint_amount, 256)) => *mint_amount,
        _ => U256::default(),
    };

    Ok(mint_amount)
}
