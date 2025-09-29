use std::collections::HashMap;
use std::fmt::{self, Display};
use crate::structs::text_item::TextItem;

// Helper extension so this module can format TextItem without altering canonical struct.
trait TextItemExt {
    fn to_debug_block(&self) -> String;
}

impl TextItemExt for TextItem {
    fn to_debug_block(&self) -> String { self.to_layout_block() }
}


/// Wrapper for the serialized layout text
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LayoutText(pub String);

impl Display for LayoutText {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Debug)]
pub enum ParseLayoutError {
    InvalidBlock(String),
    InvalidNumber(String),
    UnexpectedFormat(String),
}

impl Display for ParseLayoutError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseLayoutError::InvalidBlock(b) => write!(f, "Invalid layout block: {}", b),
            ParseLayoutError::InvalidNumber(n) => write!(f, "Invalid numeric value: {}", n),
            ParseLayoutError::UnexpectedFormat(s) => write!(f, "Unexpected format: {}", s),
        }
    }
}

impl std::error::Error for ParseLayoutError {}


#[derive(Debug, Default, Clone)]
pub struct TextItems {
    pub items: Vec<TextItem>,
}

impl TextItems {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    /// Split incoming text by whitespace into multiple TextItems
    pub fn add(&mut self, text_item: &TextItem) {
        for part in text_item.text.split(' ') {
            if part.trim().is_empty() {
                continue;
            }
            self.items.push(TextItem::new(part.to_string(), text_item.x1, text_item.y1, text_item.x2, text_item.y2, text_item.page));
        }
    }

    /// Get a buffer of text items starting from index, up to buffer_size items.
    pub fn get_text_item_buffer(&self, index: usize, buffer_size: usize) -> Vec<TextItem> {
        self.items
            .iter()
            .skip(index)
            .take(buffer_size)
            .cloned()
            .collect()
    }

    /// Determine the most common integer height among text items (y2 - y1).
    /// Returns 0 if no items or if all heights are non-positive.
    fn most_common_height(&self) -> i32 {
        let mut counts: HashMap<i32, usize> = HashMap::new();
        for item in &self.items {
            let h = item.y2 - item.y1;
            if h <= 0 { continue; }
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

    /// Convert the TextItems into a LayoutText representation.
    pub fn to_layout_text(&self) -> LayoutText {
        if self.items.is_empty() {
            return LayoutText(String::new());
        }

        // Merge items sharing identical (page,x1,x2,y1,y2) before layout serialization
        let mut merged: Vec<TextItem> = Vec::new();
            let mut index_map: HashMap<(i32,i32,i32,i32,i32), usize> = HashMap::new();
        for it in &self.items {
            let key = (it.page, it.x1, it.x2, it.y1, it.y2);
            if let Some(&idx) = index_map.get(&key) {
                // Append with a space if not already ending with one and other not empty
                if !it.text.is_empty() {
                    if !merged[idx].text.is_empty() {
                        merged[idx].text.push(' ');
                    }
                    merged[idx].text.push_str(&it.text);
                }
            } else {
                index_map.insert(key, merged.len());
                merged.push(it.clone());
            }
        }

        let items = merged; // work on merged snapshot
        let mut out = String::new();
        let mut curr_page = items[0].page;
        // Removed extra blank line after page header
        out.push_str(&format!("[Page {}]\n", curr_page));
        // Recompute common height based on merged items (could differ slightly)
        let line_height = if items.len() == self.items.len() { self.most_common_height() } else {
            let mut counts: HashMap<i32, usize> = HashMap::new();
            for item in &items { let h = item.y2 - item.y1; if h > 0 { *counts.entry(h).or_insert(0) += 1; } }
            let mut best_h = 0; let mut best_c = 0; for (h,c) in counts { if c > best_c { best_c = c; best_h = h; } } best_h
        };
        let mut curr_y = items[0].y1; // integer baseline
        for item in &items {
            if item.page != curr_page {
                curr_page = item.page;
                out.push_str(&format!("\n[Page {}]\n", curr_page));
                curr_y = item.y1;
            } else if line_height > 0 && (item.y1 - curr_y).abs() >= line_height {
                out.push('\n');
                curr_y = item.y1;
            }
            out.push_str(&item.to_debug_block());
        }
        LayoutText(out)
    }

    /// Convenience: generate layout text and print it to stdout.
    pub fn print_layout(&self) {
        let lt = self.to_layout_text();
        println!("{}", lt.0);
    }

    pub fn read_from_layout_text(&mut self, layout: &LayoutText) -> Result<(), ParseLayoutError> {
        self.items.clear();
            let mut curr_page: i32 = 1;
        for line in layout.0.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }
            if trimmed.starts_with("[Page") {
                // Format: [Page N]
                let inner = trimmed.trim_matches(|c| c == '[' || c == ']');
                let parts: Vec<&str> = inner.split_whitespace().collect();
                if parts.len() != 2 || parts[0] != "Page" {
                    return Err(ParseLayoutError::UnexpectedFormat(trimmed.to_string()));
                }
                    curr_page = parts[1]
                        .parse::<i32>()
                    .map_err(|_| ParseLayoutError::InvalidNumber(parts[1].to_string()))?;
                continue;
            }

            // Split into blocks, conceptually separated by '][' boundaries.
            let mut start = 0usize;
            let bytes = trimmed.as_bytes();
            for i in 0..bytes.len() {
                // when we find '][' boundary or end of line, slice
                let is_boundary = i + 1 < bytes.len() && bytes[i] == b']' && bytes[i + 1] == b'[';
                let is_end = i + 1 == bytes.len();
                if is_boundary || is_end {
                    let slice_end = if is_end { i + 1 } else { i + 1 };
                    let segment = &trimmed[start..slice_end];
                    if !segment.trim().is_empty() {
                        self.parse_and_push_block(segment, curr_page)?;
                    }
                    if is_boundary {
                        start = i + 1; // next block starts with '['
                    }
                }
            }
        }
        Ok(())
    }

