use std::collections::HashMap;
use crate::structs::text_item::TextItem;

/// Determine the most common integer height among text items (y2 - y1).
/// Returns 0 if no items or if all heights are non-positive.
pub fn most_common_height(items: &[TextItem]) -> i32 {
    let mut counts: HashMap<i32, usize> = HashMap::new();
    for item in items {
        let h = item.y2 - item.y1;
        if h <= 0 {
            continue;
        }
        *counts.entry(h).or_insert(0) += 1;
    }
    let mut best_height: i32 = 0;
    let mut best_count: usize = 0;
    for (h, c) in counts {
        if c > best_count {
            best_count = c;
            best_height = h;
        }
    }
    best_height
}