use std::collections::HashSet;

/// Returns the list of fields ending multiple transaction formats.
///
/// Example:
///   [["date","description","amount"], ["description","amount", "balance"]] -> ["amount", "balance"]
pub fn get_end_line_fields(transaction_formats: Vec<Vec<String>>) -> Vec<String> {
	let mut seen: HashSet<String> = HashSet::new();
	let mut new_line_fields: Vec<String> = Vec::new();

	for format in transaction_formats {
		if let Some(last) = format.last() {
			if !seen.contains(last) {
				seen.insert(last.clone());
				new_line_fields.push(last.clone());
			}
		}
	}

	new_line_fields
}

#[cfg(test)]
mod tests {
	use super::get_end_line_fields;

	fn ss(v: &[&str]) -> Vec<String> { v.iter().map(|s| s.to_string()).collect() }

	#[test]
	fn end_line_fields_distinct_last_tokens() {
		let formats = vec![
			ss(&["date", "description", "amount"]),
			ss(&["description", "amount", "balance"]),
		];
		let got = get_end_line_fields(formats);
		assert_eq!(got, ss(&["amount", "balance"]));
	}

	#[test]
	fn end_line_fields_duplicate_last_token_collapses() {
		let formats = vec![
			ss(&["date", "description", "amount"]),
			ss(&["description", "amount"]),
			ss(&["amount"]),
		];
		let got = get_end_line_fields(formats);
		assert_eq!(got, ss(&["amount"]));
	}

	#[test]
	fn end_line_fields_empty_input_is_empty() {
		let formats: Vec<Vec<String>> = vec![];
		let got = get_end_line_fields(formats);
		assert!(got.is_empty());
	}
}