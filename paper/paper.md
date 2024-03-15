---
title: 'startinpy: A Python library for modelling and processing 2.5D terrains'
tags:
  - Delaunay triangulation
  - Python
  - GIS
  - terrain
  - interpolation
authors:
  - name: Hugo Ledoux
    orcid: 0000-0002-1251-8654
affiliations:
 - name: Delft University of Technology, the Netherlands
date: 15 March 2024
bibliography: ref.bib

---

# Summary

The Python library startinpy can construct, modify, and manipulate triangulated terrains, commonly called TINs (triangulated irregular networks).
Given a dataset formed of elevation samples (eg collected with lidar or photogrammetry), it is possible to construct a TIN, remove points (eg outliers), convert to a gridded terrain (with spatial interpolation) or other known formats, etc.
Observe that while it is built primarily for points having a *z*-elevation, a Delaunay triangulation (DT) in 2D is computed (a TIN is a so-called 2.5D object); this means that startinpy can be used for applications where a standard 2D DT is necessary.
Moreover, unlike several triangulation libraries, startinpy exposes its data structure and this allows users to obtain incident and adjacent triangles to vertices/triangles. 
This can be useful for quality control, to derive properties like slope, to convert to different formats, etc. 
The underlying code of startinpy is written in the language Rust (so it is rather fast), robust arithmetic is used (so it should not crash), and it use NumPy for input/output of data, which allows it to integrate with other Python libraries.

![A lidar dataset terrain reconstructed with startinpy and visualised with another Python library (Polyscope).](polyscope_gui.jpg){ width=75% }


# Statement of need

There exists several Python libraries for computing the DT in 2D.
A search for `"Delaunay triangulation"` in the *Python Package Index* (PyPI) returns 227 packages, the most notable ones being SciPy (specifically `scipy.spatial.Delaunay`, which is a wrapper around Qhull [@Barber96], written in C) and Triangle (which is a wrapper around the fast and robust C library that performs constrained DT and meshing [@Shewchuk96a]).

When it comes to modelling 2.5D terrains, the existing Python libraries have in general four main shortcomings:

  1. Libraries written in pure Python are simply too slow for modern datasets. Indeed, with recent lidar scanner, we can easily collect 50 samples/$m^2$ and this means that a small area will already contain several millions samples. 
  2. While a 2D DT should be calculated, the *z*-values of the points should also be preserved. Some libraries allows us to attach extra information to a vertex, but most often one has to build auxiliary data structure in Python to manage those, which is error-prone, tedious, and makes operations in 3D (eg calculating the slope of an area, finding the normals of a point, calculating volumes) complex operations.
  3. Both SciPy and Triangle construct a 2D DT in a *batch* operation: the DT for a set of points is constructed and cannot be modified after construction. Being able to construct *incrementally* a DT has several benefits: one can for instance construct a simplified TIN that best approximate the original terrain with only 10% of the points, see for instance @Garland95 for different strategies. Also, the existing libraries do not allow to remove/delete points, which is useful when outliers are identified (by analysing the neighbouring triangles of vertices).
  4. The data structure of the DT is not exposed, only a list of vertices and triangles (triplets of vertex identifiers) are returned. This means that the user has to build a graph to be able to find the adjacent triangles of a given one, or to find all the triangles that are incident to a given vertex (eg to calculate the normal).


# Design and details of startinpy

startinpy was developed specifically for needs of 2.5D terrain modelling, and addresses the four issues described above.

