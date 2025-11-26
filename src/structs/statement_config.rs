use regex::Regex;
use serde::Deserialize;
use std::fs;
use std::path::Path;

/// Raw struct used only for deserialization (all fields optional so we can overlay defaults)
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct StatementConfigPartial {
    key: Option<String>,
    bank_name: Option<String>,
    account_type: Option<String>,
    account_terms: Option<Vec<String>>,
    account_examples: Option<Vec<String>>,
    apply_y_patch: Option<bool>,

    account_number_terms: Option<Vec<String>>,
    account_number_patterns: Option<Vec<String>>,
    account_number_alignment: Option<String>,
    account_number_alignment_tol: Option<i32>,

    opening_balance_terms: Option<Vec<String>>,
    opening_balance_formats: Option<Vec<String>>,
    opening_balance_alignment: Option<String>,
    opening_balance_alignment_tol: Option<i32>,
    opening_balance_invert: Option<bool>,

    closing_balance_terms: Option<Vec<String>>,
    closing_balance_formats: Option<Vec<String>>,
    closing_balance_alignment: Option<String>,
    closing_balance_alignment_tol: Option<i32>,
    closing_balance_invert: Option<bool>,

    start_date_terms: Option<Vec<String>>,
    start_date_formats: Option<Vec<String>>,
    start_date_alignment: Option<String>,
    start_date_alignment_tol: Option<i32>,

    transaction_terms: Option<Vec<String>>,
    transaction_terms_stop: Option<Vec<String>>,
    transaction_formats: Option<Vec<Vec<String>>>,
    transaction_new_line_tol: Option<i32>,
    transaction_start_date_required: Option<bool>,
    transaction_alignment_tol: Option<i32>,

    transaction_date_formats: Option<Vec<String>>,
    transaction_date_headers: Option<Vec<String>>,
    transaction_date_alignment: Option<String>,

    transaction_description_headers: Option<Vec<String>>,
    transaction_description_alignment: Option<String>,
    transaction_description_exclude: Option<Vec<String>>,

    transaction_amount_formats: Option<Vec<String>>,
    transaction_amount_headers: Option<Vec<String>>,
    transaction_amount_alignment: Option<String>,
    transaction_amount_invert_headers: Option<Vec<String>>,
    transaction_amount_invert_alignment: Option<String>,
    transaction_amount_invert: Option<bool>,

    transaction_balance_formats: Option<Vec<String>>,
    transaction_balance_headers: Option<Vec<String>>,
    transaction_balance_alignment: Option<String>,
    transaction_balance_invert: Option<bool>,
}

/// Configuration for parsing a bank statement layout.
#[derive(Debug, Clone)]
pub struct StatementConfig {
    // BANK & ACCOUNT DETAILS
    /// Unique key identifying this layout configuration.
    ///  2-letter region Code, bank acronym or short name, account type.
    ///  (e.g., AU__CBA__Debit)
    pub key: String,
    /// Full name of the bank (e.g., Commonwealth Bank of Australia)
    pub bank_name: String,
    /// Account type label (e.g., "Debit", "Credit Card")
    pub account_type: String,
    /// A set of terms on the statement that can uniquely identify the layout type.
    pub account_terms: Vec<String>,
    /// Account types that should work with this layout (e.g., "Streamline", "Everyday Offset")
    pub account_examples: Vec<String>,
    /// Force re-ordering of items by Y coordinate to fix minor PDF extraction issues.
    pub apply_y_patch: bool,

    // ACCOUNT NUMBER READ PARAMS
    /// Array of terms to identify the account number line (e.g., "Account Number", "Acct No")
    pub account_number_terms: Vec<String>,
    /// Array of regex patterns to extract the account number
    pub account_number_patterns: Vec<Regex>,
    /// Alignment of the account number relative to the term ("x1", "x2", "y1", "y2", "")
    pub account_number_alignment: String,
    /// Tolerance for alignment matching of account number
    pub account_number_alignment_tol: i32,

