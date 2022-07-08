
[![GitHub license](https://img.shields.io/github/license/hugoledoux/startin_python)](https://github.com/hugoledoux/startin_python/blob/master/LICENSE) [![PyPI version](https://badge.fury.io/py/startinpy.svg)](https://badge.fury.io/py/startinpy) [![docs](https://img.shields.io/badge/docs-startinpy.rtfd.io-brightgreen)](https://startinpy.rtfd.io/)

startinpy
=========

Python bindings for [startin](https://github.com/hugoledoux/startin), a Delaunay triangulator for the modelling of terrains.


Installation
============

pip
---

To install the latest release: `pip install startinpy`


If you want to compile it yourself
----------------------------------

1. install latest [Rust](https://www.rust-lang.org/)
2. install [maturin](https://github.com/PyO3/maturin)
3. `maturin build --release`
4. `cd ./target/wheels/`
5. `pip install [name-wheel].whl` will install it to your local Python

Development
-----------

  1. install [Rust](https://www.rust-lang.org/) (v1.39+)
  2. install [maturin](https://github.com/PyO3/maturin) 
  3. `maturin develop`
  4. move to another folder, and `import startinpy` shouldn't return any error


Documentation
=============

https://startinpy.rtfo.io


Examples
========

```python
import startinpy
import numpy as np


#-- generate 100 points randomly in the plane
rng = np.random.default_rng()
pts = rng.random((100, 3))
pts = pts * 100

dt = startinpy.DT()
dt.insert(pts)

#-- remove vertex #4
try:
    dt.remove(4)
except Exception as e:
    print(e)

print("# vertices:", dt.number_of_vertices())
print("# triangles:", dt.number_of_triangles())

print("CH: ", dt.convex_hull())

print(dt.is_triangle([4, 12, 6]) )
print(dt.is_triangle([5, 12, 6]) )

print("--- /Points ---")
for each in dt.points:
    print(each)
print("--- Points/ ---")

alltr = dt.triangles
print(alltr[3])

try:
    zhat = dt.interpolate_tin_linear(55.2, 33.1)
    print("result: ", zhat)
except Exception as e:
    print(e)
```

It can read LAS/LAZ and output GeoJSON files too:

```python
import startinpy
t = startinpy.DT()
t.read_las("/home/elvis/myfile.laz")
print("# vertices:", t.number_of_vertices())
print("# triangles:", t.number_of_triangles())
t.write_geojson("/home/elvis/output.geojson")
```




