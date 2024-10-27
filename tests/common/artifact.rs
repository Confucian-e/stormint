use alloy::json_abi::JsonAbi;
use eyre::Result;
use serde::{Deserialize, Serialize};
use std::{env, fs};

pub fn get_distributor_artifact() -> Result<(JsonAbi, String)> {
    let current_dir = env::current_dir()?;
    let path = current_dir.join("contracts/out/Distributor.sol/Distributor.json");

    let file = fs::read_to_string(path)?;
    let content: Artifact = serde_json::from_str(&file)?;

    let abi = serde_json::to_value(content.abi)?;
    let abi: JsonAbi = serde_json::from_value(abi)?;

    let bytecode = content.bytecode.object;

    Ok((abi, bytecode))
}

#[derive(Debug, Deserialize)]
struct Artifact {
    abi: Vec<AbiComponent>,
    bytecode: Bytecode,
}

#[derive(Serialize, Deserialize, Debug)]
struct AbiComponent {
    #[serde(rename = "type")]
    abi_type: String,
    name: Option<String>,
    inputs: Option<Vec<AbiInput>>,
    outputs: Option<Vec<AbiInput>>,
    state_mutability: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct AbiInput {
    #[serde(rename = "type")]
    abi_type: String,
    name: String,
    internal_type: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Bytecode {
    object: String,
}