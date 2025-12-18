use pyo3::create_exception;
use pyo3::exceptions::PyException;

// Define custom exceptions
create_exception!(transtractor, NoErrorFreeStatementData, PyException);
create_exception!(transtractor, ConfigLoadError, PyException);
create_exception!(transtractor, ConfigAccessError, PyException);
