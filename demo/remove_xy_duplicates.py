import numpy as np
import laspy
import startinpy
from tqdm import tqdm

las = laspy.read("/Users/hugo/data/ahn4/bk.laz")
las.points = las.points[::1] #-- thinning
d = np.vstack((las.x, las.y, las.z)).transpose()

dt = startinpy.DT()
dt.snap_tolerance = 0.02
#-- keep the highest z-value when xy-duplicates arise
dt.duplicates_handling = "Highest"

#-- insert and and keep track of duplicates
mask = []
for p in tqdm(d):
    i, b = dt.insert_one_pt(p[0], p[1], p[2])
    mask.append(b)

#-- new LAZ
new_las = laspy.LasData(las.header)
#-- make a mask for the points that were kept
new_las.points = las.points[np.asarray(mask)]
#-- save subset to a new laz file
new_las.write("out.laz")

print("*"*22)
print("input: {:>15}".format(len(las.points)))
print("output: {:>14}".format(len(new_las.points)))
print("xy-duplicates: {:>7}".format(len(las.points) - len(new_las.points)))



#-- more complex alternative

# import numpy as np
# import laspy
# import startinpy
# from tqdm import tqdm

# las = laspy.read("/Users/hugo/data/ahn4/bk.laz")
# las.points = las.points[::1] #-- thinning
# d = np.vstack((las.x, las.y, las.z)).transpose()

# dt = startinpy.DT(extra_attributes=True)
# #-- keep the highest z-value when xy-duplicates arise
# dt.duplicates_handling = "Highest"

# #-- insert and assign an id to each point
# i = 0
# for p in tqdm(d):
#     dt.insert_one_pt(p[0], p[1], p[2], theid=i)
#     i += 1

# #-- new LAZ
# new_las = laspy.LasData(las.header)

# #-- make a mask for the points that were kept
# a = dt.attributes('theid')
# mask = np.full((len(las.points)), False)
# for each in a[1:]:
#     mask[int(each)] = True
# new_las.points = las.points[mask]

# #-- save subset to a new laz file
# new_las.write("out.laz")