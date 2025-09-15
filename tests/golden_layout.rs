//! Integration test: verifies the PDF text extraction and layout serialization.
//!
//! This test loads a small sample PDF from `tests/fixtures/test.pdf`,
//! runs the extractor (`extract_text_items`) and converts the result
//! to the serialized layout format via `to_layout_text()`.
//!
//! It then writes the actual output to `target/test-output/test.txt`
//! and compares it byte-for-byte against the golden file at
//! `tests/fixtures/test.txt`. The comparison is exact; any difference
//! (including whitespace or encoding changes) will fail the test and
//! point you to the path of the generated actual file for inspection.
//!
//! Update the golden file only when output changes are intentional.
//! This protects against regressions in formatting, encoding, or
//! coordinate serialization.
use std::fs;
use std::path::PathBuf;

#[test]
fn pdf_layout_matches_golden() {
    // Locate fixtures relative to crate root
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let pdf = root.join("tests/fixtures/test.pdf");
    let expected_txt = root.join("tests/fixtures/test.txt");

    assert!(pdf.exists(), "Missing test PDF: {:?}", pdf);
    assert!(expected_txt.exists(), "Missing expected text: {:?}", expected_txt);

    // Extract layout text using library API
    let items = transtractor::parsers::text_items_from_pdf::extract_text_items(pdf.to_str().unwrap());
    let layout = items.to_layout_text();

    // Write to a temporary output under target/test-output to avoid polluting the repo
    let out_dir = root.join("target/test-output");
    fs::create_dir_all(&out_dir).expect("create test-output dir");
    let actual_path = out_dir.join("test.txt");
    fs::write(&actual_path, layout.0.as_bytes()).expect("write actual layout text");

    // Compare exact bytes with expected
    let expected = fs::read(&expected_txt).expect("read expected text");
    let actual = fs::read(&actual_path).expect("read actual text");

    assert_eq!(actual, expected, "Layout text differs. Actual output at {:?}", actual_path);
}

/// Round-trip test for the layout text format.
///
/// This test reads the golden layout text from `tests/fixtures/test.txt`,
/// parses it into `TextItems` using `TextItems::read_from_layout_text`, and
/// then serializes it back with `to_layout_text()`. The re-serialized output
/// is written to `target/test-output/test_roundtrip.txt` and compared
/// byte-for-byte with the original golden file to ensure the format is parsed
/// and reproduced exactly (including spacing and ordering).
#[test]
fn layout_text_roundtrips_exactly() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let expected_txt = root.join("tests/fixtures/test.txt");
    assert!(expected_txt.exists(), "Missing expected text: {:?}", expected_txt);

    // Read the golden layout text as UTF-8 (generator writes UTF-8)
    let expected_str = fs::read_to_string(&expected_txt).expect("read expected layout text as UTF-8");

    // Parse into TextItems
    let mut items = transtractor::structs::text_items::TextItems::new();
    let layout_in = transtractor::structs::text_items::LayoutText(expected_str.clone());
    items
        .read_from_layout_text(&layout_in)
        .expect("parse layout text");

    // Serialize back to layout text
    let layout_out = items.to_layout_text();

    // Write and compare exact bytes
    let out_dir = root.join("target/test-output");
    fs::create_dir_all(&out_dir).expect("create test-output dir");
    let roundtrip_path = out_dir.join("test_roundtrip.txt");
    fs::write(&roundtrip_path, layout_out.0.as_bytes()).expect("write roundtrip layout text");

    let expected_bytes = fs::read(&expected_txt).expect("read expected bytes");
    let actual_bytes = fs::read(&roundtrip_path).expect("read actual bytes");
    assert_eq!(actual_bytes, expected_bytes, "Round-tripped layout differs. Output at {:?}", roundtrip_path);
}
