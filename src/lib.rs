use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use rand::seq::SliceRandom;

#[pyfunction]
fn get_random_os() -> &'static str {
    let os_list = ["Linux", "Windows", "macOS", "FreeBSD", "Solaris"];
    let os = os_list.choose(&mut rand::thread_rng()).unwrap();
    os
}

#[pymodule]
fn randomos(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_random_os, m)?)?;
    Ok(())
}
