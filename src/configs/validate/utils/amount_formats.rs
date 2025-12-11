use crate::formats::amount::get_valid_formats;

/// Validate amount formats.
pub fn validate_amount_formats(amount_formats: &[String]) -> Result<(), String> {
    let valid_formats = get_valid_formats();
    for format in amount_formats {
        if !valid_formats.contains(&format.as_str()) {
            return Err(format!(
                "Invalid amount format: '{}'. Valid formats are: {:?}",
                format, valid_formats
            ));
        }
    }
    Ok(())
}
