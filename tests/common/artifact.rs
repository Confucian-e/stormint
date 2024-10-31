use alloy::{hex, json_abi::JsonAbi};
use eyre::Result;
use serde::Deserialize;
use std::{env, fs};

pub fn get_distributor_artifact() -> Result<(JsonAbi, Vec<u8>)> {
    let current_dir = env::current_dir()?;
    let path = current_dir.join("contracts/out/Distributor.sol/Distributor.json");

    let file = fs::read_to_string(path)?;
    let content: Artifact = serde_json::from_str(&file)?;

    let (abi, bytecode) = (content.abi, content.bytecode.object);
    let bytecode = hex::decode(&bytecode)?;

    Ok((abi, bytecode))
}

#[derive(Debug, Deserialize)]
struct Artifact {
    abi: JsonAbi,
    bytecode: Bytecode,
}

#[derive(Debug, Deserialize)]
struct Bytecode {
    object: String,
}
