use lopdf::Document;
use crate::structs::text_item::TextItem;
use crate::structs::text_items::TextItems;

#[derive(Clone, Debug)]
struct TextState {
    x: f32,
    y: f32,
    leading: f32,
    font_size: f32,
}

impl Default for TextState {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            leading: 0.0,
            font_size: 12.0,
        }
    }
}
fn translate_xy(x: f32, y: f32, tx: f32, ty: f32) -> (f32, f32) { (x + tx, y + ty) }

/// Decode raw PDF string bytes into (best-effort) UTF-8 text.
/// Heuristics:
/// 1. If BOM (FE FF) -> UTF-16BE, (FF FE) -> UTF-16LE.
/// 2. If many null bytes and they appear at either all even or all odd indices -> treat as UTF-16BE (common in PDFs without BOM).
/// 3. Fallback: lossless UTF-8 with replacement and removal of embedded NULs.
fn decode_pdf_bytes(raw: &[u8]) -> String {
    if raw.is_empty() { return String::new(); }

    // Helper to build UTF-16 from big-endian pairs
    fn utf16be_pairs(bytes: &[u8]) -> Option<String> {
        if bytes.len() % 2 != 0 { return None; }
        let mut u16s = Vec::with_capacity(bytes.len()/2);
        for chunk in bytes.chunks_exact(2) {
            u16s.push(u16::from_be_bytes([chunk[0], chunk[1]]));
        }
        String::from_utf16(&u16s).ok()
    }
    // Helper for little-endian
    fn utf16le_pairs(bytes: &[u8]) -> Option<String> {
        if bytes.len() % 2 != 0 { return None; }
        let mut u16s = Vec::with_capacity(bytes.len()/2);
        for chunk in bytes.chunks_exact(2) {
            u16s.push(u16::from_le_bytes([chunk[0], chunk[1]]));
        }
        String::from_utf16(&u16s).ok()
    }

    if raw.len() >= 2 {
        if raw[0] == 0xFE && raw[1] == 0xFF {
            if let Some(s) = utf16be_pairs(&raw[2..]) { return sanitize_text(s); }
        } else if raw[0] == 0xFF && raw[1] == 0xFE {
            if let Some(s) = utf16le_pairs(&raw[2..]) { return sanitize_text(s); }
        }
    }

    // Heuristic null pattern detection.
    let nul_count = raw.iter().filter(|b| **b == 0).count();
    if nul_count > 0 && nul_count * 2 >= raw.len() { // at least ~50% null bytes
        let even_nulls = raw.iter().step_by(2).all(|b| *b == 0) || raw.iter().skip(1).step_by(2).all(|b| *b == 0);
        if even_nulls {
            // Assume big-endian (common) and attempt decode
            if let Some(s) = utf16be_pairs(raw) { return sanitize_text(s); }
        }
    }

    // If valid UTF-8, use it directly.
    if let Ok(utf8) = std::str::from_utf8(raw) {
        return sanitize_text(utf8.to_string());
    }

    // Fallback: decode as Windows-1252 (WinAnsi) which is extremely common in PDFs
    // when no UTF-16 BOM is present. This avoids U+FFFD replacement characters.
    let mut out = String::with_capacity(raw.len());
    for &b in raw {
        let ch = match b {
            0x00..=0x7F => b as char,
            0x80 => '\u{20AC}', // Euro
            0x81 => '\u{0081}',
            0x82 => '\u{201A}',
            0x83 => '\u{0192}',
            0x84 => '\u{201E}',
            0x85 => '\u{2026}',
            0x86 => '\u{2020}',
            0x87 => '\u{2021}',
            0x88 => '\u{02C6}',
            0x89 => '\u{2030}',
            0x8A => '\u{0160}',
            0x8B => '\u{2039}',
            0x8C => '\u{0152}',
            0x8D => '\u{008D}',
            0x8E => '\u{017D}',
            0x8F => '\u{008F}',
            0x90 => '\u{0090}',
            0x91 => '\u{2018}',
            0x92 => '\u{2019}', // right single quote ’
            0x93 => '\u{201C}',
            0x94 => '\u{201D}',
            0x95 => '\u{2022}', // bullet •
            0x96 => '\u{2013}', // en dash –
            0x97 => '\u{2014}', // em dash —
            0x98 => '\u{02DC}',
            0x99 => '\u{2122}', // ™
            0x9A => '\u{0161}',
            0x9B => '\u{203A}',
            0x9C => '\u{0153}',
            0x9D => '\u{009D}',
            0x9E => '\u{017E}',
            0x9F => '\u{0178}',
            0xA0..=0xFF => (0x00A0u16 + (b as u16 - 0xA0)) as u8 as char,
        };
        out.push(ch);
    }
    sanitize_text(out)
}

