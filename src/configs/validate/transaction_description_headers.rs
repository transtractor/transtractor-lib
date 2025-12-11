use crate::configs::validate::utils::validate_terms;

pub fn transaction_description_headers(terms: &Vec<String>) -> Result<(), String> {
    let result = validate_terms(terms, false);
    if result.is_err() {
        return Err(format!(
            "Invalid transaction_description_headers. {}",
            result.err().unwrap()
        ));
    }
    Ok(())
}
