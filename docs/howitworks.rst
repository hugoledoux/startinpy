
How startinpy works
===================

startinpy source code in written in `Rust <https://www.rust-lang.org/>`_ (called just 'startin', `original source code <https://github.com/hugoledoux/startin>`_).

It uses an incremental algorithm for the construction of a Delaunay triangulation (constraints are *not* supported), that is each point are inserted one after another and triangulation is updated between each insertion.
The algorithm is based on flips.

The data structure is a cheap implementation of the star-based structure defined in `Blandford et al. (2003) <https://citeseerx.ist.psu.edu/viewdoc/summary?doi=10.1.1.9.6823>`_, cheap because the link of each vertex is stored a simple array and not in an optimised blob like they did.
It results in a pretty fast library (comparison will come at some point), but it uses more space than the optimised one.

The deletion of a vertex is also possible. 
The algorithm implemented is a modification of the one of `Mostafavi, Gold, and Dakowicz (2003) <https://doi.org/10.1016/S0098-3004(03)00017-7>`_. 
The ears are filled by flipping, so it's in theory more robust. 
I have also extended the algorithm to allow the deletion of vertices on the boundary of the convex hull. 
The algorithm is sub-optimal, but in practice the number of neighbours of a given vertex in a DT is only 6, so it doesn't really matter.

Robust arithmetic for the geometric predicates are used (`Shewchuk's predicates <https://www.cs.cmu.edu/~quake/robust.html>`_, well the `Rust port of the code <https://crates.io/crates/robust>`_), so startin/py is robust and shouldn't crash (touch wood). 

The implementation of startinpy uses the idea of *infinite triangles* and of *infinite vertex*, this simplifies a lot the algorithm and ensures that one can insert new points outside the convex hull of a dataset (or even delete some vertices on the boundary of the convex hull).
The CGAL library also does this, and `it is well explained here <https://doc.cgal.org/latest/Triangulation_2/classCGAL_1_1Triangulation__2.html>`_.
This is why the set of points (:func:`startinpy.DT.points`) has its first vertex as the *infinity vertex*.