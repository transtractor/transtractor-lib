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
    /// The page number where the text item is located (i32 for downstream interoperability)
    pub page: i32,
}

impl TextItem {
    /// TextItem constructor
    pub fn new(text: String, x1: i32, y1: i32, x2: i32, y2: i32, page: i32) -> Self {
        TextItem { text, x1, y1, x2, y2, page }
    }

    /// Returns a default TextItem with empty text and zeroed coordinates/page.
    pub fn default() -> Self {
        TextItem {
            text: String::new(),
            x1: 0,
            y1: 0,
            x2: 0,
            y2: 0,
            page: 0,
        }
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
        // take the smallest x1 and y1 from self and other 
        self.x1 = self.x1.min(other.x1);
        self.x2 = self.x2.max(other.x2);

        // If y-axis is inverted (0 at top), take largest y1 and smallest y2
        if (self.y1 > self.y2) && (other.y1 > other.y2) {
            self.y1 = self.y1.max(other.y1);
            self.y2 = self.y2.min(other.y2);
            return;
        }
        if (self.y1 < self.y2) && (other.y1 < other.y2) {
            self.y1 = self.y1.min(other.y1);
            self.y2 = self.y2.max(other.y2);
            return;
        }
        // Mixed y-axis orientation, raise panic
        panic!("Inconsistent y-axis orientation when merging TextItems");
    }

    /// Create merged TextItem from a slice of TextItems
    pub fn from_items(items: &[TextItem]) -> Option<TextItem> {
        if items.is_empty() {
            return None;
        }
        let first = &items[0];
        let last = &items[items.len() - 1];
        let merged_text = items.iter().map(|it| it.text.clone()).collect::<Vec<_>>().join(" ");
        Some(TextItem {
            text: merged_text,
            x1: first.x1,
            y1: last.y1,
            x2: last.x2,
            y2: first.y2,
            page: first.page, // Just take the page of the first item
        })
    }

    /// Return a string of format ["text",x1,x2,y1,y2] with raw integer coordinates.
    pub fn to_layout_block(&self) -> String {
        // Keeping page excluded from list for backward compatibility; add if needed.
        format!("[\"{}\",{},{},{},{}]", self.text, self.x1, self.x2, self.y1, self.y2)
    }

    /// Clone a new TextItem from self
    pub fn clone(&self) -> TextItem {
        TextItem {
            text: self.text.clone(),
            x1: self.x1,
            y1: self.y1,
            x2: self.x2,
            y2: self.y2,
            page: self.page,
        }
    }

    /// Get x1 coordinate
    pub fn x1(&self) -> i32 {
        self.x1
    }

    /// Get y1 coordinate
    pub fn y1(&self) -> i32 {
        self.y1
    }

    /// Get x2 coordinate
    pub fn x2(&self) -> i32 {
        self.x2
    }

    /// Get y2 coordinate
    pub fn y2(&self) -> i32 {
        self.y2
    }

    /// Get page number
    pub fn page(&self) -> i32 {
        self.page
    }
}