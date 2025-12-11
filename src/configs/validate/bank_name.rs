pub fn bank_name(bank_name: &str) -> Result<(), String> {
    if bank_name.trim().is_empty() {
        return Err("Invalid bank_name. Cannot be empty.".to_string());
    }
    if bank_name.len() > 100 {
        return Err(format!(
            "Invalid bank_name: '{}'. Too long. Maximum length is 100 characters.",
            bank_name
        ));
    }
    Ok(())
}
