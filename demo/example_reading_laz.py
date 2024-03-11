import startinpy
import numpy as np
import laspy

las = laspy.read("../data/small.laz")
pts = np.vstack((las.x, las.y, las.z)).transpose()
pts = pts[::1] #-- thinning to speed up, put ::10 to keep 1/10 of the points
dt = startinpy.DT()
dt.insert(pts)
print("number vertices:", dt.number_of_vertices())
# -- number vertices: 39231