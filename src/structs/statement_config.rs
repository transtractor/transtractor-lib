/// Configuration for parsing a bank statement layout.
#[derive(Debug, Clone)]
pub struct StatementConfig {
    pub key: String,
    pub bank_name: String,
    pub account_type: String,
    pub account_terms: Vec<String>,
    pub account_examples: Vec<String>,
    pub apply_y_patch: bool,
    pub apply_y_patch_line_height: f64,

    // Opening balance read config
    pub opening_balance_terms: Vec<String>,
    pub opening_balance_formats: Vec<String>,
    pub opening_balance_same_x: bool,
    pub opening_balance_x_tol: f64,
    pub opening_balance_same_y: bool,
    pub opening_balance_y_tol: f64,
    pub opening_balance_invert: bool,

    // Closing balance read config
    pub closing_balance_terms: Vec<String>,
    pub closing_balance_formats: Vec<String>,
    pub closing_balance_same_x: bool,
    pub closing_balance_x_tol: f64,
    pub closing_balance_same_y: bool,
    pub closing_balance_y_tol: f64,
    pub closing_balance_invert: bool,

    // Start date read config
    pub start_date_terms: Vec<String>,
    pub start_date_formats: Vec<String>,
    pub start_date_same_x: bool,
    pub start_date_x_tol: f64,
    pub start_date_same_y: bool,
    pub start_date_y_tol: f64,

    // Transaction read config
    pub transaction_terms: Vec<String>,
    pub transaction_terms_stop: Vec<String>,
    pub transaction_formats: Vec<String>,
    pub transaction_new_line_y_tol: f64,
    pub transaction_start_date_required: bool,

    // Transaction date read config
    pub transaction_date_formats: Vec<String>,
    pub transaction_date_x1_range: (f64, f64),
    pub transaction_date_x2_range: (f64, f64),
    pub transaction_date_header_terms: Vec<String>,
    pub transaction_date_header_align: String,

    // Transaction Description read config
    pub transaction_description_x1_range: (f64, f64),
    pub transaction_description_x2_range: (f64, f64),
    pub transaction_description_excluded: Vec<String>, // RegExp not directly supported; use strings or regex crate

    // Transaction Amount read config
    pub transaction_amount_formats: Vec<String>,
    pub transaction_amount_x1_range: (f64, f64),
    pub transaction_amount_x2_range: (f64, f64),
    pub transaction_amount_header_terms: Vec<String>,
    pub transaction_amount_header_align: String,
    pub transaction_amount_invert_x1_range: (f64, f64),
    pub transaction_amount_invert_x2_range: (f64, f64),
    pub transaction_amount_invert: bool,
    pub transaction_amount_invert_header_terms: Vec<String>,
    pub transaction_amount_invert_header_align: String,

    // Transaction Balance read config
    pub transaction_balance_formats: Vec<String>,
    pub transaction_balance_x1_range: (f64, f64),
    pub transaction_balance_x2_range: (f64, f64),
    pub transaction_balance_invert: bool,
    pub transaction_balance_header_terms: Vec<String>,
    pub transaction_balance_header_align: String,
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
            apply_y_patch_line_height: 5.0,

            opening_balance_terms: vec![],
            opening_balance_formats: vec![],
            opening_balance_same_x: false,
            opening_balance_x_tol: 0.1,
            opening_balance_same_y: true,
            opening_balance_y_tol: 0.1,
            opening_balance_invert: false,

            closing_balance_terms: vec![],
            closing_balance_formats: vec![],
            closing_balance_same_x: false,
            closing_balance_x_tol: 0.1,
            closing_balance_same_y: true,
            closing_balance_y_tol: 0.1,
            closing_balance_invert: false,

            start_date_terms: vec![],
            start_date_formats: vec![],
            start_date_same_x: false,
            start_date_x_tol: 0.1,
            start_date_same_y: true,
            start_date_y_tol: 0.1,

            transaction_terms: vec![],
            transaction_terms_stop: vec![],
            transaction_formats: vec![],
            transaction_new_line_y_tol: 2.0,
            transaction_start_date_required: false,

            transaction_date_formats: vec![],
            transaction_date_x1_range: (-10000.0, 10000.0),
            transaction_date_x2_range: (-10000.0, 10000.0),
            transaction_date_header_terms: vec![],
            transaction_date_header_align: "x".to_string(),

            transaction_description_x1_range: (-10000.0, 10000.0),
            transaction_description_x2_range: (-10000.0, 10000.0),
            transaction_description_excluded: vec![],

            transaction_amount_formats: vec![],
            transaction_amount_x1_range: (-10000.0, 10000.0),
            transaction_amount_x2_range: (-10000.0, 10000.0),
            transaction_amount_header_terms: vec![],
            transaction_amount_header_align: "x".to_string(),
            transaction_amount_invert_x1_range: (-10000.0, -10000.0),
            transaction_amount_invert_x2_range: (-10000.0, -10000.0),
            transaction_amount_invert: false,
            transaction_amount_invert_header_terms: vec![],
            transaction_amount_invert_header_align: "x".to_string(),

            transaction_balance_formats: vec![],
            transaction_balance_x1_range: (-10000.0, 10000.0),
            transaction_balance_x2_range: (-10000.0, 10000.0),
            transaction_balance_invert: false,
            transaction_balance_header_terms: vec![],
            transaction_balance_header_align: "x".to_string(),
        }
    }
}