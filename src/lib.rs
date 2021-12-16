use pyo3::exceptions;
use pyo3::prelude::*;

extern crate startin;

extern crate las;
use las::Read;

/// A Delaunay triangulator where the input are 2.5D points,
/// the DT is computed in 2D but the elevation of the vertices are kept.
/// This is used mostly for the modelling of terrains.
/// This is the Python bindings of Rust's startin:
/// (https://github.com/hugoledoux/startin)
#[pymodule]
fn startinpy(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<DT>()?;
    Ok(())
}

#[pyclass(unsendable)]
/// A Delaunay triangulation (DT), containing vertices+triangles
pub struct DT {
    t: startin::Triangulation,
}

#[pymethods]
impl DT {
    /// Constructor for a DT (returns an empty DT)
    #[new]
    fn new() -> Self {
        let tmp = startin::Triangulation::new();
        DT { t: tmp }
    }

    /// Insert one new point in the DT.
    /// If there is a point at the same location (based on 2D tolerance), then the point is not inserted.
    #[pyo3(text_signature = "($self, px, py, pz)")]
    fn insert_one_pt(&mut self, px: f64, py: f64, pz: f64) -> PyResult<usize> {
        let re = self.t.insert_one_pt(px, py, pz);
        match re {
            Ok(x) => return Ok(x),
            Err(x) => return Ok(x),
        };
    }

    /// removes/delete the vertex i from the DT, and updates it for the Delaunay criterion
    /// returns 1 if the operation was successful; and -1 if the vertex doesn't exist
    #[pyo3(text_signature = "($self, v)")]
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

    /// calls insert_one_pt() for each vertex in the list
    /// returns nothing           
    #[pyo3(text_signature = "($self, pts)")]
    fn insert(&mut self, pts: Vec<Vec<f64>>) {
        self.t.insert(&pts);
    }

    /// reads the LAS/LAZ file "path_file" (a string) and inserts all the points in the DT
    /// throws an error if the path is invalid
    #[pyo3(text_signature = "($self, path)")]
    fn read_las(&mut self, path: String) -> PyResult<()> {
        let re = las::Reader::from_path(path);
        if re.is_err() {
            return Err(PyErr::new::<exceptions::PyIOError, _>(
                "Invalid path for LAS/LAZ file.",
            ));
        }
        let mut reader = re.unwrap();
        let _count = reader.header().number_of_points();
        for each in reader.points() {
            let p = each.unwrap();
            let _re = self.t.insert_one_pt(p.x, p.y, p.z);
        }
        Ok(())
    }

    /// returns the snap tolerance (2 vertices closer will be the same)
    fn get_snap_tolerance(&self) -> PyResult<f64> {
        Ok(self.t.get_snap_tolerance())
    }

    /// sets the snap tolerance (for insertion of points in the DT) to this value 
    /// (default=0.001)
    /// returns nothing
    #[pyo3(text_signature = "($self, snaptol)")]
    fn set_snap_tolerance(&mut self, snaptol: f64) {
        self.t.set_snap_tolerance(snaptol);
    }

    /// returns the number of (finite) vertices in the DT
    fn number_of_vertices(&self) -> PyResult<usize> {
        Ok(self.t.number_of_vertices())
    }
    /// returns the number of (finite) Triangles in the DT
    fn number_of_triangles(&self) -> PyResult<usize> {
        Ok(self.t.number_of_triangles())
    }

    /// returns a list of all vertices in the DT (including the infinite one, vertex "0")
    fn all_vertices(&self) -> PyResult<Vec<Vec<f64>>> {
        Ok(self.t.all_vertices())
    }

    /// returns the point at the index i  
    #[pyo3(text_signature = "($self, i)")]
    fn get_point(&self, v: usize) -> PyResult<Vec<f64>> {
        let re = self.t.get_point(v);
        if re.is_some() {
            Ok(self.t.get_point(v).unwrap())
        } else {
            let pt = vec![-99999.99999, -99999.99999, -99999.99999];
            Ok(pt)
        }
    }

    /// returns a list of (finite) Triangles (which is a list with 3 indices)
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

    /// returns the convex hull as a list of vertex indices
    fn convex_hull(&self) -> PyResult<Vec<usize>> {
        Ok(self.t.convex_hull())
    }

    /// returns true if (x,y) is inside the convex hull or on the boundary, false otherwise
    #[pyo3(text_signature = "($self, px, py)")]
    fn is_inside_convex_hull(&self, px: f64, py: f64) -> PyResult<bool> {
        let re = self.t.locate(px, py);
        if re.is_none() == true {
            return Ok(false);
        } else {
            Ok(true)
        }
    }

