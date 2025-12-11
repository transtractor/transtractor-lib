pub fn account_examples(account_examples: &Vec<String>) -> Result<(), String> {
    // An example cannot be more then 100 chars
    for example in account_examples {
        if example.len() > 100 {
            return Err(format!(
                "Invalid account_examples. '{}' is too long. Maximum length is 100 characters.",
                example
            ));
        }
    }
    Ok(())
}
