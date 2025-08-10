use stormint::distributor::DistributeParam;
use alloy::primitives::{address, U256};

#[test]
fn test_distribute_param_creation_and_access() {
    let receiver1 = address!("d8dA6BF26964aF9D7eEd9e03E53415D37aA96045");
    let receiver2 = address!("f39Fd6e51aad88F6F4ce6aB8827279cffFb92266");
    let amount1 = U256::from(1000);
    let amount2 = U256::from(2000);
    
    let param1 = DistributeParam { receiver: receiver1, amount: amount1 };
    let param2 = DistributeParam { receiver: receiver2, amount: amount2 };
    
    assert_eq!(param1.receiver, receiver1);
    assert_eq!(param1.amount, amount1);
    assert_eq!(param2.receiver, receiver2);
    assert_eq!(param2.amount, amount2);
}

#[test]
fn test_distribute_param_vector_operations() {
    let params = vec![
        DistributeParam {
            receiver: address!("d8dA6BF26964aF9D7eEd9e03E53415D37aA96045"),
            amount: U256::from(1000),
        },
        DistributeParam {
            receiver: address!("f39Fd6e51aad88F6F4ce6aB8827279cffFb92266"),
            amount: U256::from(2000),
        },
        DistributeParam {
            receiver: address!("70997970C51812dc3A010C7d01b50e0d17dc79C8"),
            amount: U256::from(3000),
        },
    ];
    
    assert_eq!(params.len(), 3);
    
    let total: U256 = params.iter().map(|p| p.amount).sum();
    assert_eq!(total, U256::from(6000));
    
    let fold_total = params.iter().fold(U256::ZERO, |acc, param| acc + param.amount);
    assert_eq!(fold_total, U256::from(6000));
}

#[test]
fn test_distribute_param_zero_amounts() {
    let param = DistributeParam {
        receiver: address!("d8dA6BF26964aF9D7eEd9e03E53415D37aA96045"),
        amount: U256::ZERO,
    };
    
    assert_eq!(param.amount, U256::ZERO);
    
    let params = vec![param];
    let total = params.iter().fold(U256::ZERO, |acc, p| acc + p.amount);
    assert_eq!(total, U256::ZERO);
}

#[test]
fn test_distribute_param_large_amounts() {
    let large_amount = U256::MAX;
    let param = DistributeParam {
        receiver: address!("d8dA6BF26964aF9D7eEd9e03E53415D37aA96045"),
        amount: large_amount,
    };
    
    assert_eq!(param.amount, large_amount);
}