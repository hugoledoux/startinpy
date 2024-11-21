extern crate startin;

use numpy::{PyArray, PyArrayDescr};
use pyo3::exceptions;

use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList, PyTuple};

use std::fs::File;
use std::io::Write;

use geojson::{Feature, FeatureCollection, Geometry, Value as GeoValue};
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::Value;
use serde_json::{to_value, Map};

#[derive(Debug, Serialize, Deserialize)]
struct Cityjson {
    r#type: String,
    version: String,
    transform: Value,
    #[serde(rename = "CityObjects")]
    city_objects: Value,
    vertices: Vec<Vec<i64>>,
}

fn convert_json_value_to_pyobject(py: Python, value: &Value) -> PyResult<PyObject> {
    match value {
        Value::Null => Ok(py.None()),
        Value::Bool(b) => Ok(b.to_object(py)),
        Value::Number(num) => {
            if let Some(i) = num.as_i64() {
                Ok(i.to_object(py))
            } else if let Some(u) = num.as_u64() {
                Ok(u.to_object(py))
            } else if let Some(f) = num.as_f64() {
                Ok(f.to_object(py))
            } else {
                Err(pyo3::exceptions::PyTypeError::new_err("Invalid number"))
            }
        }
        Value::String(s) => Ok(s.to_object(py)),
        Value::Array(arr) => {
            let py_list = PyList::new(
                py,
                arr.iter()
                    .map(|v| convert_json_value_to_pyobject(py, v))
                    .collect::<Result<Vec<_>, _>>()?,
            );
            Ok(py_list.to_object(py))
        }
        Value::Object(map) => {
            let py_dict = PyDict::new(py);
            for (k, v) in map {
                py_dict.set_item(k, convert_json_value_to_pyobject(py, v)?)?;
            }
            Ok(py_dict.to_object(py))
        }
    }
}

/// A Delaunay triangulator where the input are 2.5D points,
/// the DT is computed in 2D but the elevation of the vertices are kept.
/// This is used mostly for the modelling of terrains.
/// This is the Python bindings of Rust's startin:
/// (https://github.com/hugoledoux/startin)
#[pymodule]
fn startinpy(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<DT>()?;
    Ok(())
}

#[pyclass(unsendable)]
/// A Delaunay triangulation (DT), containing vertices+triangles
pub struct DT {
    t: startin::Triangulation,
    dtype: Vec<(String, String)>,
}

#[pymethods]
impl DT {
    /// Constructor for a DT (returns an empty DT).
    ///
    /// :param attributes_schema: Optional schema for attributes.
    /// :type attributes_schema: Optional[PyAny]
    #[new]
    fn new(attributes_schema: Option<&PyAny>) -> Self {
        let tmp = startin::Triangulation::new();
        let tmp2 = Vec::new();
        let mut dt = DT {
            t: tmp,
            dtype: tmp2,
        };
        if attributes_schema.is_some() {
            let _ = dt.set_attributes_schema(&attributes_schema.unwrap());
        }
        dt
    }

