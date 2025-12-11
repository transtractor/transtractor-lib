use regex::Regex;

pub fn validate_patterns(patterns: &Vec<Regex>, allow_empty: bool) -> Result<(), String> {
    if !allow_empty && patterns.is_empty() {
        return Err("Patterns cannot be an empty array.".to_string());
    }
    // Pattern must not be more than 10 words when converted to string
    for pattern in patterns {
        let pattern_str = pattern.as_str();
        // Count only whitespace separators: \s, \s+, \s*, literal space
        let separator_count =
            pattern_str.matches(r"\s").count() + pattern_str.matches(" ").count();
        // Add 1 because N separators means N+1 tokens
        let token_count = separator_count + 1;
        if token_count > 10 {
            return Err(format!(
                "Pattern '{}' has too many tokens ({}). Maximum allowed is 10.",
                pattern_str, token_count
            ));
        }
    }
    Ok(())
}
