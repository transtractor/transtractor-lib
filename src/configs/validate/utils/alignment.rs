/// Validate 1D or 2D alignment settings.
pub fn validate_alignment(alignment: &str, full: bool, allow_blank: bool) -> Result<(), String> {
    if alignment.is_empty() {
        if allow_blank {
            return Ok(());
        } else {
            return Err("Cannot be empty.".to_string());
        }
    }
    if full {
        let valid_alignments_full = ["x1", "x2", "y1", "y2"];
        if !valid_alignments_full.contains(&alignment) {
            return Err(format!("{} must be one of {:?}", alignment, valid_alignments_full));
        }
        return Ok(());
    }
    let valid_alignments = ["x1", "x2"];
    if !valid_alignments.contains(&alignment) {
        return Err(format!("{} must be one of {:?}", alignment, valid_alignments));
    }
    Ok(())
}
