use crate::structs::TextItem;
use pdfsink_rs::PdfDocument;

/// Extract a PDF document into a vector of TextItems.
pub fn pdf_to_text_items(pdf_doc: &PdfDocument) -> Result<Vec<TextItem>, String> {
    let mut text_items = Vec::new();

    for (page_index, page) in pdf_doc.pages().iter().enumerate() {
        for word in page.extract_words() {
            let text = word.text.trim();
            if text.is_empty() {
                continue;
            }

            text_items.push(TextItem::new(
                text.to_string(),
                (word.x0.round() as i32).max(0),
                (word.bottom.round() as i32).max(0),
                (word.x1.round() as i32).max(0),
                (word.top.round() as i32).max(0),
                page_index as i32,
            ));
        }
    }

    Ok(text_items)
}
