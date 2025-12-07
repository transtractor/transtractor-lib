use crate::structs::text_item::TextItem;

/// Retrieves a buffer of tokenised TextItems starting
/// from a specified index up to the defined buffer size.
pub fn get_text_item_buffer(
    items: &Vec<TextItem>,
    index: usize,
    buffer_size: usize,
) -> Vec<TextItem> {
    items
        .iter()
        .skip(index)
        .take(buffer_size)
        .cloned()
        .collect()
}
