import startinpy
import meshio
import laspy
import numpy as np

las = laspy.read("../data/small.laz")
pts = np.vstack((las.x, las.y, las.z)).transpose()
dt = startinpy.DT()
dt.insert(pts)
vs = dt.points
vs[0] = vs[1] #-- to ensure that infinite vertex is not blocking the viz
cells = [("triangle", dt.triangles)]
meshio.write_points_cells("mydt.vtu", vs, cells)