    fn parse_and_push_block(&mut self, raw: &str, page: i32) -> Result<(), ParseLayoutError> {
        let cleaned = raw.trim().trim_matches(|c| c == '[' || c == ']');
        if cleaned.is_empty() {
            return Ok(());
        }
        // We expect: "text",x1,x2,y1,y2
        // We'll parse by walking and respecting quotes.
        let mut parts: Vec<String> = Vec::new();
        let mut buf = String::new();
        let mut in_quotes = false;
        for c in cleaned.chars() {
            match c {
                '"' => {
                    in_quotes = !in_quotes;
                    buf.push(c);
                }
                ',' if !in_quotes => {
                    parts.push(buf.trim().to_string());
                    buf.clear();
                }
                _ => buf.push(c),
            }
        }
        if !buf.trim().is_empty() {
            parts.push(buf.trim().to_string());
        }
        if parts.len() != 5 {
            return Err(ParseLayoutError::InvalidBlock(raw.to_string()));
        }

        let text_part = parts[0].trim().trim_matches('"').to_string();
        let x1: i32 = parts[1]
            .parse()
            .map_err(|_| ParseLayoutError::InvalidNumber(parts[1].clone()))?;
        let x2: i32 = parts[2]
            .parse()
            .map_err(|_| ParseLayoutError::InvalidNumber(parts[2].clone()))?;
        let y1: i32 = parts[3]
            .parse()
            .map_err(|_| ParseLayoutError::InvalidNumber(parts[3].clone()))?;
        let y2: i32 = parts[4]
            .parse()
            .map_err(|_| ParseLayoutError::InvalidNumber(parts[4].clone()))?;

        for token in text_part.split_whitespace() {
            if token.is_empty() {
                continue;
            }
            self.items.push(TextItem::new(token.to_string(), x1, y1, x2, y2, page));
        }
        Ok(())
    }
}