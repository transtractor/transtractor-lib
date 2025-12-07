use crate::structs::text_item::TextItem;
use std::collections::HashMap;

/// Calculate average character width for a single TextItem
fn average_char_width(item: &TextItem) -> f32 {
    let width = item.x2 - item.x1;
    let num_chars = item.text.len() as i32;
    if num_chars == 0 {
        0.0
    } else {
        width as f32 / num_chars as f32
    }
}

// Return new array of items sorted by x position with close items merged
fn fix_by_x(items: &mut Vec<TextItem>, x_gap: f32) -> Vec<TextItem> {
    // Return if x_gap is zero
    if x_gap == 0.0 {
        return items.clone();
    }
    // Sort items by increasing X position
    items.sort_by(|a, b| a.x1.cmp(&b.x1));
    let mut fixed_items = Vec::new();
    for item in items {
        if let Some(last_item) = fixed_items.last_mut() {
            // Merge new line into last item if close enough
            let avg_char_width = average_char_width(&last_item);
            let x_merge_tol = (avg_char_width * x_gap) as i32;
            // x1 of next item overlaps within x range of last item
            let x1_within_tol =
                item.x1 >= last_item.x1 - x_merge_tol && item.x1 <= last_item.x2 + x_merge_tol;
            if x1_within_tol {
                // Merge into last item
                last_item.merge(item);
                continue;
            }
        }
        fixed_items.push(item.clone());
    }
    fixed_items
}

/// Ensure items are sorted by page, y position, and x position
pub fn sort_items(items: &Vec<TextItem>, x_gap: f32, y_bin: f32) -> Vec<TextItem> {
    // Return if no items or t_bin is zero
    if items.is_empty() || y_bin == 0.0 {
        return items.clone();
    }
    // {page: {y1_bin: Vec<TextItem>}}
    let mut num_ascending = 0;
    let mut num_descending = 0;
    let mut page_map: HashMap<i32, HashMap<i32, Vec<TextItem>>> = HashMap::new();
    for item in items {
        if item.y1 < item.y2 {
            num_descending += 1;
        } else {
            num_ascending += 1;
        }
        let page_entry = page_map.entry(item.page).or_insert_with(HashMap::new);
        let y1_bin = (item.y1 as f32 / y_bin) as i32;
        let y_bin_entry = page_entry.entry(y1_bin).or_insert_with(Vec::new);
        y_bin_entry.push(item.clone());
    }

    let mut sorted_items: Vec<TextItem> = Vec::new();
    // Add items in ascending page order and ascending or descending Y order
    let mut page_keys: Vec<i32> = page_map.keys().cloned().collect();
    page_keys.sort_unstable();
    let y_ascending = num_ascending >= num_descending;
    for page in page_keys {
        if let Some(y_bin_map) = page_map.get(&page) {
            let mut y_bin_keys: Vec<i32> = y_bin_map.keys().cloned().collect();
            if y_ascending {
                y_bin_keys.sort_unstable();
            } else {
                y_bin_keys.sort_unstable_by(|a, b| b.cmp(a));
            }
            for y_bin in y_bin_keys {
                if let Some(mut bin_items) = y_bin_map.get(&y_bin).cloned() {
                    let fixed_items = fix_by_x(&mut bin_items, x_gap);
                    sorted_items.extend(fixed_items);
                }
            }
        }
    }
    sorted_items
}
