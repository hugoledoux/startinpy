import time
from pathlib import Path

import laspy
import numpy as np
import rasterio
from py_markdown_table.markdown_table import markdown_table

root = Path(__file__).parents[1]
data = root / "data"

# -- TIF dataset
path_tif = data / "dem_01.tif"

# -- LAZ datasets
path_laz_2m = data / "04GN2_21.LAZ"
path_laz_33m = data / "69EZ1_21.LAZ"

# -- random datasets
rng = np.random.default_rng(seed=42)
pts_2d_10k = rng.random((10000, 2))
pts_2d_50k = rng.random((50000, 2))
pts_3d_10k = rng.random((10000, 3))
pts_3d_50k = rng.random((50000, 3))

t = []


def t_delaunator():
    from Delaunator import Delaunator

    t.append({"library": "delaunator"})
    # -- 10k
    t1 = time.perf_counter()
    Delaunator(pts_2d_10k)
    t2 = time.perf_counter()
    t[-1]["random_10k"] = "{:.3f}".format(t2 - t1)
    # -- 50k
    t1 = time.perf_counter()
    Delaunator(pts_2d_50k)
    t2 = time.perf_counter()
    t[-1]["random_50k"] = "{:.3f}".format(t2 - t1)
    # -- LAZ_2M
    las = laspy.read(path_laz_2m)
    pts = np.vstack((las.x, las.y)).transpose()
    t1 = time.perf_counter()
    Delaunator(pts)
    t2 = time.perf_counter()
    t[-1]["LAZ_2M"] = "{:.3f}".format(t2 - t1)
    # -- LAZ_33M
    las = laspy.read(path_laz_33m)
    pts = np.vstack((las.x, las.y)).transpose()
    t1 = time.perf_counter()
    Delaunator(pts)
    t2 = time.perf_counter()
    t[-1]["LAZ_33M"] = "{:.3f}".format(t2 - t1)
    # t[-1]["LAZ_33M"] = "X"
    # -- dem.tiff
    d = rasterio.open(path_tif)
    band1 = d.read(1)
    tr = d.transform
    pts = []
    for i in range(band1.shape[0]):
        for j in range(band1.shape[1]):
            x = tr[2] + (j * tr[0]) + (tr[0] / 2)
            y = tr[5] + (i * tr[4]) + (tr[4] / 2)
            z = band1[i][j]
            if z != d.nodatavals:
                pts.append([x, y])
    t1 = time.perf_counter()
    Delaunator(pts)
    t2 = time.perf_counter()
    t[-1]["dem.tiff"] = "{:.3f}".format(t2 - t1)


def t_startinpy():
    import startinpy

    t.append({"library": "startinpy"})
    np.random.default_rng(seed=42)
    # -- 10k
    t1 = time.perf_counter()
    dt = startinpy.DT()
    dt.snap_tolerance = 1e-8
    dt.insert(pts_3d_10k)
    t2 = time.perf_counter()
    t[-1]["random_10k"] = "{:.3f}".format(t2 - t1)
    # -- 50k
    t1 = time.perf_counter()
    dt = startinpy.DT()
    dt.snap_tolerance = 1e-8
    dt.insert(pts_3d_50k)
    t2 = time.perf_counter()
    t[-1]["random_50k"] = "{:.3f}".format(t2 - t1)
    # -- dem.tiff
    d = rasterio.open(path_tif)
    band1 = d.read(1)
    tr = d.transform
    pts = []
    for i in range(band1.shape[0]):
        for j in range(band1.shape[1]):
            x = tr[2] + (j * tr[0]) + (tr[0] / 2)
            y = tr[5] + (i * tr[4]) + (tr[4] / 2)
            z = band1[i][j]
            if z != d.nodatavals:
                pts.append([x, y, z])
    t1 = time.perf_counter()
    dt = startinpy.DT()
    dt.insert(pts, insertionstrategy="BBox")
    t2 = time.perf_counter()
    t[-1]["dem.tiff"] = "{:.3f}".format(t2 - t1)
    # -- LAZ_2M
    las = laspy.read(path_laz_2m)
    pts = np.vstack((las.x, las.y, las.z)).transpose()
    t1 = time.perf_counter()
    dt = startinpy.DT()
    dt.insert(pts)
    t2 = time.perf_counter()
    t[-1]["LAZ_2M"] = "{:.3f}".format(t2 - t1)
    # -- LAZ_33M
    las = laspy.read(path_laz_33m)
    pts = np.vstack((las.x, las.y, las.z)).transpose()
    t1 = time.perf_counter()
    dt = startinpy.DT()
    dt.insert(pts)
    t2 = time.perf_counter()
    t[-1]["LAZ_33M"] = "{:.3f}".format(t2 - t1)


