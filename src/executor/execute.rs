use alloy::{
    contract::{ContractInstance, Interface},
    dyn_abi::DynSolValue,
    json_abi::JsonAbi,
    network::{Ethereum, EthereumWallet},
    primitives::{Address, TxHash, U256},
    providers::ProviderBuilder,
    signers::local::PrivateKeySigner,
    transports::http::{reqwest::Url, Client, Http},
};
use eyre::Result;

#[derive(Debug)]
pub struct Execution {
    pub caller: Address,
    pub tx_hash: TxHash,
}

impl Execution {
    fn new(caller: Address, tx_hash: TxHash) -> Self {
        Self { caller, tx_hash }
    }
}

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
    let wallet = EthereumWallet::new(account);
    let provider = ProviderBuilder::new()
        .with_recommended_fillers()
        .wallet(wallet)
        .on_http(rpc_http);

    let contract: ContractInstance<Http<Client>, _, Ethereum> =
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
