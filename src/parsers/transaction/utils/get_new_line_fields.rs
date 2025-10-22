use std::collections::HashSet;

/// Returns the list of fields expected at the start of a transaction line across formats.
///
/// Example:
///   [["date","description","amount"], ["description","amount"]] -> ["date", "description"]
pub fn get_new_line_fields(transaction_formats: Vec<Vec<String>>) -> Vec<String> {
	// Preserve insertion order like JS Set
	let mut seen: HashSet<String> = HashSet::new();
	let mut start_line_fields: Vec<String> = Vec::new();

	for format in transaction_formats {
		if let Some(first) = format.first() {
			if !seen.contains(first) {
				seen.insert(first.clone());
				start_line_fields.push(first.clone());
			}
		}
	}

	start_line_fields
}

#[cfg(test)]
mod tests {
	use super::get_new_line_fields;

	fn ss(v: &[&str]) -> Vec<String> { v.iter().map(|s| s.to_string()).collect() }

	#[test]
	fn new_line_fields_distinct_first_tokens() {
		let formats = vec![
			ss(&["date", "description", "amount"]),
			ss(&["description", "amount"]),
		];
		let got = get_new_line_fields(formats);
		assert_eq!(got, ss(&["date", "description"]));
	}

	#[test]
	fn new_line_fields_duplicate_first_token_collapses() {
		let formats = vec![
			ss(&["date", "description", "amount"]),
			ss(&["date", "amount"]),
			ss(&["date"]),
		];
		let got = get_new_line_fields(formats);
		assert_eq!(got, ss(&["date"]));
	}

	#[test]
	fn new_line_fields_empty_input_is_empty() {
		let formats: Vec<Vec<String>> = vec![];
		let got = get_new_line_fields(formats);
		assert!(got.is_empty());
	}
}
