use crate::structs::TextItem;

fn parse_quoted_text(input: &str) -> Option<(String, usize)> {
    let bytes = input.as_bytes();
    if bytes.first() != Some(&b'"') {
        return None;
    }

    let mut cursor = 1;
    let mut escaped = false;
    let mut text = String::new();

    while cursor < bytes.len() {
        match bytes[cursor] {
            b'\\' if !escaped => escaped = true,
            b'"' if !escaped => return Some((text, cursor + 1)),
            b'\\' => {
                text.push('\\');
                escaped = false;
            }
            other => {
                if escaped {
                    text.push(other as char);
                    escaped = false;
                } else {
                    text.push(other as char);
                }
            }
        }
        cursor += 1;
    }

    None
}

fn parse_layout_item(input: &str) -> Option<(String, i32, i32, i32, i32, usize)> {
    let input = input.trim_start();
    let end = input.find(']')?;
    let block = &input[..end + 1];
    let contents = &block[1..block.len() - 1];

    let trimmed_contents = contents.trim_start();
    let (text, text_len) = parse_quoted_text(trimmed_contents)?;
    let remainder = &trimmed_contents[text_len..].trim_start();
    let remainder = remainder.trim_start_matches(',').trim_start();

    let values: Vec<i32> = remainder
        .split(',')
        .map(str::trim)
        .map(|value| value.parse::<i32>())
        .collect::<Result<Vec<_>, _>>()
        .ok()?;

    if values.len() != 4 {
        return None;
    }

    Some((text, values[0], values[2], values[1], values[3], end + 1))
}

/// Converts layout text format to a collection of TextItems
pub fn layout_to_text_items(layout_text: &str) -> Result<Vec<TextItem>, String> {
    let mut text_items: Vec<TextItem> = Vec::new();
    let mut current_page = 0;
    let mut cursor = 0;

    while cursor < layout_text.len() {
        let remaining = &layout_text[cursor..];
        let trimmed = remaining.trim_start();
        let trimmed_start = cursor + (remaining.len() - trimmed.len());

        if trimmed.starts_with("[Page") {
            let page_end = trimmed.find(']').unwrap_or(trimmed.len());
            let page_text = trimmed["[Page".len()..page_end].trim();
            if let Ok(page) = page_text.parse::<i32>() {
                current_page = page;
            }
            cursor = trimmed_start + page_end + 1;
            continue;
        }

        if let Some((text, x1, y1, x2, y2, consumed)) = parse_layout_item(trimmed) {
            text_items.push(TextItem::new(text, x1, y1, x2, y2, current_page));
            cursor = trimmed_start + consumed;
        } else {
            cursor += 1;
        }
    }

    Ok(text_items)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parsers::flows::text_items_to_layout::text_items_to_layout;

    #[test]
    fn round_trips_layout_text_back_to_text_items() {
        let items = vec![
            TextItem::new("Alpha".to_string(), 1, 3, 5, 7, 0),
            TextItem::new("Beta".to_string(), 8, 10, 12, 14, 1),
        ];

        let layout = text_items_to_layout(&items, 10.0, 1.0).unwrap();
        let parsed = layout_to_text_items(&layout).unwrap();

        assert_eq!(parsed, items);
    }
}
