import laspy
import numpy as np

import startinpy

las = laspy.read("../data/small.laz")
pts = np.vstack((las.x, las.y, las.z)).transpose()
pts = pts[::10]  # -- thinning to speed up, put ::10 to keep 1/10 of the points
dt = startinpy.DT()
dt.insert(pts)


# Export to FlatGeoBuf format
# The TIN triangles are exported as Polygon features
# with z-values stored as properties (z0, z1, z2)
dt.write_flatgeobuf("mytin.fgb")
dt.write_flatgeobuf("mytin.ply")

print(f"Exported {dt.number_of_triangles()} triangles to myfile.fgb")
