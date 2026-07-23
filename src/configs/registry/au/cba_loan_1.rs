use crate::structs::StatementConfig;
use regex::Regex;

pub const KEY: &str = "au__cba__loan__1";

pub fn get_config() -> StatementConfig {
    StatementConfig {
        key: KEY.to_string(),
        bank_name: "Commonwealth Bank of Australia".to_string(),
        account_type: "Loan".to_string(),
        account_terms: vec!["CommBank".to_string(), "Account number".to_string()],
        account_examples: vec!["Complete Home Loan".to_string()],
        fix_text_order: vec![0.0, 0.0],

        account_number_terms: vec!["Account number".to_string()],
        account_number_patterns: vec![Regex::new(r"\b\d+\b").unwrap()],
        account_number_alignment: "y1".to_string(),
        account_number_alignment_tol: 5,

        opening_balance_terms: vec!["Opening balance".to_string(), "Opening Balance".to_string()],
        opening_balance_formats: vec!["format3".to_string(), "format5".to_string()],
        opening_balance_alignment: "y1".to_string(),
        opening_balance_alignment_tol: 5,
        opening_balance_invert: false,

        closing_balance_terms: vec!["Closing balance".to_string()],
        closing_balance_formats: vec!["format3".to_string(), "format5".to_string()],
        closing_balance_alignment: "y1".to_string(),
        closing_balance_alignment_tol: 5,
        closing_balance_invert: false,

        start_date_terms: vec!["Statement period".to_string()],
        start_date_formats: vec!["format2".to_string()],
        start_date_alignment: "y1".to_string(),
        start_date_alignment_tol: 20,

        transaction_terms: vec!["Date Transaction description".to_string()],
        transaction_terms_stop: vec!["Closing balance".to_string()],
        transaction_formats: vec![vec![
            "date".to_string(),
            "description".to_string(),
            "amount".to_string(),
            "balance".to_string(),
        ]],
        transaction_new_line_tol: 5,
        transaction_start_date_required: true,
        transaction_alignment_tol: 10,

        transaction_date_formats: vec!["format1".to_string()],
        transaction_date_headers: vec!["Date".to_string()],
        transaction_date_alignment: "x1".to_string(),

        transaction_description_headers: vec!["Transaction description".to_string()],
        transaction_description_alignment: "x1".to_string(),
        transaction_description_exclude: vec![
            Regex::new(r" -$").unwrap(),
            Regex::new(r" \$$").unwrap(),
        ],

        transaction_amount_formats: vec!["format1".to_string(), "format2".to_string()],
        transaction_amount_headers: vec!["Credits".to_string()],
        transaction_amount_alignment: "x2".to_string(),
        transaction_amount_invert_headers: vec!["Debits".to_string()],
        transaction_amount_invert_alignment: "x2".to_string(),
        transaction_amount_invert: false,

        transaction_balance_formats: vec!["format3".to_string()],
        transaction_balance_headers: vec!["Balance".to_string()],
        transaction_balance_alignment: "x2".to_string(),
        transaction_balance_invert: false,
    }
}
