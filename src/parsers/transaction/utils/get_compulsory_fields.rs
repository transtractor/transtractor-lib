use std::collections::HashSet;

/// Returns the list of compulsory fields across multiple transaction formats.
///
/// Given multiple transaction formats, a field is compulsory if it appears in all formats.
/// Uses a fixed universe of fields: ["date", "description", "amount", "balance"].
///
/// Example:
///   [["date","description","amount"], ["description","amount"]] -> ["description","amount"]
pub fn get_compulsory_fields(transaction_formats: Vec<Vec<String>>) -> Vec<String> {
    // Fixed list of all possible fields
    let all_fields: [&str; 4] = ["date", "description", "amount", "balance"];

    // Fields absent from any format are optional
    let mut optional_fields: HashSet<&str> = HashSet::new();
    for field in &all_fields {
        for format in &transaction_formats {
            // If a particular format does not include the field, it's optional
            let present = format.iter().any(|f| f == *field);
            if !present {
                optional_fields.insert(*field);
            }
        }
    }

    // Compulsory fields are those not marked optional
    let mut compulsory_fields: Vec<String> = Vec::new();
    for field in &all_fields {
        if !optional_fields.contains(field) {
            compulsory_fields.push((*field).to_string());
        }
    }
    compulsory_fields
}

#[cfg(test)]
mod tests {
    use super::get_compulsory_fields;

    fn ss(v: &[&str]) -> Vec<String> { v.iter().map(|s| s.to_string()).collect() }

    #[test]
    fn compulsory_fields_basic_intersection() {
        let formats = vec![
            ss(&["date", "description", "amount"]),
            ss(&["description", "amount"]),
        ];
        let got = get_compulsory_fields(formats);
        assert_eq!(got, ss(&["description", "amount"]));
    }

    #[test]
    fn compulsory_fields_empty_input_yields_all() {
        let formats: Vec<Vec<String>> = vec![];
        let got = get_compulsory_fields(formats);
        assert_eq!(got, ss(&["date", "description", "amount", "balance"]));
    }

    #[test]
    fn compulsory_fields_all_present_in_all_formats() {
        let formats = vec![
            ss(&["date", "description", "amount", "balance"]),
            ss(&["date", "description", "amount", "balance"]),
        ];
        let got = get_compulsory_fields(formats);
        assert_eq!(got, ss(&["date", "description", "amount", "balance"]));
    }

    #[test]
    fn compulsory_fields_disjoint_formats_yield_empty() {
        let formats = vec![
            ss(&["date"]),
            ss(&["amount"]),
            ss(&["balance"]),
        ];
        let got = get_compulsory_fields(formats);
        assert!(got.is_empty());
    }
}