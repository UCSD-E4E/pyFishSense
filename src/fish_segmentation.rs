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
            Ok(mut fish_segmentation) => {
                match fish_segmentation.load_model() {
                    Ok(_) => Ok(FishSegmentationPy { fish_segmentation }),
                    Err(error) => Err(PyErr::new::<PyTypeError, _>(error.to_string()))
                }
            },
            Err(error) => Err(PyErr::new::<PyTypeError, _>(error.to_string()))
        }
    }

    fn inference<'py>(&self, py: Python<'py>, img: PyReadonlyArray3<'py, u8>) -> PyResult<Bound<'py, PyArray2<u8>>> {
        let img_arr = img.as_array().mapv(|v| v.to_owned());
        
        match self.fish_segmentation.inference(&img_arr) {
            Ok(masks) => Ok(masks.into_pyarray_bound(py)),
            Err(error) => Err(PyErr::new::<PyTypeError, _>(error.to_string()))
        }
    }
}
