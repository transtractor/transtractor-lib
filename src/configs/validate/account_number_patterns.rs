use crate::configs::validate::utils::validate_patterns;
use regex::Regex;

pub fn account_number_patterns(patterns: &Vec<Regex>) -> Result<(), String> {
    let result = validate_patterns(patterns, false);
    if result.is_err() {
        return Err(format!(
            "Invalid account_number_patterns. {}",
            result.err().unwrap()
        ));
    }
    Ok(())
}
