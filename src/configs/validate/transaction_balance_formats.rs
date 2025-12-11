use crate::configs::validate::utils::validate_amount_formats;

pub fn transaction_balance_formats(formats: &[String]) -> Result<(), String> {
    if formats.is_empty() {
        return Ok(());
    }
    let valid_formats = validate_amount_formats(formats);
    if valid_formats.is_err() {
        return Err(format!(
            "Invalid transaction_balance_formats. {}",
            valid_formats.err().unwrap()
        ));
    }
    Ok(())
}
