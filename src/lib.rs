use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

extern crate startin;

#[pyclass]
struct DT {
    t: startin::Triangulation,
}

#[pymethods]
impl DT {
    #[new]
    fn new(obj: &PyRawObject) {
        obj.init(DT {
            t: startin::Triangulation::new(),
        });
    }

    fn insert_one_pt(&mut self, px: f64, py: f64, pz: f64) -> PyResult<usize> {
        let re = self.t.insert_one_pt(px, py, pz);
        match re {
            Ok(x) => return Ok(x),
            Err(x) => return Ok(x),
        };
    }

    fn number_of_vertices(&self) -> PyResult<usize> {
        Ok(self.t.number_of_vertices())
    }

    fn number_of_triangles(&self) -> PyResult<usize> {
        Ok(self.t.number_of_triangles())
    }
}

#[pymodule]
fn startin(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<DT>()?;

    Ok(())
}