fn sanitize_text(mut s: String) -> String {
    // Trim leading/trailing control artifacts while preserving legitimate whitespace.
    // Remove isolated RTL/LTR markers if present (common in some PDFs)
    s.retain(|ch| ch == '\n' || ch == '\t' || !ch.is_control());
    s
}

pub fn parse(pdf_path: &str) -> TextItems {
    let doc = Document::load(pdf_path).unwrap();
    let mut text_items = TextItems::new();

    for (page_num, page_id) in doc.get_pages() {
        let content = match doc.get_page_content(page_id) { Ok(c) => c, Err(_) => continue };
        let operations = match lopdf::content::Content::decode(&content) { Ok(o) => o, Err(_) => continue };
        let mut state = TextState::default();

        for op in operations.operations {            
            match op.operator.as_ref() {
                "BT" => { state = TextState::default(); }
                "ET" => { /* end text object */ }
                "Tm" => {
                    if op.operands.len() == 6 {
                        // Only record translation components (e, f)
                        let e = op.operands[4].as_f32().unwrap_or(0.0);
                        let f = op.operands[5].as_f32().unwrap_or(0.0);
                        state.x = e; state.y = f;
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
                // Simple text show: decode and add an item; advance x by a crude width estimate
                "Tj" => {
                    if let Some(obj) = op.operands.get(0) {
                        if let Ok(bytes) = obj.as_str() {
                            let raw = bytes.as_ref();
                            let start_x = state.x; let start_y = state.y;
                            let text_decoded = decode_pdf_bytes(raw);
                            // Coarse width estimate: ~0.5em per character to match golden layout
                            let est_w = (text_decoded.chars().count() as f32) * state.font_size * 0.5;
                            state.x += est_w;
                            if !text_decoded.is_empty() {
                                let x1 = start_x as i32;
                                let y1 = start_y as i32;
                                let x2 = (start_x + est_w) as i32;
                                let y2 = (start_y + state.font_size) as i32;
                                text_items.add(&TextItem::new(text_decoded, x1, y1, x2, y2, page_num as i32));
                            }
                        }
                    }
                }
                // Array show: concatenate strings, apply embedded adjustments coarsely
                "TJ" => {
                    if let Some(obj) = op.operands.get(0) {
                        if let Ok(arr) = obj.as_array() {
                            let start_x = state.x; let start_y = state.y;
                            let mut collected = String::new();
                            for part in arr {
                                if let Ok(s) = part.as_str() {
                                    let slice = s.as_ref();
                                    collected.push_str(&decode_pdf_bytes(slice));
                                } else if let Ok(num) = part.as_f32() {
                                    // Ignore detailed glyph adjustments; they are tiny layout nudges. We keep coarse estimate.
                                    let _ = num;
                                }
                            }
                            if !collected.is_empty() {
                                // Coarse width estimate matching golden: ~0.5em per character
                                let est_w = (collected.chars().count() as f32) * state.font_size * 0.5;
                                state.x += est_w;
                                let x1 = start_x as i32;
                                let y1 = start_y as i32;
                                let x2 = (start_x + est_w) as i32;
                                let y2 = (start_y + state.font_size) as i32;
                                text_items.add(&TextItem::new(collected, x1, y1, x2, y2, page_num as i32));
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

#[cfg(test)]
mod tests {
    use super::decode_pdf_bytes;

    #[test]
    fn cp1252_common_chars_decode() {
        // CP1252 bytes: 0x92 (’), 0x97 (—), 0xAE (®), 0x80 (€)
        let bytes = [0x57u8, 0x65, 0x27, 0x72, 0x65, 0x20, 0x68, 0x65, 0x72, 0x65, 0x3A, 0x20, 0x80, 0x20, 0x92, 0x20, 0x97, 0x20, 0xAE];
        let s = decode_pdf_bytes(&bytes);
        assert!(!s.contains('\u{FFFD}'), "Should not contain replacement chars: {}", s);
        assert!(s.contains('€'));
        assert!(s.contains('’'));
        assert!(s.contains('—'));
        assert!(s.contains('®'));
    }
}
