use std::collections::HashMap;

/// Return a mapping from each field to its immediate next fields across formats.
///
/// Given tokenized formats like [["date","description","amount"], ["description","amount"]],
/// returns a map such as:
///   date -> ["description"],
///   description -> ["amount"],
///   amount -> []
pub fn get_next_fields(transaction_formats: Vec<Vec<String>>) -> HashMap<String, Vec<String>> {
	let mut related: HashMap<String, Vec<String>> = HashMap::new();

	for format in transaction_formats {
		for i in 0..format.len() {
			let current = &format[i];
			let entry = related.entry(current.clone()).or_insert_with(Vec::new);
			// If no next field, continue after ensuring key exists
			if i + 1 > format.len().saturating_sub(1) {
				continue;
			}
			let next = &format[i + 1];
			if !entry.iter().any(|s| s == next) {
				entry.push(next.clone());
			}
		}
	}

	related
}

#[cfg(test)]
mod tests {
	use super::get_next_fields;
	use std::collections::HashMap;

	fn ss(v: &[&str]) -> Vec<String> { v.iter().map(|s| s.to_string()).collect() }

	#[test]
	fn next_fields_basic_chain() {
		let formats = vec![
			ss(&["date","description","amount"]),
			ss(&["description","amount"]),
		];
		let got = get_next_fields(formats);
		let mut expected: HashMap<String, Vec<String>> = HashMap::new();
		expected.insert("date".into(), ss(&["description"]));
		expected.insert("description".into(), ss(&["amount"]));
		expected.insert("amount".into(), vec![]);
		assert_eq!(got.get("date"), expected.get("date"));
		assert_eq!(got.get("description"), expected.get("description"));
		assert_eq!(got.get("amount"), expected.get("amount"));
	}

	#[test]
	fn next_fields_deduplicates() {
		let formats = vec![
			ss(&["date","description","amount"]),
			ss(&["date","description"]),
		];
		let got = get_next_fields(formats);
		assert_eq!(got.get("date").cloned().unwrap_or_default(), ss(&["description"]));
	}

	#[test]
	fn next_fields_empty_input() {
		let formats: Vec<Vec<String>> = vec![];
		let got = get_next_fields(formats);
		assert!(got.is_empty());
	}

	#[test]
	fn next_fields_single_field_only_key_exists() {
		let formats = vec![ ss(&["amount"]) ];
		let got = get_next_fields(formats);
		// Key should exist with empty vec
		assert_eq!(got.get("amount").cloned().unwrap_or_default(), Vec::<String>::new());
	}
}
