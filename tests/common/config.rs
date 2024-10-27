// use alloy::primitives::Address;
// use eyre::Result;
// use serde::Deserialize;
// use std::{env, fs};
//
// pub fn get_account_config() -> Result<(String, u32, u32)> {
//     let config = get_config()?;
//
//     let (mnemonic, start_index, end_index) = (
//         config.account.mnemonic,
//         config.account.start_index,
//         config.account.end_index,
//     );
//
//     Ok((mnemonic, start_index, end_index))
// }
//
// pub fn get_distributor_config() -> Result<(String, Address, String, String)> {
//     let config = get_config()?;
//
//     let (sender_private_key, distributor_contract_address, artifact_path, each_amount) = (
//         config.distributor.sender.private_key,
//         config.distributor.contract.address,
//         config.distributor.contract.artifact_path,
//         config.distributor.each_amount,
//     );
//
//     Ok((sender_private_key, distributor_contract_address, artifact_path, each_amount))
// }
//
// fn get_config() -> Result<Config> {
//     let current_dir = env::current_dir()?;
//     let config_path = current_dir.join("tests/config.toml");
//
//     let config_file = fs::read_to_string(config_path)?;
//     let config: Config = toml::from_str(&config_file)?;
//
//     Ok(config)
// }
//
//
// #[derive(Debug, Deserialize)]
// pub struct Config {
//     pub account: Account,
//     pub distributor: Distributor,
//     pub network: Network,
// }
//
// #[derive(Debug, Deserialize)]
// pub struct Account {
//     pub mnemonic: String,
//     pub start_index: u32,
//     pub end_index: u32,
// }
//
// #[derive(Debug, Deserialize)]
// pub struct Distributor {
//     pub sender: Sender,
//     pub contract: Contract,
//     pub each_amount: String,
// }
//
// #[derive(Debug, Deserialize)]
// pub struct Sender {
//     pub private_key: String,
// }
//
// #[derive(Debug, Deserialize)]
// pub struct Contract {
//     pub address: Address,
//     pub artifact_path: String,
// }
//
// #[derive(Debug, Deserialize)]
// pub struct Network {
//     pub rpc_url: String,
// }
