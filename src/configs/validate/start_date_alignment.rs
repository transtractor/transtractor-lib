use crate::configs::validate::utils::validate_alignment;

pub fn start_date_alignment(alignment: &str) -> Result<(), String> {
    let result = validate_alignment(alignment, true, true);
    if result.is_err() {
        return Err(format!(
            "Invalid start_date_alignment: {}. {}",
            alignment,
            result.err().unwrap()
        ));
    }
    Ok(())
}
