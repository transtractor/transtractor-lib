use crate::configs::validate::utils::validate_alignment;

pub fn transaction_amount_alignment(alignment: &str) -> Result<(), String> {
    let result = validate_alignment(alignment, false, false);
    if result.is_err() {
        return Err(format!(
            "Invalid transaction_amount_alignment: {}. {}",
            alignment,
            result.err().unwrap()
        ));
    }
    Ok(())
}