    // OPENING BALANCE READ PARAMS
    /// Array of terms to identify the opening balance line (e.g., "Opening Balance", "Previous Balance")
    pub opening_balance_terms: Vec<String>,
    /// Array of accepted formats to parse the opening balance amount
    pub opening_balance_formats: Vec<String>,
    /// Alignment of the opening balance relative to the term ("x1", "x2", "y1", "y2", "")
    pub opening_balance_alignment: String,
    /// Tolerance for alignment matching of opening balance
    pub opening_balance_alignment_tol: i32,
    /// Invert the sign of the opening balance amount
    pub opening_balance_invert: bool,

    // CLOSING BALANCE READ PARAMS
    /// Array of terms to identify the closing balance line (e.g., "Closing Balance", "New Balance")
    pub closing_balance_terms: Vec<String>,
    /// Array of accepted formats to parse the closing balance amount
    pub closing_balance_formats: Vec<String>,
    /// Alignment of the closing balance relative to the term ("x1", "x2", "y1", "y2", "")
    pub closing_balance_alignment: String,
    /// Tolerance for alignment matching of closing balance
    pub closing_balance_alignment_tol: i32,
    /// Invert the sign of the closing balance amount
    pub closing_balance_invert: bool,

    // START DATE READ PARAMS
    /// Array of terms to identify the statement start date line (e.g., "Statement Period", "From")
    pub start_date_terms: Vec<String>,
    /// Array of accepted formats to parse the statement start date
    pub start_date_formats: Vec<String>,
    /// Alignment of the start date relative to the term ("x1", "x2", "y1", "y2", "")
    pub start_date_alignment: String,
    /// Tolerance for alignment matching of start date
    pub start_date_alignment_tol: i32,

    // GENERAL TRANSACTION READ PARAMS
    /// Array of terms that can indicate start, or nearing the start of transaction table
    /// (e.g., "Transactions").
    pub transaction_terms: Vec<String>,
    /// Array of terms that indicate the end, or close after the end of the transaction table.
    pub transaction_terms_stop: Vec<String>,
    /// Fields expected for a complete transaction line, in order.
    /// E.g., [["date", "description", "amount"], ["description", "amount"]]
    /// Is a common format for credit card statements where the date is only specified
    /// on the first transaction of each day.
    pub transaction_formats: Vec<Vec<String>>,
    /// Y-coordinate tolerance to identify a new line in the transaction list
    pub transaction_new_line_tol: i32,
    /// Parsing transaction requires the start date value to have been read
    /// so that the year can be inferred for each transaction date.
    pub transaction_start_date_required: bool,
    /// Tolerance for X alignment mismatch between value and header
    pub transaction_alignment_tol: i32,

    // TRANSACTION DATE READ PARAMS
    /// Array of accepted formats to parse the transaction date
    pub transaction_date_formats: Vec<String>,
    /// Headers that identify the transaction date column
    pub transaction_date_headers: Vec<String>,
    /// Alignment of the transaction date column ("x1, "x2")
    pub transaction_date_alignment: String,

    // TRANSACTION DESCRIPTION READ PARAMS
    /// Headers that identify the transaction description column
    pub transaction_description_headers: Vec<String>,
    /// Alignment of the transaction description column ("x1, "x2")
    pub transaction_description_alignment: String,
    /// Regex patterns to exclude from being considered as part of the description.
    /// E.g., [/\.\./g] to exclude "......." patterns.
    pub transaction_description_exclude: Vec<Regex>,

    // TRANSACTION AMOUNT READ PARAMS
    /// Array of accepted formats to parse the transaction amount
    pub transaction_amount_formats: Vec<String>,
    /// Headers that identify the transaction amount column
    pub transaction_amount_headers: Vec<String>,
    /// Alignment of the transaction amount column ("x1, "x2")
    pub transaction_amount_alignment: String,
    /// Headers that identify when to invert the transaction amount sign
    pub transaction_amount_invert_headers: Vec<String>,
    /// Alignment of the transaction amount invert column ("x1, "x2")
    pub transaction_amount_invert_alignment: String,
    /// Invert the sign of all transaction amounts. Often needed for credit card statements.
    pub transaction_amount_invert: bool,

