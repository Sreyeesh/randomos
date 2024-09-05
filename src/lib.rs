use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use rand::seq::SliceRandom;
use std::fmt;

// Define a custom error type for better error handling
#[derive(Debug)]
enum RandomOsError {
    EmptyList,
}

impl fmt::Display for RandomOsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RandomOsError::EmptyList => write!(f, "The list of operating systems is empty"),
        }
    }
}

impl std::error::Error for RandomOsError {}

// Convert custom error to PyO3 PyErr
fn random_os_error_to_pyerr(e: RandomOsError) -> PyErr {
    PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string())
}

#[pyfunction]
fn get_random_os() -> PyResult<&'static str> {
    let os_list = ["Linux", "Windows", "macOS", "FreeBSD", "Solaris"];
    let os = os_list.choose(&mut rand::thread_rng())
        .ok_or_else(|| random_os_error_to_pyerr(RandomOsError::EmptyList))?;
    Ok(os)
}

#[pymodule]
fn randomos(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_random_os, m)?)?;
    Ok(())
}
