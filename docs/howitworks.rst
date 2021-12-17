
How startinpy works
===================


The vertices are stored in a list of points, each vertex is indexed by its position (0-based).

The first vertex is the *infinite vertex*, and has no coordinates (it has this: [-99999.99999, -99999.99999, -99999.99999]).


A triangle is simply a triplet of vertex indices, eg [4, 2, 11], these are always ordered counter-clockwise (CCW).