    // TRANSACTION BALANCE READ PARAMS
    /// Array of accepted formats to parse the transaction balance amount
    pub transaction_balance_formats: Vec<String>,
    /// Headers that identify the transaction balance column
    pub transaction_balance_headers: Vec<String>,
    /// Alignment of the transaction balance column ("x1, "x2")
    pub transaction_balance_alignment: String,
    /// Invert the sign of all transaction balance amounts.
    pub transaction_balance_invert: bool,
}

impl Default for StatementConfig {
    fn default() -> Self {
        StatementConfig {
            key: "Generic Statement".to_string(),
            bank_name: "Generic Bank".to_string(),
            account_type: "Generic Account".to_string(),
            account_terms: vec![],
            account_examples: vec![],
            apply_y_patch: false,

            account_number_terms: vec![],
            account_number_patterns: vec![],
            account_number_alignment: "y1".to_string(),
            account_number_alignment_tol: 5,

            opening_balance_terms: vec![],
            opening_balance_formats: vec![],
            opening_balance_alignment: "y1".to_string(),
            opening_balance_alignment_tol: 1,
            opening_balance_invert: false,

            closing_balance_terms: vec![],
            closing_balance_formats: vec![],
            closing_balance_alignment: "y1".to_string(),
            closing_balance_alignment_tol: 1,
            closing_balance_invert: false,

            start_date_terms: vec![],
            start_date_formats: vec![],
            start_date_alignment: "y1".to_string(),
            start_date_alignment_tol: 1,

            transaction_terms: vec![],
            transaction_terms_stop: vec![],
            transaction_formats: vec![],
            transaction_new_line_tol: 2,
            transaction_start_date_required: false,
            transaction_alignment_tol: 20,

            transaction_date_formats: vec![],
            transaction_date_headers: vec![],
            transaction_date_alignment: "x1".to_string(),

            transaction_description_headers: vec![],
            transaction_description_alignment: "x1".to_string(),
            transaction_description_exclude: vec![],

            transaction_amount_formats: vec![],
            transaction_amount_headers: vec![],
            transaction_amount_alignment: "x1".to_string(),
            transaction_amount_invert_headers: vec![],
            transaction_amount_invert_alignment: "x1".to_string(),
            transaction_amount_invert: false,

            transaction_balance_formats: vec![],
            transaction_balance_headers: vec![],
            transaction_balance_alignment: "x1".to_string(),
            transaction_balance_invert: false,
        }
    }
}

