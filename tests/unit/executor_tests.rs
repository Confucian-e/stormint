use stormint::executor::Execution;
use alloy::primitives::{address, TxHash, Address, B256};

#[test]
fn test_execution_struct_creation() {
    let caller = address!("d8dA6BF26964aF9D7eEd9e03E53415D37aA96045");
    let tx_hash = TxHash::from(B256::ZERO);
    
    let execution = Execution { caller, tx_hash };
    
    assert_eq!(execution.caller, caller);
    assert_eq!(execution.tx_hash, tx_hash);
}

#[test]
fn test_execution_struct_field_access() {
    let caller = address!("f39Fd6e51aad88F6F4ce6aB8827279cffFb92266");
    let tx_hash = TxHash::from([1u8; 32]);
    
    let execution = Execution { caller, tx_hash };
    
    // Test direct field access
    let extracted_caller = execution.caller;
    let extracted_tx_hash = execution.tx_hash;
    
    assert_eq!(extracted_caller, caller);
    assert_eq!(extracted_tx_hash, tx_hash);
}

#[test]
fn test_execution_debug_trait() {
    let execution = Execution {
        caller: address!("d8dA6BF26964aF9D7eEd9e03E53415D37aA96045"),
        tx_hash: TxHash::from(B256::ZERO),
    };
    
    let debug_string = format!("{:?}", execution);
    assert!(debug_string.contains("Execution"));
    assert!(debug_string.contains("caller"));
    assert!(debug_string.contains("tx_hash"));
}

#[test]
fn test_execution_multiple_instances() {
    let caller1 = address!("d8dA6BF26964aF9D7eEd9e03E53415D37aA96045");
    let caller2 = address!("f39Fd6e51aad88F6F4ce6aB8827279cffFb92266");
    let tx_hash1 = TxHash::from([1u8; 32]);
    let tx_hash2 = TxHash::from([2u8; 32]);
    
    let execution1 = Execution { caller: caller1, tx_hash: tx_hash1 };
    let execution2 = Execution { caller: caller2, tx_hash: tx_hash2 };
    
    assert_ne!(execution1.caller, execution2.caller);
    assert_ne!(execution1.tx_hash, execution2.tx_hash);
}

#[test]
fn test_execution_with_zero_address() {
    let zero_address = Address::ZERO;
    let tx_hash = TxHash::from(B256::ZERO);
    
    let execution = Execution { caller: zero_address, tx_hash };
    
    assert_eq!(execution.caller, zero_address);
    assert_eq!(execution.tx_hash, tx_hash);
}