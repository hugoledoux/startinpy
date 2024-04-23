
#-- add the normals, as extra attributes (nx, ny, nz), for all the vertices in the DT

import startinpy
import numpy as np
import laspy
import json

las = laspy.read("../data/small.laz")
dt = startinpy.DT(extra_attributes=True)
dt.duplicates_handling = "Highest"
dt.insert(las.xyz)


pts = dt.points
for vi in range(1, len(pts)):
    n = dt.normal_vertex(vi)
    dt.add_vertex_attribute(vi, nx=n[0], ny=n[1], nz=n[2])


print(dt.attribute('nx'))
print(dt.attribute('ny'))
print(dt.attribute('nz'))





