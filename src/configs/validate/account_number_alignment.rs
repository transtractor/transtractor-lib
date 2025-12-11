use crate::configs::validate::utils::alignment::validate_alignment;

pub fn account_number_alignment(alignment: &str) -> Result<(), String> {
    let result = validate_alignment(alignment, true, true);
    if result.is_err() {
        return Err(format!(
            "Invalid account_number_alignment: {}. {}",
            alignment,
            result.err().unwrap()
        ));
    }
    Ok(())
}
