use crate::configs::StatementTyper;
use crate::parsers::flows::text_items_to_debug::text_items_to_debug;
use crate::parsers::flows::text_items_to_dict::text_items_to_dict;
use crate::python::utils;
use pyo3::prelude::*;
use std::collections::HashMap;

#[pyclass]
pub struct LibParser {
    typer: StatementTyper,
}

#[pymethods]
impl LibParser {
    /// Create a new Parser instance
    #[new]
    pub fn new() -> Self {
        Self {
            typer: StatementTyper::new(),
        }
    }

    /// Process a Python list of text items and return statement data as a
    /// Python dictionary suitable for DataFrame ingestion.
    pub fn py_text_items_to_py_dict(
        &self,
        py_text_items: &Bound<'_, pyo3::types::PyAny>,
    ) -> PyResult<HashMap<String, PyObject>> {
        let mut text_items = utils::py_text_items_to_rust_text_items(py_text_items)?;
        match text_items_to_dict(&mut text_items, &self.typer) {
            Ok(dict) => utils::rust_dict_to_py_dict(&dict),
            Err(e) => Err(pyo3::exceptions::PyRuntimeError::new_err(e)),
        }
    }

    /// Process a Python list of text items and return debug information as a string.
    pub fn py_text_items_to_debug_py_str(
        &self,
        py_text_items: &Bound<'_, pyo3::types::PyAny>,
    ) -> PyResult<String> {
        let mut text_items = utils::py_text_items_to_rust_text_items(py_text_items)?;
        match text_items_to_debug(&mut text_items, &self.typer) {
            Ok(debug_str) => Ok(debug_str),
            Err(e) => Err(pyo3::exceptions::PyRuntimeError::new_err(e)),
        }
    }
}
