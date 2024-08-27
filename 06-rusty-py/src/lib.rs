use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn multiply(a: isize, b: isize) -> PyResult<isize>{
    Ok(a*b)
}


/// A Python module implemented in Rust.
#[pymodule]
fn rust(_py: Python, m: &pyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(multiply, m)?)?;
    Ok(())
}
