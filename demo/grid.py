import startinpy
import numpy as np
import json
import laspy
import rasterio
import math
from tqdm import tqdm

def main():
    las = laspy.read("/Users/hugo/data/ahn4/bk.laz")

    #-- read intensity and store it as extra_attribute in the startinpy DT
    d = np.vstack((las.x, las.y, las.z)).transpose()
    d = d[::10] #-- thinning to speed up, put ::1 to keep all the points

    dt = startinpy.DT()
    dt.duplicates_handling = "Highest"
    print("Constructing the TIN with {} points".format(len(d)))
    for each in tqdm(d):
        dt.insert_one_pt(each[0], each[1], each[2])

    #-- grid with 1m resolution the bbox
    bbox = dt.get_bbox()
    cellsize = 1.0
    deltax = math.ceil((bbox[2] - bbox[0]) / cellsize)
    deltay = math.ceil((bbox[3] - bbox[1]) / cellsize)
    # zhat = numpy.zeros(shape=(deltay,deltax))

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
    
    #-- save to a GeoTIFF with rasterio
    write_rasterio('grid2.tiff', zhat.reshape((deltay, deltax)), (bbox[0], bbox[1]), cellsize)
    #-- save to a ASC grid file (text file)
    # write_asc_file('grid.asc', zhat.reshape((deltay, deltax)), (bbox[0], bbox[1]), cellsize)


def write_asc_file(output_file, a, lowerleft, cellsize):
    fout = open(output_file, 'w')
    fout.write('NCOLS %d\n' % a.shape[1])
    fout.write('NROWS %d\n' % a.shape[0])
    fout.write('XLLCORNER %f\n' % lowerleft[0])
    fout.write('YLLCORNER %f\n' % lowerleft[1])
    fout.write('CELLSIZE %f\n' % cellsize)
    fout.write('NODATA_VALUE -9999\n')
    ndv = '-9999'
    for row in a:
        for each in row:
            if np.isnan(each):
                fout.write(ndv + ' ')
            else:
                fout.write(str(each) + ' ')
        fout.write('\n')
    fout.close()


def write_rasterio(output_file, a, bbox, cellsize):
    with rasterio.open(output_file, 'w', 
                       driver='GTiff', 
                       height=a.shape[0],
                       width=a.shape[1], 
                       count=1, 
                       dtype=np.float32,
                       crs=rasterio.crs.CRS.from_string("EPSG:28992"), 
                       nodata=np.nan,
                       transform=(cellsize, 0., bbox[0], 0., -cellsize, bbox[1])) as dst:
        dst.write(a, 1)
    print("File written to '%s'" % output_file)


if __name__ == '__main__':
    main()