Its core (construction of the DT, deletion, interpolation, etc) is written in Rust (and called simply `startin`, the source code is available at https://github.com/hugoledoux/startin) and can be used in Rust programs. 
A C-interface to the library is also available, this allows us to use, for instance, the library in Julia (https://github.com/evetion/startin.jl); it has been used recently to build a global coastal terrain using laser altimetric measurements from the space station [@Pronk24].
Observe that the robust predicates as described in @Shewchuk96 are used (the code has been converted to pure Rust, see https://docs.rs/robust/latest/robust/), which means that startinpy should not crash because of floating-point arithmetic.
Also, since the library is not written in pure Python, a GitHub Action compiles the bindings for the lastest versions of Python, and for Windows/macOS/Linux.

The name of the library comes from the fact that the data structure implemented is based on the concept of *stars* in a graph [@Blandford05], which allows us to both store adjacency and incidence and have a very compact data structure.
The construction algorithm used is an incremental insertion based on flips, and the deletion of a vertex is also possible. 
The algorithm implemented is a modification of @Mostafavi03, I have extended it to allow the deletion of vertices on the boundary of the convex hull. 

A few spatial interpolation methods that are based on the DT and/or its dual structure the Voronoi diagram have been implemented: linear interpolation in TINs, natural neighbours [@Sibson81], Laplace [@Beliko97], etc.

It is possible to store extra attributes with each vertex, each attribute is stored as a JSON object/dictionary, a key-value pair where the key is a string and the value is one of these 3 options: (1) an integer, (2) a float, or (3) a boolean.
This can be used to preserve the lidar properties of the input points (eg intensity, RGB, number of returns, etc.), those are not possible to attach with the existing Python libraries mentioned above.

To facilitate the processing and analysis of large datasets, and to integrate with other libraries (such as `laspy` https://github.com/laspy/laspy), NumPy arrays are used; a few examples in the documentation are available.

Finally, it is possible to output the TIN to several formats: OBJ, PLY, GeoJSON, and CityJSON.
More format are possible through the use of other Python libraries, there are a few examples in the documentation.


# Comparison with a few alternatives

The tables below compare a few Python packages to startinpy.

The [Delaunator package](https://github.com/HakanSeven12/Delaunator-Python) is pure Python port of a proven fast triangulator [written original in JavaScript](https://github.com/mapbox/delaunator). 
[SciPy](https://docs.scipy.org/doc/scipy/reference/generated/scipy.spatial.Delaunay.html)  is `scipy.spatial.Delaunay`, and SciPy-inc is the variation where an incremental algorithm is used.
[Triangle](https://pypi.org/project/triangle/) is the Python bindings of the C code. 

|                        | Delaunator | SciPy | SciPy-inc | Triangle | startinpy |
|------------------------|:----------:|:-----:|:---------:|:--------:|:---------:|
| constrained DT         |    --      | --    |     --    |   +      |   --      |
| data structure exposed |    +       | --    |     --    |   --     |   +       |
| efficient searches     |    +       | +     |     +     |   --     |   +       |
| elevation/2.5D         |    --      | --    |     --    |   --     |   +       |
| extra attributes       |    --      | --    |     --    |   --     |   +       |
| fast                   |    --      | --    |     --    |   ++     |   +       |
| incremental insertion  |    --      | --    |     +     |   --     |   +       |
| remove vertices        |    --      | --    |     --    |   --     |   +       |
| xy-duplicate handling  |    --      | --    |     --    |   --     |   +       |


Notice that startinpy is the only offering to store z-values and extra attributes, the others are pure 2D Delaunay triangulator.
The parameter 'efficient searches' is if a point location function is available, to find the closest triangles to a given point.
The parameter 'xy-duplicate handling' refers to the fact that startinpy allows to merge vertices that are close to each others (in the xy-plane, and it can be setup), and that if there are xy-duplicates then the z-value can be determined (lowest or highest, depending on the application).


The table below shows the time it takes to construct the 2D DT, in a batch operation, for different datasets.
The details of the (openly available) datasets are available on the [GitHub repository of startinpy](https://github.com/hugoledoux/startinpy/tree/joss/dt_comparisons), and the Python code to replicate the experiments is available.
The datasets `random_X` are randomly generated points in unit square, the first one has 10,000 points and the other 50,000 points.
The datasets `LAZ_X` are real-world lidar datasets obtained from the Netherlands, the `2M` contains exactly 2,144,049 points, and the `33M` 33,107,889 points. 
The dataset `dem.tiff` is the GeoTIFF file in `/data/` and the centre of each grid cell is inserted by reading the rows and columns, the total is 277,750 points.

|            |random_10k|random_50k|LAZ_2M|LAZ_33M|dem.tiff|
|:-----------|----------|---------:|-----:|------:|-------:|
| Delaunator |   0.219  |    0.84  | 49.2 | 898.1 |   3.55 |
| SciPy      |   0.017  |    0.09  | 10.1 | 650.3 |   1.79 |
| SciPy-inc  |   0.015  |    0.08  |    X |     X |      X |
| Triangle   |   0.004  |    0.02  |  0.9 |  16.8 |   0.19 |
| startinpy  |   0.018  |    0.18  |  4.2 |  41.8 |   0.46 |

If "X" is written, it is because the returned DT was faulty: for large inputs Scipy-inc returns a few triangles only, not the full DT.

Notice that while startinpy is somewhat slower than Triangle, it is expected since, as explained above, it offers more convenience and its data structure is exposed.
It is also faster and more stable (no crash or wrong results) than the SciPy.


# Acknowledgements

I acknowledge the help of the students following the course *Digital terrain modelling (GEO1015)* at TUDelft over the last few years, their feedback, questions, and frustrations on preliminary versions of startinpy helped me greatly.

# References