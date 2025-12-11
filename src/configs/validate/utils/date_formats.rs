use crate::formats::date::get_valid_formats;

/// Validate date formats.
pub fn validate_date_formats(date_formats: &[String]) -> Result<(), String> {
    let valid_formats = get_valid_formats();
    for format in date_formats {
        if !valid_formats.contains(&format.as_str()) {
            return Err(format!(
                "Invalid date format: '{}'. Valid formats are: {:?}",
                format, valid_formats
            ));
        }
    }
    Ok(())
}
