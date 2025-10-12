use pyo3::prelude::*;
use pyo3::types::PyModuleMethods;

pub mod state_py;

pub fn register_bindings(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<state_py::PyGameState>()?;
    Ok(())
}
