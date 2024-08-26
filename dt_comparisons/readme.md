
# Comparison of startinpy with other Python libraries

Rough benchmarks of various Delaunay triangulation libraries against random points, real-world point clouds, and a real world DEM.

## Python library tested

  1. [Delaunator-py](https://github.com/HakanSeven12/Delaunator-Python): pure Python port of a proven fast triangulator written original in JavaScript <https://github.com/mapbox/delaunator>. 
  2. [SciPy](https://docs.scipy.org/doc/scipy/reference/generated/scipy.spatial.Delaunay.html): wrapper around [Qhull](http://qhull.org/), written in C. Using the batch construction in 2D.
  3. [SciPy-inc](https://docs.scipy.org/doc/scipy/reference/generated/scipy.spatial.Delaunay.html): wrapper around [Qhull](http://qhull.org/), written in C. Using the incremental insertion (as startinpy does).
  4. [Triangle](https://pypi.org/project/triangle/): wrapper around the [fast and robust C library](https://www.cs.cmu.edu/~quake/triangle.html) that performs constrained DT and meshing
  5. [PDAL](https://pdal.io): general point data library that reads, filters, and writes point clouds. Has a [Delaunay triangulation implementation](https://pdal.io/en/2.7.2/stages/filters.delaunay.html) based on [delaunator-cpp](https://github.com/delfrrr/delaunator-cpp).


## Datasets

  1. __random_10k__: 10,000 points randomly generated in a unit square
  2. __random_50k__: 50,000 points randomly generated in a unit square
  3. __LAZ_2M__: a real-world subset of the [AHN4 dataset](https://www.ahn.nl/) (covering completely the Neterlands). The sub-tile [04GN2_21](https://geotiles.citg.tudelft.nl/AHN4_T/04GN2_21.LAZ) contains 2,144,049 points.
  4. __LAZ_33M__: a real-world subset of the [AHN4 dataset](https://www.ahn.nl/). The sub-tile [69EZ1_21.LAZ](https://geotiles.citg.tudelft.nl/AHN4_T/69EZ1_21.LAZ) contains 33,107,889 points.
  5. __dem.tiff__: the GeoTIFF file in `/data/` is a 550x505 gridded terrain. We take the centre of each cell, reading row-by-row and column-by-column, and this creates a 277,750 dataset of points that are collinear and cocircular with many others (degenerate cases for the DT).


## Results

Those results were obtained on a MacBook Pro, M3 Pro, 36GB of RAM, running macOS v14.5


|               |random_10k|random_50k|dem.tiff|LAZ_2M|LAZ_33M|
|---------------|----------|----------|--------|------|-------|
| Delaunator-py |   0.219  |    0.84  |  3.550 |  49.2| 898.1 |   
|Delaunator-pdal|   0.003  |   0.014  | 27.409 |   1.5|  27.4 |
| Sciy          |   0.017  |    0.09  | 10.1   | 650.3|   1.8 |
| SciPy-inc     |   0.015  |    0.08  |    X   |   X  |     X |
|    triangle   |   0.004  |   0.018  |  0.179 | 0.9  |  16.0 |
|   startinpy   |   0.017  |   0.175  |  0.437 | 3.9  |  41.2 |



## To replicate

  1. `pip install -r requirements.txt` 
    - make sure you have LAZ for laspy: `pip install 'laspy[lazrs]'`
    - (for macOS `pip intall triangle` is broken, use `pip install triangle2` instead, see <https://pypi.org/project/triangle2/>)
  2. put in same folder `Delaunator.py` from <https://github.com/HakanSeven12/Delaunator-Python> (there is no installer)
  3. download the 2 LAZ files:
    - `wget https://geotiles.citg.tudelft.nl/AHN4_T/04GN2_21.LAZ`
    - `wget https://geotiles.citg.tudelft.nl/AHN4_T/69EZ1_21.LAZ`
  4. `python comparisons.py`, this generates a summary table in Markdown

