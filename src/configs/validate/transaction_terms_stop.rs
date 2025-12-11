use crate::configs::validate::utils::validate_terms;

pub fn transaction_terms_stop(terms: &Vec<String>) -> Result<(), String> {
    let result = validate_terms(terms, true);
    if result.is_err() {
        return Err(format!(
            "Invalid transaction_terms_stop. {}",
            result.err().unwrap()
        ));
    }
    Ok(())
}
