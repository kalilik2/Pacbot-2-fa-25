use pyo3::prelude::*;

pub mod state_py;

pub fn register_bindings(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<state_py::PyGameState>()?;
    Ok(())
}
