use crate::configs::validate::utils::validate_date_formats;

pub fn start_date_formats(formats: &[String]) -> Result<(), String> {
    let valid_formats = validate_date_formats(formats);
    if valid_formats.is_err() {
        return Err(format!(
            "Invalid start_date_formats. {}",
            valid_formats.err().unwrap()
        ));
    }
    Ok(())
}
