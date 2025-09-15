use lopdf::{Document, Object, ObjectId};
use std::collections::HashMap;
use crate::structs::text_item::TextItem;
use crate::structs::text_items::TextItems;


// Local TextItem struct removed; using shared struct in crate::structs::text_item

#[derive(Clone, Debug, Default)]
struct FontMetrics {
    first_char: i32,
    widths: Vec<f32>, // stored in glyph space (already scaled to 1/1000 units -> we'll divide by 1000 when used)
    avg_width: f32,
}

#[derive(Clone, Debug)]
struct TextState {
    text_matrix: [f32; 6],
    text_line_matrix: [f32; 6],
    leading: f32,
    font_size: f32,
    horizontal_scaling: f32, // percentage (100 = normal)
    char_spacing: f32,
    word_spacing: f32,
    current_font: Option<String>,
}

impl Default for TextState {
    fn default() -> Self {
        Self {
            text_matrix: [1.0, 0.0, 0.0, 1.0, 0.0, 0.0],
            text_line_matrix: [1.0, 0.0, 0.0, 1.0, 0.0, 0.0],
            leading: 0.0,
            font_size: 12.0,
            horizontal_scaling: 100.0,
            char_spacing: 0.0,
            word_spacing: 0.0,
            current_font: None,
        }
    }
}

fn matrix_multiply(m1: [f32; 6], m2: [f32; 6]) -> [f32; 6] {
    // (a b c d e f) * (a' b' c' d' e' f') => (a*a'+b*c'  a*b'+b*d'  c*a'+d*c'  c*b'+d*d'  e*a'+f*c'+e'  e*b'+f*d'+f')
    let (a, b, c, d, e, f) = (m1[0], m1[1], m1[2], m1[3], m1[4], m1[5]);
    let (a2, b2, c2, d2, e2, f2) = (m2[0], m2[1], m2[2], m2[3], m2[4], m2[5]);
    [
        a * a2 + b * c2,
        a * b2 + b * d2,
        c * a2 + d * c2,
        c * b2 + d * d2,
        e * a2 + f * c2 + e2,
        e * b2 + f * d2 + f2,
    ]
}

fn translate(m: [f32; 6], tx: f32, ty: f32) -> [f32; 6] {
    matrix_multiply(m, [1.0, 0.0, 0.0, 1.0, tx, ty])
}

