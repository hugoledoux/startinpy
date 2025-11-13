import math

import laspy
import numpy as np
import rasterio
from tqdm import tqdm

import startinpy


def main():
    las = laspy.read("../data/small.laz")
    dt = startinpy.DT()
    dt.duplicates_handling = "Highest"
    d = las.xyz
    print("Constructing the TIN with {} points".format(len(d)))
    for each in tqdm(d):
        dt.insert_one_pt(each)

    # -- grid with 50cm resolution the bbox
    bbox = dt.get_bbox()
    cellsize = 0.5
    deltax = math.ceil((bbox[2] - bbox[0]) / cellsize)
    deltay = math.ceil((bbox[3] - bbox[1]) / cellsize)
    centres = []
    i = 0
    for row in range((deltay - 1), -1, -1):
        j = 0
        y = bbox[1] + (row * cellsize) + (cellsize / 2)
        for col in range(deltax):
            x = bbox[0] + (col * cellsize) + (cellsize / 2)
            centres.append([x, y])
            j += 1
        i += 1
    centres = np.asarray(centres)
    print("Interpolating at {} locations".format(centres.shape[0]))
    zhat = dt.interpolate({"method": "TIN"}, centres)

    # -- save to a GeoTIFF with rasterio
    write_rasterio(
        "grid.tiff", zhat.reshape((deltay, deltax)), (bbox[0], bbox[3]), cellsize
    )


def write_rasterio(output_file, a, bbox, cellsize):
    with rasterio.open(
        output_file,
        "w",
        driver="GTiff",
        height=a.shape[0],
        width=a.shape[1],
        count=1,
        dtype=np.float32,
        crs=rasterio.crs.CRS.from_string("EPSG:28992"),
        nodata=np.nan,
        transform=(cellsize, 0.0, bbox[0], 0.0, -cellsize, bbox[1]),
    ) as dst:
        dst.write(a, 1)
    print("File written to '%s'" % output_file)


if __name__ == "__main__":
    main()
