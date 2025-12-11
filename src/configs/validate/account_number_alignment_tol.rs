use crate::configs::validate::utils::validate_tolerance;

pub fn account_number_alignment_tol(tol: i32) -> Result<(), String> {
    let result = validate_tolerance(tol);
    if result.is_err() {
        return Err(format!(
            "Invalid account_number_alignment_tol: {}. {}",
            tol,
            result.err().unwrap()
        ));
    }
    Ok(())
}
