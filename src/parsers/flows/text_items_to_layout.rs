use crate::structs::TextItem;
use crate::structs::text_items::sort_items;

/// Converts a collection of TextItems into a structured layout text format
pub fn text_items_to_layout(
    items: &Vec<TextItem>,
    y_bin: f32,
    x_gap: f32,
) -> Result<String, String> {
    if items.is_empty() {
        return Ok(String::new());
    }

    let sorted_items = sort_items(items, x_gap, y_bin);

    let mut output = String::new();
    let mut current_page = sorted_items[0].page;
    let mut last_y1 = sorted_items[0].y1;
    let mut last_height = sorted_items[0].y2 - sorted_items[0].y1;

    // Start with the first page marker
    output.push_str(&format!("[Page {}]", current_page));

    for item in &sorted_items {
        // Check if we're on a new page
        if item.page != current_page {
            current_page = item.page;
            output.push_str(&format!("\n[Page {}]\n", current_page));
            last_y1 = item.y1;
            last_height = (item.y2 - item.y1).abs();
        } else {
            // Check if y1 deviation is more than 50% of last item's height
            let y_deviation = (item.y1 - last_y1).abs();
            let threshold = (last_height as f32 * 0.5) as i32;

            if y_deviation > threshold {
                output.push('\n');
                last_y1 = item.y1;
            }
            // Always update height for next comparison
            last_height = (item.y2 - item.y1).abs();
        }

        // Print the item in the format [text, x1, x2, y1, y2]
        output.push_str(&format!(
            "[\"{}\",{},{},{},{}]",
            item.text, item.x1, item.x2, item.y1, item.y2
        ));
    }

    Ok(output)
}
