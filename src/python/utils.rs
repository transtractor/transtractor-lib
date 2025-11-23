use crate::parsers::flows::statement_data_to_dict::ColumnData;
use crate::structs::TextItem;
use crate::structs::TextItems;
use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;
use pyo3::types::{PyAny, PyList};
use std::collections::HashMap;

/// Converts a Python list of text item dictionaries to a Rust TextItems struct
pub fn py_text_items_to_rust_text_items(py_text_items: &Bound<'_, PyAny>) -> PyResult<TextItems> {
    let mut text_items = TextItems::new();
    let py_list = py_text_items.downcast::<PyList>()?;
    for obj in py_list.iter() {
        let dict = obj.downcast::<pyo3::types::PyDict>()?;
        let text: String = dict
            .get_item("text")?
            .ok_or_else(|| PyRuntimeError::new_err("Missing 'text' field"))?
            .extract()?;
        let x1: i32 = dict
            .get_item("x1")?
            .ok_or_else(|| PyRuntimeError::new_err("Missing 'x1' field"))?
            .extract()?;
        let y1: i32 = dict
            .get_item("y1")?
            .ok_or_else(|| PyRuntimeError::new_err("Missing 'y1' field"))?
            .extract()?;
        let x2: i32 = dict
            .get_item("x2")?
            .ok_or_else(|| PyRuntimeError::new_err("Missing 'x2' field"))?
            .extract()?;
        let y2: i32 = dict
            .get_item("y2")?
            .ok_or_else(|| PyRuntimeError::new_err("Missing 'y2' field"))?
            .extract()?;
        let page: i32 = dict
            .get_item("page")?
            .ok_or_else(|| PyRuntimeError::new_err("Missing 'page' field"))?
            .extract()?;
        let text_item = TextItem::new(text, x1, y1, x2, y2, page);
        text_items.add(&text_item);
    }
    Ok(text_items)
}

/// Converts a Rust dictionary of ColumnData to a Python dictionary
pub fn rust_dict_to_py_dict(
    rust_dict: &HashMap<String, ColumnData>,
) -> PyResult<HashMap<String, PyObject>> {
    Python::with_gil(|py| {
        let mut py_dict = HashMap::new();
        for (key, column_data) in rust_dict {
            let py_list: PyObject = match column_data {
                ColumnData::DateColumn(data) => PyList::new(py, data)?.into(),
                ColumnData::IndexColumn(data) => PyList::new(py, data)?.into(),
                ColumnData::StringColumn(data) => PyList::new(py, data)?.into(),
                ColumnData::AmountColumn(data) => PyList::new(py, data)?.into(),
                ColumnData::BalanceColumn(data) => PyList::new(py, data)?.into(),
            };
            py_dict.insert(key.clone(), py_list);
        }
        Ok(py_dict)
    })
}
