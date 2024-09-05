use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pyfunction]
fn helloworld() {
    println!("Hello, World from randomos!");
}

#[pyfunction]
fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

#[pymodule]
fn randomos(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(helloworld, m)?)?;
    m.add_function(wrap_pyfunction!(multiply, m)?)?;
    Ok(())
}
