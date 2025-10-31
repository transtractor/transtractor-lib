use std::collections::HashSet;

/// Returns all unique fields that appear in any of the transaction formats.
///
/// Given multiple transaction formats, this function collects all field names
/// that appear in any format and returns them as a sorted vector of unique strings.
///
/// Example:
///   [["date","description"], ["amount","balance"], ["date","amount"]] -> ["amount","balance","date","description"]
pub fn get_all_fields(transaction_formats: Vec<Vec<String>>) -> Vec<String> {
    let mut all_fields: HashSet<String> = HashSet::new();
    
    // Collect all unique fields from all formats
    for format in transaction_formats {
        for field in format {
            all_fields.insert(field);
        }
    }
    
    // Convert to sorted vector for consistent output
    let mut result: Vec<String> = all_fields.into_iter().collect();
    result.sort();
    result
}

#[cfg(test)]
mod tests {
    use super::get_all_fields;

    fn ss(v: &[&str]) -> Vec<String> { v.iter().map(|s| s.to_string()).collect() }

    #[test]
    fn test_get_all_fields_basic_union() {
        let formats = vec![
            ss(&["date", "description"]),
            ss(&["amount", "balance"]),
        ];
        let got = get_all_fields(formats);
        assert_eq!(got, ss(&["amount", "balance", "date", "description"]));
    }

    #[test]
    fn test_get_all_fields_empty_input() {
        let formats: Vec<Vec<String>> = vec![];
        let got = get_all_fields(formats);
        assert!(got.is_empty());
    }

    #[test]
    fn test_get_all_fields_single_format() {
        let formats = vec![
            ss(&["date", "description", "amount"]),
        ];
        let got = get_all_fields(formats);
        assert_eq!(got, ss(&["amount", "date", "description"]));
    }

    #[test]
    fn test_get_all_fields_overlapping_formats() {
        let formats = vec![
            ss(&["date", "description", "amount"]),
            ss(&["description", "amount", "balance"]),
            ss(&["date", "balance"]),
        ];
        let got = get_all_fields(formats);
        assert_eq!(got, ss(&["amount", "balance", "date", "description"]));
    }

    #[test]
    fn test_get_all_fields_duplicate_fields_in_format() {
        let formats = vec![
            ss(&["date", "date", "description"]), // duplicate "date"
            ss(&["amount", "amount", "balance"]),   // duplicate "amount"
        ];
        let got = get_all_fields(formats);
        assert_eq!(got, ss(&["amount", "balance", "date", "description"]));
    }

    #[test]
    fn test_get_all_fields_identical_formats() {
        let formats = vec![
            ss(&["date", "description", "amount"]),
            ss(&["date", "description", "amount"]),
            ss(&["date", "description", "amount"]),
        ];
        let got = get_all_fields(formats);
        assert_eq!(got, ss(&["amount", "date", "description"]));
    }

    #[test]
    fn test_get_all_fields_single_field_formats() {
        let formats = vec![
            ss(&["date"]),
            ss(&["amount"]),
            ss(&["balance"]),
            ss(&["description"]),
        ];
        let got = get_all_fields(formats);
        assert_eq!(got, ss(&["amount", "balance", "date", "description"]));
    }

    #[test]
    fn test_get_all_fields_empty_format() {
        let formats = vec![
            ss(&["date", "description"]),
            ss(&[]), // empty format
            ss(&["amount", "balance"]),
        ];
        let got = get_all_fields(formats);
        assert_eq!(got, ss(&["amount", "balance", "date", "description"]));
    }

    #[test]
    fn test_get_all_fields_custom_field_names() {
        let formats = vec![
            ss(&["transaction_id", "vendor_name"]),
            ss(&["purchase_date", "tax_amount"]),
            ss(&["transaction_id", "purchase_date"]),
        ];
        let got = get_all_fields(formats);
        assert_eq!(got, ss(&["purchase_date", "tax_amount", "transaction_id", "vendor_name"]));
    }

    #[test]
    fn test_get_all_fields_output_is_sorted() {
        let formats = vec![
            ss(&["zebra", "alpha", "beta"]),
            ss(&["gamma", "delta"]),
        ];
        let got = get_all_fields(formats);
        assert_eq!(got, ss(&["alpha", "beta", "delta", "gamma", "zebra"]));
    }
}
