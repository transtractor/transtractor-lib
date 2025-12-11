use crate::configs::validate::utils::validate_amount_formats;

pub fn closing_balance_formats(formats: &[String]) -> Result<(), String> {
    let valid_formats = validate_amount_formats(formats);
    if valid_formats.is_err() {
        return Err(format!(
            "Invalid closing_balance_formats. {}",
            valid_formats.err().unwrap()
        ));
    }
    Ok(())
}
