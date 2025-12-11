/// Validate terms fields.
pub fn validate_terms(terms: &Vec<String>, allow_empty: bool) -> Result<(), String> {
    // Example validation: No term should be empty
    for term in terms {
        if term.trim().is_empty() && !allow_empty {
            return Err("Terms cannot be empty.".to_string());
        }
        let word_count = term.split_whitespace().count();
        if word_count > 10 {
            return Err(format!(
                "Term '{}' has too many words ({}). Maximum allowed is 10.",
                term, word_count
            ));
        }
    }
    Ok(())
}
