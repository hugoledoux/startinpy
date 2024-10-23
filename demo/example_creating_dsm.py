import laspy
import numpy as np
import startinpy

las = laspy.read("../data/small.laz")
pts = np.vstack((las.x, las.y, las.z, las.intensity)).transpose()
dt = startinpy.DT(np.dtype([("intensity", float)]))
dt.snap_tolerance = 0.10
dt.duplicates_handling = "Highest"
for pt in pts:
    dt.insert_one_pt([pt[0], pt[1], pt[2]], intensity=pt[3])
dt.write_ply("mydt.ply")
