use lopdf::Document;
use crate::structs::text_item::TextItem;
use crate::structs::text_items::TextItems;

#[derive(Clone, Debug)]
struct TextState {
    x: f32,
    y: f32,
    leading: f32,
    font_size: f32,
    hscale: f32, // horizontal scaling factor (PDF Tz), 1.0 = 100%
    text_matrix: [f32; 6], // CTM for text positioning
    word_spacing: f32,     // Tw operator
    char_spacing: f32,     // Tc operator
}

impl Default for TextState {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            leading: 0.0,
            font_size: 12.0,
            hscale: 1.0,
            text_matrix: [1.0, 0.0, 0.0, 1.0, 0.0, 0.0], // Identity matrix
            word_spacing: 0.0,
            char_spacing: 0.0,
        }
    }
}

fn translate_xy(x: f32, y: f32, tx: f32, ty: f32) -> (f32, f32) { (x + tx, y + ty) }

// Calculate text width more accurately, accounting for kerning adjustments
fn calculate_text_width(text: &str, font_size: f32, hscale: f32, char_spacing: f32, word_spacing: f32, kerning_adjustments: &[f32]) -> f32 {
    let char_count = text.chars().count() as f32;
    let space_count = text.chars().filter(|&c| c == ' ').count() as f32;
    
    // Base width estimation (improved from simple 0.5 multiplier)
    // Using 0.6 as a better average for typical fonts
    let base_width = 0.6 * font_size * hscale * char_count;
    
    // Add character spacing (applied between all characters)
    let char_spacing_total = char_spacing * (char_count - 1.0).max(0.0);
    
    // Add word spacing (applied to space characters)
    let word_spacing_total = word_spacing * space_count;
    
    // Subtract kerning adjustments (PDF kerning values are typically negative for tighter spacing)
    let kerning_total: f32 = kerning_adjustments.iter().sum::<f32>() * font_size / 1000.0;
    
    base_width + char_spacing_total + word_spacing_total - kerning_total
}

// Calculate text advance from current position
fn calculate_text_advance(text: &str, font_size: f32, hscale: f32, char_spacing: f32, word_spacing: f32, kerning_adjustments: &[f32]) -> f32 {
    calculate_text_width(text, font_size, hscale, char_spacing, word_spacing, kerning_adjustments) * hscale
}

// Decode raw PDF string bytes into best-effort UTF-8 text with simple heuristics
fn decode_pdf_bytes(raw: &[u8]) -> String {
    if raw.is_empty() { return String::new(); }

    fn utf16be_pairs(bytes: &[u8]) -> Option<String> {
        if bytes.len() % 2 != 0 { return None; }
        let mut u16s = Vec::with_capacity(bytes.len()/2);
        for chunk in bytes.chunks_exact(2) { u16s.push(u16::from_be_bytes([chunk[0], chunk[1]])); }
        String::from_utf16(&u16s).ok()
    }
    fn utf16le_pairs(bytes: &[u8]) -> Option<String> {
        if bytes.len() % 2 != 0 { return None; }
        let mut u16s = Vec::with_capacity(bytes.len()/2);
        for chunk in bytes.chunks_exact(2) { u16s.push(u16::from_le_bytes([chunk[0], chunk[1]])); }
        String::from_utf16(&u16s).ok()
    }

    if raw.len() >= 2 {
        if raw[0] == 0xFE && raw[1] == 0xFF { if let Some(s) = utf16be_pairs(&raw[2..]) { return sanitize_text(s); } }
        if raw[0] == 0xFF && raw[1] == 0xFE { if let Some(s) = utf16le_pairs(&raw[2..]) { return sanitize_text(s); } }
    }

    let nul_count = raw.iter().filter(|b| **b == 0).count();
    if nul_count > 0 && nul_count * 2 >= raw.len() {
        let even_nulls = raw.iter().step_by(2).all(|b| *b == 0) || raw.iter().skip(1).step_by(2).all(|b| *b == 0);
        if even_nulls { if let Some(s) = utf16be_pairs(raw) { return sanitize_text(s); } }
    }

    if let Ok(utf8) = std::str::from_utf8(raw) { return sanitize_text(utf8.to_string()); }

    // Fallback: CP-1252 mapping for common PDFs
    let mut out = String::with_capacity(raw.len());
    for &b in raw {
        let ch = match b {
            0x00..=0x7F => b as char,
            0x80 => '\u{20AC}', 0x81 => '\u{0081}', 0x82 => '\u{201A}', 0x83 => '\u{0192}',
            0x84 => '\u{201E}', 0x85 => '\u{2026}', 0x86 => '\u{2020}', 0x87 => '\u{2021}',
            0x88 => '\u{02C6}', 0x89 => '\u{2030}', 0x8A => '\u{0160}', 0x8B => '\u{2039}',
            0x8C => '\u{0152}', 0x8D => '\u{008D}', 0x8E => '\u{017D}', 0x8F => '\u{008F}',
            0x90 => '\u{0090}', 0x91 => '\u{2018}', 0x92 => '\u{2019}', 0x93 => '\u{201C}',
            0x94 => '\u{201D}', 0x95 => '\u{2022}', 0x96 => '\u{2013}', 0x97 => '\u{2014}',
            0x98 => '\u{02DC}', 0x99 => '\u{2122}', 0x9A => '\u{0161}', 0x9B => '\u{203A}',
            0x9C => '\u{0153}', 0x9D => '\u{009D}', 0x9E => '\u{017E}', 0x9F => '\u{0178}',
            0xA0..=0xFF => (0x00A0u16 + (b as u16 - 0xA0)) as u8 as char,
        };
        out.push(ch);
    }
    sanitize_text(out)
}

