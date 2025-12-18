use crate::configs::db::ConfigDB;
use crate::configs::typer::StatementTyper;
use crate::parsers::flows::config_json_file_to_config;
use crate::parsers::flows::text_items_to_debug::text_items_to_debug;
use crate::parsers::flows::text_items_to_layout::text_items_to_layout;
use crate::parsers::flows::text_items_to_statement_datas::text_items_to_statement_datas;
use crate::python::exceptions::{ConfigLoadError, NoErrorFreeStatementData};
use crate::python::utils;
use crate::structs::StatementConfig;
use pyo3::prelude::*;

#[pyclass]
pub struct LibParser {
    typer: StatementTyper,
    db: ConfigDB,
}

impl LibParser {
    /// Get list of configs from provided keys
    fn get_configs_from_keys(&self, keys: &Vec<String>) -> Result<Vec<StatementConfig>, String> {
        let mut configs: Vec<StatementConfig> = Vec::new();
        for key in keys {
            if !self.db.has_config(key) {
                return Err(format!("Config with key '{}' is not registered", key));
            }
            let cfg = self.db.get_config(key)?;
            configs.push(cfg);
        }
        Ok(configs)
    }
}

#[pymethods]
impl LibParser {
    /// Create a new Parser instance
    #[new]
    pub fn new() -> Self {
        Self {
            typer: StatementTyper::new(),
            db: ConfigDB::new(true, false),
        }
    }

    /// Import JSON configuration str into the parser database and update the StatementTyper.
    pub fn import_config_from_json_str(&mut self, py_json_str: &str) -> PyResult<()> {
        match self.db.register_from_str(py_json_str) {
            Ok(_) => {}
            Err(e) => return Err(ConfigLoadError::new_err(e)),
        }
        let cfg = match config_json_file_to_config::from_json_str(py_json_str) {
            Ok(c) => c,
            Err(e) => return Err(ConfigLoadError::new_err(e)),
        };
        self.typer.add_account_terms(&cfg.key, &cfg.account_terms);
        Ok(())
    }

    /// Register JSON configuration string into the parser database without updating the StatementTyper.
    pub fn register_config_from_json_str(&mut self, py_json_str: &str) -> PyResult<()> {
        match self.db.register_from_str(py_json_str) {
            Ok(_) => Ok(()),
            Err(e) => Err(ConfigLoadError::new_err(e)),
        }
    }

    /// Import JSON configuration file into the parser database and update the StatementTyper.
    pub fn import_config_from_file(&mut self, py_file_path: &str) -> PyResult<()> {
        match self.db.register_from_file(py_file_path) {
            Ok(_) => {}
            Err(e) => return Err(ConfigLoadError::new_err(e)),
        }
        let cfg = match config_json_file_to_config::from_json_file(py_file_path) {
            Ok(c) => c,
            Err(e) => return Err(ConfigLoadError::new_err(e)),
        };
        self.typer.add_account_terms(&cfg.key, &cfg.account_terms);
        Ok(())
    }

    /// Add account terms (list of strings) to the StatementTyper for a given config key.
    pub fn add_account_terms(&mut self, key: &str, terms: Vec<String>) -> PyResult<()> {
        self.typer.add_account_terms(key, &terms);
        Ok(())
    }

    /// Return a list of keys applicable to the provided text items.
    pub fn get_applicable_config_keys(
        &self,
        py_text_items: &Bound<'_, pyo3::types::PyAny>,
    ) -> PyResult<Vec<String>> {
        let text_items = utils::py_text_items_to_rust_text_items(py_text_items)?;
        let keys = self.typer.identify(&text_items);
        Ok(keys)
    }

    /// Return a list of keys not yet registered in the parser database. Input
    /// a list of keys to check against.
    pub fn get_unregistered_config_keys(&self, keys: Vec<String>) -> PyResult<Vec<String>> {
        let unregistered_keys: Vec<String> = keys
            .into_iter()
            .filter(|key| !self.db.has_config(key))
            .collect();
        Ok(unregistered_keys)
    }

    /// Process a Python list of text items and return statement data as a
    /// Python object of type StatementData.
    pub fn py_text_items_to_py_statement_data(
        &self,
        py_text_items: &Bound<'_, pyo3::types::PyAny>,
        applicable_config_keys: Vec<String>,
    ) -> PyResult<PyObject> {
        let text_items = utils::py_text_items_to_rust_text_items(py_text_items)?;
        let configs = self
            .get_configs_from_keys(&applicable_config_keys)
            .map_err(|e| {
                pyo3::exceptions::PyRuntimeError::new_err(format!("Failed to get configs: {}", e))
            })?;

        let statement_data_results = text_items_to_statement_datas(&text_items, &configs)
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e))?;

        // Find the first error-free StatementData
        for data in statement_data_results {
            if data.errors.is_empty() {
                return utils::rust_statement_data_to_py_statement_data(&data);
            }
        }

        Err(NoErrorFreeStatementData::new_err(format!(
            "No error-free StatementData found. This indicates an error in one of the configurations: {}. See further details using the \"debug\" method. Please contact a developer if this config is part of the default set provided by the package.",
            applicable_config_keys.join(", ")
        )))
    }

    /// Process a Python list of text items and return debug information as a string.
    pub fn py_text_items_to_debug_py_str(
        &self,
        py_text_items: &Bound<'_, pyo3::types::PyAny>,
        applicable_config_keys: Vec<String>,
    ) -> PyResult<String> {
        let text_items = utils::py_text_items_to_rust_text_items(py_text_items)?;
        let configs = self
            .get_configs_from_keys(&applicable_config_keys)
            .map_err(|e| {
                pyo3::exceptions::PyRuntimeError::new_err(format!("Failed to get configs: {}", e))
            })?;

        match text_items_to_debug(&text_items, &configs) {
            Ok(debug_str) => Ok(debug_str),
            Err(e) => Err(pyo3::exceptions::PyRuntimeError::new_err(e)),
        }
    }

    /// Process a Python list of text items and return layout text as a string.
    pub fn py_text_items_to_layout_py_str(
        &self,
        py_text_items: &Bound<'_, pyo3::types::PyAny>,
        y_bin: f32,
        x_gap: f32,
    ) -> PyResult<String> {
        let text_items = utils::py_text_items_to_rust_text_items(py_text_items)?;
        match text_items_to_layout(&text_items, y_bin, x_gap) {
            Ok(layout_str) => Ok(layout_str),
            Err(e) => Err(pyo3::exceptions::PyRuntimeError::new_err(e)),
        }
    }
}
