use crate::common::TestProvider;
use alloy::network::EthereumWallet;
use alloy::providers::ProviderBuilder;
use alloy::signers::local::PrivateKeySigner;
use alloy::transports::http::reqwest::Url;
use alloy_node_bindings::{Anvil, AnvilInstance};
use eyre::Result;

pub struct TestEnvironment {
    pub provider: TestProvider,
    pub url: Url,
    pub signers: Vec<PrivateKeySigner>,
    // Hold the Anvil instance to keep it alive
    _anvil: AnvilInstance,
}

impl TestEnvironment {
    /// Generates a `TestProvider` along with its URL and a list of `PrivateKeySigner` instances.
    pub fn try_default() -> Result<Self> {
        Self::new(None)
    }
    /// Generates a `TestProvider` along with its URL and a list of `PrivateKeySigner` instances.
    ///
    /// # Arguments
    ///
    /// * `accounts_len` - An optional number of accounts to generate.
    ///
    /// # Returns
    ///
    /// A tuple containing:
    /// - `TestProvider`: The configured provider.
    /// - `Url`: The endpoint URL of the Anvil instance.
    /// - `Vec<PrivateKeySigner>`: A vector of private key signers.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The Anvil instance cannot be spawned.
    /// - The provider cannot be built.
    pub fn new(accounts_len: Option<usize>) -> Result<TestEnvironment> {
        let anvil = Anvil::default().try_spawn()?;
        let private_keys = anvil.keys();

        let signers: Vec<PrivateKeySigner> = private_keys
            .iter()
            .take(accounts_len.unwrap_or(1))
            .map(|key| key.clone().into())
            .collect();

        let deployer: PrivateKeySigner = private_keys[0].clone().into();
        let wallet = EthereumWallet::new(deployer.clone());
        let url = anvil.endpoint_url();
        let provider = ProviderBuilder::new()
            .with_recommended_fillers()
            .wallet(wallet)
            .on_http(url.clone());

        Ok(TestEnvironment {
            provider,
            url,
            signers,
            _anvil: anvil,
        })
    }
}
