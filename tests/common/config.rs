use eyre::Result;
use serde::Deserialize;
use std::{env, fs};

pub fn get_account_config() -> Result<(String, u32, u32)> {
    let config = get_config()?;

    let (mnemonic, start_index, end_index) = (
        config.account.mnemonic,
        config.account.start_index,
        config.account.end_index,
    );

    Ok((mnemonic, start_index, end_index))
}

fn get_config() -> Result<Config> {
    let current_dir = env::current_dir()?;
    let config_path = current_dir.join("tests/config.toml");

    let config_file = fs::read_to_string(config_path)?;
    let config: Config = toml::from_str(&config_file)?;

    Ok(config)
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub account: Account,
}

#[derive(Debug, Deserialize)]
pub struct Account {
    pub mnemonic: String,
    pub start_index: u32,
    pub end_index: u32,
}
