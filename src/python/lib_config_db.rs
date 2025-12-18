use crate::configs::db::ConfigDB;
use crate::python::exceptions::{ConfigLoadError, ConfigAccessError};
use pyo3::prelude::*;

#[pyclass]
pub struct LibConfigDB {
    db: ConfigDB,
}

#[pymethods]
impl LibConfigDB {
    /// Create a new Parser instance
    #[new]
    pub fn new(caching: bool, str_caching: bool) -> Self {
        Self {
            db: ConfigDB::new(caching, str_caching),
        }
    }

    /// Add config directly from a JSON string. Caching must be enabled.
    pub fn register_from_str(&mut self, py_json_str: &str) -> PyResult<()> {
        match self.db.register_from_str(py_json_str) {
            Ok(_) => Ok(()),
            Err(e) => Err(ConfigLoadError::new_err(e)),
        }
    }

    /// Import JSON configuration file into the parser database.
    pub fn register_from_file(&mut self, py_file_path: &str) -> PyResult<()> {
        match self.db.register_from_file(py_file_path) {
            Ok(_) => Ok(()),
            Err(e) => Err(ConfigLoadError::new_err(e)),
        }
    }

    /// Get JSON configuration string for a given key from the parser database.
    pub fn get_config_json_str(&self, key: &str) -> PyResult<String> {
        match self.db.get_config_json_str(key) {
            Ok(json_str) => Ok(json_str.clone()),
            Err(e) => Err(ConfigAccessError::new_err(e)),
        }
    }

    /// Get list of all account_terms from all registered configs by key.
    pub fn get_account_terms(&self, key: &str) -> PyResult<Vec<String>> {
        match self.db.get_account_terms(key) {
            Ok(terms) => Ok(terms),
            Err(e) => Err(ConfigAccessError::new_err(e)),
        }
    }

    /// Get list of all registered config keys.
    pub fn get_all_config_keys(&self) -> PyResult<Vec<String>> {
        Ok(self.db.get_config_keys())
    }

    /// Check if a config with the given key is registered.
    pub fn has_config(&self, key: &str) -> PyResult<bool> {
        Ok(self.db.has_config(key))
    }
}
