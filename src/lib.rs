mod fish_module;
mod fish_segmentation;

use fish_module::FishModule;
use pyo3::prelude::*;

/// A Python module implemented in Rust.
#[pymodule]
fn pyfishsense<'a>(_py: Python, m: Bound<'a, PyModule>) -> PyResult<()> {
    let fish = FishModule::new(&m)?;

    m.add_submodule(&fish.module)?;

    Ok(())
}
