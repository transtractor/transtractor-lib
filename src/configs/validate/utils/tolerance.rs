/// General validator for tolerance values.
pub fn validate_tolerance(val: i32) -> Result<(), String> {
    if val < 0 {
        return Err(format!("Must be >= 0"));
    }
    Ok(())
}
