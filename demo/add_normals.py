
#-- add the normals, as extra attributes (nx, ny, nz), for all the vertices in the DT

import startinpy
import numpy as np
import laspy
import json

las = laspy.read("../data/small.laz")
dt = startinpy.DT()
dt.set_attributes_schema(np.dtype([("nx", np.float32), ("ny", float), ("nz", float)]))
dt.duplicates_handling = "Highest"

dt.insert(las.xyz)

pts = dt.points
for vi in range(1, len(pts)):
    n = dt.normal_vertex(vi)
    dt.add_vertex_attributes(vi, nx=n[0], ny=n[1], nz=n[2])

print(dt)
print(dt.attributes)
print(dt.get_attributes_schema())
print(dt.attributes.dtype)


# a = dt.attributes
# print(dt.points.shape)
# print(a.shape)

# together = np.column_stack((dt.points, a['nx'], a['ny'], a['nz']))
# # together = np.stack((dt.points, a))
# # together = np.concatenate((dt.points, a), axis=1)
# print(together)





