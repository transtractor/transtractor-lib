use crate::configs::validate::utils::validate_terms;

pub fn account_terms(terms: &Vec<String>) -> Result<(), String> {
    let result = validate_terms(terms, false);
    if result.is_err() {
        return Err(format!(
            "Invalid account_terms. {}",
            result.err().unwrap()
        ));
    }
    Ok(())
}
