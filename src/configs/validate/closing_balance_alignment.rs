use crate::configs::validate::utils::validate_alignment;

pub fn closing_balance_alignment(alignment: &str) -> Result<(), String> {
    let result = validate_alignment(alignment, true, true);
    if result.is_err() {
        return Err(format!(
            "Invalid closing_balance_alignment: {}. {}",
            alignment,
            result.err().unwrap()
        ));
    }
    Ok(())
}
