# Comparison 

The two tables below compare startinpy to a few of its main alternatives:

* [__Delaunator-py__](https://github.com/HakanSeven12/Delaunator-Python) is a pure Python port of a proven fast triangulator [originally written in JavaScript](https://github.com/mapbox/delaunator).
* [__Delaunay-pdal__](https://pdal.io/en/2.7.2/stages/filters.delaunay.html) is the Delaunay triangulator of [PDAL](https://pdal.io), a general point data library that reads, filters, and writes point clouds. It uses a [CPP implementation of the Delaunator algorithm](https://github.com/delfrrr/delaunator-cpp).
* [__SciPy__](https://docs.scipy.org/doc/scipy/reference/generated/scipy.spatial.Delaunay.html) is `scipy.spatial.Delaunay`, and __SciPy-inc__ is the variation where an incremental algorithm is used (instead of a batch process).
* [__Triangle__](https://pypi.org/project/triangle/) is the Python bindings of the C code.

|                        | Delaunator-py | Delaunator-pdal | SciPy | SciPy-inc | Triangle | startinpy |
|------------------------|:-------------:|:---------------:|:-----:|:---------:|:--------:|:---------:|
| constrained DT         |    --         |    --           | --    |     --    |   +      |   --      |
| data structure exposed |    +          |    --           | --    |     --    |   --     |   +       |
| efficient searches     |    +          |    --           | +     |     +     |   --     |   +       |
| elevation/2.5D         |    --         |    +            | --    |     --    |   --     |   +       |
| extra attributes       |    --         |    --           | --    |     --    |   --     |   +       |
| speed                  |    --         |    ++           | --    |     --    |   ++     |   +       |
| incremental insertion  |    --         |    --           | --    |     +     |   --     |   +       |
| remove vertices        |    --         |    --           | --    |     --    |   --     |   +       |
| xy-duplicate handling  |    --         |    --           | --    |     --    |   --     |   +       |

Notice that startinpy is the only one capable of storing z-values and extra attributes, the others are pure 2D Delaunay triangulators.
The parameter 'efficient searches' refers to the availability of a point location function to find the closest triangles to a given point.
The parameter 'xy-duplicate handling' refers to the fact that startinpy allows us to merge vertices that are close to each other (in the xy-plane; the tolerance can be defined by the user) and that if there are xy-duplicates, then a user-defined z-value can be kept (eg lowest or highest, depending on the application).

The table below shows the time it takes to construct the 2D DT--in a batch operation--for different datasets.
The details of the (openly available) datasets are available on the [GitHub repository of startinpy](https://github.com/hugoledoux/startinpy/tree/master/dt_comparisons), and the Python code to replicate the experiments is available.
The datasets `random_X` are randomly generated points in a unit square, the first one has 10,000 points and the other 50,000 points.
The datasets `LAZ_X` are real-world aerial lidar datasets publicly available in the Netherlands, the `2M` contains exactly 2,144,049 points, and the `33M` contains exactly 33,107,889 points.
The dataset `dem.tiff` is the GeoTIFF file in `/data/`, the centre of each grid cell is inserted by reading sequentially the rows and columns, the total is 277,750 points.

|               |random_10k|random_50k|dem.tiff|LAZ_2M|LAZ_33M|
|---------------|----------|----------|--------|------|-------|
| Delaunator-py |   0.219  |   0.840  |  3.550 | 49.2 | 898.1 |   
|Delaunator-pdal|   0.003  |   0.014  | 27.409 |  1.5 |  27.4 |
|     SciPy     |   0.026  |   0.120  |  1.563 |  9.9 | 650.3 |
|   SciPy-inc   |   0.021  |   0.136  |      X |    X |     X |
|    triangle   |   0.004  |   0.018  |  0.179 |  0.9 |  16.0 |
|   startinpy   |   0.017  |   0.175  |  0.437 |  3.9 |  41.2 |

If "X" is written, it is because the returned DT was faulty: for large inputs, Scipy-inc returns a few triangles only, not the full DT.

Notice that while startinpy is somewhat slower than Triangle and Delaunator-pdal, it is expected since, as explained above, it offers more convenience for the modelling of triangulated terrains, and its data structure is exposed.
Notice also that startinpy is faster and more stable than SciPy (no crash or wrong results) for large datasets.