    /// returns true if vertex i is on the boundary of the convex hull, false if not
    #[pyo3(text_signature = "($self, i)")]
    fn is_vertex_convex_hull(&self, v: usize) -> PyResult<bool> {
        Ok(self.t.is_vertex_convex_hull(v))
    }

    /// returns the closest vertex index to (x,y) (distance in 2D)
    #[pyo3(text_signature = "($self, px, py)")]
    fn closest_point(&self, px: f64, py: f64) -> PyResult<usize> {
        let re = self.t.closest_point(px, py);
        if re.is_none() == true {
            return Err(PyErr::new::<exceptions::PyIOError, _>("Outside CH"));
        } else {
            Ok(re.unwrap())
        }
    }

    /// returns a list of Triangles incident to vertex i
    #[pyo3(text_signature = "($self, i)")]
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

    /// returns a list of vertex indices that are adjacent to vertex i
    #[pyo3(text_signature = "($self, i)")]
    fn adjacent_vertices_to_vertex(&self, v: usize) -> PyResult<Vec<usize>> {
        let re = self.t.adjacent_vertices_to_vertex(v);
        if re.is_some() {
            Ok(re.unwrap())
        } else {
            let l: Vec<usize> = Vec::new();
            Ok(l)
        }
    }

    /// returns true if triangle abc exists, false if not
    #[pyo3(text_signature = "($self, t)")]
    fn is_triangle(&self, t: Vec<usize>) -> PyResult<bool> {
        let tr = startin::Triangle {
            v: [t[0], t[1], t[2]],
        };
        Ok(self.t.is_triangle(&tr))
    }

    /// returns the Triangle containing the point (x, y)
    #[pyo3(text_signature = "($self, px, py)")]
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

    /// returns the value, interpolated with the nearest neighbour method, at location (x, y)
    /// an error is thrown if outside the DT
    #[pyo3(text_signature = "($self, px, py)")]
    fn interpolate_nn(&self, px: f64, py: f64) -> PyResult<f64> {
        let re = self.t.interpolate_nn(px, py);
        if re.is_none() {
            return Err(PyErr::new::<exceptions::PyIOError, _>("Outside CH"));
        }
        Ok(re.unwrap())
    }

    /// returns the value, interpolated with the linear interpolation in TIN, at location (x, y)
    /// an error is thrown if outside the DT
    #[pyo3(text_signature = "($self, px, py)")]
    fn interpolate_tin_linear(&self, px: f64, py: f64) -> PyResult<f64> {
        let re = self.t.interpolate_tin_linear(px, py);
        if re.is_none() {
            return Err(PyErr::new::<exceptions::PyIOError, _>("Outside CH"));
        }
        Ok(re.unwrap())
    }

    /// returns the value, interpolated with the Laplace method, at location (x, y)  
    /// an error is thrown if outside the DT
    #[pyo3(text_signature = "($self, px, py)")]
    fn interpolate_laplace(&mut self, px: f64, py: f64) -> PyResult<f64> {
        let re = self.t.interpolate_laplace(px, py);
        if re.is_none() {
            return Err(PyErr::new::<exceptions::PyIOError, _>("Outside CH"));
        }
        Ok(re.unwrap())
    }

    /// returns the value, interpolated with the nearest neighbour method, at location (x, y)  
    /// an error is thrown if outside the DT
    #[pyo3(text_signature = "($self, px, py)")]
    fn interpolate_nni(&mut self, px: f64, py: f64) -> PyResult<f64> {
        let re = self.t.interpolate_nni(px, py);
        if re.is_none() {
            return Err(PyErr::new::<exceptions::PyIOError, _>("Outside CH"));
        }
        Ok(re.unwrap())
    }

    /// writes an OBJ of the DT to the path (a string)
    /// throws an error if the path is invalid
    #[pyo3(text_signature = "($self, path)")]
    fn write_obj(&self, path: String) -> PyResult<()> {
        let re = self.t.write_obj(path.to_string(), false);
        if re.is_err() {
            return Err(PyErr::new::<exceptions::PyIOError, _>("Invalid path"));
        }
        Ok(())
    }

    /// writes an GeoJSON of the DT to the path (a string)
    /// throws an error if the path is invalid
    #[pyo3(text_signature = "($self, path)")]
    fn write_geojson(&self, path: String) -> PyResult<()> {
        let re = self.t.write_geojson(path.to_string());
        if re.is_err() {
            return Err(PyErr::new::<exceptions::PyIOError, _>("Invalid path"));
        }
        Ok(())
    }
}
