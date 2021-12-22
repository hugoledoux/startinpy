use numpy::PyArray;
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

    /// Get the points [x, y, z] of all vertices in the DT (including the infinite one, vertex "0").
    ///
    /// :Example:
    ///
    /// >>> pts = dt.points
    /// >>> print(pts.shape)
    /// (102, 3) #-- this is a numpy array
    /// >>> for p in pts:
    /// >>>     print(p[0], p[1], p[2])
    #[getter]
    fn points<'py>(&self, py: Python<'py>) -> PyResult<&'py PyArray<f64, numpy::Ix2>> {
        let vs = self.t.all_vertices();
        Ok(PyArray::from_vec2(py, &vs).unwrap())
    }

    /// Get the triangles in the DT.
    ///
    /// :Example:
    ///
    /// >>> trs = dt.triangles
    /// >>> print(trs.shape)
    /// (224, 3) #-- this is a numpy array
    /// >>> one_triangle = trs[22]
    /// >>> first_vertex = one_triangle[0]
    /// >>> print("x-coordinate of first vertex: ", dt.points[first_vertex])
    /// x-coordinate of first vertex: [25.98 35.12 4.78]
    #[getter]
    fn triangles<'py>(&self, py: Python<'py>) -> PyResult<&'py PyArray<usize, numpy::Ix2>> {
        let mut trs: Vec<Vec<usize>> = Vec::with_capacity(self.t.number_of_triangles());
        for each in self.t.all_triangles() {
            let mut tr = Vec::with_capacity(3);
            tr.push(each.v[0]);
            tr.push(each.v[1]);
            tr.push(each.v[2]);
            trs.push(tr);
        }
        Ok(PyArray::from_vec2(py, &trs).unwrap())
    }

    /// Insert one new point in the DT.
    /// If there is a point at the same location (based on 2D tolerance),
    /// then the point is not inserted and the index of the already existing vertex is returned.
    ///
    /// :param x: x-coordinate of point to insert
    /// :param y: y-coordinate of point to insert
    /// :param z: z-coordinate of point to insert
    /// :return: index of the vertex in the DT   
    /// :Example:
    ///
    /// >>> dt.insert_one_pt(3.2, 1.1, 17.0)
    /// 5
    /// (the vertex index in the DT is 5)
    #[pyo3(text_signature = "($self, x, y, z)")]
    fn insert_one_pt(&mut self, x: f64, y: f64, z: f64) -> PyResult<usize> {
        let re = self.t.insert_one_pt(x, y, z);
        match re {
            Ok(x) => return Ok(x),
            Err(x) => return Ok(x),
        };
    }

    /// Remove/delete the vertex vi (an index) from the DT, and update the DT for the Delaunay criterion.
    ///
    /// :param vi: index of vertex to delete
    /// :return: (Exception is thrown if *vi* is invalid)
    ///
    /// :Example:
    ///
    /// >>> try:
    /// >>>     t.remove(45)
    /// >>> except Exception as e:
    /// >>>     print(e)
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
    ///
    /// :param pts: a list of points (which is a list)
    /// :return: (nothing)
    ///      
    /// :Example:
    ///
    /// >>> pts = []
    /// >>> pts.append([0.0, 0.0, 11.11])
    /// >>> pts.append([1.0, 0.3, 22.22])
    /// >>> pts.append([12.3, 21.0, 4.52])
    /// >>> dt = startinpy.DT()
    /// >>> dt.insert(pts)
    #[pyo3(text_signature = "($self, pts)")]
    fn insert(&mut self, pts: Vec<Vec<f64>>) {
        self.t.insert(&pts);
    }

    /// Read the LAS/LAZ file (a string) and insert all the points in the DT.
    ///
    /// :param path: full path (a string) on disk of the file to read
    /// :return: throws an exception if the path is invalid
    /// :Example:
    ///
    /// >>> dt = startinpy.DT()
    /// >>> dt.read_las("/home/elvis/myfile.laz")
    /// >>> print("# vertices:", dt.number_of_vertices())
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

    /// Get/set the snap tolerance used to merge vertices during insertion.
    /// Two vertices closer than this will be the merged during insertion.
    /// (default=0.001)
    ///
    /// :Example:
    ///
    /// >>> dt = startinpy.DT()
    /// >>> dt.snap_tolerance = 0.05 #-- modify to 0.05unit
    /// >>> print("The snap tolerance is:", dt.snap_tolerance)
    /// The snap tolerance is: 0.05
    #[getter(snap_tolerance)]
    fn get_snap_tolerance(&self) -> PyResult<f64> {
        Ok(self.t.get_snap_tolerance())
    }

    #[setter(snap_tolerance)]
    fn set_snap_tolerance(&mut self, snaptol: f64) {
        self.t.set_snap_tolerance(snaptol);
    }

    /// :return: number of (finite) vertices    
    fn number_of_vertices(&self) -> PyResult<usize> {
        Ok(self.t.number_of_vertices())
    }

    /// :return: number of (finite) triangles    
    fn number_of_triangles(&self) -> PyResult<usize> {
        Ok(self.t.number_of_triangles())
    }

    /// Return the point for the vertex with index *vi*.
    /// An exception is thrown if vertex index is invalid.
    ///
    /// :param vi: the index of the vertex
    /// :return: the point, a list [x, y, z].
    /// :Example:
    ///
    /// >>> v = dt.get_point(4)
    /// [13.0, 2.0, 11.11]
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

    /// Return the convex hull as a list of vertex indices
    ///
    /// :return: a list of vertex indices, oriented counter-clockwise (CCW)
    fn convex_hull(&self) -> PyResult<Vec<usize>> {
        Ok(self.t.convex_hull())
    }

    /// Is the point [x, y] located inside the convex hull of the DT
    ///
    /// :param x: the x-coordinate
    /// :param y: the y-coordinate
    /// :return: True if [x,y] is inside the convex hull or on its boundary, False otherwise.
    #[pyo3(text_signature = "($self, x, y)")]
    fn is_inside_convex_hull(&self, x: f64, y: f64) -> PyResult<bool> {
        let re = self.t.locate(x, y);
        if re.is_none() == true {
            return Ok(false);
        } else {
            Ok(true)
        }
    }

    /// Return True if vertex *vi* is on the boundary of the convex hull, False otherwise.
    ///
    /// :param vi: the vertex index
    /// :return: True if *vi* is on the boundary of the convex hull, False otherwise.
    ///          Also False is returned if the vertex doesn't exist in the DT.
    #[pyo3(text_signature = "($self, vi)")]
    fn is_vertex_convex_hull(&self, vi: usize) -> PyResult<bool> {
        Ok(self.t.is_vertex_convex_hull(vi))
    }

    /// Return the closest vertex index to [x, y] (distance in 2D).
    /// An Exception is thrown if [x, y] is outside the convex hull.
    ///
    /// :param x: the x-coordinate
    /// :param y: the y-coordinate
    /// :return: the vertex index of the closest.
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

    /// Return the triangles incident to vertex *vi*.
    /// Exception thrown if vertex index doesn't exist in the DT.
    ///
    /// :param vi: the vertex index
    /// :return: a list of triangles (ordered counter-clockwise)
    ///
    /// :Example:
    ///
    /// >>> tri = dt.incident_triangles_to_vertex(3)
    /// >>> for i, dt in enumerate(tri):
    /// >>>     print(i, t)    
    /// 0 [3, 4, 6]
    /// 1 [3, 6, 7]
    /// 2 [3, 7, 8]
    /// 3 [3, 8, 2]
    /// 4 [3, 2, 9]
    /// 5 [3, 9, 4]
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

    /// Return a list of vertex indices that are adjacent to vertex *vi*,
    /// that is those on the edges incident to *vi*.
    /// An exception is thrown if *vi* does not exist in teh DT.
    ///
    /// :param vi: the vertex index
    /// :return: a list of vertex indices (ordered counter-clockwise)
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

    /// Verify if a triangle exists in the DT.
    ///
    /// :param t: the triangle, a list of 3 vertex indices
    /// :return: True if t exists, False otherwise.
    #[pyo3(text_signature = "($self, t)")]
    fn is_triangle(&self, t: Vec<usize>) -> PyResult<bool> {
        let tr = startin::Triangle {
            v: [t[0], t[1], t[2]],
        };
        Ok(self.t.is_triangle(&tr))
    }

    /// Locate the triangle containing the point [x, y] (projected to 2D).
    /// An error is thrown if it is outside the convex hull.
    ///
    /// :param x: the x-coordinate
    /// :param y: the y-coordinate
    /// :return: the triangle.
    #[pyo3(text_signature = "($self, x, y)")]
    fn locate(&self, x: f64, y: f64) -> PyResult<Vec<usize>> {
        let re = self.t.locate(x, y);
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

    /// Interpolation method: nearest neighbour (or closest method).
    /// An Exception is thrown if [x, y] is outside the convex hull.    
    ///
    /// :param x: the x-coordinate
    /// :param y: the y-coordinate
    /// :return: the estimated value
    #[pyo3(text_signature = "($self, x, y)")]
    fn interpolate_nn(&self, x: f64, y: f64) -> PyResult<f64> {
        let re = self.t.interpolate_nn(x, y);
        if re.is_none() {
            return Err(PyErr::new::<exceptions::PyException, _>("Outside CH"));
        }
        Ok(re.unwrap())
    }

    /// Interpolation method: linear interpolation in TIN.
    /// An Exception is thrown if [x, y] is outside the convex hull.    
    ///
    /// :param x: the x-coordinate
    /// :param y: the y-coordinate
    /// :return: the estimated value
    #[pyo3(text_signature = "($self, x, y)")]
    fn interpolate_tin_linear(&self, x: f64, y: f64) -> PyResult<f64> {
        let re = self.t.interpolate_tin_linear(x, y);
        if re.is_none() {
            return Err(PyErr::new::<exceptions::PyException, _>("Outside CH"));
        }
        Ok(re.unwrap())
    }

    /// Interpolation method: Laplace interpolation ([details about the method](http://dilbert.engr.ucdavis.edu/~suku/nem/index.html)).
    /// This is a variation of natural interpolation method with distances used instead of stolen areas.
    /// Thus faster in practice.
    /// An Exception is thrown if [x, y] is outside the convex hull.    
    ///
    /// :param x: the x-coordinate
    /// :param y: the y-coordinate
    /// :return: the estimated value
    #[pyo3(text_signature = "($self, x, y)")]
    fn interpolate_laplace(&mut self, x: f64, y: f64) -> PyResult<f64> {
        let re = self.t.interpolate_laplace(x, y);
        if re.is_none() {
            return Err(PyErr::new::<exceptions::PyException, _>("Outside CH"));
        }
        Ok(re.unwrap())
    }

    /// Interpolation method: natural neighbour method (also called Sibson's method).
    /// An Exception is thrown if [x, y] is outside the convex hull.    
    ///
    /// :param x: the x-coordinate
    /// :param y: the y-coordinate
    /// :return: the estimated value
    #[pyo3(text_signature = "($self, x, y)")]
    fn interpolate_nni(&mut self, x: f64, y: f64) -> PyResult<f64> {
        let re = self.t.interpolate_nni(x, y);
        if re.is_none() {
            return Err(PyErr::new::<exceptions::PyException, _>("Outside CH"));
        }
        Ok(re.unwrap())
    }

    /// Write an OBJ of the DT to the path (a string).
    /// Throws an exception if the path is invalid.
    ///
    /// :param path: full path (a string) on disk of the file to create (will overwrite)
    /// :return: (nothing)
    /// :Example:
    ///
    /// >>> dt.write_obj("/home/elvis/myfile.obj")
    #[pyo3(text_signature = "($self, path)")]
    fn write_obj(&self, path: String) -> PyResult<()> {
        let re = self.t.write_obj(path.to_string(), false);
        if re.is_err() {
            return Err(PyErr::new::<exceptions::PyIOError, _>("Invalid path"));
        }
        Ok(())
    }

    /// Write a GeoJSON file of the DT (vertices+triangles) to the path (a string).
    /// Throws an exception if the path is invalid.
    ///
    /// :param path: full path (a string) on disk of the file to create (will overwrite)
    /// :return: (nothing)
    /// :Example:
    ///
    /// >>> dt.write_obj("/home/elvis/myfile.geojson")
    #[pyo3(text_signature = "($self, path)")]
    fn write_geojson(&self, path: String) -> PyResult<()> {
        let re = self.t.write_geojson(path.to_string());
        if re.is_err() {
            return Err(PyErr::new::<exceptions::PyIOError, _>("Invalid path"));
        }
        Ok(())
    }
}
