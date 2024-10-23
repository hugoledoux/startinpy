# Examples

## Reading a LAZ file

We recommend you use [laspy](https://laspy.readthedocs.io) (`pip install 'laspy[lazrs]'` to install) to read a LAS/LAZ into a NumPy array, and then pass that array to startinpy:

```{literalinclude} ../demo/example_reading_laz.py
```


## Exporting the DT to GeoJSON

```{literalinclude} ../demo/example_exporting_geojson.py
```


## Exporting the DT to several mesh formats with [meshio](https://github.com/nschloe/meshio)

```{literalinclude} ../demo/example_exporting_meshio.py
```

## Reading a GeoTIFF file with rasterio

We can use [rasterio](https://rasterio.readthedocs.io) to read a GeoTIFF and triangulate the centre of the pixels/cells directly.
Notice that retrieving the (*x,y*)-coordinates of the centres with the [xy() function of rasterio](https://rasterio.readthedocs.io/en/latest/api/rasterio.io.html?highlight=xy#rasterio.io.DatasetReader.xy) is **super slow** and it's better to use the code below.

Notice that we use the insertion strategy "BBox" because it is several orders of magnitude faster for gridded datasets.
The code also randomly selects 1% of the points.

The `no_data` values are not inserted in the triangulation.

This code saves the resulting triangulation to a [PLY file](<https://en.wikipedia.org/wiki/PLY_(file_format)>) that can be opened directly in QGIS (with the newish [MDAL mesh](https://docs.qgis.org/3.34/en/docs/user_manual/working_with_mesh/mesh_properties.html)).

```{literalinclude} ../demo/example_reading_geotiff.py
```

```{image} figs/mdal.jpg
:width: 80%     
:align: center
```

## 3D visualisation with Polyscope

You need to install [Polyscope](https://polyscope.run/py/) (basically `pip install polyscope`).

```{literalinclude} ../demo/example_polyscope.py
```

```{image} figs/polyscope_gui.jpg
```

## Plotting the DT with matplotlib

```{literalinclude} ../demo/example_matplotlib.py
```

```{image} figs/matplotlib.png
:width: 80%     
:align: center
```


## Gridding the dataset with spatial interpolation

```{literalinclude} ../demo/example_gridding.py
```

```{image} figs/grid.png
:width: 60%     
:align: center
```


## Creating a DSM

To create the DSM from an aerial lidar dataset of a city, one wants to preserve the highest z-value for each location.

With startinpy, this can be performed easily, as the example below shows.
The LAZ file is read using the Python library [laspy](https://laspy.readthedocs.io), a rather large tolerance for xy-duplicates is set (0.10m), and the highest z-value for each xy-location is kept.
Furthermore, the intensity property of the input LAZ points is preserved.

The resulting file is exported to the [PLY format](https://en.wikipedia.org/wiki/PLY_(file_format)), which can be read by several software, the open-source [QGIS](https://qgis.org/) being one of them.

```{literalinclude} ../demo/example_creating_dsm.py
```

```{image} figs/qgis.png
:width: 60%     
:align: center
```