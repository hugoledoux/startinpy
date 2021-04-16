
[![GitHub license](https://img.shields.io/github/license/hugoledoux/startin_python)](https://github.com/hugoledoux/startin_python/blob/master/LICENSE) [![PyPI version](https://badge.fury.io/py/startinpy.svg)](https://badge.fury.io/py/startinpy)
startinpy
=========

Python bindings for [startin](https://github.com/hugoledoux/startin), a Delaunay triangulator for the modelling of terrains.


Installation
============

pip
---

To install the latest release: `pip install startinpy`


Development
-----------

  1. install [Rust](https://www.rust-lang.org/) (v1.39+)
  2. install [maturin](https://github.com/PyO3/maturin) 
  3. `maturin develop`
  4. move to another folder, and `import startinpy` shouldn't return any error


Documentation
=============

Simple documentation of the API is available in the folder `/docs/`


Examples
========

```python
import startinpy

pts = []
pts.append([0.0, 0.0, 11.11])
pts.append([1.0, 0.0, 22.22])
pts.append([1.0, 1.0, 33.33])
pts.append([0.0, 1.0, 44])
pts.append([0.5, 0.49, 44])
pts.append([0.45, 0.69, 44])
pts.append([0.65, 0.49, 44])
pts.append([0.75, 0.29, 44])
pts.append([1.5, 1.49, 44])
pts.append([0.6, 0.2, 44])
pts.append([0.45, 0.4, 44])
pts.append([0.1, 0.8, 44])

t = startinpy.DT()
t.insert(pts)

#-- remove vertex #4
t.remove(4)

print("# vertices:", t.number_of_vertices())
print("# triangles:", t.number_of_triangles())

print("CH: ", t.convex_hull())

itrs = t.incident_triangles_to_vertex(4);
print(itrs)

print(t.is_triangle([4, 12, 6]) )
print(t.is_triangle([5, 12, 6]) )

print("--- /Vertices ---")
for each in t.all_vertices():
    print(each)
print("--- Vertices/ ---")

alltr = t.all_triangles()
print(alltr[3])
```

It can read LAS/LAZ and output OBJ files too:

```python
import startinpy

t = startinpy.DT()
t.read_las("/home/elvis/myfile.laz")

print("# vertices:", t.number_of_vertices())
print("# triangles:", t.number_of_triangles())

t.write_obj("/home/elvis/output.obj")
```




