use crate::configs::validate::utils::validate_terms;

pub fn account_number_terms(terms: &Vec<String>) -> Result<(), String> {
    let result = validate_terms(terms, true);
    if result.is_err() {
        return Err(format!(
            "Invalid account_number_terms. {}",
            result.err().unwrap()
        ));
    }
    Ok(())
}
