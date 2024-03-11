import startinpy
import rasterio
import random

d = rasterio.open('../data/dem_01.tif')
band1 = d.read(1)
t = d.transform
pts = []
for i in range(band1.shape[0]):
    for j in range(band1.shape[1]):
         x = t[2] + (j * t[0]) + (t[0] / 2)
         y = t[5] + (i * t[4]) + (t[4] / 2)
         z = band1[i][j]
         if (z != d.nodatavals) and (random.randint(0, 100) == 5):
             pts.append([x, y, z])
dt = startinpy.DT()
dt.insert(pts, insertionstrategy="BBox")
#-- exaggerate the elevation by a factor 2.0
dt.vertical_exaggeration(2.0)
dt.write_ply("mydt.ply")