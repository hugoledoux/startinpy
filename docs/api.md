# API 

startinpy does not have specific classes and/or objects for points, vertices, and triangles.
[NumPy arrays](https://numpy.org/doc/stable/reference/arrays.html) of floats and integers are instead used.

A **Point** is an array of 3 floats (x-coordinate, y-coordinate, z-coordinate):

```python
>>> import startinpy
>>> dt = startinpy.DT()
>>> dt.insert_one_pt([11.3, 22.2, 4.7])
>>> dt.points[1]
array([11.3, 22.2, 4.7])
```

A **Vertex** is an integer, it is the index in the array of points ({func}`startinpy.DT.points`, which is 0-based).

A **Triangle** is an array of 3 integers, the values of the indices of the 3 vertices (ordered counter-clockwise) in the array of points ({func}`startinpy.DT.points`, which is 0-based).

```python
>>> dt.triangles[2]
array([1, 3, 2], dtype=uint64)
>>> #-- z-coordinate of 3rd vertex of the same triangle
>>> dt.points[dt.triangles[2][2]][2]
3.3
```

:::{IMPORTANT}
The first vertex in the list of points is the **infinite vertex**, and has "infinity" coordinates (`[inf, inf, inf]`). It is used internally to ensure that the whole DT is consistent. No finite Triangle refers to the vertex, but infinite Triangles do. {ref}`See more info and some examples <infinite>`.
:::

```{eval-rst}
.. autoclass:: startinpy.DT
   :members:
```