    /// Get the points [x, y, z] of all vertices in the DT.
    /// This includes the infinite vertex (vertex at position 0), which is not part of the DT
    /// (no finite Triangle reference it, but infinite Triangles do)
    ///
    /// >>> pts = dt.points
    /// >>> print(pts.shape) #-- this is a numpy array
    /// (102, 3)
    /// >>> for p in pts:
    /// >>>     print(p[0], p[1], p[2])
    /// ...
    /// >>> dt.points[27]
    /// array([101.122, 72.293, 11.223])
    /// >>> dt.points[0]
    /// array([inf, inf, inf])
    #[getter]
    fn points<'py>(&self, py: Python<'py>) -> PyResult<&'py PyArray<f64, numpy::Ix2>> {
        let vs = self.t.all_vertices();
        Ok(PyArray::from_vec2(py, &vs).unwrap())
    }

    /// Get the triangles in the DT (only finite triangles).
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
        for each in self.t.all_finite_triangles() {
            let mut tr = Vec::with_capacity(3);
            tr.push(each.v[0]);
            tr.push(each.v[1]);
            tr.push(each.v[2]);
            trs.push(tr);
        }
        Ok(PyArray::from_vec2(py, &trs).unwrap())
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(format!("{}", self.t))
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{}", self.t))
    }

    /// Insert one new point in the DT.
    ///
    /// If there is a point at the same location (based on :func:`startinpy.DT.snap_tolerance`),
    /// then :func:`startinpy.DT.duplicates_handling` decides which z-value (and eventually extra
    /// attributes) are kept.
    ///
    /// :param p3: array with [x, y, z]-coordinates of point to insert
    /// :param optional extra attributes: extra parameters with values
    /// :return: a tuple containing:
    ///          1) the index of the (created or kept) vertex in the triangulation;
    ///          2) whether a new vertex was inserted: True if yes; False is there was already
    ///          a vertex at that xy-location.
    ///          3) whether the z/attributes were updated or not, based on
    ///          the :func:`startinpy.DT.duplicates_handling`
    ///
    /// >>> (vi, bNewVertex, bZUpdated) = dt.insert_one_pt([3.2, 1.1, 17.0])
    /// (37, True)
    /// >>> dt.insert_one_pt([13.2, 44.1, 74.2], intensity=77.2)
    #[pyo3(signature = (p3, **py_kwargs))]
    fn insert_one_pt(
        &mut self,
        p3: [f64; 3],
        py_kwargs: Option<&PyDict>,
    ) -> PyResult<(usize, bool, bool)> {
        // Result<usize, (usize, bool)>
        let re = self.t.insert_one_pt(p3[0], p3[1], p3[2]);
        match re {
            Ok(x) => {
                let _ = self.set_vertex_attributes(x, py_kwargs);
                return Ok((x, true, true));
            }
            Err((x, b)) => {
                if b == true {
                    let _ = self.set_vertex_attributes(x, py_kwargs);
                }
                return Ok((x, false, b));
            }
        };
    }

    /// Remove/delete the vertex vi (an index) from the DT, and update the DT for the Delaunay criterion.
    ///
    /// :param vi: index of vertex to delete
    /// :return: (Exception is thrown if *vi* is invalid)
    ///
    /// >>> try:
    /// >>>     t.remove(45)
    /// >>> except Exception as e:
    /// >>>     print(e)
    fn remove(&mut self, vi: usize) -> PyResult<()> {
        let re = self.t.remove(vi);
        match re {
            Ok(_x) => return Ok(()),
            Err(why) => match why {
                startin::StartinError::VertexInfinite => {
                    return Err(exceptions::PyIndexError::new_err(
                        "Invalid vertex index: cannot remove infinite vertex",
                    ));
                }
                _ => {
                    return Err(exceptions::PyIndexError::new_err(
                        "Invalid vertex index: vertex doesn't exist",
                    ));
                }
            },
        };
    }

    /// Insert each point in the array of points (a 2D array) by calling insert_one_pt() for each.
    /// Different insertion strategies can be used: "AsIs" (*default*: inserts points in the order
    /// given) or "BBox" (inserts first the BBox of the points, which speeds up the construction,
    /// works especially good for rasters).
    ///
    /// :param pts: an array of points (which is itself an array)
    /// :param optional insertionstrategy:  "AsIs" (*default*) or "BBox"
    /// :return: (nothing)
    ///
    /// >>> pts = []
    /// >>> pts.append([1.0, 1.0, 11.11])
    /// >>> pts.append([1.0, 2.3, 22.22])
    /// >>> pts.append([12.3, 21.0, 4.52])
    /// >>> ...
    /// >>> dt = startinpy.DT()
    /// >>> dt.insert(pts)
    /// OR
    /// >>> dt.insert(pts, insertionstrategy="BBox")
    #[pyo3(signature = (pts, insertionstrategy="AsIs"))]
    fn insert(&mut self, pts: Vec<[f64; 3]>, insertionstrategy: &str) -> PyResult<()> {
        match insertionstrategy {
            "AsIs" => self.t.insert(&pts, startin::InsertionStrategy::AsIs),
            "BBox" => self.t.insert(&pts, startin::InsertionStrategy::BBox),
            _ => {
                let s = format!(
                    "'{}' is an unknown insertion strategy for insert()",
                    insertionstrategy
                );
                return Err(exceptions::PyAttributeError::new_err(s));
            }
        }
        Ok(())
    }

    /// Get/set the snap tolerance used to merge vertices during insertion.
    /// Two vertices closer than this will be the merged during insertion.
    /// (default=0.001)
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

    /// Activate/deactivate the jump-and-walk for the point location.
    /// If deactivated the walk starts from the last inserted triangle.
    /// This should be activate when the spatial coherence in the dataset
    /// is very low (ie if the points are randomly shuffled)
    /// (default=False)
    ///
    /// >>> dt = startinpy.DT()
    /// >>> dt.jump_and_walk = True
    #[getter(jump_and_walk)]
    fn get_jump_and_walk(&self) -> PyResult<bool> {
        Ok(self.t.get_jump_and_walk())
    }

    #[setter(jump_and_walk)]
    fn set_jump_and_walk(&mut self, b: bool) {
        self.t.set_jump_and_walk(b);
    }

    /// Specify the method to handle xy-duplicates.
    /// That is, if the insertion of a new point in the DT is impossible
    /// because a vertex already exists (based on :func:`startinpy.DT.snap_tolerance`),
    /// then we can decide which z-value we want to keep in the DT.
    /// There are 4 options:
    ///
    /// 1. "First" (default): the z-value of the first point inserted at that xy-location is kept
    /// 2. "Last": the z-value of the last point inserted at that xy-location is kept
    /// 3. "Lowest": the lowest z-value is kept
    /// 4. "Highest": the highest z-value is kept
    ///
    /// >>> dt = startinpy.DT()
    /// >>> dt.duplicates_handling = "Highest"
    #[getter(duplicates_handling)]
    fn get_duplicates_handling(&self) -> PyResult<String> {
        Ok(self.t.get_duplicates_handling())
    }

    #[setter(duplicates_handling)]
    fn set_duplicates_handling(&mut self, m: &str) -> PyResult<()> {
        match m {
            "First" => self
                .t
                .set_duplicates_handling(startin::DuplicateHandling::First),
            "Last" => self
                .t
                .set_duplicates_handling(startin::DuplicateHandling::Last),
            "Highest" => self
                .t
                .set_duplicates_handling(startin::DuplicateHandling::Highest),
            "Lowest" => self
                .t
                .set_duplicates_handling(startin::DuplicateHandling::Lowest),

            _ => {
                let s = format!(
                    "'{}' is an unknown method to handle duplicates (First/Last/Lowest/Highest)",
                    m
                );
                return Err(exceptions::PyAttributeError::new_err(s));
            }
        }
        Ok(())
    }

    /// Set the attribute schema (the definition of the data type) for the
    /// extra attributes that each vertex can store.
    ///
    /// If that function is used, all previous stored attributes and
    /// schema will be removed.
    ///            
    /// Adding attributes to a triangulation that has no schema will
    /// result in no attributes stored, only those compliant with the
    /// schema are stored.
    ///
    /// Only the following data types for each attribute are allowed:
    /// 'numpy.bool_', 'numpy.int64', 'numpy.uint64', unicode (string), 'numpy.float64'.
    ///
    /// :param dtype: a `NumPy Data type object (dtype) <https://numpy.org/doc/stable/reference/arrays.dtypes.html#arrays-dtypes>`_
    /// :return: True if the schema is valid, otherwise an error is thrown.
    ///
    /// >>> dt = startinpy.DT()
    /// >>> myschema = np.dtype([('classification', np.uint64), ('intensity', float)])
    /// >>> dt.set_attributes_schema(myschema)
    /// >>> dt.insert_one_pt([85000.0, 444003.2, 2.2], classification=2, intensity=111.1)
    #[pyo3(signature = (dtype))]
    fn set_attributes_schema(&mut self, dtype: &PyAny) -> PyResult<bool> {
        let descr: &PyArrayDescr = dtype.extract()?;
        let names: &PyTuple = descr.getattr("names")?.extract()?;
        let mut v: Vec<(String, String)> = Vec::new();
        self.dtype.clear();
        for name in names.iter() {
            let name: &str = name.extract()?;
            let field = descr.getattr("fields")?.get_item(name)?;
            let field_type = field.get_item(0)?;
            // println!("{:?}", field_type);
            match field_type.to_string().as_ref() {
                "bool" => {
                    v.push((name.to_string(), "bool".to_string()));
                    self.dtype.push((name.to_string(), "?".to_string()));
                }
                "float32" => {
                    v.push((name.to_string(), "f64".to_string()));
                    self.dtype.push((name.to_string(), "<f4".to_string()));
                }
                "float64" => {
                    v.push((name.to_string(), "f64".to_string()));
                    self.dtype.push((name.to_string(), "<f8".to_string()));
                }
                "int32" => {
                    v.push((name.to_string(), "i64".to_string()));
                    self.dtype.push((name.to_string(), "<i4".to_string()));
                }
                "int64" => {
                    v.push((name.to_string(), "i64".to_string()));
                    self.dtype.push((name.to_string(), "<i8".to_string()));
                }
                "uint32" => {
                    v.push((name.to_string(), "u64".to_string()));
                    self.dtype.push((name.to_string(), "<u4".to_string()));
                }
                "uint64" => {
                    v.push((name.to_string(), "u64".to_string()));
                    self.dtype.push((name.to_string(), "<u8".to_string()));
                }
                other if other.starts_with("<U") => {
                    v.push((name.to_string(), "String".to_string()));
                    self.dtype.push((name.to_string(), other.to_string()));
                }
                _ => {
                    return {
                        Err(PyErr::new::<pyo3::exceptions::PyAttributeError, _>(
                            format!("{} is not a valid dype for startinpy", field_type),
                        ))
                    };
                }
            };
        }
        let _ = self.t.set_attributes_schema(v);
        Ok(true)
    }

    /// Get the attribute schema that contains the data type definitions
    /// for the extra attributes (if any).
    ///
    /// :return: a NumPy Data type object (dtype)
    ///
    /// >>> d = np.dtype([('classification', np.float64), ('name', '<U8')])
    /// >>> dt.set_attributes_schema(d)
    /// True
    /// >>> dt.get_attributes_schema()
    /// [('classification', '<f8'), ('name', '<U8')]
    fn get_attributes_schema(&self) -> Vec<(String, String)> {
        self.dtype.clone()
    }

    /// Get all the values for a given extra attribute stored for the vertices.
    /// Returns the values as a numpy structured array.
    /// Watch out, if a given vertex doesn't have a specific attribute then ``np.nan`` is inserted
    /// for f64, max-values for i64 and u64, "" for String, 0 for bool.
    ///
    /// :return: an NumPy array with all the values (including for the removed vertices and the infinite
    ///          vertex). The array is empty if the extra attributes don't exist.
    ///
    /// >>> dt = startinpy.DT()
    /// >>> dt.add_attribute_map(np.dtype([("classification", np.uint64)]))
    /// >>> dt.insert_one_pt([85000.0, 444003.2, 2.2], classification=6)
    /// >>> ...
    /// >>> dt.attributes[1:]
    /// array([6, 2, 6, 6, ..., 6, 9])
    #[getter]
    fn attributes<'py>(&self, py: Python<'py>) -> PyResult<PyObject> {
        let np = py.import("numpy")?;
        let dtype = np.call_method1("dtype", (self.dtype.clone(),))?;
        let allt = self.t.all_attributes();
        if allt.is_none() {
            let arraydtype = np.call_method1("empty", (0, dtype))?;
            return Ok(arraydtype.into());
        }
        let allt = allt.unwrap();
        let arraydtype = np.call_method1("empty", (allt.len(), dtype))?;
        for (i, each) in allt.iter().enumerate() {
            let item = arraydtype.get_item(i)?;
            let o = each.as_object().unwrap();
            for (key, dtype) in &self.t.get_attributes_schema() {
                match o.get(key) {
                    Some(x) => {
                        match dtype.as_ref() {
                            "f64" => item.set_item(key, x.as_f64())?, // TODO: dtype and f32 and f64 works?
                            "i64" => item.set_item(key, x.as_i64())?,
                            "u64" => item.set_item(key, x.as_u64())?,
                            "bool" => item.set_item(key, x.as_bool())?,
                            "String" => item.set_item(key, x.as_str())?,
                            &_ => continue,
                        };
                    }
                    None => {
                        match dtype.as_ref() {
                            "f64" => item.set_item(key, std::f64::NAN)?,
                            "i64" => item.set_item(key, std::i64::MAX)?,
                            "u64" => item.set_item(key, std::u64::MAX)?,
                            "bool" => item.set_item(key, false)?,
                            "String" => item.set_item(key, "".to_string())?,
                            &_ => continue,
                        };
                    }
                }
            }
        }
        Ok(arraydtype.into())
    }

    /// Get all the extra attributes stored for a specific vertex.
    /// Returns the values as a JSON dictionary in string.
    /// An exception is thrown if the terrain has no extra attributes stored and/or if
    /// the vertex index is invalid.
    ///
    /// :param vi: the index of the vertex
    /// :return: a JSON object
    ///
    /// >>> dt.get_vertex_attributes(17)
    /// {'intensity': 111.1, 'reflectance': 99.1}
    #[pyo3(signature = (vi))]
    fn get_vertex_attributes(&self, vi: usize) -> PyResult<PyObject> {
        match self.t.get_vertex_attributes(vi) {
            Ok(v) => {
                // Convert serde_json::Value to Python object
                Python::with_gil(|py| {
                    let json_object = convert_json_value_to_pyobject(py, &v)?;
                    Ok(json_object)
                })
            }
            Err(e) => match e {
                startin::StartinError::VertexRemoved | startin::StartinError::VertexUnknown => {
                    return Err(exceptions::PyIndexError::new_err("Invalid vertex index"))
                }
                startin::StartinError::TinHasNoAttributes => {
                    return Err(exceptions::PyException::new_err(
                        "TIN has no extra attributes",
                    ))
                }
                _ => return Err(exceptions::PyException::new_err("Error")),
            },
        }
    }

    /// Add new attributes to a vertex (even if it already has some).
    /// Returns the values as a JSON dictionary in string.
    ///
    /// :param vi: the index of the vertex
    /// :param optional extra_attributes: extra parameters with values
    /// :return: True if the attribute was assigned, False otherwise
    ///
    /// >>> dt.insert_one_pt([85000.0, 444003.2, 2.2], intensity=111.1, reflectance=29.9)
    /// >>> ...
    /// >>> dt.set_vertex_attributes(17, classification=2)
    /// >>> dt.get_vertex_attributes(17)
    /// {'intensity': 111.1, 'reflectance': 29.9, 'classification': 2, }'    
    #[pyo3(signature = (vi, **py_kwargs))]
    fn set_vertex_attributes(&mut self, vi: usize, py_kwargs: Option<&PyDict>) -> PyResult<bool> {
        let mut m = Map::new();
        if py_kwargs.is_some() {
            let tmp = py_kwargs.unwrap();
            let keys = tmp.keys();
            let am = self.t.get_attributes_schema();
            for k in keys {
                let b: &String = &k.extract()?;
                let c = am.iter().position(|(first, _)| first == b);
                if c.is_some() {
                    match am[c.unwrap()].1.as_ref() {
                        "f64" => {
                            let t1: f64 = tmp.get_item(b).unwrap().extract()?;
                            m.insert(b.to_string(), t1.into());
                        }
                        "i64" => {
                            let t1: i64 = tmp.get_item(b).unwrap().extract()?;
                            m.insert(b.to_string(), t1.into());
                        }
                        "u64" => {
                            let t1: u64 = tmp.get_item(b).unwrap().extract()?;
                            m.insert(b.to_string(), t1.into());
                        }
                        "bool" => {
                            let t1: bool = tmp.get_item(b).unwrap().extract()?;
                            m.insert(b.to_string(), t1.into());
                        }
                        "String" => {
                            let t1: String = tmp.get_item(b).unwrap().extract()?;
                            m.insert(b.to_string(), t1.into());
                        }
                        &_ => continue,
                    }
                }
            }
        }
        match self
            .t
            .add_vertex_attributes(vi, serde_json::to_value(m).unwrap())
        {
            Ok(b) => return Ok(b),
            Err(_) => return Ok(false),
        }
    }

    /// Calculate the area in 2D of a given triangle (projection xy-plane).
    /// An exception is thrown if the triangle doesn't exist or is infinite.
    ///
    /// :param t: the Triangle, an array of 3 vertex indices
    /// :return: the area in 2D
    ///
    /// >>> dt.area2d_triangle([34, 21, 1])
    /// 22.1
    #[pyo3(signature = (t))]
    fn area2d_triangle(&self, t: Vec<usize>) -> PyResult<f64> {
        let tr = startin::Triangle {
            v: [t[0], t[1], t[2]],
        };
        match self.t.area2d_triangle(&tr) {
            Ok(b) => return Ok(b),
            Err(_) => return Err(exceptions::PyIndexError::new_err("Invalid vertex index")),
        }
    }

    /// Calculate the area in 3D of a given triangle.
    /// An exception is thrown if the triangle doesn't exist or is infinite.
    ///
    /// :param t: the Triangle, an array of 3 vertex indices
    /// :return: the area in 3D
    ///
    /// >>> dt.area3d_triangle([34, 21, 1])
    /// 32.2
    #[pyo3(signature = (t))]
    fn area3d_triangle(&self, t: Vec<usize>) -> PyResult<f64> {
        let tr = startin::Triangle {
            v: [t[0], t[1], t[2]],
        };
        match self.t.area3d_triangle(&tr) {
            Ok(b) => return Ok(b),
            Err(_) => return Err(exceptions::PyIndexError::new_err("Invalid vertex index")),
        }
    }

    /// Calculate the volume of a given triangle wrt to a base z-plane.
    /// An exception is thrown if the triangle doesn't exist or is infinite.
    ///
    /// :param t: the Triangle, an array of 3 vertex indices
    /// :param zplane: (default=0.0)the z-value of the base plane
    /// :return: the signed volume in 3D
    ///
    /// >>> dt.volume_triangle([34, 21, 1], 10.0)
    /// 32.2
    #[pyo3(signature = (t, zplane=0.0))]
    fn volume_triangle(&self, t: Vec<usize>, zplane: f64) -> PyResult<f64> {
        let tr = startin::Triangle {
            v: [t[0], t[1], t[2]],
        };
        match self.t.volume_triangle(&tr, zplane) {
            Ok(b) => return Ok(b),
            Err(_) => return Err(exceptions::PyIndexError::new_err("Invalid vertex index")),
        }
    }

    /// Calculate the normal of a given vertex.
    /// An exception is thrown if the vertex index is invalid.
    ///
    /// :param vi: the index of the vertex
    /// :return: a Vec with 3 values: nx, ny, nz (normalised normal)
    ///
    /// >>> dt.normal_vertex(17)
    /// >>> dt.points[17]
    /// array([15.63303377, 26.9968598 ,  23.4])
    #[pyo3(signature = (vi))]
    fn normal_vertex(&self, vi: usize) -> PyResult<Vec<f64>> {
        match self.t.normal_vertex(vi) {
            Ok(b) => return Ok(b),
            Err(_) => return Err(exceptions::PyIndexError::new_err("Invalid vertex index")),
        }
    }

    /// Calculate the normal of a given triangle.
    /// An exception is thrown if the triangle doesn't exist or is infinite.
    ///
    /// :param t: the Triangle, an array of 3 vertex indices
    /// :return: a Vec with 3 values: nx, ny, nz (normalised normal)
    ///
    /// >>> dt.normal_triangle([17, 451, 22])
    /// >>>
    /// array([15.63303377, 26.9968598 ,  23.4])
    #[pyo3(signature = (t))]
    fn normal_triangle(&self, t: Vec<usize>) -> PyResult<Vec<f64>> {
        let tr = startin::Triangle {
            v: [t[0], t[1], t[2]],
        };
        match self.t.normal_triangle(&tr) {
            Ok(b) => return Ok(b),
            Err(_) => return Err(exceptions::PyIndexError::new_err("Invalid Triangle")),
        }
    }

    /// Update/set the z-value for a specific vertex.
    /// An exception is thrown if the vertex index is invalid.
    ///
    /// :param vi: the index of the vertex
    /// :param z: the new z-value/elevation
    /// :return: True if the attribute was assigned, False otherwise
    ///
    /// >>> dt.update_vertex_z_value(17, 23.4)
    /// >>> dt.points[17]
    /// array([15.63303377, 26.9968598 ,  23.4])
    #[pyo3(signature = (vi, z))]
    fn update_vertex_z_value(&mut self, vi: usize, z: f64) -> PyResult<bool> {
        match self.t.update_vertex_z_value(vi, z) {
            Ok(b) => return Ok(b),
            Err(_) => return Ok(false),
        }
    }

    /// :return: number of finite vertices    
    fn number_of_vertices(&self) -> PyResult<usize> {
        Ok(self.t.number_of_vertices())
    }

    /// :return: number of finite triangles    
    fn number_of_triangles(&self) -> PyResult<usize> {
        Ok(self.t.number_of_triangles())
    }

    /// Return the point for the vertex with index *vi*.
    /// An exception is thrown if the vertex index is invalid.
    ///
    /// :param vi: the index of the vertex
    /// :return: the point
    ///
    /// >>> v = dt.get_point(4)
    /// array([13., 2.0, 11.])
    #[pyo3(signature = (vi))]
    fn get_point<'py>(
        &self,
        py: Python<'py>,
        vi: usize,
    ) -> PyResult<&'py PyArray<f64, numpy::Ix1>> {
        let re = self.t.get_point(vi);
        if re.is_ok() {
            return Ok(PyArray::from_vec(py, re.unwrap()));
        } else {
            return Err(exceptions::PyIndexError::new_err("Invalid vertex index"));
        }
    }

    /// Return the convex hull as an array of vertex indices.
    ///
    /// :return: an array of vertex indices, oriented counter-clockwise (CCW)
    ///
    /// >>> dt.convex_hull()
    /// array([2, 13, 4, 51, 27], dtype=uint64)
    fn convex_hull<'py>(&self, py: Python<'py>) -> PyResult<&'py PyArray<usize, numpy::Ix1>> {
        Ok(PyArray::from_vec(py, self.t.convex_hull()))
    }

    /// Return the bbox of the dataset
    ///
    /// :return: an array of 4 coordinates: [minx, miny, maxx, maxy]
    ///
    /// >>> bbox = dt.get_bbox()
    /// array([ 0., 0., 10., 12. ])
    fn get_bbox<'py>(&self, py: Python<'py>) -> PyResult<&'py PyArray<f64, numpy::Ix1>> {
        Ok(PyArray::from_vec(py, self.t.get_bbox()))
    }

    /// Is the point [x, y] located inside the convex hull of the DT.
    ///
    /// :param p2: array with [x, y]-coordinates of point to test
    /// :return: True if [x,y] is inside the convex hull or on its boundary, False otherwise.
    #[pyo3(signature = (p2))]
    fn is_inside_convex_hull(&mut self, p2: [f64; 2]) -> PyResult<bool> {
        let re = self.t.locate(p2[0], p2[1]);
        if re.is_ok() == true {
            return Ok(true);
        } else {
            Ok(false)
        }
    }

    /// Return True if vertex *vi* is on the boundary of the convex hull, False otherwise.
    ///
    /// :param vi: the vertex index
    /// :return: True if *vi* is on the boundary of the convex hull, False otherwise.
    ///          Also False is returned if the vertex doesn't exist in the DT.
    #[pyo3(signature = (vi))]
    fn is_vertex_convex_hull(&self, vi: usize) -> PyResult<bool> {
        Ok(self.t.is_vertex_convex_hull(vi))
    }

    /// Return True if vertex *vi* is labelled as removed, False otherwise.
    ///
    /// :param vi: the vertex index
    /// :return: True if *vi* is labelled as removed, False otherwise.
    ///          An exception is raised if *vi* doesn't exist.  
    #[pyo3(signature = (vi))]
    fn is_vertex_removed(&self, vi: usize) -> PyResult<bool> {
        let re = self.t.is_vertex_removed(vi);
        if re.is_err() {
            return Err(exceptions::PyIndexError::new_err("Invalid vertex index"));
        } else {
            Ok(re.unwrap())
        }
    }

    /// Return the closest vertex index to [x, y] (distance in 2D).
    /// An Exception is thrown if [x, y] is outside the convex hull.
    ///
    /// :param p2: array with [x, y]-coordinates of point
    /// :return: the vertex index of the closest point
    ///
    /// >>> try:
    /// >>>     cp = dt.closest_point([32.1, 66.9])
    /// >>> except Exception as e:
    /// >>>     print(e)
    #[pyo3(signature = (p2))]
    fn closest_point(&mut self, p2: [f64; 2]) -> PyResult<usize> {
        let re = self.t.closest_point(p2[0], p2[1]);
        if re.is_err() {
            return Err(exceptions::PyException::new_err("Outside convex hull"));
        } else {
            Ok(re.unwrap())
        }
    }

    /// Return the triangles incident to vertex *vi*.
    /// Infinite triangles are also returned.
    /// Exception thrown if vertex index doesn't exist in the DT or
    /// if it has been removed.
    ///
    /// :param vi: the vertex index
    /// :return: an array of triangles (ordered counter-clockwise)
    ///
    /// >>> trs = dt.incident_triangles_to_vertex(3)
    /// >>> for i, t in enumerate(trs):
    /// >>>     print(i, t)    
    /// 0 [3 4 6]
    /// 1 [3 6 7]
    /// 2 [3 7 8]
    /// 3 [3 8 2]
    /// 4 [3 2 9]
    /// 5 [3 9 4]
    #[pyo3(signature = (vi))]
    fn incident_triangles_to_vertex<'py>(
        &self,
        py: Python<'py>,
        vi: usize,
    ) -> PyResult<&'py PyArray<usize, numpy::Ix2>> {
        let re = self.t.incident_triangles_to_vertex(vi);
        if re.is_ok() {
            let l = re.unwrap();
            let mut trs: Vec<Vec<usize>> = Vec::with_capacity(l.len());
            for each in l {
                let mut tr = Vec::with_capacity(3);
                tr.push(each.v[0]);
                tr.push(each.v[1]);
                tr.push(each.v[2]);
                trs.push(tr);
            }
            return Ok(PyArray::from_vec2(py, &trs).unwrap());
        } else {
            return Err(exceptions::PyIndexError::new_err("Invalid vertex index"));
        }
    }

    /// Return the triangles adjacent to Triangle *t*.
    /// Exception thrown if vertex index doesn't exist in the DT.
    ///
    /// :param t: the Triangle, an array of 3 vertex indices
    /// :return: an array of 3 Triangles (finite and infinite)
    ///
    /// >>> tris = dt.adjacent_triangles_to_triangle([1, 44, 23])
    /// >>> for i, t in enumerate(tris):
    /// >>>     print(i, t)    
    /// 0 [3 4 6]
    /// 1 [3 6 7]
    /// 2 [3 7 8]
    #[pyo3(signature = (t))]
    fn adjacent_triangles_to_triangle<'py>(
        &self,
        py: Python<'py>,
        t: Vec<usize>,
    ) -> PyResult<&'py PyArray<usize, numpy::Ix2>> {
        let tr = startin::Triangle {
            v: [t[0], t[1], t[2]],
        };
        let re = self.t.adjacent_triangles_to_triangle(&tr);
        if re.is_ok() {
            let l = re.unwrap();
            let mut trs: Vec<Vec<usize>> = Vec::with_capacity(l.len());
            for each in l {
                let mut tr = Vec::with_capacity(3);
                tr.push(each.v[0]);
                tr.push(each.v[1]);
                tr.push(each.v[2]);
                trs.push(tr);
            }
            return Ok(PyArray::from_vec2(py, &trs).unwrap());
        } else {
            return Err(exceptions::PyIndexError::new_err("Triangle not present"));
        }
    }

    /// Return an array of vertex indices that are adjacent to vertex *vi*,
    /// that is those on the edges incident to *vi*.
    /// An exception is thrown if *vi* does not exist in the DT.
    ///
    /// :param vi: the vertex index
    /// :return: an array of vertex indices (ordered counter-clockwise)
    #[pyo3(signature = (vi))]
    fn adjacent_vertices_to_vertex<'py>(
        &self,
        py: Python<'py>,
        vi: usize,
    ) -> PyResult<&'py PyArray<usize, numpy::Ix1>> {
        let re = self.t.adjacent_vertices_to_vertex(vi);
        if re.is_ok() {
            return Ok(PyArray::from_vec(py, re.unwrap()));
        } else {
            return Err(exceptions::PyIndexError::new_err("Invalid vertex index"));
        }
    }

    /// Verify whether a Triangle is finite, or not.
    /// An infinite triangle has the first 0-vertex as one
    /// of its vertices.
    /// This doesn't verify wether the triangle exists (use is_valid()).
    ///
    /// :param t: the Triangle, an array of 3 vertex indices
    /// :return: True if t is finite, False is infinite
    ///
    /// >>> re = dt.is_finite(np.array([11, 162, 666])))
    #[pyo3(signature = (t))]
    fn is_finite(&self, t: Vec<usize>) -> PyResult<bool> {
        let tr = startin::Triangle {
            v: [t[0], t[1], t[2]],
        };
        Ok(self.t.is_finite(&tr))
    }

    /// Verify if a triangle exists in the DT.
    ///
    /// :param t: the Triangle, an array of 3 vertex indices
    /// :return: True if t exists, False otherwise
    ///
    /// >>> dt.is_triangle(np.array([11, 162, 66]))
    /// False
    #[pyo3(signature = (t))]
    fn is_triangle(&self, t: Vec<usize>) -> PyResult<bool> {
        let tr = startin::Triangle {
            v: [t[0], t[1], t[2]],
        };
        Ok(self.t.is_triangle(&tr))
    }

    /// Locate the triangle containing the point [x, y] (projected to 2D).
    /// An error is thrown if it is outside the convex hull.
    ///
    /// :param p2: array with [x, y]-coordinates of point
    /// :return: the triangle
    ///
    /// >>> tr = dt.locate([34.2, 55.6])
    /// array([65, 61, 23], dtype=uint64)
    #[pyo3(signature = (p2))]
    fn locate<'py>(
        &mut self,
        py: Python<'py>,
        p2: [f64; 2],
    ) -> PyResult<&'py PyArray<usize, numpy::Ix1>> {
        let re = self.t.locate(p2[0], p2[1]);
        let mut tr: Vec<usize> = Vec::new();
        if re.is_ok() {
            let t = re.unwrap();
            tr.push(t.v[0]);
            tr.push(t.v[1]);
            tr.push(t.v[2]);
            return Ok(PyArray::from_vec(py, tr));
        } else {
            return Err(exceptions::PyException::new_err("Outside convex hull"));
        }
    }

    /// Estimate the z-value with 5 different spatial interpolation methods:
    ///
    /// 1. **IDW**: inverse distance weighing
    /// 2. **Laplace**: a faster NNI with almost the same results
    /// 3. **NN**: nearest neighbour
    /// 4. **NNI**: natural neighbour interpolation
    /// 5. **TIN**: linear interpolation in TIN
    ///
    /// The interpolation does not modify the triangulation, it only returns an estimation for
    /// the z-values at the xy-location provided as argument.
    ///
    /// :param interpolant: a JSON/dict Python object with a `"method": "IDW"` (or others). IDW has 2 more params: "power" and "radius"; NNI can have the Voronoi cells precomputed with "precompute"
    /// :param locations: an array of [x, y] locations where the function should interpolate
    /// :param strict: (default=False) if the interpolation cannot find a value (because outside convex hull or search radius too small) then strict==True will stop at the first error and return that error. If strict==False then numpy.nan is returned.
    /// :return: a numpy array containing all the interpolation values (same order as input array). numpy.nan when location is outside the convexhull of the DT.
    ///
    /// >>> locs = [ [50.0, 41.1], [101.1, 33.2], [80.0, 66.0] ]
    /// >>> re = dt.interpolate({"method": "NNI", "precompute": True}, locs)
    /// >>> re = dt.interpolate({"method": "Laplace"}, locs)
    /// >>> re = dt.interpolate({"method": "IDW", "radius": 20, "power": 2.0}, locs, strict=True)
    #[pyo3(signature = (interpolant, locations, strict=false))]
    fn interpolate<'py>(
        &mut self,
        py: Python<'py>,
        interpolant: &PyDict,
        locations: Vec<[f64; 2]>,
        strict: bool,
    ) -> PyResult<&'py PyArray<f64, numpy::Ix1>> {
        match interpolant.get_item("method") {
            None => return Err(exceptions::PyValueError::new_err("Wrong parameters")),
            Some(m) => {
                let m: String = m.extract()?;
                let mut re: Vec<f64> = Vec::with_capacity(locations.len());
                match m.as_str() {
                    "IDW" => {
                        let radius = interpolant.get_item("radius");
                        let power = interpolant.get_item("power");
                        if radius.is_none() || power.is_none() {
                            return Err(exceptions::PyValueError::new_err("Wrong parameters"));
                        } else {
                            let r1: f64 = radius.unwrap().extract()?;
                            if r1 <= 0.0 {
                                return Err(exceptions::PyValueError::new_err("Wrong parameters"));
                            }
                            let p1: f64 = power.unwrap().extract()?;
                            if p1 <= 0.0 {
                                return Err(exceptions::PyValueError::new_err("Wrong parameters"));
                            }
                            for loc in locations {
                                let a = self.interpolate_idw(loc, r1, p1);
                                if a.is_ok() {
                                    re.push(a.unwrap());
                                } else {
                                    if strict == true {
                                        let s = format!(
                                            "({}, {}) no points in search radius",
                                            loc[0], loc[1]
                                        );
                                        return Err(exceptions::PyException::new_err(s));
                                    } else {
                                        re.push(f64::NAN);
                                    }
                                }
                            }
                            Ok(PyArray::from_vec(py, re))
                        }
                    }
                    "Laplace" => {
                        for loc in locations {
                            let a = self.interpolate_laplace(loc);
                            if a.is_ok() {
                                re.push(a.unwrap());
                            } else {
                                if strict == true {
                                    let s = format!(
                                        "({}, {}) is outside the convex hull",
                                        loc[0], loc[1]
                                    );
                                    return Err(exceptions::PyException::new_err(s));
                                } else {
                                    re.push(f64::NAN);
                                }
                            }
                        }
                        Ok(PyArray::from_vec(py, re))
                    }
                    "NN" => {
                        for loc in locations {
                            let a = self.interpolate_nn(loc);
                            if a.is_ok() {
                                re.push(a.unwrap());
                            } else {
                                if strict == true {
                                    let s = format!(
                                        "({}, {}) is outside the convex hull",
                                        loc[0], loc[1]
                                    );
                                    return Err(exceptions::PyException::new_err(s));
                                } else {
                                    re.push(f64::NAN);
                                }
                            }
                        }
                        Ok(PyArray::from_vec(py, re))
                    }
                    "NNI" => {
                        let re2 = interpolant.get_item("precompute");
                        let mut precompute: bool = false;
                        if re2.is_some() {
                            precompute = re2.unwrap().extract()?;
                        }
                        for loc in locations {
                            let a = self.interpolate_nni(loc, precompute);
                            if a.is_ok() {
                                re.push(a.unwrap());
                            } else {
                                if strict == true {
                                    let s = format!(
                                        "({}, {}) is outside the convex hull",
                                        loc[0], loc[1]
                                    );
                                    return Err(exceptions::PyException::new_err(s));
                                } else {
                                    re.push(f64::NAN);
                                }
                            }
                        }
                        Ok(PyArray::from_vec(py, re))
                    }
                    "TIN" => {
                        for loc in locations {
                            let a = self.interpolate_tin_linear(loc);
                            if a.is_ok() {
                                re.push(a.unwrap());
                            } else {
                                if strict == true {
                                    let s = format!(
                                        "({}, {}) is outside the convex hull",
                                        loc[0], loc[1]
                                    );
                                    return Err(exceptions::PyException::new_err(s));
                                } else {
                                    re.push(f64::NAN);
                                }
                            }
                        }
                        Ok(PyArray::from_vec(py, re))
                    }
                    _ => {
                        return Err(exceptions::PyValueError::new_err(
                            "Unknown interpolation method",
                        ));
                    }
                }
            }
        }
    }

    /// Write an `OBJ <https://en.wikipedia.org/wiki/Wavefront_.obj_file>`_ of
    /// the DT to the path (a string).
    /// Throws an exception if the path is invalid.
    ///
    /// :param path: full path (a string) on disk of the file to create (will overwrite)
    /// :return: (nothing)
    ///
    /// >>> dt.write_obj("/home/elvis/myfile.obj")
    #[pyo3(signature = (path))]
    fn write_obj(&self, path: String) -> PyResult<()> {
        let re = self.t.write_obj(path.to_string());
        if re.is_err() {
            return Err(exceptions::PyFileNotFoundError::new_err(
                "No such file or directory",
            ));
        }
        Ok(())
    }

    /// Write an `PLY <https://en.wikipedia.org/wiki/PLY_(file_format)>`_ of the DT to the path (a string).
    /// Throws an exception if the path is invalid.
    ///
    /// :param path: full path (a string) on disk of the file to create (will overwrite)
    /// :return: (nothing)
    ///
    /// >>> dt.write_ply("/home/elvis/myfile.ply")
    #[pyo3(signature = (path))]
    fn write_ply(&self, path: String) -> PyResult<()> {
        let re = self.t.write_ply(path.to_string());
        if re.is_err() {
            return Err(exceptions::PyFileNotFoundError::new_err(
                "No such file or directory",
            ));
        }
        Ok(())
    }

    /// Write a `GeoJSON <https://geojson.org>`_ file of the DT (vertices+triangles) to the path (a string).
    /// Throws an exception if the path is invalid.
    ///
    /// :param path: full path (a string) on disk of the file to create (will overwrite)
    /// :return: (nothing)
    ///
    /// >>> dt.write_geojson("/home/elvis/myfile.geojson")
    #[pyo3(signature = (path))]
    pub fn write_geojson(&self, path: String) -> PyResult<()> {
        let mut fc = FeatureCollection {
            bbox: None,
            features: vec![],
            foreign_members: None,
        };
        //-- vertices
        let allv_f = self.t.all_vertices();
        for i in 1..allv_f.len() {
            // println!("i: {:?}", i);
            if self.t.is_vertex_removed(i).unwrap() == true {
                continue;
            }
            let pt = Geometry::new(GeoValue::Point(vec![allv_f[i][0], allv_f[i][1]]));
            let mut attributes = Map::new();
            attributes.insert(String::from("id"), to_value(i.to_string()).unwrap());
            attributes.insert(
                String::from("z"),
                to_value(allv_f[i][2].to_string()).unwrap(),
            );
            let f = Feature {
                bbox: None,
                geometry: Some(pt),
                id: None,
                properties: Some(attributes),
                foreign_members: None,
            };
            fc.features.push(f);
        }
        //-- triangles
        let trs = self.t.all_finite_triangles();
        for tr in trs.iter() {
            // s.push_str(&format!("f {} {} {}\n", tr.v[0], tr.v[1], tr.v[2]));
            let mut l: Vec<Vec<Vec<f64>>> = vec![vec![Vec::with_capacity(1); 4]];
            l[0][0].push(allv_f[tr.v[0]][0]);
            l[0][0].push(allv_f[tr.v[0]][1]);
            l[0][1].push(allv_f[tr.v[1]][0]);
            l[0][1].push(allv_f[tr.v[1]][1]);
            l[0][2].push(allv_f[tr.v[2]][0]);
            l[0][2].push(allv_f[tr.v[2]][1]);
            l[0][3].push(allv_f[tr.v[0]][0]);
            l[0][3].push(allv_f[tr.v[0]][1]);
            let gtr = Geometry::new(GeoValue::Polygon(l));
            // let mut attributes = Map::new();
            // if self.stars[]
            // attributes.insert(String::from("active"), to_value();
            let f = Feature {
                bbox: None,
                geometry: Some(gtr),
                id: None,
                properties: None, //Some(attributes),
                foreign_members: None,
            };
            fc.features.push(f);
        }
        //-- write the file to disk
        let mut fo = File::create(path)?;
        let _ = write!(fo, "{}", fc.to_string());
        Ok(())
    }

    /// Write a `CityJSON <https://www.cityjson.org>`_ file of the DT (vertices+triangles) to the path (a string).
    /// One `TINRelief <https://www.cityjson.org/specs/#tinrelief>`_ object is created.
    /// Throws an exception if the path is invalid.
    ///
    /// :param path: full path (a string) on disk of the file to create (will overwrite)
    /// :param digits: (default=3) number of digits to keep (for saving efficiently the coordinates)
    /// :return: (nothing)
    ///
    /// >>> dt.write_cityjson("/home/elvis/myfile.city.json")
    #[pyo3(signature = (path, digits=3))]
    fn write_cityjson(&self, path: String, digits: usize) -> PyResult<()> {
        let bbox = self.t.get_bbox();
        let d: f64 = 1.0 / (f64::powf(10., digits as f64));
        let trans = json!({
            "scale": vec![d, d, d],
            "translate": vec![bbox[0], bbox[1], 0.0],
        });
        //-- vertices
        let allv_f = self.t.all_vertices();
        let mut onevertex: Vec<f64> = Vec::new();
        for (i, _each) in allv_f.iter().enumerate() {
            if i != 0 && (self.t.is_vertex_removed(i).unwrap() == false) {
                onevertex = vec![allv_f[i][0], allv_f[i][1], allv_f[i][2]];
                break;
            }
        }
        let mut allv_i: Vec<Vec<i64>> = Vec::new();
        for i in 1..allv_f.len() {
            let mut x = allv_f[i][0];
            let mut y = allv_f[i][1];
            let mut z = allv_f[i][2];
            // if i == 0 || (self.t.is_vertex_removed(i).unwrap() == true) {
            if self.t.is_vertex_removed(i).unwrap() == true {
                x = onevertex[0];
                y = onevertex[1];
                z = onevertex[2];
            }
            x -= bbox[0];
            y -= bbox[1];
            let s0 = format!("{:.*}", digits, x).replace(".", "");
            let s1 = format!("{:.*}", digits, y).replace(".", "");
            let s2 = format!("{:.*}", digits, z).replace(".", "");
            allv_i.push(vec![
                s0.parse::<i64>().unwrap(),
                s1.parse::<i64>().unwrap(),
                s2.parse::<i64>().unwrap(),
            ]);
        }
        let mut alltrs: Vec<Vec<Vec<usize>>> = Vec::new();
        let trs = self.t.all_finite_triangles();
        for tr in &trs {
            let mut t: Vec<Vec<usize>> = Vec::new();
            t.push(vec![tr.v[0] - 1, tr.v[1] - 1, tr.v[2] - 1]);
            alltrs.push(t);
        }
        //-- CityObjects
        let cos = json!({
            "type": "TINRelief".to_owned(),
            "geometry": [ {
                "type": "CompositeSurface",
                "lod": "1",
                "boundaries": alltrs
            }
            ]
        });
        let cj = Cityjson {
            r#type: "CityJSON".to_owned(),
            version: "2.0".to_owned(),
            transform: trans,
            city_objects: json!({"myterrain": cos}),
            vertices: allv_i,
        };
        // Serialize it to a JSON string.
        let mut fo = File::create(path)?;
        let j = serde_json::to_string(&cj);
        let _ = write!(fo, "{}", j.unwrap());
        Ok(())
    }

    /// Vertically exaggerate the elevation values of the vertices.
    /// Used mostly for visualisation.
    ///
    /// :param factor: a positive value (can be <1.0 to remove exaggeration)
    /// :return: (nothing)
    ///
    /// >>> dt.vertical_exaggeration(2.0)
    /// >>> dt.vertical_exaggeration(0.5)
    #[pyo3(signature = (factor))]
    fn vertical_exaggeration(&mut self, factor: f64) {
        self.t.vertical_exaggeration(factor);
    }

    /// Returns True if some vertices are marked as to be deleted (but still in memory)
    /// , False otherwise.
    ///
    /// :return: True/False
    fn has_garbage(&self) -> PyResult<bool> {
        Ok(self.t.has_garbage())
    }

    /// Collect garbage, that is remove from memory the vertices
    /// marked as removed (modifies the array dt.points and all indices of the triangles).
    ///
    /// **Watch out:** the vertices get new IDs, and thus the triangles get updated too.
    /// And this can be a slow operation.
    ///
    /// >>> if dt.has_garbage():
    /// >>>     dt.collect_garbage()
    /// >>> assert (dt.has_garbage() == False)
    fn collect_garbage(&mut self) -> PyResult<()> {
        self.t.collect_garbage();
        Ok(())
    }
}

