# startinpy

```{image} figs/polyscope.jpg
:align: right
:width: 40%
```

A library for modelling and processing 2.5D terrains using a (2D) Delaunay triangulation. 
The triangulation is computed in 2D, but the *z*-elevation of the vertices is kept.

The underlying code is written in [Rust](https://www.rust-lang.org/) (so it's rather fast) and [robust arithmetic](https://crates.io/crates/robust) is used (so it shouldn't crash).

startinpy uses the [startin Rust library](https://github.com/hugoledoux/startin) and adds several utilities and functions, for instance [NumPy](https://numpy.org/) support for input/output, export to several formats, and easy-of-use.

:::{admonition} startinpy allows you to:
1. insert points incrementally
2. delete vertices (useful for simplification, interpolation, and other operations)
3. interpolate with several methods: TIN, natural neighbours, IDW, Laplace, etc
4. use other useful terrain Python libraries that are also NumPy-based, eg [laspy](https://laspy.readthedocs.io), [rasterio](https://rasterio.readthedocs.io), and [meshio](https://github.com/nschloe/meshio)
5. output the TIN to several formats: OBJ, PLY, GeoJSON, and CityJSON
6. store [extra attributes](./attributes.md) for the vertices (eg the ones from LAS/LAZ)

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

#-- print the vertices forming the convex hull, in CCW-order
print("CH: ", dt.convex_hull())

#-- fetch all the incident triangles (CCW-ordered) to the vertex #235
vi = 235
one_random_pt = dt.points[vi]
print("one random point:", one_random_pt)
print(dt.incident_triangles_to_vertex(vi))

#-- interpolate at a location with the linear in TIN method
zhat = dt.interpolate({"method": "TIN"}, [[85718.5, 447211.6]])
print("result: ", zhat[0])
```

# Table of content

```{toctree}
:maxdepth: 0

installation
howitworks
api
examples
attributes
comparison
issues
```

# Indices and tables

- {ref}`genindex`
