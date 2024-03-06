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

The Python library startinpy allows us to model and process terrains with a triangulation. 
This is used to represent the morphology of a given area (hills, valleys, but buildings and other man-structures can be included) by using elevation points and to derive and calculate properties from that triangulation.

A terrain library

(which are 2.5D objects) using a two-dimensional Delaunay triangulation.
This means that while a triangulation is computed in 2D, the *z*-elevation of the vertices are kept.

Such a library is necessary to represent the morphology of an area, when one wants to avoid using grids and prefers a leaner representation with points and triangles.

The underlying code of startinpy is written in the language Rust (so it's rather fast) and robust arithmetic is used (so it shouldn't crash).

startinpy uses the [startin Rust library](https://github.com/hugoledoux/startin) and adds several utilities and functions, for instance [NumPy](https://numpy.org/) support for input/output, exporting to several formats, and easy-of-use.


# Statement of need

- 2D DT and difficult to keep the z-values, especially with xy-duplicates
- 3D DT is the solution
- 2.5D specific triangulation == no idea how
- only batch operation available, that is you give a certain of points and you get a list of triangles.
- but for many apps one wants to modify this triangulation (to simplify it by removing least important points + perform interpolations + add points somewhere else where more differences in elevation)
- SciPy has only batch, hte incremental is buggy and is very slow
- Triangle from Shewchuk is not 2.5D and complex to manage


# Functionalities of startinpy

startinpy is incrementation insertion, deletion is possible, and is using NumPy for i/o so that it is easy to pair with laspy and others libraries.


Several functions that are usful


# Citations

Citations to entries in paper.bib should be in
[rMarkdown](http://rmarkdown.rstudio.com/authoring_bibliographies_and_citations.html)
format.

If you want to cite a software repository URL (e.g. something on GitHub without a preferred
citation) then you can do it with the example BibTeX entry below for @fidgit.

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