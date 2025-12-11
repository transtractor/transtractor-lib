/// Validate fix_text_order configuration option.
pub fn fix_text_order(fix_text_order: &Vec<f32>) -> Result<(), String> {
    if fix_text_order.len() != 2 {
        return Err(format!("Invalid fix_text_order: {:?}. Must contain exactly 2 elements: [y_bin, x_gap]", fix_text_order));
    }
    if fix_text_order[0] < 0.0 {
        return Err(format!("Invalid fix_text_order: {:?}. fix_text_order[0] (y_bin) must be >= 0.0", fix_text_order));
    }
    if fix_text_order[1] < 0.0 {
        return Err(format!("Invalid fix_text_order: {:?}. fix_text_order[1] (x_gap) must be >= 0.0", fix_text_order));
    }
    Ok(())
}
