use crate::configs::validate::utils::iso_3166_1_alpha_2::is_valid_iso_3166_1_alpha_2;

/// Validates a configuration key format.
///
/// A valid key must:
/// - Not contain any whitespace characters
/// - Have exactly 4 components separated by "__"
/// - Be all lowercase
/// - Have a valid ISO 3166-1 alpha-2 country code as the first component
/// - Have an integer as the last component
/// - Middle two components can be any lowercase text
pub fn key(key: &str) -> Result<(), String> {
    // Check if key contains whitespace
    if key.contains(char::is_whitespace) {
        return Err(format!(
            "Key must not contain whitespace. Found: '{}'",
            key
        ));
    }

    // Check if key is all lowercase
    if key != key.to_lowercase() {
        return Err(format!(
            "Key must be all lowercase. Found: '{}'",
            key
        ));
    }

    // Split by "__" and check component count
    let components: Vec<&str> = key.split("__").collect();
    if components.len() != 4 {
        return Err(format!(
            "Key must have exactly 4 components separated by '__'. Found {} components in '{}'",
            components.len(),
            key
        ));
    }

    // Validate first component is a valid ISO 3166-1 alpha-2 country code
    let country_code = components[0];
    if !is_valid_iso_3166_1_alpha_2(country_code) {
        return Err(format!(
            "First component must be a valid ISO 3166-1 alpha-2 country code. Found: '{}'",
            country_code
        ));
    }

    // Validate last component is a non-zero positive integer
    let version = components[3];
    match version.parse::<i32>() {
        Ok(v) if v > 0 => {}
        Ok(v) => {
            return Err(format!(
                "Last component must be a positive, non-zero integer. Found: '{}'",
                v
            ));
        }
        Err(_) => {
            return Err(format!(
                "Last component must be an integer. Found: '{}'",
                version
            ));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_keys() {
        assert!(key("au__cba__credit_card__1").is_ok());
        assert!(key("us__bank__debit__2").is_ok());
        assert!(key("gb__hsbc__loan__10").is_ok());
        assert!(key("de__deutsche__savings__999").is_ok());
    }

    #[test]
    fn test_invalid_uppercase() {
        let result = key("AU__cba__credit_card__1");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("must be all lowercase"));
    }

    #[test]
    fn test_invalid_mixed_case() {
        let result = key("au__CBA__credit_card__1");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("must be all lowercase"));
    }

    #[test]
    fn test_invalid_component_count_too_few() {
        let result = key("au__cba__1");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.contains("exactly 4 components"));
        assert!(err.contains("Found 3 components"));
    }

    #[test]
    fn test_invalid_component_count_too_many() {
        let result = key("au__cba__credit__card__extra__1");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.contains("exactly 4 components"));
        assert!(err.contains("Found 6 components"));
    }

    #[test]
    fn test_invalid_country_code() {
        let result = key("xx__cba__credit_card__1");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.contains("valid ISO 3166-1 alpha-2 country code"));
        assert!(err.contains("xx"));
    }

    #[test]
    fn test_invalid_version_not_integer() {
        let result = key("au__cba__credit_card__abc");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.contains("must be an integer"));
        assert!(err.contains("abc"));
    }

    #[test]
    fn test_invalid_version_empty() {
        let result = key("au__cba__credit_card__");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("must be an integer"));
    }

    #[test]
    fn test_empty_key() {
        let result = key("");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("exactly 4 components"));
    }

    #[test]
    fn test_invalid_zero_version() {
        let result = key("au__cba__credit_card__0");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("positive, non-zero integer"));
    }

    #[test]
    fn test_invalid_negative_version() {
        let result = key("au__cba__credit_card__-1");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("positive, non-zero integer"));
    }

    #[test]
    fn test_invalid_whitespace_in_key() {
        let result = key("au__cba__credit card__1");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("must not contain whitespace"));
    }

    #[test]
    fn test_invalid_leading_whitespace() {
        let result = key(" au__cba__credit_card__1");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("must not contain whitespace"));
    }

    #[test]
    fn test_invalid_trailing_whitespace() {
        let result = key("au__cba__credit_card__1 ");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("must not contain whitespace"));
    }

    #[test]
    fn test_invalid_tab_character() {
        let result = key("au__cba__credit_card\t__1");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("must not contain whitespace"));
    }
}
