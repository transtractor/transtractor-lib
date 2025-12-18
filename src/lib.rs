pub mod checkers;
pub mod configs;
pub mod fixers;
pub mod formats;
pub mod parsers;
pub mod python;
pub mod structs;

use crate::python::exceptions::{ConfigLoadError, NoErrorFreeStatementData};
use crate::python::lib_parser::LibParser;
use crate::python::lib_config_db::LibConfigDB;
use pyo3::prelude::*;

/// Python module definition
#[pymodule]
fn transtractor(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<LibParser>()?;
    m.add_class::<LibConfigDB>()?;
    m.add("NoErrorFreeStatementData", m.py().get_type::<NoErrorFreeStatementData>())?;
    m.add("ConfigLoadError", m.py().get_type::<ConfigLoadError>())?;
    Ok(())
}
