pub mod checkers;
pub mod configs;
pub mod fixers;
pub mod formats;
pub mod parsers;
pub mod structs;

use pyo3::prelude::*;
use pyo3::exceptions::PyRuntimeError;

/// Python wrapper for the Parser struct
#[pyclass]
pub struct Parser {
    inner: parsers::parser::Parser,
}

#[pymethods]
impl Parser {
    /// Create a new Parser instance
    #[new]
    pub fn new() -> Self {
        Self {
            inner: parsers::parser::Parser::new(),
        }
    }

    /// Convert a PDF or TXT bank statement to CSV format
    pub fn to_csv(&self, input_file: &str, output_csv: &str) -> PyResult<()> {
        self.inner
            .to_csv(input_file, output_csv)
            .map_err(|e| PyRuntimeError::new_err(e))
    }

    /// Test all PDF and TXT files in a directory and its subdirectories
    pub fn test_directory(&self, directory_path: &str) -> PyResult<()> {
        self.inner
            .test_directory(directory_path)
            .map_err(|e| PyRuntimeError::new_err(e))
    }
}

/// Python module definition
#[pymodule]
fn transtractor(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Parser>()?;
    Ok(())
}
