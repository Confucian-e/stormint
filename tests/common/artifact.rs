use alloy::{hex, json_abi::JsonAbi};
use eyre::Result;
use serde::Deserialize;
use std::{env, fs};

pub fn get_artifact(path: &str) -> Result<(JsonAbi, Vec<u8>)> {
    let current_dir = env::current_dir()?;

    let file = current_dir.join(path);
    let content = fs::read_to_string(file)?;
    let artifact: Artifact = serde_json::from_str(&content)?;

    let (abi, bytecode) = (artifact.abi, artifact.bytecode.object);
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
