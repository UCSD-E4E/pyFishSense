use numpy::{IntoPyArray, PyArray2, PyReadonlyArray3};
use pyo3::{exceptions::PyTypeError, prelude::*};

use fishsense::fish::FishSegmentation;

#[pyclass(name = "FishSegmentation")]
pub struct FishSegmentationPy {
    fish_segmentation: FishSegmentation,
}

#[pymethods]
impl FishSegmentationPy {
    #[new]
    pub fn new() -> PyResult<Self> {
        match FishSegmentation::from_web() {
            Ok(fish_segmentation) => Ok(FishSegmentationPy { fish_segmentation }),
            Err(_) => Err(PyErr::new::<PyTypeError, _>("An error occurred downloading the segmentation model."))
        }
    }

    fn inference<'py>(&self, py: Python<'py>, img: PyReadonlyArray3<'py, u8>) -> PyResult<Bound<'py, PyArray2<u8>>> {
        let img_arr = img.as_array().mapv(|v| v.to_owned());
        
        match self.fish_segmentation.inference(&img_arr) {
            Ok(masks) => Ok(masks.into_pyarray_bound(py)),
            Err(_) => Err(PyErr::new::<PyTypeError, _>("An error occurred during segmentation."))
        }
    }
}
