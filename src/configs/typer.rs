use crate::structs::TextItem;
use crate::structs::text_items::get_text_item_buffer;
use crate::structs::text_items::tokenise_items;
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
    /// Initialize empty StatementTyper
    pub fn new() -> Self {
        let account_terms = vec![];
        let max_lookahead = 0;
        let keys_by_term: HashMap<String, Vec<String>> = HashMap::new();
        let expected_terms_by_key: HashMap<String, usize> = HashMap::new();

        StatementTyper {
            account_terms,
            keys_by_term,
            expected_terms_by_key,
            max_lookahead,
        }
    }

    pub fn add_account_terms(&mut self, key: &str, terms: &Vec<String>) {
        // Remove existing terms for this key first
        self.remove_account_terms(key);

        self.expected_terms_by_key
            .insert(key.to_string(), terms.len());

        for term in terms {
            // Track max lookahead
            let word_count = term.split_whitespace().count();
            if word_count > self.max_lookahead {
                self.max_lookahead = word_count;
            }

            // Map term to config keys
            self.keys_by_term
                .entry(term.clone())
                .and_modify(|keys| {
                    if !keys.contains(&key.to_string()) {
                        keys.push(key.to_string());
                    }
                })
                .or_insert_with(|| vec![key.to_string()]);
        }

        // Update account_terms collection
        self.account_terms = self.keys_by_term.keys().cloned().collect();
    }

    /// Return a list of config keys whose account_terms are all found in the provided text items.
    pub fn identify(&self, text_items: &Vec<TextItem>) -> Vec<String> {
        let tokenised_items = tokenise_items(text_items);
        // Incremented for each found term found for a key
        let mut matches_by_key: HashMap<String, usize> = HashMap::new();
        // Lookup set of account_terms already encountered, to prevent double counting
        let mut found_terms: HashSet<String> = HashSet::new();

        // Iterate through text items, attempting to match account_terms
        let len = tokenised_items.len();
        if len == 0 {
            return vec![];
        }
        let mut i: usize = 0;
        while i < len {
            let buffer_size = self.max_lookahead.min(len - i);
            let buffer = get_text_item_buffer(&tokenised_items, i, buffer_size);
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

        complete_keys
    }

    /// Remove account terms for a given config key and all other data associated with it.
    fn remove_account_terms(&mut self, key: &str) {
        self.expected_terms_by_key.remove(key);

        self.keys_by_term.retain(|_term, keys| {
            keys.retain(|k| k != key);
            !keys.is_empty()
        });

        // Recalculate max_lookahead based on remaining terms
        self.max_lookahead = self
            .keys_by_term
            .keys()
            .map(|term| term.split_whitespace().count())
            .max()
            .unwrap_or(0);
    }
}
