use crate::configs::validate::validate_config;
use crate::structs::statement_config::StatementConfig;
use regex::Regex;
use serde::Deserialize;
use std::fs;
use std::path::Path;

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

/// Raw struct used only for deserialization (all fields optional so we can overlay defaults)
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct StatementConfigPartial {
    key: Option<String>,
    bank_name: Option<String>,
    account_type: Option<String>,
    account_terms: Option<Vec<String>>,
    account_examples: Option<Vec<String>>,
    fix_text_order: Option<Vec<f32>>,

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

pub fn from_json_file<P: AsRef<Path>>(path: P) -> Result<StatementConfig, String> {
    let path_ref = path.as_ref();
    let data = fs::read_to_string(&path)
        .map_err(|e| format!("Failed reading config {:?}: {}", path_ref, e))?;
    let cfg = from_json_str(&data)?;
    Ok(cfg)
}

pub fn from_json_str(src: &str) -> Result<StatementConfig, String> {
    let partial: StatementConfigPartial =
        serde_json::from_str(src).map_err(|e| format!("JSON parse error: {}", e))?;
    let mut cfg = StatementConfig::default();

    macro_rules! overlay {
        ($field:ident) => {
            if let Some(v) = partial.$field {
                cfg.$field = v;
            }
        };
    }

    overlay!(key);
    overlay!(bank_name);
    overlay!(account_type);
    overlay!(account_terms);
    overlay!(account_examples);
    overlay!(fix_text_order);

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

    validate_config(&cfg).map_err(|e| format!("Config validation error: {}", e))?;
    Ok(cfg)
}
