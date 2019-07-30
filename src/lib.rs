use pyo3::prelude::*;
// use pyo3::wrap_pyfunction;

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

    fn remove(&mut self, v: usize) -> PyResult<i8> {
        let re = self.t.remove(v);
        match re {
            Ok(_x) => return Ok(1),
            Err(_x) => {
                if _x == "Cannot remove the infinite vertex" {
                    return Ok(0);
                } else {
                    return Ok(-1);
                }
            }
        };
    }

    fn insert(&mut self, pts: Vec<Vec<f64>>) {
        self.t.insert(&pts);
    }

    fn get_snap_tolerance(&self) -> PyResult<f64> {
        Ok(self.t.get_snap_tolerance())
    }
    fn set_snap_tolerance(&mut self, snaptol: f64) {
        self.t.set_snap_tolerance(snaptol);
    }

    fn number_of_vertices(&self) -> PyResult<usize> {
        Ok(self.t.number_of_vertices())
    }
    fn number_of_triangles(&self) -> PyResult<usize> {
        Ok(self.t.number_of_triangles())
    }

    fn all_vertices(&self) -> PyResult<Vec<Vec<f64>>> {
        Ok(self.t.all_vertices())
    }

    fn get_point(&self, v: usize) -> PyResult<Vec<f64>> {
        let re = self.t.get_point(v);
        if re.is_some() {
            Ok(self.t.get_point(v).unwrap())
        } else {
            let pt = vec![-99999.99999, -99999.99999, -99999.99999];
            Ok(pt)
        }
    }

    fn all_triangles(&self) -> PyResult<Vec<Vec<usize>>> {
        let mut trs: Vec<Vec<usize>> = Vec::with_capacity(self.t.number_of_triangles());
        for each in self.t.all_triangles() {
            let mut tr = Vec::with_capacity(3);
            tr.push(each.v[0]);
            tr.push(each.v[1]);
            tr.push(each.v[2]);
            trs.push(tr);
        }
        Ok(trs)
    }

    fn convex_hull(&self) -> PyResult<Vec<usize>> {
        Ok(self.t.convex_hull())
    }
    fn is_vertex_convex_hull(&self, v: usize) -> PyResult<bool> {
        Ok(self.t.is_vertex_convex_hull(v))
    }

    fn incident_triangles_to_vertex(&self, v: usize) -> PyResult<Vec<Vec<usize>>> {
        let re = self.t.incident_triangles_to_vertex(v);
        if re.is_some() {
            let l = re.unwrap();
            let mut trs: Vec<Vec<usize>> = Vec::with_capacity(l.len());
            for each in l {
                let mut tr = Vec::with_capacity(3);
                tr.push(each.v[0]);
                tr.push(each.v[1]);
                tr.push(each.v[2]);
                trs.push(tr);
            }
            Ok(trs)
        } else {
            let trs: Vec<Vec<usize>> = Vec::new();
            Ok(trs)
        }
    }

    fn adjacent_vertices_to_vertex(&self, v: usize) -> PyResult<Vec<usize>> {
        let re = self.t.adjacent_vertices_to_vertex(v);
        if re.is_some() {
            Ok(re.unwrap())
        } else {
            let l: Vec<usize> = Vec::new();
            Ok(l)
        }
    }

    fn is_triangle(&self, t: Vec<usize>) -> PyResult<bool> {
        let tr = startin::Triangle {
            v: [t[0], t[1], t[2]],
        };
        Ok(self.t.is_triangle(&tr))
    }

    fn locate(&self, px: f64, py: f64) -> PyResult<Vec<usize>> {
        let re = self.t.locate(px, py);
        let mut tr: Vec<usize> = Vec::new();
        if re.is_some() {
            let t = re.unwrap();
            tr.push(t.v[0]);
            tr.push(t.v[1]);
            tr.push(t.v[2]);
        }
        Ok(tr)
    }
}

#[pymodule]
fn startin(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<DT>()?;

    Ok(())
}