impl DT {
    fn interpolate_nn(&mut self, p2: [f64; 2]) -> PyResult<f64> {
        let i_nn = startin::interpolation::NN {};
        let mut re = startin::interpolation::interpolate(&i_nn, &mut self.t, &vec![p2]);
        let re1 = re.pop().expect("no results");
        if re1.is_err() {
            return Err(exceptions::PyException::new_err("Outside convex hull"));
        }
        Ok(re1.unwrap())
    }

    fn interpolate_tin_linear(&mut self, p2: [f64; 2]) -> PyResult<f64> {
        let i_tin = startin::interpolation::TIN {};
        let mut re = startin::interpolation::interpolate(&i_tin, &mut self.t, &vec![p2]);
        let re1 = re.pop().expect("no results");
        if re1.is_err() {
            return Err(exceptions::PyException::new_err("Outside convex hull"));
        }
        Ok(re1.unwrap())
    }

    fn interpolate_laplace(&mut self, p2: [f64; 2]) -> PyResult<f64> {
        let i_lp = startin::interpolation::Laplace {};
        let mut re = startin::interpolation::interpolate(&i_lp, &mut self.t, &vec![p2]);
        let re1 = re.pop().expect("no results");
        if re1.is_err() {
            return Err(exceptions::PyException::new_err("Outside convex hull"));
        }
        Ok(re1.unwrap())
    }

    fn interpolate_nni(&mut self, p2: [f64; 2], precompute: bool) -> PyResult<f64> {
        let i_nni = startin::interpolation::NNI {
            precompute: precompute,
        };
        let mut re = startin::interpolation::interpolate(&i_nni, &mut self.t, &vec![p2]);
        let re1 = re.pop().expect("no results");
        if re1.is_err() {
            return Err(exceptions::PyException::new_err("Outside convex hull"));
        }
        Ok(re1.unwrap())
    }

    fn interpolate_idw(&mut self, p2: [f64; 2], radius: f64, pow: f64) -> PyResult<f64> {
        let i_idw = startin::interpolation::IDW {
            radius: radius,
            power: pow,
        };
        let mut re = startin::interpolation::interpolate(&i_idw, &mut self.t, &vec![p2]);
        let re1 = re.pop().expect("no results");
        if re1.is_err() {
            return Err(exceptions::PyException::new_err("Search Circle Empty"));
        }
        Ok(re1.unwrap())
    }
}
