pub fn account_type(account_type: &str) -> Result<(), String> {
    let valid_types = vec![
        "Checking",
        "Savings",
        "Credit Card",
        "Investment",
        "Loan",
        "Mortgage",
        "Mixed",
        "Other",
    ];
    if valid_types.contains(&account_type) {
        Ok(())
    } else {
        Err(format!(
            "Invalid account type: {}. Valid types are: {:?}",
            account_type, valid_types
        ))
    }
}