impl StatementConfig {
    /// Load a StatementConfig from a JSON file, overlaying onto defaults and validating fields.
    pub fn from_json_file<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let data = fs::read_to_string(&path)
            .map_err(|e| format!("Failed reading config {:?}: {}", path.as_ref(), e))?;
        Self::from_json_str(&data)
    }

    /// Load from a JSON &str.
    pub fn from_json_str(src: &str) -> Result<Self, String> {
        let partial: StatementConfigPartial = serde_json::from_str(src)
            .map_err(|e| format!("JSON parse error: {}", e))?;
        let mut cfg = StatementConfig::default();

        macro_rules! overlay { ($field:ident) => { if let Some(v) = partial.$field { cfg.$field = v; } }; }

        overlay!(key);
        overlay!(bank_name);
        overlay!(account_type);
        overlay!(account_terms);
        overlay!(account_examples);
        overlay!(apply_y_patch);

        overlay!(account_number_terms);
        if let Some(patterns) = partial.account_number_patterns {
            cfg.account_number_patterns = compile_regex_vec(patterns)?;
        }
        overlay!(account_number_alignment);
        overlay!(account_number_alignment_tol);

        overlay!(opening_balance_terms);
        overlay!(opening_balance_formats);
        overlay!(opening_balance_alignment);
        overlay!(opening_balance_alignment_tol);
        overlay!(opening_balance_invert);

        overlay!(closing_balance_terms);
        overlay!(closing_balance_formats);
        overlay!(closing_balance_alignment);
        overlay!(closing_balance_alignment_tol);
        overlay!(closing_balance_invert);

        overlay!(start_date_terms);
        overlay!(start_date_formats);
        overlay!(start_date_alignment);
        overlay!(start_date_alignment_tol);

        overlay!(transaction_terms);
        overlay!(transaction_terms_stop);
        overlay!(transaction_formats);
        overlay!(transaction_new_line_tol);
        overlay!(transaction_start_date_required);
        overlay!(transaction_alignment_tol);

        overlay!(transaction_date_formats);
        overlay!(transaction_date_headers);
        overlay!(transaction_date_alignment);

        overlay!(transaction_description_headers);
        overlay!(transaction_description_alignment);

        if let Some(ex_patterns) = partial.transaction_description_exclude {
            cfg.transaction_description_exclude = compile_regex_vec(ex_patterns)?;
        }

        overlay!(transaction_amount_formats);
        overlay!(transaction_amount_headers);
        overlay!(transaction_amount_alignment);
        overlay!(transaction_amount_invert_headers);
        overlay!(transaction_amount_invert_alignment);
        overlay!(transaction_amount_invert);

        overlay!(transaction_balance_formats);
        overlay!(transaction_balance_headers);
        overlay!(transaction_balance_alignment);
        overlay!(transaction_balance_invert);

        cfg.validate()?;
        Ok(cfg)
    }

    /// Validate invariants and value ranges.
    pub fn validate(&self) -> Result<(), String> {
        if self.key.trim().is_empty() { return Err("key cannot be empty".into()); }
        if self.bank_name.trim().is_empty() { return Err("bank_name cannot be empty".into()); }
        if self.account_type.trim().is_empty() { return Err("account_type cannot be empty".into()); }

        // Header alignment sanity
        fn valid_alignment(alignment: &str, name: &str) -> Result<(), String> {
            let valid_alignments = ["x1", "x2"];
            if !valid_alignments.contains(&alignment) {
                return Err(format!("{} must be one of {:?}", name, valid_alignments));
            }
            Ok(())
        }
        valid_alignment(&self.transaction_date_alignment, "transaction_date_alignment")?;
        valid_alignment(&self.transaction_description_alignment, "transaction_description_alignment")?;
        valid_alignment(&self.transaction_amount_alignment, "transaction_amount_alignment")?;
        valid_alignment(&self.transaction_amount_invert_alignment, "transaction_amount_invert_alignment")?;
        valid_alignment(&self.transaction_balance_alignment, "transaction_balance_alignment")?;

        // Other alignment sanity
        fn valid_full_alignment(alignment: &str, name: &str) -> Result<(), String> {
            let valid_alignments = ["x1", "x2", "y1", "y2", ""];
            if !valid_alignments.contains(&alignment) {
                return Err(format!("{} must be one of {:?}", name, valid_alignments));
            }
            Ok(())
        }
        valid_full_alignment(&self.account_number_alignment, "account_number_alignment")?;
        valid_full_alignment(&self.opening_balance_alignment, "opening_balance_alignment")?;
        valid_full_alignment(&self.closing_balance_alignment, "closing_balance_alignment")?;
        valid_full_alignment(&self.start_date_alignment, "start_date_alignment")?;

        // Basic tolerance sanity
        fn non_negative(val: i32, name:&str) -> Result<(),String> {
            if val < 0 { return Err(format!("{} must be >= 0", name)); }
            Ok(())
        }
        non_negative(self.account_number_alignment_tol, "account_number_alignment_tol")?;
        non_negative(self.opening_balance_alignment_tol, "opening_balance_alignment_tol")?;
        non_negative(self.closing_balance_alignment_tol, "closing_balance_alignment_tol")?;
        non_negative(self.start_date_alignment_tol, "start_date_alignment_tol")?;
        non_negative(self.transaction_alignment_tol, "`transaction_alignment_tol`")?;
        non_negative(self.transaction_new_line_tol, "transaction_new_line_tol")?;

        // Formats sanity (simple: strings non-empty)
        for f in &self.opening_balance_formats { if f.trim().is_empty() { return Err("Empty opening_balance_formats entry".into()); } }
        for f in &self.closing_balance_formats { if f.trim().is_empty() { return Err("Empty closing_balance_formats entry".into()); } }
        for f in &self.start_date_formats { if f.trim().is_empty() { return Err("Empty start_date_formats entry".into()); } }
        for f in &self.transaction_date_formats { if f.trim().is_empty() { return Err("Empty transaction_date_formats entry".into()); } }
        for f in &self.transaction_amount_formats { if f.trim().is_empty() { return Err("Empty transaction_amount_formats entry".into()); } }
        for f in &self.transaction_balance_formats { if f.trim().is_empty() { return Err("Empty transaction_balance_formats entry".into()); } }

        // Regex patterns sanity (non-empty) for account number
        if self.account_number_patterns.is_empty() {
            return Err("account_number_patterns cannot be empty".into());
        }

        // Transaction formats: each non-empty vector with allowed tokens (light validation)
        let allowed_tokens = ["date", "description", "amount", "balance"]; // extend as needed
        for fmt in &self.transaction_formats {
            if fmt.is_empty() { return Err("transaction_formats entry cannot be empty".into()); }
            for token in fmt {
                if !allowed_tokens.iter().any(|a| a == token) {
                    return Err(format!("Unknown token '{}' in transaction_formats", token));
                }
            }
        }

        Ok(())
    }
}

