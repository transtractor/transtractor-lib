pub fn transaction_formats(formats: &Vec<Vec<String>>) -> Result<(), String> {
    let allowed_tokens = ["date", "description", "amount", "balance"]; // extend as needed
    for fmt in formats {
        if fmt.is_empty() {
            return Err("Invalid transaction_formats. Cannot be empty".into());
        }
        for token in fmt {
            if !allowed_tokens.iter().any(|a| a == token) {
                return Err(format!("Invalid transaction_formats. Unknown token '{}'", token));
            }
        }
    }
    Ok(())
}
