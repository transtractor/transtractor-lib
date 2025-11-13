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

    /// Convert a PDF or TXT bank statement to a dictionary of lists
    pub fn to_dict(&self, input_file: &str) -> PyResult<std::collections::HashMap<String, PyObject>> {
        use pyo3::types::PyList;
        use parsers::dict_from_statement_data::ColumnData;
        
        let dict = self.inner
            .to_dict(input_file)
            .map_err(|e| PyRuntimeError::new_err(e))?;
        
        Python::with_gil(|py| {
            let mut py_dict = std::collections::HashMap::new();
            
            for (key, column_data) in dict {
                let py_list: PyObject = match column_data {
                    ColumnData::DateColumn(data) => {
                        PyList::new(py, &data)?.into()
                    },
                    ColumnData::IndexColumn(data) => {
                        PyList::new(py, &data)?.into()
                    },
                    ColumnData::StringColumn(data) => {
                        PyList::new(py, &data)?.into()
                    },
                    ColumnData::AmountColumn(data) => {
                        PyList::new(py, &data)?.into()
                    },
                    ColumnData::BalanceColumn(data) => {
                        PyList::new(py, &data)?.into()
                    },
                };
                py_dict.insert(key, py_list);
            }
            
            Ok(py_dict)
        })
    }

    /// Debug a PDF or TXT bank statement and write detailed parsing information to a file
    pub fn debug(&self, input_file: &str, output_file: &str) -> PyResult<()> {
        self.inner
            .debug(input_file, output_file)
            .map_err(|e| PyRuntimeError::new_err(e))
    }

    /// Test all PDF and TXT files in a directory and its subdirectories
    pub fn test_directory(&self, directory_path: &str) -> PyResult<()> {
        self.inner
            .test_directory(directory_path)
            .map_err(|e| PyRuntimeError::new_err(e))
    }

    /// Convert a PDF file to layout text format and write it to a file
    pub fn to_layout_text(&self, input_file: &str, output_file: &str, fix_y_disorder: bool) -> PyResult<()> {
        self.inner
            .to_layout_text(input_file, output_file, fix_y_disorder)
            .map_err(|e| PyRuntimeError::new_err(e))
    }
}

/// Python module definition
#[pymodule]
fn transtractor(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Parser>()?;
    Ok(())
}
