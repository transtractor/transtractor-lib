use crate::structs::TextItems;

/// Converts a PDF file to layout text format and writes it to a file.
pub fn text_items_to_layout_text(
    items: &mut TextItems,
    fix_y_disorder: bool,
) -> Result<String, String> {
    // Apply Y-coordinate disorder fix if requested
    // Create a copy of items for each config to avoid side effects
    let items_copy = if fix_y_disorder {
        items.clone().fix_y_disorder()
    } else {
        items.clone()
    };
    Ok(items_copy.to_layout_text().0)
}