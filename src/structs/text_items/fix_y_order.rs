use crate::structs::text_item::TextItem;
use crate::structs::text_items::most_common_height::most_common_height;

#[derive(Clone)]
struct Line {
    y: i32,
    items: Vec<TextItem>,
}

impl Line {
    fn new(y: i32) -> Self {
        Line {
            y,
            items: Vec::new(),
        }
    }

    fn append(&mut self, item: TextItem) {
        self.items.push(item);
    }
}

fn create_and_insert_new_line(lines: &mut Vec<Line>, item: TextItem) {
    let mut new_line = Line::new(item.y1);
    new_line.append(item);
    lines.push(new_line);
    lines.sort_by(|a, b| b.y.cmp(&a.y)); // Sort in descending Y order
}

fn append_item_to_previous_line(lines: &mut Vec<Line>, item: TextItem, line_height: i32) {
    for line in lines.iter_mut() {
        let dist = (line.y - item.y1).abs();
        if dist < line_height {
            line.append(item);
            return;
        }
    }
    create_and_insert_new_line(lines, item);
}

fn append_line_items_to_result(lines: &mut [Line], items: &mut Vec<TextItem>) {
    for line in lines {
        for item in &line.items {
            items.push(item.clone());
        }
    }
}

pub fn fix_y_order(text_items: &[TextItem]) -> Vec<TextItem> {
    let line_height = most_common_height(text_items);
    let mut result: Vec<TextItem> = Vec::new();
    let mut lines: Vec<Line> = Vec::new();
    let mut curr_page = -1;
    let mut curr_y_pos = -1;

    for item in text_items {
        // Append last page items and reset state
        if item.page != curr_page {
            append_line_items_to_result(&mut lines, &mut result);
            curr_page = item.page;
            lines = Vec::new();
            curr_y_pos = item.y1;
            let mut curr_line = Line::new(curr_y_pos);
            curr_line.append(item.clone());
            lines.push(curr_line);
            continue;
        }

        let dist = (curr_y_pos - item.y1).abs();

        // Append to current line
        if dist < line_height {
            // Find the current line (last one in lines) and append to it
            if let Some(last_line) = lines.last_mut() {
                last_line.append(item.clone());
            }
            continue;
        }

        // Create new line
        if item.y1 < curr_y_pos {
            let mut new_line = Line::new(item.y1);
            new_line.append(item.clone());
            lines.push(new_line);
            curr_y_pos = item.y1;
            continue;
        }

        // Append to a previous line
        append_item_to_previous_line(&mut lines, item.clone(), line_height);
    }

    // Append last page items
    append_line_items_to_result(&mut lines, &mut result);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_item(text: &str, y1: i32, page: i32) -> TextItem {
        TextItem::new(text.to_string(), 0, y1, 100, y1 + 12, page)
    }

    #[test]
    fn test_fix_y_order_single_page() {
        let text_items = vec![
            create_test_item("Item1", 100, 1), // Top
            create_test_item("Item3", 50, 1),  // Bottom  
            create_test_item("Item2", 75, 1),  // Middle
        ];

        let reordered = fix_y_order(&text_items);
        
        // Should be ordered by Y position (descending)
        assert_eq!(reordered.len(), 3);
        assert_eq!(reordered[0].text, "Item1"); // Y=100
        assert_eq!(reordered[1].text, "Item2"); // Y=75
        assert_eq!(reordered[2].text, "Item3"); // Y=50
    }

    #[test]
    fn test_fix_y_order_same_line_grouping() {
        let text_items = vec![
            create_test_item("Word1", 100, 1),
            create_test_item("Word3", 95, 1),  // Close enough to be same line
            create_test_item("Word2", 102, 1), // Close enough to be same line
            create_test_item("NextLine", 80, 1), // Different line
        ];

        let reordered = fix_y_order(&text_items);
        
        assert_eq!(reordered.len(), 4);
        // First three should be grouped together, then the last one
        assert_eq!(reordered[0].text, "Word1");
        assert_eq!(reordered[1].text, "Word3");
        assert_eq!(reordered[2].text, "Word2");
        assert_eq!(reordered[3].text, "NextLine");
    }

    #[test]
    fn test_fix_y_order_multiple_pages() {
        let text_items = vec![
            create_test_item("P1_Item2", 50, 1),
            create_test_item("P1_Item1", 100, 1),
            create_test_item("P2_Item2", 60, 2),
            create_test_item("P2_Item1", 90, 2),
        ];

        let reordered = fix_y_order(&text_items);
        
        assert_eq!(reordered.len(), 4);
        
        // Page 1 items should be first and ordered (higher Y first)
        assert_eq!(reordered[0].text, "P1_Item1"); // Y=100
        assert_eq!(reordered[0].page, 1);
        assert_eq!(reordered[1].text, "P1_Item2"); // Y=50
        assert_eq!(reordered[1].page, 1);
        
        // Page 2 items should follow and be ordered (higher Y first)
        assert_eq!(reordered[2].text, "P2_Item1"); // Y=90
        assert_eq!(reordered[2].page, 2);
        assert_eq!(reordered[3].text, "P2_Item2"); // Y=60
        assert_eq!(reordered[3].page, 2);
    }

    #[test]
    fn test_fix_y_order_empty_items() {
        let text_items: Vec<TextItem> = vec![];
        let reordered = fix_y_order(&text_items);
        assert_eq!(reordered.len(), 0);
    }
}