fn build_font_metrics(doc: &Document, page_id: ObjectId) -> HashMap<String, FontMetrics> {
    let mut result = HashMap::new();
    let page_obj = match doc.get_object(page_id) { Ok(o) => o, Err(_) => return result };
    let page_dict = match page_obj.as_dict() { Ok(d) => d, Err(_) => return result };
    let resources_obj = match page_dict.get(b"Resources") { Ok(o) => o, Err(_) => return result };
    let resources_dict = match resources_obj.as_dict() { Ok(d) => d, Err(_) => return result };
    let fonts_obj = match resources_dict.get(b"Font") { Ok(o) => o, Err(_) => return result };
    let fonts_dict = match fonts_obj.as_dict() { Ok(d) => d, Err(_) => return result };
    for (name, font_ref_obj) in fonts_dict.iter() {
        // Resolve to an owned Object we can inspect
        let font_dict_obj: Object = if let Ok(r) = font_ref_obj.as_reference() {
            match doc.get_object(r) { Ok(o) => o.clone(), Err(_) => continue }
        } else {
            font_ref_obj.clone()
        };
        let font_dict = match font_dict_obj.as_dict() { Ok(d) => d, Err(_) => continue };
        let first_char = font_dict.get(b"FirstChar").ok().and_then(|o| o.as_i64().ok()).unwrap_or(0) as i32;
        let widths_vec: Vec<f32> = font_dict
            .get(b"Widths").ok()
            .and_then(|o| o.as_array().ok())
            .map(|arr| {
                arr.iter()
                    .filter_map(|w| {
                        if let Ok(v) = w.as_f32() { Some(v) }
                        else if let Ok(i) = w.as_i64() { Some(i as f32) }
                        else { None }
                    })
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        let avg_width = if widths_vec.is_empty() { 500.0 } else { widths_vec.iter().copied().sum::<f32>() / widths_vec.len() as f32 };
        result.insert(
            String::from_utf8_lossy(name).to_string(),
            FontMetrics { first_char, widths: widths_vec, avg_width },
        );
    }
    result
}

fn glyph_advance(c: u8, fm: &FontMetrics) -> f32 {
    let idx = (c as i32 - fm.first_char) as usize;
    let w = fm.widths.get(idx).copied().unwrap_or(fm.avg_width);
    w // still in glyph units (1/1000 of text space * font size will be applied outside)
}

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

    // Fallback: lossless UTF-8, strip NULs and high control chars except tab/newline.
    let mut s = String::from_utf8_lossy(raw).to_string();
    s.retain(|ch| ch != '\0' && (ch == '\n' || ch == '\t' || !ch.is_control()));
    sanitize_text(s)
}

fn sanitize_text(mut s: String) -> String {
    // Trim leading/trailing control artifacts while preserving legitimate whitespace.
    // Remove isolated RTL/LTR markers if present (common in some PDFs)
    s.retain(|ch| ch == '\n' || ch == '\t' || !ch.is_control());
    s
}

pub fn extract_text_items(pdf_path: &str) -> TextItems {
    let doc = Document::load(pdf_path).unwrap();
    let mut text_items = TextItems::new();

    for (page_num, page_id) in doc.get_pages() {
        let content = match doc.get_page_content(page_id) { Ok(c) => c, Err(_) => continue };
        let operations = match lopdf::content::Content::decode(&content) { Ok(o) => o, Err(_) => continue };
        let fonts = build_font_metrics(&doc, page_id);

        // Page height for optional top-left conversion (currently we keep PDF bottom-left coordinates)
        let page_height = match doc.get_object(page_id) {
            Ok(obj) => match obj.as_dict() {
                Ok(d) => match d.get(b"MediaBox") {
                    Ok(mb) => match mb.as_array() {
                        Ok(a) if a.len() == 4 => {
                            // Provide robust numeric extraction for the y-max
                            let num_obj = &a[3];
                            if let Ok(v) = num_obj.as_f32() { v } else if let Ok(i) = num_obj.as_i64() { i as f32 } else { 0.0 }
                        }
                        _ => 0.0,
                    },
                    Err(_) => 0.0,
                },
                Err(_) => 0.0,
            },
            Err(_) => 0.0,
        };

        let mut state = TextState::default();

        for op in operations.operations {            
            match op.operator.as_ref() {
                "BT" => { state = TextState::default(); }
                "ET" => { /* end text object */ }
                "Tm" => {
                    if op.operands.len() == 6 {
                        let a = op.operands[0].as_f32().unwrap_or(1.0);
                        let b = op.operands[1].as_f32().unwrap_or(0.0);
                        let c = op.operands[2].as_f32().unwrap_or(0.0);
                        let d = op.operands[3].as_f32().unwrap_or(1.0);
                        let e = op.operands[4].as_f32().unwrap_or(0.0);
                        let f = op.operands[5].as_f32().unwrap_or(0.0);
                        state.text_matrix = [a, b, c, d, e, f];
                        state.text_line_matrix = [a, b, c, d, e, f];
                    }
                }
                "TD" => {
                    if op.operands.len() == 2 {
                        let tx = op.operands[0].as_f32().unwrap_or(0.0);
                        let ty = op.operands[1].as_f32().unwrap_or(0.0);
                        state.leading = -ty;
                        state.text_line_matrix = translate(state.text_line_matrix, tx, ty);
                        state.text_matrix = state.text_line_matrix;
                    }
                }
                "Td" => {
                    if op.operands.len() == 2 {
                        let tx = op.operands[0].as_f32().unwrap_or(0.0);
                        let ty = op.operands[1].as_f32().unwrap_or(0.0);
                        state.text_line_matrix = translate(state.text_line_matrix, tx, ty);
                        state.text_matrix = state.text_line_matrix;
                    }
                }
                "T*" => {
                    let ty = -state.leading;
                    state.text_line_matrix = translate(state.text_line_matrix, 0.0, ty);
                    state.text_matrix = state.text_line_matrix;
                }
                "Tf" => {
                    if op.operands.len() == 2 {
                        if let Ok(name_bytes) = op.operands[0].as_name() {
                            state.current_font = Some(String::from_utf8_lossy(name_bytes).to_string());
                        }
                        state.font_size = op.operands[1].as_f32().unwrap_or(state.font_size);
                    }
                }
                // character spacing Tw (word) and Tc (char) not implemented as they rarely appear directly; pdf.js also handles Th (?) we skip for now
                "Tj" => {
                    if let Some(obj) = op.operands.get(0) {
                        if let Ok(bytes) = obj.as_str() {
                            let raw = bytes.as_ref();
                            let fm = state.current_font.as_ref().and_then(|n| fonts.get(n));
                            let start_matrix = state.text_matrix; // capture position before drawing
                            let mut advance_glyph_space: f32 = 0.0; // in glyph space (1/1000 text units)
                            for &bch in raw {
                                if let Some(fm) = fm { advance_glyph_space += glyph_advance(bch, fm); } else { advance_glyph_space += 500.0; }
                                // Add spacing for space char (0x20)
                                if bch == 0x20 { advance_glyph_space += state.word_spacing * 1000.0 / state.font_size; }
                                advance_glyph_space += state.char_spacing * 1000.0 / state.font_size;
                            }
                            let h_scale = state.horizontal_scaling / 100.0;
                            let advance_text_space = (advance_glyph_space / 1000.0) * state.font_size * h_scale;
                            // Update text matrix translate by advance_text_space
                            state.text_matrix = translate(state.text_matrix, advance_text_space, 0.0);

                            let text_decoded = decode_pdf_bytes(raw);
                            let x1 = start_matrix[4] as i32;
                            let y1 = start_matrix[5] as i32;
                            let x2 = x1 + advance_text_space as i32;
                            let y2 = y1 + state.font_size as i32; // simple baseline + font size box
                            if !text_decoded.is_empty() {
                                text_items.add(&TextItem::new(text_decoded, x1, y1, x2, y2, page_num as usize));
                            }
                        }
                    }
                }
                "TJ" => {
                    if let Some(obj) = op.operands.get(0) {
                        if let Ok(arr) = obj.as_array() {
                            let start_matrix = state.text_matrix;
                            let fm = state.current_font.as_ref().and_then(|n| fonts.get(n));
                            let mut collected = String::new();
                            let mut advance_glyph_space: f32 = 0.0; // glyph units
                            for part in arr {
                                if let Ok(s) = part.as_str() {
                                    let slice = s.as_ref();
                                    for &bch in slice {
                                        if let Some(fm) = fm { advance_glyph_space += glyph_advance(bch, fm); } else { advance_glyph_space += 500.0; }
                                        if bch == 0x20 { advance_glyph_space += state.word_spacing * 1000.0 / state.font_size; }
                                        advance_glyph_space += state.char_spacing * 1000.0 / state.font_size;
                                    }
                                    collected.push_str(&decode_pdf_bytes(slice));
                                } else if let Ok(num) = part.as_f32() {
                                    // adjustment: move current position backwards by (num / 1000 * font_size * h_scale)
                                    advance_glyph_space -= num; // num already in glyph units where pdf.js: spacing -= value
                                    // the direct adjustment is applied by subtracting (num/1000 * font_size * h_scale) from advance
                                }
                            }
                            if !collected.is_empty() {
                                let _h_scale = state.horizontal_scaling / 100.0;
                                let advance_text_space = (advance_glyph_space / 1000.0) * state.font_size * _h_scale;
                                state.text_matrix = translate(state.text_matrix, advance_text_space, 0.0);
                                let x1 = start_matrix[4] as i32;
                                let y1 = start_matrix[5] as i32;
                                let x2 = x1 + advance_text_space as i32;
                                let y2 = y1 + state.font_size as i32;
                                text_items.add(&TextItem::new(collected, x1, y1, x2, y2, page_num as usize));
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        // _page_height currently unused; kept for possible top-left coordinate transform
        let _ = page_height;
    }
    text_items
}
