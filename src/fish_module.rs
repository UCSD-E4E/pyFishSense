use pyo3::prelude::*;

use crate::fish_segmentation::FishSegmentationPy;

pub struct FishModule<'a> {
    pub module: Bound<'a, PyModule>,
}

impl<'a> FishModule<'a> {
    pub fn new(parent: &Bound<'a, PyModule>) -> PyResult<FishModule<'a>> {
        let fish_module = PyModule::new_bound(parent.py(), "fish")?;
        fish_module.add_class::<FishSegmentationPy>()?;
    
        Ok(FishModule { module: fish_module })
    }
}
