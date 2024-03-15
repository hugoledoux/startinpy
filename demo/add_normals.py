
#-- add the normals, as extra attributes (nx, ny, nz), for all the vertices in the DT

import startinpy
import numpy as np
import laspy
import json


def normal_triangle(tr, pts):
    v0 = pts[tr[1]] - pts[tr[0]]
    v1 = pts[tr[2]] - pts[tr[0]]
    n = np.cross(v0, v1)
    l = np.linalg.norm(n)
    return n / l

las = laspy.read("../data/small.laz")
pts = np.vstack((las.x, las.y, las.z)).transpose()
dt = startinpy.DT(extra_attributes=True)
dt.duplicates_handling = "Highest"
dt.insert(pts)

pts = dt.points
for vi in range(1, len(pts)):
    trs = dt.incident_triangles_to_vertex(vi)
    nns = np.zeros((trs.shape[0], 3))
    for (i, tr) in enumerate(trs):
        if dt.is_finite(tr):
            nn = normal_triangle(tr, pts)
            nns[i] = nn
    n = np.average(nns, axis=0)
    new_a = {'nx': n[0], 'ny': n[1], 'nz': n[2]} 
    dt.set_vertex_attributes(vi, json.dumps(new_a))


print(dt.attribute('nx'))
print(dt.attribute('ny'))
print(dt.attribute('nz'))





