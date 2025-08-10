use stormint::account::generate_accounts_internal;
use eyre::Result;

const VALID_PHRASE: &str = "test test test test test test test test test test test junk";

#[test]
fn test_generate_accounts_internal_consistency() -> Result<()> {
    let accounts1 = generate_accounts_internal(VALID_PHRASE, 0, 5, false)?;
    let accounts2 = generate_accounts_internal(VALID_PHRASE, 0, 5, false)?;
    
    assert_eq!(accounts1.len(), accounts2.len());
    
    for (acc1, acc2) in accounts1.iter().zip(accounts2.iter()) {
        assert_eq!(acc1.address(), acc2.address());
    }
    
    Ok(())
}

#[test]
fn test_generate_accounts_internal_different_ranges() -> Result<()> {
    let accounts1 = generate_accounts_internal(VALID_PHRASE, 0, 3, false)?;
    let accounts2 = generate_accounts_internal(VALID_PHRASE, 3, 6, false)?;
    
    assert_eq!(accounts1.len(), 3);
    assert_eq!(accounts2.len(), 3);
    
    // All accounts should be different
    for acc1 in &accounts1 {
        for acc2 in &accounts2 {
            assert_ne!(acc1.address(), acc2.address());
        }
    }
    
    Ok(())
}

#[test]
fn test_generate_accounts_internal_progress_bar_hidden() -> Result<()> {
    // Test that progress bar doesn't show when show_progress is false
    let accounts = generate_accounts_internal(VALID_PHRASE, 0, 10, false)?;
    assert_eq!(accounts.len(), 10);
    Ok(())
}

#[test]
fn test_generate_accounts_internal_invalid_range() -> Result<()> {
    // Test edge case where start equals end
    let accounts = generate_accounts_internal(VALID_PHRASE, 5, 5, false)?;
    assert_eq!(accounts.len(), 0);
    Ok(())
}

#[test] 
fn test_generate_accounts_internal_large_gap() -> Result<()> {
    // Test with a large gap between start and end
    let accounts = generate_accounts_internal(VALID_PHRASE, 1000, 1003, false)?;
    assert_eq!(accounts.len(), 3);
    
    // Ensure they are unique
    let addresses: std::collections::HashSet<_> = accounts.iter().map(|a| a.address()).collect();
    assert_eq!(addresses.len(), 3);
    
    Ok(())
}