use crate::configs::registry::get_config_map;
use crate::configs::typer::StatementTyper;
use crate::parsers::flows::config_json_file_to_config::from_json_file;
use crate::parsers::flows::config_json_file_to_config::from_json_str;
use crate::structs::StatementConfig;
use crate::structs::TextItem;
use std::collections::HashMap;

/// Struct to store or index statement configurations.
#[derive(Debug, Clone)]
pub struct ConfigDB {
    /// Collection of config instances by key
    configs: HashMap<String, StatementConfig>,
    /// StatementTyper matched to this ConfigDB
    pub typer: StatementTyper,
}

impl ConfigDB {
    /// Initialise ConfigDB with entire registry
    pub fn new() -> Self {
        let configs = get_config_map();
        let mut typer = StatementTyper::new();
        for cfg in configs.values() {
            typer.add_account_terms(&cfg.key, &cfg.account_terms);
        }
        Self { configs, typer }
    }
}

impl Default for ConfigDB {
    fn default() -> Self {
        Self::new()
    }
}

impl ConfigDB {
    /// Add config directly from a JSON string, overwriting any existing config with the
    /// same key.
    pub fn register_from_str(&mut self, json_str: &str) -> Result<(), String> {
        let cfg = from_json_str(json_str)?;
        self.configs.insert(cfg.key.clone(), cfg.clone());
        self.typer.add_account_terms(&cfg.key, &cfg.account_terms);
        Ok(())
    }

    /// Add config from a JSON file path.
    pub fn register_from_file(&mut self, file_path: &str) -> Result<(), String> {
        let cfg = from_json_file(file_path)?;
        self.configs.insert(cfg.key.clone(), cfg.clone());
        self.typer.add_account_terms(&cfg.key, &cfg.account_terms);
        Ok(())
    }

    /// Identify applicable config keys from a list of text items and return a list
    /// of StatementConfig instances.
    pub fn identify(&self, text_items: &Vec<TextItem>) -> Vec<StatementConfig> {
        let keys = self.typer.identify(text_items);
        let mut applicable_configs = Vec::new();
        for key in keys {
            if let Some(cfg) = self.configs.get(&key) {
                applicable_configs.push(cfg.clone());
            }
        }
        applicable_configs
    }

    /// Get list of all registered config keys
    pub fn get_config_keys(&self) -> Vec<String> {
        let mut keys: Vec<String> = self.configs.keys().cloned().collect();
        keys.sort();
        keys
    }
}
