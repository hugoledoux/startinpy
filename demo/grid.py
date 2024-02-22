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
    d = d[::1] #-- thinning to speed up, put ::1 to keep all the points

    dt = startinpy.DT()
    dt.duplicates_handling = "Highest"
    # dt.insert(d)
    for each in tqdm(d):
        dt.insert_one_pt(each[0], each[1], each[2])

    #-- grid with 1m resolution the bbox
    bbox = dt.get_bbox()
    cellsize = 0.5
    deltax = math.ceil((bbox[2] - bbox[0]) / cellsize)
    deltay = math.ceil((bbox[3] - bbox[1]) / cellsize)
    # zhat = numpy.zeros(shape=(deltay,deltax))

    centres = []
    i = 0
    for row in tqdm(range((deltay - 1), -1, -1)):
        j = 0
        y = bbox[1] + (row * cellsize) + (cellsize / 2)
        for col in range(deltax):
            x = bbox[0] + (col * cellsize) + (cellsize / 2)
            centres.append([x, y])
            j += 1
        i += 1
    centres = np.asarray(centres)
    zhat = dt.interpolate({"method": "TIN"}, centres)
    write_asc_file('grid.tiff', zhat.reshape((deltay, deltax)), (bbox[0], bbox[1]), cellsize)

def write_asc_file(namefile, a, lowerleft, cellsize):
    fout = open(namefile, 'w')
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


if __name__ == '__main__':
    main()
