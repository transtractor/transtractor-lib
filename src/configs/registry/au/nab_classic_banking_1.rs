use crate::structs::StatementConfig;
use regex::Regex;

pub const KEY: &str = "au__nab__classic_banking__1";

pub fn get_config() -> StatementConfig {
    StatementConfig {
        key: KEY.to_string(),
        bank_name: "National Australia Bank".to_string(),
        account_type: "Savings".to_string(),
        account_terms: vec!["NAB".to_string(), "Classic Banking".to_string()],
        account_examples: vec!["Classic Banking".to_string()],
        fix_text_order: vec![5.0, 2.0],

        account_number_terms: vec!["Account number".to_string()],
        account_number_patterns: vec![Regex::new(r"\b\d+-\d+-\d+\b").unwrap()],
        account_number_alignment: "y1".to_string(),
        account_number_alignment_tol: 5,

        opening_balance_terms: vec!["Opening balance".to_string()],
        opening_balance_formats: vec!["format3".to_string()],
        opening_balance_alignment: "y1".to_string(),
        opening_balance_alignment_tol: 5,
        opening_balance_invert: false,

        closing_balance_terms: vec!["Closing balance".to_string()],
        closing_balance_formats: vec!["format3".to_string()],
        closing_balance_alignment: "y1".to_string(),
        closing_balance_alignment_tol: 5,
        closing_balance_invert: false,

        start_date_terms: vec!["Statement starts".to_string()],
        start_date_formats: vec!["format2".to_string()],
        start_date_alignment: "y1".to_string(),
        start_date_alignment_tol: 5,

        transaction_terms: vec!["Transaction Details".to_string()],
        transaction_terms_stop: vec![],
        transaction_formats: vec![
            vec![
                "date".to_string(),
                "description".to_string(),
                "amount".to_string(),
                "balance".to_string(),
            ],
            vec![
                "date".to_string(),
                "description".to_string(),
                "amount".to_string(),
            ],
            vec![
                "description".to_string(),
                "amount".to_string(),
                "balance".to_string(),
            ],
            vec!["description".to_string(), "amount".to_string()],
        ],
        transaction_new_line_tol: 5,
        transaction_start_date_required: false,
        transaction_alignment_tol: 10,

        transaction_date_formats: vec!["format2".to_string()],
        transaction_date_headers: vec!["Date".to_string()],
        transaction_date_alignment: "x1".to_string(),

        transaction_description_headers: vec!["Particulars".to_string()],
        transaction_description_alignment: "x1".to_string(),
        transaction_description_exclude: vec![
            Regex::new(r"\.\.").unwrap(),
            Regex::new(r"Carried forward").unwrap(),
            Regex::new(r"Brought forward").unwrap(),
            Regex::new(r"Particulars").unwrap(),
            Regex::new(r"\.$").unwrap(),
            Regex::new(r"  NAB Classic Banking.*?Debits ").unwrap(),
            Regex::new(r"  National Australia Bank.*?Debits ").unwrap(),
        ],

        transaction_amount_formats: vec!["format1".to_string()],
        transaction_amount_headers: vec!["Credits".to_string()],
        transaction_amount_alignment: "x2".to_string(),
        transaction_amount_invert_headers: vec!["Debits".to_string()],
        transaction_amount_invert_alignment: "x2".to_string(),
        transaction_amount_invert: false,

        transaction_balance_formats: vec!["format4".to_string()],
        transaction_balance_headers: vec!["Balance".to_string()],
        transaction_balance_alignment: "x2".to_string(),
        transaction_balance_invert: false,
    }
}