def t_scipy():
    from scipy.spatial import Delaunay

    t.append({"library": "scipy"})
    np.random.default_rng(seed=42)
    # -- 10k
    t1 = time.perf_counter()
    Delaunay(pts_2d_10k)
    t2 = time.perf_counter()
    t[-1]["random_10k"] = "{:.3f}".format(t2 - t1)
    # -- 50k
    t1 = time.perf_counter()
    Delaunay(pts_2d_50k)
    t2 = time.perf_counter()
    t[-1]["random_50k"] = "{:.3f}".format(t2 - t1)
    # -- LAZ_2M
    las = laspy.read(path_laz_2m)
    pts = np.vstack((las.x, las.y)).transpose()
    t1 = time.perf_counter()
    Delaunay(pts)
    t2 = time.perf_counter()
    t[-1]["LAZ_2M"] = "{:.3f}".format(t2 - t1)
    # -- LAZ_33M
    las = laspy.read(path_laz_33m)
    pts = np.vstack((las.x, las.y)).transpose()
    t1 = time.perf_counter()
    Delaunay(pts)
    t2 = time.perf_counter()
    t[-1]["LAZ_33M"] = "{:.3f}".format(t2 - t1)
    # -- dem.tiff
    d = rasterio.open(path_tif)
    band1 = d.read(1)
    tr = d.transform
    pts = []
    for i in range(band1.shape[0]):
        for j in range(band1.shape[1]):
            x = tr[2] + (j * tr[0]) + (tr[0] / 2)
            y = tr[5] + (i * tr[4]) + (tr[4] / 2)
            z = band1[i][j]
            if z != d.nodatavals:
                pts.append([x, y])
    t1 = time.perf_counter()
    Delaunay(pts)
    t2 = time.perf_counter()
    t[-1]["dem.tiff"] = "{:.3f}".format(t2 - t1)


def t_scipy_inc():
    from scipy.spatial import Delaunay

    t.append({"library": "scipy-inc"})
    np.random.default_rng(seed=42)
    # -- 10k
    t1 = time.perf_counter()
    Delaunay(pts_2d_10k, incremental=True)
    t2 = time.perf_counter()
    t[-1]["random_10k"] = "{:.3f}".format(t2 - t1)
    # -- 50k
    t1 = time.perf_counter()
    Delaunay(pts_2d_50k, incremental=True)
    t2 = time.perf_counter()
    t[-1]["random_50k"] = "{:.3f}".format(t2 - t1)
    # -- LAZ_2M
    las = laspy.read(path_laz_2m)
    pts = np.vstack((las.x, las.y, las.z)).transpose()
    t1 = time.perf_counter()
    Delaunay(pts, incremental=True)
    t2 = time.perf_counter()
    t[-1]["LAZ_2M"] = "{:.3f}".format(t2 - t1)
    # t[-1]["LAZ_2M"] = "X"
    # -- LAZ_33M
    las = laspy.read(path_laz_33m)
    pts = np.vstack((las.x, las.y, las.z)).transpose()
    t1 = time.perf_counter()
    Delaunay(pts, incremental=True)
    t2 = time.perf_counter()
    t[-1]["LAZ_33M"] = "{:.3f}".format(t2 - t1)
    # t[-1]["LAZ_33M"] = "X"
    # -- dem.tiff
    d = rasterio.open(path_tif)
    band1 = d.read(1)
    tr = d.transform
    pts = []
    for i in range(band1.shape[0]):
        for j in range(band1.shape[1]):
            x = tr[2] + (j * tr[0]) + (tr[0] / 2)
            y = tr[5] + (i * tr[4]) + (tr[4] / 2)
            z = band1[i][j]
            if z != d.nodatavals:
                pts.append([x, y, z])
    t1 = time.perf_counter()
    Delaunay(pts, incremental=True)
    t2 = time.perf_counter()
    t[-1]["dem.tiff"] = "{:.3f}".format(t2 - t1)
    # t[-1]["dem.tiff"] = "X"


