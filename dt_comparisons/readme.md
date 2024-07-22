
## Python library tested

  1. [Delaunator](https://github.com/HakanSeven12/Delaunator-Python): pure Python port of a proven fast triangulator written original in JavaScript https://github.com/mapbox/delaunator. 
  2. [SciPy](https://docs.scipy.org/doc/scipy/reference/generated/scipy.spatial.Delaunay.html): wrapper around [Qhull](http://qhull.org/), written in C. Using the batch construction in 2D.
  3. [SciPy-inc](https://docs.scipy.org/doc/scipy/reference/generated/scipy.spatial.Delaunay.html): wrapper around [Qhull](http://qhull.org/), written in C. Using the incremental insertion (as startinpy does).
  4. [Triangle](https://pypi.org/project/triangle/): wrapper around the [fast and robust C library](https://www.cs.cmu.edu/~quake/triangle.html) that performs constrained DT and meshing


## Datasets

  1. __random_10k__: 10,000 points randomly generated in a unit square
  2. __random_50k__: 50,000 points randomly generated in a unit square
  3. __LAZ_2M__: a real-world subset of the [AHN4 dataset](https://www.ahn.nl/) (covering completely the Neterlands). The sub-tile [04GN2_21](https://geotiles.citg.tudelft.nl/AHN4_T/04GN2_21.LAZ) contains 2,144,049 points.
  4. __LAZ_33M__: a real-world subset of the [AHN4 dataset](https://www.ahn.nl/). The sub-tile [69EZ1_21.LAZ](https://geotiles.citg.tudelft.nl/AHN4_T/69EZ1_21.LAZ) contains 33,107,889 points.
  5. __dem.tiff__: the GeoTIFF file in `/data/` is a 550x505 gridded terrain. We take the centre of each cell, reading row-by-row and column-by-column, and this creates a 277,750 dataset of points that are collinear and cocircular with many others (degenerate cases for the DT).


## Results


|            |random_10k|random_50k|LAZ_2M|LAZ_33M|dem.tiff|
|:-----------|----------|---------:|-----:|------:|-------:|
| delaunator |   0.219  |    0.84  | 49.2 | 898.1 |   3.55 |
| scipy      |   0.017  |    0.09  | 10.1 | 650.3 |   1.79 |
| scipy-inc  |   0.015  |    0.08  |    X |     X |      X |
| triangle   |   0.004  |    0.02  |  0.9 |  16.8 |   0.19 |
| startinpy  |   0.018  |    0.18  |  4.2 |  41.8 |   0.46 |

## To replicate

  1. install those packages:
    
    - numpy
    - laspy
    - rasterio
    - time
    - startinpy
    - triangle
    - scipy
    - https://github.com/HakanSeven12/Delaunator-Python
    - py_markdown_table

  2. download the 2 LAZ files
  3. change the path (lines 17+18)
  4. `python comparisons.py`, this generates a summary table in Markdown


# Comparison with the main alternatives

The two tables below compare startinpy to its main alternatives.

The [Delaunator package](https://github.com/HakanSeven12/Delaunator-Python) is a pure Python port of a proven fast triangulator [originally written in JavaScript](https://github.com/mapbox/delaunator).
[SciPy](https://docs.scipy.org/doc/scipy/reference/generated/scipy.spatial.Delaunay.html) is `scipy.spatial.Delaunay`, and SciPy-inc is the variation where an incremental algorithm is used (instead of a batch process).
[Triangle](https://pypi.org/project/triangle/) is the Python bindings of the C code.

|                        | Delaunator | SciPy | SciPy-inc | Triangle | startinpy |
|------------------------|:----------:|:-----:|:---------:|:--------:|:---------:|
| constrained DT         |    --      | --    |     --    |   +      |   --      |
| data structure exposed |    +       | --    |     --    |   --     |   +       |
| efficient searches     |    +       | +     |     +     |   --     |   +       |
| elevation/2.5D         |    --      | --    |     --    |   --     |   +       |
| extra attributes       |    --      | --    |     --    |   --     |   +       |
| speed                  |    --      | --    |     --    |   ++     |   +       |
| incremental insertion  |    --      | --    |     +     |   --     |   +       |
| remove vertices        |    --      | --    |     --    |   --     |   +       |
| xy-duplicate handling  |    --      | --    |     --    |   --     |   +       |

Notice that startinpy is the only one capable of storing z-values and extra attributes, the others are pure 2D Delaunay triangulators.
The parameter 'efficient searches' refers to the availability of a point location function to find the closest triangles to a given point.
The parameter 'xy-duplicate handling' refers to the fact that startinpy allows us to merge vertices that are close to each other (in the xy-plane; the tolerance can be defined by the user) and that if there are xy-duplicates, then a user-defined z-value can be kept (eg lowest or highest, depending on the application).

The table below shows the time it takes to construct the 2D DT--in a batch operation--for different datasets.
The details of the (openly available) datasets are available on the [GitHub repository of startinpy](https://github.com/hugoledoux/startinpy/tree/master/dt_comparisons), and the Python code to replicate the experiments is available.
The datasets `random_X` are randomly generated points in a unit square, the first one has 10,000 points and the other 50,000 points.
The datasets `LAZ_X` are real-world aerial lidar datasets publicly available in the Netherlands, the `2M` contains exactly 2,144,049 points, and the `33M` contains exactly 33,107,889 points.
The dataset `dem.tiff` is the GeoTIFF file in `/data/`, the centre of each grid cell is inserted by reading sequentially the rows and columns, the total is 277,750 points.

|            |random_10k|random_50k|LAZ_2M|LAZ_33M|dem.tiff|
|:-----------|---------:|---------:|-----:|------:|-------:|
| Delaunator |   0.219  |    0.84  | 49.2 | 898.1 |   3.55 |
| SciPy      |   0.017  |    0.09  | 10.1 | 650.3 |   1.79 |
| SciPy-inc  |   0.015  |    0.08  |    X |     X |      X |
| Triangle   |   0.004  |    0.02  |  0.9 |  16.8 |   0.19 |
| startinpy  |   0.018  |    0.18  |  4.2 |  41.8 |   0.46 |

If "X" is written, it is because the returned DT was faulty: for large inputs, Scipy-inc returns a few triangles only, not the full DT.

Notice that while startinpy is somewhat slower than Triangle, it is expected since, as explained above, it offers more convenience for the modelling of triangulated terrains, and its data structure is exposed.
Notice also that startinpy is faster and more stable than SciPy (no crash or wrong results) for large datasets.

