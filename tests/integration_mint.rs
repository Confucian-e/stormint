use alloy::dyn_abi::DynSolValue;
use alloy::json_abi::JsonAbi;
use alloy::network::{EthereumWallet, TransactionBuilder};
use alloy::primitives::{Address, U256};
use alloy::providers::{Provider, ProviderBuilder};
use alloy::rpc::types::TransactionRequest;
use alloy::signers::local::PrivateKeySigner;
use alloy::transports::http::reqwest::Url;
use alloy_node_bindings::Anvil;
use eyre::Result;

use stormint::executor::call;
use stormint::mint::mint_loop;

mod common;
use common::get_artifact;

const ARTIFACT_PATH: &str = "contracts/out/FreeMint.sol/FreeMint.json";

#[tokio::test]
async fn test_mint() -> Result<()> {
    let anvil = Anvil::default().try_spawn()?;
    let private_keys = anvil.keys();

    let deployer: PrivateKeySigner = private_keys[0].clone().into();
    let alice: PrivateKeySigner = private_keys[1].clone().into();
    let bob: PrivateKeySigner = private_keys[2].clone().into();

    let wallet = EthereumWallet::new(deployer.clone());
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
            get_balance(url.clone(), abi.clone(), contract_address, result.signer).await?;
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

async fn get_balance(
    url: Url,
    abi: JsonAbi,
    contract_address: Address,
    account: Address,
) -> Result<U256> {
    let balance = call(
        url,
        abi,
        contract_address,
        "balanceOf",
        &[DynSolValue::from(account)],
    )
    .await?;

    let balance = match balance.first() {
        Some(DynSolValue::Uint(balance, 256)) => *balance,
        _ => U256::default(),
    };

    Ok(balance)
}
