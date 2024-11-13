
<img src="docs/figs/polyscope.jpg" width="350">

[![PyPI](https://img.shields.io/pypi/v/startinpy?style=for-the-badge)](https://pypi.org/project/startinpy/) [![docs](https://img.shields.io/badge/docs-startinpy.rtfd.io-brightgreen?style=for-the-badge)](https://startinpy.rtfd.io/) [![GitHub license](https://img.shields.io/github/license/hugoledoux/startinpy?style=for-the-badge)](https://github.com/hugoledoux/startinpy/blob/master/LICENSE)

startinpy
=========

A library for modelling and processing 2.5D terrains using a (2D) Delaunay triangulation.
The triangulation is computed in 2D, but the *z*-elevation of the vertices are kept.

The underlying code is written in [Rust](https://www.rust-lang.org/) (so it's rather fast) and [robust arithmetic](https://crates.io/crates/robust) is used (so it shouldn't crash).
startinpy uses the [startin Rust library](https://github.com/hugoledoux/startin) and adds several utilities and functions, for instance [NumPy](https://numpy.org/) support for input/output, exporting to several formats, and easy-of-use.

startinpy allows you to:

  1. insert incrementally points
  2. delete vertices (useful for simplification, interpolation, and other operations)
  3. interpolate with several methods: TIN, natural neighbours, IDW, Laplace, etc.
  4. use other useful terrain Python libraries that are also NumPy-based, eg [laspy](https://laspy.readthedocs.io), [rasterio](https://rasterio.readthedocs.io), [meshio](https://github.com/nschloe/meshio)
  5. output the TIN to several formats: OBJ, PLY, GeoJSON, and CityJSON
  6. store extra attributes with the vertices (the ones from LAS/LAZ)

Documentation
=============

<https://startinpy.rtfd.io>

Installation
============

pip
---

To install the latest release: `pip install startinpy`

(watch out: this does not work with Linux currently, it installs an old version!)

If you want to compile it yourself
----------------------------------

  1. install latest [Rust](https://www.rust-lang.org/)
  2. install [maturin](https://github.com/PyO3/maturin)
  3. `maturin build --release`
  4. `cd ./target/wheels/`
  5. `pip install [name-wheel].whl` will install it to your local Python

Development
-----------

  1. install [Rust](https://www.rust-lang.org/) (v1.39+)
  2. install [maturin](https://github.com/PyO3/maturin)
  3. `maturin develop`
  4. move to another folder, and `import startinpy` shouldn't return any error

Testing
-------

To run the automated test suite:

 1. install the test requirements: `pip install -r tests/requirements.txt`
 2. `pytest`

Examples
========

The folder `./demo` contains a few examples.

```python
import laspy
import numpy as np
import startinpy

las = laspy.read("../data/small.laz")
pts = np.vstack((las.x, las.y, las.z)).transpose()

dt = startinpy.DT()
dt.insert(pts)

# -- remove vertex #4
try:
    dt.remove(4)
except Exception as e:
    print(e)

print("# vertices:", dt.number_of_vertices())
print("# triangles:", dt.number_of_triangles())

# -- print the vertices forming the convex hull, in CCW-order
print("CH: ", dt.convex_hull())

# -- fetch all the incident triangles (CCW-ordered) to the vertex #235
vi = 235
one_random_pt = dt.points[vi]
print("one random point:", one_random_pt)
print(dt.incident_triangles_to_vertex(vi))

# -- interpolate at a location with the linear in TIN method
zhat = dt.interpolate({"method": "TIN"}, [[85718.5, 447211.6]])
print("result: ", zhat[0])
```

If you use this software, please cite this article 
==================================================

```bibtex
@article{Ledoux24,
  author = {Ledoux, Hugo},
  title = {{startinpy}: {A} {P}ython library for modelling and processing {2.5D} triangulated terrains},
  journal = {Journal of Open Source Software},
  year = {2024},
  volume = {9},
  number = {103},
  pages = {7123},
  doi = {10.21105/joss.07123}
}
```
