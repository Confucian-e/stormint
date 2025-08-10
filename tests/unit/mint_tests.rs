use alloy::primitives::{address, TxHash, B256};
use eyre::{eyre, Report};
use stormint::mint::MintResult;

#[test]
fn test_mint_result_success_case() {
    let signer = address!("d8dA6BF26964aF9D7eEd9e03E53415D37aA96045");
    let tx_hash = TxHash::from(B256::ZERO);
    let successful_result: Result<TxHash, Report> = Ok(tx_hash);

    let mint_result = MintResult {
        signer,
        result: successful_result,
    };

    assert_eq!(mint_result.signer, signer);
    assert!(mint_result.result.is_ok());

    if let Ok(hash) = &mint_result.result {
        assert_eq!(*hash, tx_hash);
    }
}

#[test]
fn test_mint_result_error_case() {
    let signer = address!("f39Fd6e51aad88F6F4ce6aB8827279cffFb92266");
    let error_result: Result<TxHash, Report> = Err(eyre!("Transaction failed"));

    let mint_result = MintResult {
        signer,
        result: error_result,
    };

    assert_eq!(mint_result.signer, signer);
    assert!(mint_result.result.is_err());

    if let Err(err) = &mint_result.result {
        assert!(err.to_string().contains("Transaction failed"));
    }
}

#[test]
fn test_mint_result_debug_trait() {
    let signer = address!("d8dA6BF26964aF9D7eEd9e03E53415D37aA96045");
    let tx_hash = TxHash::from([42u8; 32]);
    let result = Ok(tx_hash);

    let mint_result = MintResult { signer, result };
    let debug_string = format!("{:?}", mint_result);

    assert!(debug_string.contains("MintResult"));
    assert!(debug_string.contains("signer"));
    assert!(debug_string.contains("result"));
}

#[test]
fn test_mint_result_vector_operations() {
    let results = vec![
        MintResult {
            signer: address!("d8dA6BF26964aF9D7eEd9e03E53415D37aA96045"),
            result: Ok(TxHash::from([1u8; 32])),
        },
        MintResult {
            signer: address!("f39Fd6e51aad88F6F4ce6aB8827279cffFb92266"),
            result: Err(eyre!("Failed")),
        },
        MintResult {
            signer: address!("70997970C51812dc3A010C7d01b50e0d17dc79C8"),
            result: Ok(TxHash::from([2u8; 32])),
        },
    ];

    assert_eq!(results.len(), 3);

    let successful_count = results.iter().filter(|r| r.result.is_ok()).count();
    let failed_count = results.iter().filter(|r| r.result.is_err()).count();

    assert_eq!(successful_count, 2);
    assert_eq!(failed_count, 1);
}

#[test]
fn test_mint_result_with_different_signers() {
    let signers = vec![
        address!("d8dA6BF26964aF9D7eEd9e03E53415D37aA96045"),
        address!("f39Fd6e51aad88F6F4ce6aB8827279cffFb92266"),
        address!("70997970C51812dc3A010C7d01b50e0d17dc79C8"),
    ];

    let results: Vec<MintResult> = signers
        .into_iter()
        .enumerate()
        .map(|(i, signer)| MintResult {
            signer,
            result: Ok(TxHash::from([i as u8; 32])),
        })
        .collect();

    assert_eq!(results.len(), 3);

    // Verify all signers are different
    let unique_signers: std::collections::HashSet<_> = results.iter().map(|r| r.signer).collect();
    assert_eq!(unique_signers.len(), 3);
}
