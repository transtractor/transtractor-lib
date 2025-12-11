use crate::configs::validate::utils::validate_terms;

pub fn start_date_terms(terms: &Vec<String>) -> Result<(), String> {
    let result = validate_terms(terms, true);
    if result.is_err() {
        return Err(format!(
            "Invalid start_date_terms. {}",
            result.err().unwrap()
        ));
    }
    Ok(())
}
