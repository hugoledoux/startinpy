---
title: 'startinpy: A Python library for modelling and processing 2.5D triangulated terrains'
tags:
  - Delaunay triangulation
  - Python
  - terrain
  - interpolation
  - GIS
authors:
  - name: Hugo Ledoux
    orcid: 0000-0002-1251-8654
    affiliation: "1"
affiliations:
 - name: Delft University of Technology, the Netherlands
   index: 1
date: 17 July 2024
bibliography: ref.bib

---

# Summary

*startinpy* is a Python library for modelling and processing terrains using a 2D Delaunay triangulation (DT).
The triangulation is computed in 2D, but the z-elevation of the vertices is kept (which is referred to as 2.5D modelling).

Given a dataset formed of elevation samples (eg collected with lidar or photogrammetry), it allows us, among other things, to reconstruct a terrain, to remove points, to search efficiently in the triangulation, to attach attributes to the vertices, and to convert to a gridded terrain (a few spatial interpolation methods based on the DT and/or its dual structure, the Voronoi diagram, have been implemented: linear interpolation, natural neighbours [@Sibson81], and Laplace interpolation [@Belikov97]).

The core of the library is written in the language Rust (so it can process large datasets quickly), robust arithmetic is used for the updating of the DT (the robust predicates of @Shewchuk96 are used), and it uses NumPy for input/output of data, which allows it to integrate with other Python libraries used by researchers.

The core of the library has been used recently to build a global coastal terrain using laser altimetric measurements from the space station [@Pronk24], and it has been used for a few projects dealing with aerial and space lidar datasets, eg @Meschin23 and @Kan24.

While startinpy was developed primarily for terrains, it can be used as an easy-to-use and fast 2D Delaunay triangulator (and Voronoi diagram generator), which, as elaborated in @Aurenhammer12, are two structures that play an essential role in several disciplines: astronomy, geology, ecology, engineering, etc.

![A lidar dataset terrain reconstructed with startinpy and visualised with another Python library (Polyscope).](polyscope_gui.jpg)

# Statement of need

While there exist many Python libraries for computing the DT in 2D (a search for "Delaunay triangulation" in the *Python Package Index* (PyPI) returns 227 packages), most of them are not fully suitable for the modelling of 2.5D triangulated terrain.

startinpy has the following properties, which greatly improve the modelling and processing of 2.5D terrains.

**It is fast for large datasets.**
With a recent lidar scanner, we can easily collect 50 samples/$m^2$, which means that a 1$km^2$ area will contain 50+ million samples. 
Since constructing a DT requires several steps, if those steps are implemented in pure Python then the library becomes very slow.
As can be seen in the [DT construction comparison](https://startinpy.readthedocs.io/latest/comparison.html), startinpy is faster than most other libraries for large datasets.
This is partly because it is 100% developed in Rust; the core library is called "startin" and its the source code is available at https://github.com/hugoledoux/startin.

**Its data structure is exposed.**
Most libraries only return a list of vertices and triangles (triplets of vertex identifiers), which means that the user has to build an auxiliary graph to be able to find the adjacent triangles of a given one, or to find all the triangles that are incident to a given vertex (eg to calculate the normal).
The library's name comes from the fact that the data structure implemented is based on the concept of *stars* in a graph [@Blandford05], which allows us to store adjacency and incidence, and have a very compact data structure.
startinpy exposes methods to search triangles, to find the adjacent triangles of a triangle and the incident triangles to a vertex.


**The DT is incrementally constructed and deletion of vertices is possible.**
Unlike the majority of 2D DT implementations, startinpy implements an *incremental* insertion algorithm [@Lawson72], which allows for, for instance, constructing a simplified TIN that best approximates the original terrain with only 10% of the points, see @Garland95 for different strategies.
startinpy also implements a modification of the deletion algorithm in @Mostafavi03, I have extended it to allow the deletion of vertices on the boundary of the convex hull.
The deletion of vertices in a DT is useful to remove outliers (which are detected by analysing neighbouring triangles in the DT) and for the implementation of the natural neighbours interpolation method [@Sibson81].

**The z-values are stored and xy-duplicates handled.**
Some libraries allow us to attach extra information to a vertex, but most often one has to build an auxiliary data structure in Python to manage those.
Doing so is error-prone, tedious, and makes operations in 3D more complex (eg calculating the slope of an area, calculating the normal of a vertex, estimating the elevation with spatial interpolation, calculating volumes).
Furthermore, startinpy allows us to merge vertices that are close to each other (in the xy-plane; the tolerance can be defined by the user) and if there are xy-duplicates, then a user-defined z-value can be kept (eg lowest or highest, depending on the application).


**Extra attributes can be stored in the DT.**
It is possible to attach extra attributes with each vertex of the terrain.
This can be used to preserve the lidar properties of the input points (eg intensity, RGB, number of returns, etc.).


The [documentation of startinpy](https://startinpy.rtfd.io) contains several examples of the library and how it can be used to prepare datasets for input to machine learning algorithms, to convert to different formats used in practice, to interpolate, etc.


# Acknowledgements

I acknowledge the help of the students following the course [*Digital terrain modelling (GEO1015)* at TUDelft](https://3d.bk.tudelft.nl/courses/geo1015/) over the last few years.
Their feedback, questions, and frustrations on preliminary versions of startinpy helped me greatly.

# References
