# Comparisons

Rough benchmarks of various Delaunay triangulation libraries against random points, real-world point clouds, and a real world DEM.

## Python libraries tested

  1. [Delaunator](https://github.com/HakanSeven12/Delaunator-Python): pure Python port of a proven fast triangulator written original in JavaScript <https://github.com/mapbox/delaunator>.
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

  1. install those packages: `pip install -r requirements.txt`
  2. download the 2 LAZ files and Delaunator.py: `./download`
  3. `python comparisons.py`, this generates a summary table in Markdown
