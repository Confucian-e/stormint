use alloy::{hex, json_abi::JsonAbi};
use eyre::Result;
use serde::Deserialize;
use std::{env, fs};

/// Reads and parses a JSON artifact file to extract the ABI and bytecode.
///
/// # Arguments
///
/// * `path` - A string slice that holds the path to the artifact file.
///
/// # Returns
///
/// A tuple containing the `JsonAbi` and the bytecode as a `Vec<u8>`.
///
/// # Errors
///
/// This function will return an error if:
/// - The current directory cannot be retrieved.
/// - The file cannot be read.
/// - The file content cannot be parsed as JSON.
/// - The bytecode cannot be decoded from hex.
pub fn parse_artifact(path: &str) -> Result<(JsonAbi, Vec<u8>)> {
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
