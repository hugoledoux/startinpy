
## Python library tested

  1. [Delaunator](https://github.com/HakanSeven12/Delaunator-Python): pure Python
  2. [delaunay](https://pypi.org/project/delaunay/): pure Python
  3. [SciPy](https://docs.scipy.org/doc/scipy/reference/generated/scipy.spatial.Delaunay.html): wrapper around [Qhull](http://qhull.org/), written in C. Using the batch construction in 2D.
  4. [SciPy-inc](https://docs.scipy.org/doc/scipy/reference/generated/scipy.spatial.Delaunay.html): wrapper around [Qhull](http://qhull.org/), written in C. Using the incremental insertion (as startinpy does).
  5. [Triangle](https://pypi.org/project/triangle/): wrapper around the [fast and robust C library](https://www.cs.cmu.edu/~quake/triangle.html) that performs constrained DT and meshing


## Datasets

  1. __random_10k__: 10000 points randomly generated in a unit square
  2. __random_50k__: 50000 points randomly generated in a unit square
  3. __LAZ_2M__: a real-world subset of the [AHN4 dataset](https://www.ahn.nl/) (covering completely the Neterlands). The sub-tile [04GN2_21](https://geotiles.citg.tudelft.nl/AHN4_T/04GN2_21.LAZ) contains 2 144 049 points.
  4. __LAZ_33M__: a real-world subset of the [AHN4 dataset](https://www.ahn.nl/). The sub-tile [69EZ1_21.LAZ](https://geotiles.citg.tudelft.nl/AHN4_T/69EZ1_21.LAZ) contains 33 107 889 points.
  5. __dem.tiff__: the GeoTIFF file in `/data/` is a 550x505 gridded terrain. We take the centre of each cell and this create a 277 750 dataset.


## To replicate

  1. install all the packages above
  2. download the 2 LAZ files
  3. change the path (lines 17+18)
  4. `python comparisons`