fn compile_regex_vec(patterns: Vec<String>) -> Result<Vec<Regex>, String> {
    let mut result = Vec::with_capacity(patterns.len());
    for p in patterns {
        match Regex::new(&p) {
            Ok(r) => result.push(r),
            Err(e) => return Err(format!("Invalid regex '{}': {}", p, e)),
        }
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use std::fs::File;
    use std::env;

    fn write_temp(name: &str, contents: &str) -> std::path::PathBuf {
        let mut path = env::temp_dir();
        path.push(format!("{}_{}.json", name, uuid::Uuid::new_v4()));
        let mut f = File::create(&path).expect("create temp file");
        f.write_all(contents.as_bytes()).expect("write temp file");
        path
    }

    #[test]
    fn test_from_json_file_success_minimal() {
        let json = r#"{
            "key": "au__test__acct",
            "bank_name": "Test Bank",
            "account_type": "Test Account",
            "account_number_patterns": ["\\b\\d+\\b"],
            "opening_balance_terms": ["Opening balance"],
            "opening_balance_formats": ["format2"],
            "transaction_amount_formats": ["format1"]
        }"#;
        let path = write_temp("cfg_success", json);
        let cfg = StatementConfig::from_json_file(&path).expect("load config");
        assert_eq!(cfg.key, "au__test__acct");
        assert_eq!(cfg.bank_name, "Test Bank");
        assert!(cfg.opening_balance_terms.contains(&"Opening balance".to_string()));
        assert_eq!(cfg.transaction_amount_formats, vec!["format1".to_string()]);
    }

    #[test]
    fn test_from_json_file_overlay_defaults() {
        let json = r#"{
            "key": "k",
            "bank_name": "B",
            "account_type": "T",
            "account_number_patterns": ["\\b\\d+\\b"]
        }"#;
        let path = write_temp("cfg_defaults", json);
        let cfg = StatementConfig::from_json_file(&path).expect("load config");
        assert_eq!(cfg.key, "k");
        assert_eq!(cfg.bank_name, "B");
        assert_eq!(cfg.account_type, "T");
        // Defaults
        assert_eq!(cfg.opening_balance_invert, false);
        assert_eq!(cfg.transaction_start_date_required, false);
    }

    #[test]
    fn test_from_json_file_invalid_regex() {
        let json = r#"{
            "key": "k",
            "bank_name": "B",
            "account_type": "T",
            "transaction_description_exclude": ["("]
        }"#;
        let path = write_temp("cfg_bad_regex", json);
        let err = StatementConfig::from_json_file(&path).unwrap_err();
        assert!(err.contains("Invalid regex"));
    }

    #[test]
    fn test_from_json_file_unknown_field() {
        let json = r#"{
            "key": "k",
            "bank_name": "B",
            "account_type": "T",
            "unknown_field": 123
        }"#;
        let path = write_temp("cfg_unknown", json);
        let err = StatementConfig::from_json_file(&path).unwrap_err();
        assert!(err.contains("unknown field"));
    }
}