# How startinpy works

## Original code written in Rust

The library for calculating the Delaunay triangulation is originally written in [Rust](https://www.rust-lang.org/), it is called 'startin' and its [source code is open](https://github.com/hugoledoux/startin).

Robust arithmetic for the geometric predicates is used ([Shewchuk's predicates](https://www.cs.cmu.edu/~quake/robust.html), well the [Rust port of the code](https://crates.io/crates/robust)), so startin/py is robust and shouldn't crash (touch wood).


## Insertion + deletion are possible

It uses an incremental algorithm for the construction of a Delaunay triangulation (constraints are *not* supported), that is each point is inserted one after another and the triangulation is updated between each insertion.
The algorithm is based on flips to transform the triangulation (see [Lawson (1972)](https://doi.org/10.1016/0012-365X(72)90093-3)).

The deletion of a vertex is also possible.
The algorithm implemented is a modification of the one of [Mostafavi, Gold, and Dakowicz (2003)](https://doi.org/10.1016/S0098-3004(03)00017-7).
The ears are also filled by flipping, so it's in theory more robust.
I have also extended the algorithm to allow the deletion of vertices on the boundary of the convex hull.
The algorithm is sub-optimal, but, in practice, the number of neighbours of a given vertex in a DT is only 6, so it doesn't really matter.


## The data structure

The data structure of the Rust code is a cheap implementation of the star-based structure defined in [Blandford et al. (2003)](https://citeseerx.ist.psu.edu/viewdoc/summary?doi=10.1.1.9.6823); cheap because the link of each vertex is stored a simple array and not in an optimised blob like they did.
It results in a pretty fast library (see [comparison to alternatives](./comparison.md)), but it uses more space than the optimised original data structure.

However, notice that in startinpy the stars are *not* exposed to the user, to keep it a simple and higher-level library.

The data structure of startinpy is composed of 2 arrays:

1. an array of **Points**, where each entry is an array of 3 floats (x-coordinate, y-coordinate, z-coordinate)
2. an array of **Triangles**, where each **Triangle** is an array of 3 integers, the values of the indices of the 3 vertices (ordered counter-clockwise) in the array of **Points** ({func}`startinpy.DT.points`, which is 0-based, 0 being the [infinite vertex](#infinite)).

A **Vertex** is an integer, it is the index in the array of points ({func}`startinpy.DT.points`, which is 0-based).

If you delete a vertex (with {func}`startinpy.DT.remove`), then the entry in the array of **Points** is not deleted (this would be slow because arrays are contiguous and a lot of copying would be necessary), instead the vertex/point is flagged as being removed and none of the **Triangles** will refer to it ({func}`startinpy.DT.is_vertex_removed` can be used to test).


(infinite)=
## Infinite vertex and triangles

```{image} figs/infinite.png
:align: right
:width: 40%
```

The implementation of startinpy has one *infinite vertex* and *infinite triangles*, this greatly simplifies the algorithms and ensures that one can insert new points outside the convex hull of a dataset (or even delete some vertices on the boundary of the convex hull).
The CGAL library also does this, and [the internal workings are well explained here](https://doc.cgal.org/latest/Triangulation_2/classCGAL_1_1Triangulation__2.html).

The *infinite vertex* is the first vertex in the array of points ({func}`startinpy.DT.points`), and, thus, it has the index of 0 (zero).
It has infinite coordinates (`[inf inf inf]`), those are of type [numpy infinity](https://numpy.org/devdocs/reference/constants.html#numpy.inf).

An *infinite triangle* is a triangle having the infinite vertex as one of its vertices; a finite triangle doesn't have the infinite vertex as one of its 3 vertices.

In the figure, notice that there are 5 finite triangles (126, 236, 346, 456, 516), but the data structure actually stores 5 extra infinite triangles (102, 150, 540, 304, 203).
Those are adjacent to the 5 edges on the boundary of the convex hull of the dataset.
You can conceptualise the triangulation as being embedded on a sphere, and the infinite vertex is on the other side.


## Parameters to setup that will influence the DT

(snap_tolerance)=
### Snap tolerance

{func}`startinpy.DT.snap_tolerance`
(default=0.001)

Get/set the snap tolerance used to merge vertices during insertion.
Two vertices closer than this value (calculated in the xy-plane) will be merged during insertion.
The z-value preserved for that vertex is based on the [`duplicates_handling` parameter](#duplicates_handling). 

```python
dt = startinpy.DT()
dt.snap_tolerance = 0.05 #-- modify to 0.05unit
print("The snap tolerance is:", dt.snap_tolerance)
#-- The snap tolerance is: 0.05
```


(point_location)=
### Point location: latest triangle or first "jump"?

{func}`startinpy.DT.jump_and_walk`
(default=False)

Activate/deactivate the jump-and-walk for the point location.
If deactivated, the walk starts from the last inserted triangle; this is the default and should work fine for most real-world datasets.
If activated, then before a walk starts (to insert a new points in the DT), a subset of the points (a hard-coded value: n<sup>0.25</sup>) are sampled, the Euclidean distance to each is calculated, and the walk starts from the closest one.
This should be activated when the spatial coherence in the dataset is very low (eg if the points are randomly shuffled).

```python
dt = startinpy.DT()
dt.jump_and_walk = True
```

(duplicates_handling)=
### How xy-duplicates are handling

{func}`startinpy.DT.duplicates_handling`
(default="First")

Specify the method to handle xy-duplicates.
That is, if the insertion of a new point in the DT is impossible because a vertex already exists (based on {func}`startinpy.DT.snap_tolerance`), then we can decide which z-value we want to keep in the DT.

There are 4 options:

1. **First** (default): the z-value of the first point inserted at that xy-location is kept
2. **Last**: the z-value of the last point inserted at that xy-location is kept
3. **Lowest**: the lowest z-value is kept
4. **Highest**: the highest z-value is kept

```python
dt = startinpy.DT()
dt.duplicates_handling = "Highest"
```


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
t.insert_one_pt([0.5, 0.5, 1.0])
t.insert_one_pt([0.0, 0.0, 2.0])
t.insert_one_pt([1.0, 0.0, 3.0])
t.insert_one_pt([1.0, 1.0, 4.0])
t.insert_one_pt([0.0, 1.0, 5.0])

print(t.points)
print(t.triangles)
```

Which outputs this below.
Notice first that there are a total of 6 vertices: the 5 we inserted plus the infinite vertex (at index-0 with infinity coordinates `[inf inf inf]`).
Notice also that no finite triangle refers to the vertex 0.

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

:::{warning} 
The function {func}`startinpy.DT.collect_garbage` should be used with care as it is a very slow operation that requires copying all points/triangles in an array and recomputing the indices. Apply it before exporting the triangulation, not after each delete operation.
:::

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
