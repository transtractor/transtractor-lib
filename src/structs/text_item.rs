/// Represents a text item from a PDF document with its position and size.
#[derive(Debug, Clone, PartialEq)]
pub struct TextItem {
    /// The text content of the item
    pub text: String,
    /// Starting x-coordinate of the text item
    pub x1: i32,
    /// Starting y-coordinate of the text item
    pub y1: i32,
    /// Ending x-coordinate of the text item
    pub x2: i32,
    /// Ending y-coordinate of the text item
    pub y2: i32,
    /// The page number where the text item is located
    pub page: usize,
}

impl TextItem {
    /// TextItem constructor
    pub fn new(text: String, x1: i32, y1: i32, x2: i32, y2: i32, page: usize) -> Self {
        TextItem { text, x1, y1, x2, y2, page }
    }

    /// Check if object has the same x, y, x2, y2 and page properties
    pub fn has_same_props(&self, other: &TextItem) -> bool {
        self.x1 == other.x1 &&
        self.y1 == other.y1 &&
        self.x2 == other.x2 &&
        self.y2 == other.y2 &&
        self.page == other.page
    }

    /// Merge the text of this TextItem with another TextItem
    pub fn merge(&mut self, other: &TextItem) {
        self.text = format!("{} {}", self.text, other.text);
    }

    /// Return a string of format ["text",x1,x2,y1,y2] with raw integer coordinates.
    pub fn to_layout_block(&self) -> String {
        // Keeping page excluded from list for backward compatibility; add if needed.
        format!("[\"{}\",{},{},{},{}]", self.text, self.x1, self.x2, self.y1, self.y2)
    }
}