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
    /// Returns the index of the vertices.
    /// If there is a point at the same location (based on 2D tolerance),
    /// then the point is not inserted and the index of the already existing vertex is returned.
    #[pyo3(text_signature = "($self, x, y, z)")]
    fn insert_one_pt(&mut self, x: f64, y: f64, z: f64) -> PyResult<usize> {
        let re = self.t.insert_one_pt(x, y, z);
        match re {
            Ok(x) => return Ok(x),
            Err(x) => return Ok(x),
        };
    }

    /// Remove/delete the vertex vi (an index) from the DT, and update the DT for the Delaunay criterion.
    /// Returns 1 if the operation was successful; and -1 if the vertex doesn't exist.
    #[pyo3(text_signature = "($self, vi)")]
    fn remove(&mut self, vi: usize) -> PyResult<()> {
        let re = self.t.remove(vi);
        match re {
            Ok(_x) => return Ok(()),
            Err(_x) => {
                if _x == "Cannot remove the infinite vertex" {
                    return Err(PyErr::new::<exceptions::PyIndexError, _>(
                        "Invalid index, cannot remove infinite vertex.",
                    ));
                } else {
                    return Err(PyErr::new::<exceptions::PyIndexError, _>(
                        "Invalid index, vertex doesn't exist.",
                    ));
                }
            }
        };
    }

    /// Insert each point in the array of points (a 2D array) by calling insert_one_pt() for each.
    /// Return nothing.           
    #[pyo3(text_signature = "($self, pts)")]
    fn insert(&mut self, pts: Vec<Vec<f64>>) {
        self.t.insert(&pts);
    }

    /// Read the LAS/LAZ file "path" (a string) and insert all the points in the DT.
    /// Throws an error if the path is invalid.
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

    /// Return the snap tolerance (2 vertices closer than this will be the merged during insertion)
    fn get_snap_tolerance(&self) -> PyResult<f64> {
        Ok(self.t.get_snap_tolerance())
    }

    /// Set the snap tolerance (for insertion of points in the DT) to this value.
    /// (default=0.001)
    /// Returns nothing.
    #[pyo3(text_signature = "($self, snaptol)")]
    fn set_snap_tolerance(&mut self, snaptol: f64) {
        self.t.set_snap_tolerance(snaptol);
    }

    /// Return the number of (finite) vertices in the DT.
    fn number_of_vertices(&self) -> PyResult<usize> {
        Ok(self.t.number_of_vertices())
    }

    /// Return the number of (finite) triangles in the DT.
    fn number_of_triangles(&self) -> PyResult<usize> {
        Ok(self.t.number_of_triangles())
    }

    /// Return a list of all vertices in the DT (including the infinite one, vertex "0").
    fn all_vertices(&self) -> PyResult<Vec<Vec<f64>>> {
        Ok(self.t.all_vertices())
    }

    /// Return the point (x, y, z) for the vertex with index vi.
    /// Exception is thrown is vertex index is invalid.
    #[pyo3(text_signature = "($self, vi)")]
    fn get_point(&self, vi: usize) -> PyResult<Vec<f64>> {
        let re = self.t.get_point(vi);
        if re.is_some() {
            Ok(self.t.get_point(vi).unwrap())
        } else {
            return Err(PyErr::new::<exceptions::PyIndexError, _>(
                "Invalid vertex index.",
            ));
        }
    }

    /// Return a list of (finite) triangles (which is a list with 3 indices)
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

    /// Return the convex hull as a list of vertex indices (oriented CCW)
    fn convex_hull(&self) -> PyResult<Vec<usize>> {
        Ok(self.t.convex_hull())
    }

    /// Return True if (x,y) is inside the convex hull or on its boundary, False otherwise.
    #[pyo3(text_signature = "($self, px, py)")]
    fn is_inside_convex_hull(&self, px: f64, py: f64) -> PyResult<bool> {
        let re = self.t.locate(px, py);
        if re.is_none() == true {
            return Ok(false);
        } else {
            Ok(true)
        }
    }

    /// Return True if vertex vi is on the boundary of the convex hull, False otherwise.
    #[pyo3(text_signature = "($self, vi)")]
    fn is_vertex_convex_hull(&self, vi: usize) -> PyResult<bool> {
        Ok(self.t.is_vertex_convex_hull(vi))
    }

    /// Returns the closest vertex index to (x,y) (distance in 2D).
    #[pyo3(text_signature = "($self, x, y)")]
    fn closest_point(&self, x: f64, y: f64) -> PyResult<usize> {
        let re = self.t.closest_point(x, y);
        if re.is_none() == true {
            return Err(PyErr::new::<exceptions::PyException, _>(
                "(x, y) is outside the convex hull.",
            ));
        } else {
            Ok(re.unwrap())
        }
    }

    /// Return a list of triangles incident to vertex vi (ordered CCW).
    /// Exception thrown is vertex index is invalid.
    #[pyo3(text_signature = "($self, vi)")]
    fn incident_triangles_to_vertex(&self, vi: usize) -> PyResult<Vec<Vec<usize>>> {
        let re = self.t.incident_triangles_to_vertex(vi);
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
            return Err(PyErr::new::<exceptions::PyIndexError, _>(
                "Invalid vertex index.",
            ));
        }
    }

    /// Return a list of vertex indices that are adjacent to vertex vi.
    /// Exception thrown is vertex index is invalid.
    #[pyo3(text_signature = "($self, vi)")]
    fn adjacent_vertices_to_vertex(&self, vi: usize) -> PyResult<Vec<usize>> {
        let re = self.t.adjacent_vertices_to_vertex(vi);
        if re.is_some() {
            Ok(re.unwrap())
        } else {
            return Err(PyErr::new::<exceptions::PyIndexError, _>(
                "Invalid vertex index.",
            ));
        }
    }

    /// Return True if triangle [a, b, c] exists, False otherwise.
    #[pyo3(text_signature = "($self, t)")]
    fn is_triangle(&self, t: Vec<usize>) -> PyResult<bool> {
        let tr = startin::Triangle {
            v: [t[0], t[1], t[2]],
        };
        Ok(self.t.is_triangle(&tr))
    }

    /// Returns the triangle containing the point (x, y) (projected to 2D),
    /// An error is thrown if (x, y) is outside the convex hull.
    #[pyo3(text_signature = "($self, px, py)")]
    fn locate(&self, px: f64, py: f64) -> PyResult<Vec<usize>> {
        let re = self.t.locate(px, py);
        let mut tr: Vec<usize> = Vec::new();
        if re.is_some() {
            let t = re.unwrap();
            tr.push(t.v[0]);
            tr.push(t.v[1]);
            tr.push(t.v[2]);
            return Ok(tr);
        } else {
            return Err(PyErr::new::<exceptions::PyException, _>("Outside CH"));
        }
    }

    /// Return the value interpolated with the nearest neighbour method,
    /// at location (x, y).
    /// An error is thrown if (x, y) is outside the convex hull.
    #[pyo3(text_signature = "($self, x, y)")]
    fn interpolate_nn(&self, x: f64, y: f64) -> PyResult<f64> {
        let re = self.t.interpolate_nn(x, y);
        if re.is_none() {
            return Err(PyErr::new::<exceptions::PyException, _>("Outside CH"));
        }
        Ok(re.unwrap())
    }

    /// Return the value interpolated with the linear interpolation in TIN method,
    /// at location (x, y).
    /// An error is thrown if (x, y) is outside the convex hull.
    #[pyo3(text_signature = "($self, x, y)")]
    fn interpolate_tin_linear(&self, x: f64, y: f64) -> PyResult<f64> {
        let re = self.t.interpolate_tin_linear(x, y);
        if re.is_none() {
            return Err(PyErr::new::<exceptions::PyException, _>("Outside CH"));
        }
        Ok(re.unwrap())
    }

    /// Return the value interpolated with the Laplace interpolation method
    /// (http://dilbert.engr.ucdavis.edu/~suku/nem/index.html), which is a variation
    /// of natural interpolation method with distances used instead of stolen areas.
    /// Thus faster in practice.
    /// An error is thrown if (x, y) is outside the convex hull.
    #[pyo3(text_signature = "($self, x, y)")]
    fn interpolate_laplace(&mut self, x: f64, y: f64) -> PyResult<f64> {
        let re = self.t.interpolate_laplace(x, y);
        if re.is_none() {
            return Err(PyErr::new::<exceptions::PyException, _>("Outside CH"));
        }
        Ok(re.unwrap())
    }

    /// Return the value interpolated with the natural neighbour interpolation method,
    /// at location (x, y).
    /// An error is thrown if (x, y) is outside the convex hull.
    #[pyo3(text_signature = "($self, x, y)")]
    fn interpolate_nni(&mut self, x: f64, y: f64) -> PyResult<f64> {
        let re = self.t.interpolate_nni(x, y);
        if re.is_none() {
            return Err(PyErr::new::<exceptions::PyException, _>("Outside CH"));
        }
        Ok(re.unwrap())
    }

    /// Write an OBJ of the DT to the path (a string).
    /// Throw an error if the path is invalid.
    #[pyo3(text_signature = "($self, path)")]
    fn write_obj(&self, path: String) -> PyResult<()> {
        let re = self.t.write_obj(path.to_string(), false);
        if re.is_err() {
            return Err(PyErr::new::<exceptions::PyIOError, _>("Invalid path"));
        }
        Ok(())
    }

    /// Write an GeoJSON of the DT to the path (a string).
    /// Throw an error if the path is invalid.
    #[pyo3(text_signature = "($self, path)")]
    fn write_geojson(&self, path: String) -> PyResult<()> {
        let re = self.t.write_geojson(path.to_string());
        if re.is_err() {
            return Err(PyErr::new::<exceptions::PyIOError, _>("Invalid path"));
        }
        Ok(())
    }
}
