import numpy as np
import laspy
import startinpy
from tqdm import tqdm

las = laspy.read("../data/small.laz")
las.points = las.points[::1] #-- thinning
d = np.vstack((las.x, las.y, las.z)).transpose()

dt = startinpy.DT(extra_attributes=True)
#-- keep the highest z-value when xy-duplicates arise
dt.duplicates_handling = "Highest"
dt.snap_tolerance = 0.05

#-- insert and assign an id to each point
i = 0
for p in tqdm(d):
    dt.insert_one_pt(p, pid=i)
    i += 1

#-- new LAZ
new_las = laspy.LasData(las.header)

#-- make a mask for the points that were kept
a = dt.attribute('pid')
mask = np.full((len(las.points)), False)
for each in a[1:]:
    mask[int(each)] = True
new_las.points = las.points[mask]

#-- save subset to a new laz file
new_las.write("out.laz")

print("*"*22)
print("input: {:>15}".format(len(las.points)))
print("output: {:>14}".format(len(new_las.points)))
print("xy-duplicates: {:>7}".format(len(las.points) - len(new_las.points)))