def t_triangle():
    import triangle

    t.append({"library": "triangle"})
    np.random.default_rng(seed=42)
    # -- 10k
    t1 = time.perf_counter()
    A = dict(vertices=pts_2d_10k)
    triangle.triangulate(A)
    t2 = time.perf_counter()
    t[-1]["random_10k"] = "{:.3f}".format(t2 - t1)
    # -- 50k
    t1 = time.perf_counter()
    A = dict(vertices=pts_2d_50k)
    triangle.triangulate(A)
    t2 = time.perf_counter()
    t[-1]["random_50k"] = "{:.3f}".format(t2 - t1)
    # -- LAZ_2M
    las = laspy.read(path_laz_2m)
    pts = np.vstack((las.x, las.y)).transpose()
    t1 = time.perf_counter()
    A = dict(vertices=pts)
    triangle.triangulate(A)
    t2 = time.perf_counter()
    t[-1]["LAZ_2M"] = "{:.3f}".format(t2 - t1)
    # -- LAZ_33M
    las = laspy.read(path_laz_33m)
    pts = np.vstack((las.x, las.y)).transpose()
    t1 = time.perf_counter()
    A = dict(vertices=pts)
    triangle.triangulate(A)
    t2 = time.perf_counter()
    t[-1]["LAZ_33M"] = "{:.3f}".format(t2 - t1)
    # -- dem.tiff
    d = rasterio.open(path_tif)
    band1 = d.read(1)
    tr = d.transform
    pts = []
    for i in range(band1.shape[0]):
        for j in range(band1.shape[1]):
            x = tr[2] + (j * tr[0]) + (tr[0] / 2)
            y = tr[5] + (i * tr[4]) + (tr[4] / 2)
            z = band1[i][j]
            if z != d.nodatavals:
                pts.append([x, y])
    t1 = time.perf_counter()
    A = dict(vertices=pts)
    triangle.triangulate(A)
    t2 = time.perf_counter()
    t[-1]["dem.tiff"] = "{:.3f}".format(t2 - t1)


def t_pdal():
    from pdal import Filter, Reader

    def make_pdal_points(points):
        return np.array(
            [(x, y, z) for x, y, z in zip(points[:, 0], points[:, 1], points[:, 2])],
            dtype=[("X", float), ("Y", float), ("Z", float)],
        )

    t.append({"library": "pdal"})
    np.random.default_rng(seed=42)

    points = make_pdal_points(pts_3d_10k)
    t1 = time.perf_counter()
    Filter.delaunay().pipeline(points).execute()
    t2 = time.perf_counter()
    t[-1]["random_10k"] = "{:.3f}".format(t2 - t1)

    points = make_pdal_points(pts_3d_50k)
    t1 = time.perf_counter()
    Filter.delaunay().pipeline(points).execute()
    t2 = time.perf_counter()
    t[-1]["random_50k"] = "{:.3f}".format(t2 - t1)

    d = rasterio.open(path_tif)
    band1 = d.read(1)
    tr = d.transform
    pts = []
    for i in range(band1.shape[0]):
        for j in range(band1.shape[1]):
            x = tr[2] + (j * tr[0]) + (tr[0] / 2)
            y = tr[5] + (i * tr[4]) + (tr[4] / 2)
            z = band1[i][j]
            if z != d.nodatavals:
                pts.append([x, y, z])
    points = make_pdal_points(np.array(pts))
    t1 = time.perf_counter()
    Filter.delaunay().pipeline(points).execute()
    t2 = time.perf_counter()
    t[-1]["dem.tiff"] = "{:.3f}".format(t2 - t1)

    pipeline = Reader.las(path_laz_2m).pipeline()
    pipeline.execute()
    t1 = time.perf_counter()
    Filter.delaunay().pipeline(pipeline.arrays[0]).execute()
    t2 = time.perf_counter()
    t[-1]["LAZ_2M"] = "{:.3f}".format(t2 - t1)

    pipeline = Reader.las(path_laz_33m).pipeline()
    pipeline.execute()
    t1 = time.perf_counter()
    Filter.delaunay().pipeline(pipeline.arrays[0]).execute()
    t2 = time.perf_counter()
    t[-1]["LAZ_33M"] = "{:.3f}".format(t2 - t1)


if __name__ == "__main__":
    t_delaunator()
    t_startinpy()
    t_scipy()
    t_scipy_inc()
    t_triangle()
    t_pdal()
    markdown = markdown_table(t).get_markdown()
    print(markdown)
