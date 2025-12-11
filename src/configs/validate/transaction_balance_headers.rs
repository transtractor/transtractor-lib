use crate::configs::validate::utils::validate_terms;

pub fn transaction_balance_headers(terms: &Vec<String>) -> Result<(), String> {
    let result = validate_terms(terms, true);
    if result.is_err() {
        return Err(format!(
            "Invalid transaction_balance_headers. {}",
            result.err().unwrap()
        ));
    }
    Ok(())
}
