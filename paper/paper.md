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
 - name: Delft University of Technology, Delft, the Netherlands
date: 6 March 2024
bibliography: ref.bib

---

# Summary

The Python library `startinpy` allows us to construct, modify, and manipulate triangulated terrains, commonly called TINs (triangulated irregular networks).
Given a dataset formed of elevation samples (eg collected with lidar or photogrammetry), it is possible to construct a TIN, identify outliers, convert to a gridded terrain (with spatial interpolation), etc.
Observe that while it is built primarily for points having a *z*-elevation, a Delaunay triangulation (DT) in 2D is computed (a TIN is a so-called 2.5D object). 
This means that startinpy can also be used for applications where a standard 2D DT is necessary.
Also, unlike several triangulation libraries, `startinpy` exposes its topological data structure and this allows users to obtain incident and adjacent triangles to vertices/triangles. This can be useful for quality control, to derive properties like slope, to convert to different formats, etc. 
The underlying code of `startinpy` is written in the language Rust (so it's rather fast), robust arithmetic is used (so it shouldn't crash), and it use NumPy for input/output of data, which allows it to integrate with other Python libraries.


<!-- Such a library is necessary to represent the morphology of an area, when one wants to avoid using grids and prefers a leaner representation with points and triangles. -->


# Statement of need

There exists several Python libraries for computing the DT in 2D.
A simple search for `"Delaunay"` in the *Python Package Index* (PyPI) returns 85 packages, the most notable ones being SciPy (specifically `scipy.spatial.Delaunay`, which is a wrapper around Qhull [@Barber96], written in C) and Triangle (which is a wrapper around the fast and robust C library that performs constrained DT and meshing [@Shewchuk96a]).

When it comes to modelling 2.5D terrains, the existing Python libraries have four main shortcomings:

  1. Libraries written in pure Python are simply too slow for modern datasets. Indeed, with recent lidar scanner, we can easily collect 50 samples/$m^2$ and this means that a small dataset will already contain several millions samples. 
  2. While a 2D DT should be calculated, the *z*-values of the points should be preserved. Some libraries allows us to attach extra information to a vertex, but most often one has to build auxiliary data structure in Python to manage those, which is error-prone, tedious, and makes operations in 3D (eg calculating the slope of an area, finding the normals of a point, calculating volumes) complex operations.
  3. Both SciPy and Triangle construct a 2D DT in a *batch* operation, that the DT for a set of points is constructed and cannot be modified. Being able to construct *incrementally* a DT has several benefits: one can for instance construct a simplified TIN that best approximate the original terrain with only 10% of the points, see for instance @Garland95 for different strategies. Also, available libraries do not allow to remove/delete points, which is useful when outliers are identified (by analysing the neighbouring triangles of vertices).
  4. The data structure of the DT is not exposed, only a list of vertices and triangles (triplets of vertex identifiers) are returned. This means that the user has to build a network to be able to find the adjacent triangles of a given one, or to find all the triangles that are incident to a given vertex (eg to calculate the normal).


# Design and details of startinpy

startinpy was developed specifically for needs of 2.5D terrain modelling.

Its core (construction of the DT, deletion, interpolation, etc) is written in Rust (and called simply `startin`, source code is available at https://github.com/hugoledoux/startin) and can be used in Rust programs. 
A C-interface to the library is also available, this allows us to use, for instance, the library in Julia (https://github.com/evetion/startin.jl); it has been used recently to build a global coastal terrain using measurements from the space station [@Pronk24].
Observe that the robust predicates as described in @Shewchuk96 are used (the code has been converted to pure Rust, see https://docs.rs/robust/latest/robust/, which means that startinpy should not crash because of floating-point arithmetic.
Since the library is not written in pure Python, a GitHub Action compiles the bindings for the lastest versions of Python and for Windows/macOS/Linux.

The name of the library comes from the fact that the data structure implemented is based on the concept of *stars* in a graph [@Blandford05], which allows us to both store adjacency and incidence and have a very compact data structure.
The construction algorithm used is an incremental insertion based on flips, and the deletion of a vertex is also possible. 
The algorithm implemented is a modification of @Mostafavi03, I have extended it to allow the deletion of vertices on the boundary of the convex hull. 

A few spatial interpolation methods that are based on the DT and its dual structure the Voronoi diagram have been implemented: TIN, natural neighbours, Laplace, etc.

It is possible to store extra attributes with each vertex, each attribute is stored as a JSON object/dictionary, a key-value pair where the key is a string and the value is one of these 3 options: (1) an integer, (2) a float, or (3) a boolean.
This can be used to preserve the lidar properties of the input points (eg intensity, RGB, number of returns, etc.), those are not possible to attach with the Python libraries mentioned above.

To facilitate the processing and analysis of large datasets, and to integrate with other libraries (such as laspy https://github.com/laspy/laspy), NumPy arrays are used; a few examples in the documentation are available.

Finally, it is possible to output the TIN to several formats: OBJ, PLY, GeoJSON, and CityJSON (examples are also in the documentation).


# Citations

For a quick reference, the following citation commands can be used:
- `@author:2001`  ->  "Author et al. (2001)"
- `[@author:2001]` -> "(Author et al., 2001)"
- `[@author1:2001; @author2:2001]` -> "(Author1 et al., 2001; Author2 et al., 2002)"

# Figures
<!-- 
Figures can be included like this:
![Caption for example figure.\label{fig:example}](figure.png)
and referenced from text using \autoref{fig:example}.

Figure sizes can be customized by adding an optional second parameter:
![Caption for example figure.](figure.png){ width=20% } -->

# Acknowledgements

We acknowledge contributions from Brigitta Sipocz, Syrtis Major, and Semyeong
Oh, and support from Kathryn Johnston during the genesis of this project.

# References