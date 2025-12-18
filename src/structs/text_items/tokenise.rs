use crate::structs::text_item::TextItem;

/// Splits each TextItem's text into separate tokens based on whitespace,
/// creating a new TextItem for each token while preserving the original
/// positional and page information.
pub fn tokenise_items(items: &Vec<TextItem>) -> Vec<TextItem> {
    let mut tokenised_items: Vec<TextItem> = Vec::new();
    for item in items {
        let parts = item.text.split_whitespace();
        for part in parts {
            let token_item = TextItem {
                text: part.to_string(),
                x1: item.x1,
                y1: item.y1,
                x2: item.x2,
                y2: item.y2,
                page: item.page,
            };
            tokenised_items.push(token_item);
        }
    }
    tokenised_items
}
