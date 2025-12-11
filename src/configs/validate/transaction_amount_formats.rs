use crate::configs::validate::utils::validate_amount_formats;

pub fn transaction_amount_formats(formats: &[String]) -> Result<(), String> {
    let valid_formats = validate_amount_formats(formats);
    if valid_formats.is_err() {
        return Err(format!(
            "Invalid transaction_amount_formats. {}",
            valid_formats.err().unwrap()
        ));
    }
    Ok(())
}
