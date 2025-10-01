use regex::Regex;

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
    /// Line height tolerance for Y patching (default 5)
    pub apply_y_patch_line_height: i32,

    // OPENING BALANCE READ PARAMS
    /// Array of terms to identify the opening balance line (e.g., "Opening Balance", "Previous Balance")
    pub opening_balance_terms: Vec<String>,
    /// Array of accepted formats to parse the opening balance amount
    pub opening_balance_formats: Vec<String>,
    /// Require opening balance to be on the same X1 coordinate as the term
    pub opening_balance_same_x1: bool,
    /// Tolerance for X1 coordinate matching of opening balance
    pub opening_balance_x1_tol: i32,
    /// Require opening balance to be on the same Y1 coordinate as the term
    pub opening_balance_same_y1: bool,
    /// Tolerance for Y1 coordinate matching of opening balance
    pub opening_balance_y1_tol: i32,
    /// Invert the sign of the opening balance amount
    pub opening_balance_invert: bool,

    // CLOSING BALANCE READ PARAMS
    /// Array of terms to identify the closing balance line (e.g., "Closing Balance", "New Balance")
    pub closing_balance_terms: Vec<String>,
    /// Array of accepted formats to parse the closing balance amount
    pub closing_balance_formats: Vec<String>,
    /// Require closing balance to be on the same X1 coordinate as the term
    pub closing_balance_same_x1: bool,
    /// Tolerance for X1 coordinate matching of closing balance
    pub closing_balance_x1_tol: i32,
    /// Require closing balance to be on the same Y1 coordinate as the term
    pub closing_balance_same_y1: bool,
    /// Tolerance for Y1 coordinate matching of closing balance
    pub closing_balance_y1_tol: i32,
    /// Invert the sign of the closing balance amount
    pub closing_balance_invert: bool,

    // START DATE READ PARAMS
    /// Array of terms to identify the statement start date line (e.g., "Statement Period", "From")
    pub start_date_terms: Vec<String>,
    /// Array of accepted formats to parse the statement start date
    pub start_date_formats: Vec<String>,
    /// Require start date to be on the same X1 coordinate as the term
    pub start_date_same_x1: bool,
    /// Tolerance for X1 coordinate matching of start date
    pub start_date_x1_tol: i32,
    /// Require start date to be on the same Y1 coordinate as the term
    pub start_date_same_y1: bool,
    /// Tolerance for Y1 coordinate matching of start date
    pub start_date_y1_tol: i32,

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
    pub transaction_new_line_y1_tol: i32,
    /// Parsing transaction requires the start date value to have been read
    /// so that the year can be inferred for each transaction date.
    pub transaction_start_date_required: bool,

    // TRANSACTION DATE READ PARAMS
    /// Array of accepted formats to parse the transaction date
    pub transaction_date_formats: Vec<String>,
    /// X1 coordinate range to identify the transaction date
    pub transaction_date_x1_range: (i32, i32),
    /// X2 coordinate range to identify the transaction date
    pub transaction_date_x2_range: (i32, i32),
    /// Terms that can identify the transaction date column header.
    /// Specifying this overrides the X1/X2 range.
    pub transaction_date_header_terms: Vec<String>,
    /// Coordinate values are aligned with header by "x1", "x2"
    pub transaction_date_header_align: String,

    // TRANSACTION DESCRIPTION READ PARAMS
    /// X1 coordinate range to identify the transaction description
    pub transaction_description_x1_range: (i32, i32),
    /// X2 coordinate range to identify the transaction description
    pub transaction_description_x2_range: (i32, i32),
    /// Terms that can identify the transaction description column header.
    /// Specifying this overrides the X1/X2 range.
    pub transaction_description_header_terms: Vec<String>,
    /// Coordinate values are aligned with header by "x1", "x2"
    pub transaction_description_header_align: String,
    /// Regex patterns to exclude from being considered as part of the description.
    /// E.g., [/\.\./g] to exclude "......." patterns.
    pub transaction_description_exclude: Vec<Regex>,

    // TRANSACTION AMOUNT READ PARAMS
    /// Array of accepted formats to parse the transaction amount
    pub transaction_amount_formats: Vec<String>,
    /// X1 coordinate range to identify the transaction amount
    pub transaction_amount_x1_range: (i32, i32),
    /// X2 coordinate range to identify the transaction amount
    pub transaction_amount_x2_range: (i32, i32),
    /// Terms that can identify the transaction amount column header.
    /// Specifying this overrides the X1/X2 range.
    pub transaction_amount_header_terms: Vec<String>,
    /// Coordinate values are aligned with header by "x1", "x2"
    pub transaction_amount_header_align: String,
    /// Invert amounts falling within X1 coordinate range
    /// E.g., for statements where credits are on the left and debits are on the right.
    pub transaction_amount_invert_x1_range: (i32, i32),
    /// Invert amounts falling within X2 coordinate range
    pub transaction_amount_invert_x2_range: (i32, i32),
    /// Invert the sign of all transaction amounts. Often needed for credit card statements.
    pub transaction_amount_invert: bool,
    /// Terms that can identify the transaction amount column header for inverted amounts.
    pub transaction_amount_invert_header_terms: Vec<String>,
    /// Coordinate values are aligned with header by "x1", "x2"
    pub transaction_amount_invert_header_align: String,

    // TRANSACTION BALANCE READ PARAMS
    /// Array of accepted formats to parse the transaction balance amount
    pub transaction_balance_formats: Vec<String>,
    /// X1 coordinate range to identify the transaction balance
    pub transaction_balance_x1_range: (i32, i32),
    /// X2 coordinate range to identify the transaction balance
    pub transaction_balance_x2_range: (i32, i32),
    /// Invert the sign of all transaction balance amounts.
    pub transaction_balance_invert: bool,
    /// Terms that can identify the transaction balance column header.
    pub transaction_balance_header_terms: Vec<String>,
    /// Coordinate values are aligned with header by "x1", "x2"
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
            apply_y_patch_line_height: 5,

            opening_balance_terms: vec![],
            opening_balance_formats: vec![],
            opening_balance_same_x1: false,
            opening_balance_x1_tol: 1,
            opening_balance_same_y1: true,
            opening_balance_y1_tol: 1,
            opening_balance_invert: false,

            closing_balance_terms: vec![],
            closing_balance_formats: vec![],
            closing_balance_same_x1: false,
            closing_balance_x1_tol: 1,
            closing_balance_same_y1: true,
            closing_balance_y1_tol: 1,
            closing_balance_invert: false,

            start_date_terms: vec![],
            start_date_formats: vec![],
            start_date_same_x1: false,
            start_date_x1_tol: 1,
            start_date_same_y1: true,
            start_date_y1_tol: 1,

            transaction_terms: vec![],
            transaction_terms_stop: vec![],
            transaction_formats: vec![],
            transaction_new_line_y1_tol: 2,
            transaction_start_date_required: false,

            transaction_date_formats: vec![],
            transaction_date_x1_range: (-10000, 10000),
            transaction_date_x2_range: (-10000, 10000),

            transaction_description_x1_range: (-10000, 10000),
            transaction_description_x2_range: (-10000, 10000),
            transaction_description_exclude: vec![],

            transaction_amount_formats: vec![],
            transaction_amount_x1_range: (-10000, 10000),
            transaction_amount_x2_range: (-10000, 10000),
            transaction_amount_invert_x1_range: (-10000, -10000),
            transaction_amount_invert_x2_range: (-10000, -10000),
            transaction_amount_invert: false,

            transaction_balance_formats: vec![],
            transaction_balance_x1_range: (-10000, 10000),
            transaction_balance_x2_range: (-10000, 10000),
            transaction_balance_invert: false,
        }
    }
}