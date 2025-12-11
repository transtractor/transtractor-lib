use crate::configs::validate::utils::validate_tolerance;

pub fn transaction_alignment_tol(tol: i32) -> Result<(), String> {
    let result = validate_tolerance(tol);
    if result.is_err() {
        return Err(format!(
            "Invalid transaction_alignment_tol: {}. {}",
            tol,
            result.err().unwrap()
        ));
    }
    Ok(())
}
