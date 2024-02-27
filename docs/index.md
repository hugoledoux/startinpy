# startinpy

```{image} figs/polyscope.jpg
:align: right
:width: 40%
```

A library to model and manipulate terrains/DTMs using a (2D) Delaunay triangulation. 
The triangulation is computed in 2D, but the *z*-elevation of the vertices are kept.

The underlying library is written in [Rust](https://www.rust-lang.org/) so it's rather fast ([see Rust code](https://github.com/hugoledoux/startin)) and [robust arithmetic](https://crates.io/crates/robust) is used (so it shouldn't crash).

startinpy uses the Rust library and adds several utilities and functions, for instance [NumPy](https://numpy.org/) support for input/output, exporting to several formats, and easy-of-use.

:::{admonition} startinpy allows you to:
1. insert incrementally points
2. delete vertices (useful for simplification, interpolation, and other operations)
3. interpolate and create grids with several methods: TIN, natural neighbours, IDW, Laplace, etc.
4. use other useful terrain Python libraries that are also NumPy-based, eg [laszy](https://laspy.readthedocs.io), [meshio](https://github.com/nschloe/meshio)
5. outputs the TIN to several formats: OBJ, PLY, GeoJSON, and CityJSON
6. [extra attributes](attributes) (the ones from LAS/LAZ) can be stored with the vertices
:::

```python
import startinpy
import numpy as np
import laspy

las = laspy.read("../data/small.laz")
pts = np.vstack((las.x, las.y, las.z)).transpose()

dt = startinpy.DT()
dt.insert(pts)

#-- remove vertex #4
try:
    dt.remove(4)
except Exception as e:
    print(e)

print("# vertices:", dt.number_of_vertices())
print("# triangles:", dt.number_of_triangles())

print("CH: ", dt.convex_hull())

vi = 235
one_random_pt = dt.points[vi]
print("one random point:", one_random_pt)
print(dt.incident_triangles_to_vertex(vi))

zhat = dt.interpolate({"method": "TIN"}, [[85718.5, 447211.6]])
print("result: ", zhat[0])
```

# Table of content

```{toctree}
:maxdepth: 0

installation
howitworks
examples
attributes
api
issues
```

# Indices and tables

- {ref}`genindex`