fn sanitize_text(mut s: String) -> String {
    s.retain(|ch| ch == '\n' || ch == '\t' || !ch.is_control());
    s
}

pub fn parse(pdf_path: &str) -> TextItems {
    let doc = Document::load(pdf_path).unwrap();
    let mut text_items = TextItems::new();

    for (page_num, page_id) in doc.get_pages() {
        // Decode the page's content stream
        let content = match doc.get_page_content(page_id) { Ok(c) => c, Err(_) => continue };
        let operations = match lopdf::content::Content::decode(&content) { Ok(o) => o, Err(_) => continue };
        let mut state = TextState::default();

        for op in operations.operations {
            match op.operator.as_ref() {
                "BT" => { state = TextState::default(); }
                "ET" => { /* end text object */ }
                "Tm" => {
                    if op.operands.len() == 6 {
                        // Set text matrix [a b c d e f]
                        for i in 0..6 {
                            state.text_matrix[i] = op.operands[i].as_f32().unwrap_or(0.0);
                        }
                        state.x = state.text_matrix[4]; // e value
                        state.y = state.text_matrix[5]; // f value
                    }
                }
                "TD" => {
                    if op.operands.len() == 2 {
                        let tx = op.operands[0].as_f32().unwrap_or(0.0);
                        let ty = op.operands[1].as_f32().unwrap_or(0.0);
                        state.leading = -ty;
                        let (nx, ny) = translate_xy(state.x, state.y, tx, ty);
                        state.x = nx; state.y = ny;
                    }
                }
                "Td" => {
                    if op.operands.len() == 2 {
                        let tx = op.operands[0].as_f32().unwrap_or(0.0);
                        let ty = op.operands[1].as_f32().unwrap_or(0.0);
                        let (nx, ny) = translate_xy(state.x, state.y, tx, ty);
                        state.x = nx; state.y = ny;
                    }
                }
                "T*" => {
                    let ty = -state.leading;
                    let (nx, ny) = translate_xy(state.x, state.y, 0.0, ty);
                    state.x = nx; state.y = ny;
                }
                "Tf" => {
                    if op.operands.len() == 2 {
                        state.font_size = op.operands[1].as_f32().unwrap_or(state.font_size);
                    }
                }
                "Tz" => {
                    if let Some(val) = op.operands.get(0) {
                        let pct = val.as_f32().unwrap_or(100.0);
                        state.hscale = if pct.is_finite() { pct / 100.0 } else { 1.0 };
                    }
                }
                "Tw" => {
                    // Word spacing
                    if let Some(val) = op.operands.get(0) {
                        state.word_spacing = val.as_f32().unwrap_or(0.0);
                    }
                }
                "Tc" => {
                    // Character spacing  
                    if let Some(val) = op.operands.get(0) {
                        state.char_spacing = val.as_f32().unwrap_or(0.0);
                    }
                }
                "Tj" => {
                    if let Some(obj) = op.operands.get(0) {
                        if let Ok(bytes) = obj.as_str() {
                            let text_decoded = decode_pdf_bytes(bytes.as_ref());
                            if !text_decoded.is_empty() {
                                let x1 = state.x.floor();
                                let y1 = state.y.floor();
                                
                                // Calculate more accurate width
                                let width_est = calculate_text_width(
                                    &text_decoded, 
                                    state.font_size, 
                                    state.hscale, 
                                    state.char_spacing, 
                                    state.word_spacing, 
                                    &[]
                                );
                                let height_est = state.font_size;
                                let x2 = (x1 + width_est).floor();
                                let y2 = (y1 + height_est).floor();
                                
                                // Update state position for next text
                                state.x += calculate_text_advance(
                                    &text_decoded, 
                                    state.font_size, 
                                    state.hscale, 
                                    state.char_spacing, 
                                    state.word_spacing, 
                                    &[]
                                );
                                
                                text_items.add(&TextItem::new(text_decoded, x1 as i32, y1 as i32, x2 as i32, y2 as i32, page_num as i32));
                            }
                        }
                    }
                }
                "TJ" => {
                    if let Some(obj) = op.operands.get(0) {
                        if let Ok(arr) = obj.as_array() {
                            let mut collected = String::new();
                            let mut kerning_adjustments = Vec::new();
                            
                            for part in arr {
                                if let Ok(s) = part.as_str() {
                                    collected.push_str(&decode_pdf_bytes(s.as_ref()));
                                } else if let Ok(num) = part.as_f32() {
                                    // Collect kerning adjustments for more precise width calculation
                                    kerning_adjustments.push(num);
                                }
                            }
                            
                            if !collected.is_empty() {
                                let x1 = state.x.floor();
                                let y1 = state.y.floor();
                                
                                // Calculate width with kerning adjustments
                                let width_est = calculate_text_width(
                                    &collected, 
                                    state.font_size, 
                                    state.hscale, 
                                    state.char_spacing, 
                                    state.word_spacing, 
                                    &kerning_adjustments
                                );
                                let height_est = state.font_size;
                                let x2 = (x1 + width_est).floor();
                                let y2 = (y1 + height_est).floor();
                                
                                // Update state position for next text
                                state.x += calculate_text_advance(
                                    &collected, 
                                    state.font_size, 
                                    state.hscale, 
                                    state.char_spacing, 
                                    state.word_spacing, 
                                    &kerning_adjustments
                                );
                                
                                text_items.add(&TextItem::new(collected, x1 as i32, y1 as i32, x2 as i32, y2 as i32, page_num as i32));
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }
    text_items
}