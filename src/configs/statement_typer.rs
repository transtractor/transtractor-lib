use crate::configs::STATEMENT_CONFIG_REGISTRY;
use crate::structs::StatementConfig;
use crate::structs::TextItem;
use crate::structs::text_items::get_text_item_buffer;
use std::collections::{HashMap, HashSet};

/// Struct to identify statement types from text items.
#[derive(Debug, Clone)]
pub struct StatementTyper {
    /// Collection of all account_terms identifying statement types (case-sensitive),
    account_terms: Vec<String>,
    /// Maps each term to one or more statement config keys
    keys_by_term: HashMap<String, Vec<String>>,
    /// Maps each statement config key to the number of expected terms
    expected_terms_by_key: HashMap<String, usize>,
    /// Maximum number of space-delimited words in any account_term
    max_lookahead: usize,
}

impl StatementTyper {
    /// Initialize a StatementTyper from the global STATEMENT_CONFIG_REGISTRY.
    pub fn new() -> Self {
        let registry = STATEMENT_CONFIG_REGISTRY.clone();
        let mut max_lookahead = 0;
        let mut keys_by_term: HashMap<String, Vec<String>> = HashMap::new();
        let mut expected_terms_by_key: HashMap<String, usize> = HashMap::new();

        for (key, cfg) in registry.iter() {
            let terms = &cfg.account_terms;
            expected_terms_by_key.insert(key.clone(), terms.len());

            for term in terms {
                // Track max lookahead
                let word_count = term.split_whitespace().count();
                if word_count > max_lookahead {
                    max_lookahead = word_count;
                }

                // Map term to config keys
                keys_by_term
                    .entry(term.clone())
                    .and_modify(|keys| {
                        if !keys.contains(key) {
                            keys.push(key.clone());
                        }
                    })
                    .or_insert_with(|| vec![key.clone()]);
            }
        }

        // Collect all unique account_terms
        let account_terms = keys_by_term.keys().cloned().collect();

        StatementTyper {
            account_terms,
            keys_by_term,
            expected_terms_by_key,
            max_lookahead,
        }
    }

    /// Identify statement types from tokenised TextItems. Returns None if no type identified.
    pub fn identify_from_text_items(
        &self,
        text_items: &Vec<TextItem>,
    ) -> Option<Vec<StatementConfig>> {
        // Incremented for each found term found for a key
        let mut matches_by_key: HashMap<String, usize> = HashMap::new();
        // Lookup set of account_terms already encountered, to prevent double counting
        let mut found_terms: HashSet<String> = HashSet::new();

        // Iterate through text items, attempting to match account_terms
        let len = text_items.len();
        if len == 0 {
            return None;
        }
        let mut i: usize = 0;
        while i < len {
            let buffer_size = self.max_lookahead.min(len - i);
            let buffer = get_text_item_buffer(&text_items, i, buffer_size);
            if buffer.is_empty() {
                break;
            }
            let phrase = buffer
                .iter()
                .map(|ti| ti.text.as_str())
                .collect::<Vec<_>>()
                .join(" ");

            for term in &self.account_terms {
                // Skip if term longer than phrase
                if term.len() > phrase.len() {
                    continue;
                }

                // Check if phrase starts with term (case-sensitive)
                if phrase.starts_with(term) {
                    // Log term if not already found
                    if !found_terms.contains(term) {
                        found_terms.insert(term.clone());
                        if let Some(keys) = self.keys_by_term.get(term) {
                            for key in keys {
                                matches_by_key
                                    .entry(key.clone())
                                    .and_modify(|count| *count += 1)
                                    .or_insert(1);
                            }
                        }
                    }
                }
            }

            // Advance i by 1 to continue scanning
            i += 1;
        }

        // Return list of keys that have all terms satisfied
        let complete_keys: Vec<String> = matches_by_key
            .iter()
            .filter_map(|(key, &count)| {
                if let Some(&expected) = self.expected_terms_by_key.get(key) {
                    if count == expected {
                        return Some(key.clone());
                    }
                }
                None
            })
            .collect();

        // If no complete keys, return None
        if complete_keys.is_empty() {
            return None;
        }

        // Return Lookup configs for complete keys
        Some(complete_keys).map(|keys| {
            keys.iter()
                .filter_map(|k| STATEMENT_CONFIG_REGISTRY.get(k).cloned())
                .collect()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::structs::text_item::TextItem;
    use crate::structs::text_items::tokenise::tokenise_items;

    fn ti(text: &str) -> TextItem {
        TextItem::new(text.to_string(), 0, 0, 0, 0, 1)
    }

    #[test]
    fn identify_returns_cba_when_all_terms_present() {
        // Given a typer built from embedded registry and items containing both terms
        let typer = StatementTyper::new();
        let mut items = Vec::new();
        items.push(ti("Hello CommBank world"));
        items.push(ti("noise"));
        items.push(ti("Available credit here"));
        let items = tokenise_items(items);

        let result = typer.identify_from_text_items(&items);
        assert!(result.is_some(), "Expected at least one match");
        let matches = result.unwrap();
        assert!(
            matches
                .iter()
                .any(|cfg| cfg.key == "au__cba__credit_card__1"),
            "Expected au__cba__credit_card__1 among matches, got: {:?}",
            matches.iter().map(|c| c.key.clone()).collect::<Vec<_>>()
        );
    }

    #[test]
    fn identify_none_when_only_one_term_or_duplicates() {
        let typer = StatementTyper::new();
        let mut items = Vec::new();
        // Only one of the required terms present (and duplicated), should not complete
        items.push(ti("CommBank CommBank more text"));

        let items = tokenise_items(items);

        let result = typer.identify_from_text_items(&items);
        assert!(
            result.is_none(),
            "Should not match when not all terms are present"
        );
    }

    #[test]
    fn identify_none_on_empty_input() {
        let typer = StatementTyper::new();
        let items = Vec::new();
        assert!(typer.identify_from_text_items(&items).is_none());
    }

    #[test]
    fn identify_is_case_sensitive() {
        let typer = StatementTyper::new();
        let mut items = Vec::new();
        // Lowercased term should not match the case-sensitive 'CommBank'
        items.push(ti("commbank"));
        items.push(ti("Available credit"));
        let items = tokenise_items(items);
        assert!(typer.identify_from_text_items(&items).is_none());
    }
}
