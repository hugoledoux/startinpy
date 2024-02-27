# How startinpy works

## Original code written in rust

The library for calculating the Delaunay triangulation is originally written in [Rust](https://www.rust-lang.org/), it is called just 'startin' and its [ source code is open](https://github.com/hugoledoux/startin).

Robust arithmetic for the geometric predicates are used ([Shewchuk's predicates](https://www.cs.cmu.edu/~quake/robust.html), well the [Rust port of the code](https://crates.io/crates/robust)), so startin/py is robust and shouldn't crash (touch wood).

## Insertion + deletion are possible

It uses an incremental algorithm for the construction of a Delaunay triangulation (constraints are *not* supported), that is each point are inserted one after another and triangulation is updated between each insertion.
The algorithm is based on flips.

The deletion of a vertex is also possible.
The algorithm implemented is a modification of the one of [Mostafavi, Gold, and Dakowicz (2003)](<https://doi.org/10.1016/S0098-3004(03)00017-7>).
The ears are filled by flipping, so it's in theory more robust.
I have also extended the algorithm to allow the deletion of vertices on the boundary of the convex hull.
The algorithm is sub-optimal, but in practice the number of neighbours of a given vertex in a DT is only 6, so it doesn't really matter.

## The data structure

The data structure of the Rust code is a cheap implementation of the star-based structure defined in [Blandford et al. (2003)](https://citeseerx.ist.psu.edu/viewdoc/summary?doi=10.1.1.9.6823), cheap because the link of each vertex is stored a simple array and not in an optimised blob like they did.
It results in a pretty fast library (comparison will come at some point), but it uses more space than the optimised one.

However, the stars are *not* exposed in startinpy to keep it a simple and higher-level library.

The data structure of startinpy has 2 arrays:

1. an array of **Points**, where each entry is an array of 3 floats (x-coordinate, y-coordinate, z-coordinate)
2. an array of **Triangles**, where each **Triangle** is an array of 3 integers, the values of the indices of the 3 vertices (ordered counter-clockwise) in the array of **Points** ({func}`startinpy.DT.points`, which is 0-based, 0 being the infinite vertex).

A **Vertex** is an integer, it is the index in the array of points ({func}`startinpy.DT.points`, which is 0-based).

If you delete a vertex (with {func}`startinpy.DT.remove`) then the entry in the array of **Points** is not deleted (this would be slow because arrays are contiguous and a lot of copying would be necessary), instead the vertex/point is flagged as being removed and none of the **Triangles** will refer to it.

(infinite)=

## Infinite vertex and triangles

```{image} figs/infinite.png
:align: right
:width: 40%
```

The implementation of startinpy has *infinite triangles* and one *infinite vertex*, this simplifies a lot the algorithms and ensures that one can insert new points outside the convex hull of a dataset (or even delete some vertices on the boundary of the convex hull).
The CGAL library also does this, and [it is well explained here](https://doc.cgal.org/latest/Triangulation_2/classCGAL_1_1Triangulation__2.html).

The *infinite vertex* is the first vertex in the array of points ({func}`startinpy.DT.points`) and thus it has the index of 0 (zero).
It has infinite coordinates (`[inf inf inf]`), they are of type [numpy infinity](https://numpy.org/devdocs/reference/constants.html#numpy.inf).

An *infinite triangle* is a triangle having the infinite vertex as one of its vertices.

In the figure, notice that there are 5 finite triangles (126, 236, 346, 456, 516), but the data structure actually stores 5 extra infinite triangles (102, 150, 540, 304, 203).
Those are adjacent to the 5 edges on the boundary of the convex hull of the dataset.
You can conceptualise the triangulation has being embedded on a sphere, and the infinite vertex is on the other side.

## Some examples of the data structure and infinity

```{image} figs/tr.png
:align: right
:width: 50%
```

For instance, consider this 5-vertex Delaunay triangulation:

```python
import startinpy
import numpy as np

np.set_printoptions(precision=10)

t = startinpy.DT()
t.insert_one_pt(0.5, 0.5, 1.0)
t.insert_one_pt(0.0, 0.0, 2.0)
t.insert_one_pt(1.0, 0.0, 3.0)
t.insert_one_pt(1.0, 1.0, 4.0)
t.insert_one_pt(0.0, 1.0, 5.0)

print(t.points)
print(t.triangles)
```

Which outputs this below.
Notice first that there are a total of 6 vertices: the 5 we inserted plus the infinite vertex (at index-0 with infinity coordinates `[inf inf inf]`).
Notice also no finite triangles refers to the vertex 0.

```
[[inf inf inf]
 [0.5 0.5 1. ]
 [0.  0.  2. ]
 [1.  0.  3. ]
 [1.  1.  4. ]
 [0.  1.  5. ]]
[[1 2 3]
 [1 3 4]
 [1 4 5]
 [1 5 2]]
```

However, startinpy stores internally infinite triangles.
For instance, if you retrieve the triangles incident to a given vertex on the convex hull:

```python
re = t.incident_triangles_to_vertex(2)
for each in re:
    print(each)
```

```
[2 0 3]
[2 3 1]
[2 1 5]
[2 5 0]
```

you will notice that 2 triangles are infinite: `[2 0 3]` and `[2 5 0]`.

Also, if you remove one vertex (eg the one in the middle of the square, vertex 1), observe that now its coordinates are ["Not a Number (nan)"](https://numpy.org/devdocs/reference/constants.html#numpy.nan), and that no triangle in the DT refers to it anymore:

```python
t.remove(1)
print(t.points)
print(t.triangles)
print(t.is_vertex_removed(1))
```

```
[[inf inf inf]
 [nan nan nan]
 [ 0.  0.  2.]
 [ 1.  0.  3.]
 [ 1.  1.  4.]
 [ 0.  1.  5.]]
[[2 3 4]
 [2 4 5]]
True
```

Finally, you can remove the unused/deleted vertices from the {func}`startinpy.DT.points` array by using {func}`startinpy.DT.collect_garbage`, which will assign a new ID to most vertices, and triangles will be updated too.
Notice that now 5 vertices are in the array, and only 2 finite triangles are in the DT.

```python
t.collect_garbage()
print(t.points)
print(t.triangles)
```

```
[[inf inf inf]
 [ 0.  0.  2.]
 [ 1.  0.  3.]
 [ 1.  1.  4.]
 [ 0.  1.  5.]]
[[1 2 3]
 [1 3 4]]
```
