use crate::parsers::flows::config_json_file_to_config::from_json_file;
use crate::parsers::flows::config_json_file_to_config::from_json_str;
use crate::structs::StatementConfig;
use std::collections::HashMap;

/// Struct to store or index statement configurations.
#[derive(Debug, Clone)]
pub struct ConfigDB {
    /// If True, keep instances of loaded configs in memory, else
    /// they are dynamically loaded from file each time.
    caching: bool,
    /// If true, the text str of each config is also cached.
    str_caching: bool,
    /// Collection of config instances by key
    configs: HashMap<String, StatementConfig>,
    /// Map of config keys to their file paths
    config_paths: HashMap<String, String>,
    /// Map of config keys to their JSON str (if str_caching enabled)
    config_strs: HashMap<String, String>,
}

impl ConfigDB {
    /// Initialize empty ConfigDB
    pub fn new(caching: bool, str_caching: bool) -> Self {
        ConfigDB {
            caching,
            str_caching,
            configs: HashMap::new(),
            config_paths: HashMap::new(),
            config_strs: HashMap::new(),
        }
    }

    /// Add config directly from a JSON string. Caching must be enabled.
    pub fn register_from_str(&mut self, json_str: &str) -> Result<(), String> {
        //  Return Error if caching is disabled
        if !self.caching {
            return Err("Caching must be enabled to add a config from JSON string".to_string());
        }
        let cfg = from_json_str(json_str)?;
        self.configs.insert(cfg.key.clone(), cfg.clone());
        if self.str_caching {
            self.config_strs.insert(cfg.key.clone(), json_str.to_string());
        }
        Ok(())
    }

    /// Add config from a JSON file path.
    pub fn register_from_file(&mut self, file_path: &str) -> Result<(), String> {
        let cfg = from_json_file(file_path)?;
        if self.caching {
            self.configs.insert(cfg.key.clone(), cfg.clone());
        }
        self.config_paths
            .insert(cfg.key.clone(), file_path.to_string());
        if self.str_caching {
            let json_str = std::fs::read_to_string(file_path)
                .map_err(|e| format!("Failed to read config file '{}': {}", file_path, e))?;
            self.config_strs.insert(cfg.key.clone(), json_str);
        }
        Ok(())
    }

    /// Fetch a config by key.
    pub fn get_config(&self, key: &str) -> Result<StatementConfig, String> {
        // If caching enabled, try to get from memory first
        if self.caching {
            if let Some(cfg) = self.configs.get(key) {
                return Ok(cfg.clone());
            }
        }
        // Else, try to load from file path
        if let Some(path) = self.config_paths.get(key) {
            let cfg = from_json_file(path)?;
            return Ok(cfg);
        }
        Err(format!("Config with key '{}' not found", key))
    }

    pub fn get_config_json_str(&self, key: &str) -> Result<String, String> {
        // If str caching enabled, try to get from memory first
        if self.str_caching {
            if let Some(json_str) = self.config_strs.get(key) {
                return Ok(json_str.clone());
            }
        }
        // Else, try to load from file path
        if let Some(path) = self.config_paths.get(key) {
            let json_str = std::fs::read_to_string(path)
                .map_err(|e| format!("Failed to read config file '{}': {}", path, e))?;
            return Ok(json_str);
        }
        Err(format!("Config with key '{}' not found", key))
    }

    /// Get list of all account_terms from all registered configs by key.
    pub fn get_account_terms(&self, key: &str) -> Result<Vec<String>, String> {
        let cfg = self.get_config(key)?;
        Ok(cfg.account_terms)
    }

    /// Get list of all registered config keys.
    pub fn get_config_keys(&self) -> Vec<String> {
        let mut keys: Vec<String> = self.config_paths.keys().cloned().collect();
        if self.caching {
            for key in self.configs.keys() {
                if !keys.contains(key) {
                    keys.push(key.clone());
                }
            }
        }
        keys
    }

    /// Check if a config with the given key is registered.
    pub fn has_config(&self, key: &str) -> bool {
        if self.caching {
            if self.configs.contains_key(key) {
                return true;
            }
        }
        self.config_paths.contains_key(key)
    }
}