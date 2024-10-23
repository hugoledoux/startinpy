import laspy
import numpy as np
import startinpy

las = laspy.read("../data/small.laz")
pts = np.vstack((las.x, las.y, las.z)).transpose()

dt = startinpy.DT()
dt.insert(pts)

# -- remove vertex #4
try:
    dt.remove(4)
except Exception as e:
    print(e)

print("# vertices:", dt.number_of_vertices())
print("# triangles:", dt.number_of_triangles())

# -- print the vertices forming the convex hull, in CCW-order
print("CH: ", dt.convex_hull())

# -- fetch all the incident triangles (CCW-ordered) to the vertex #235
vi = 235
one_random_pt = dt.points[vi]
print("one random point:", one_random_pt)
print(dt.incident_triangles_to_vertex(vi))

# -- interpolate at a location with the linear in TIN method
zhat = dt.interpolate({"method": "TIN"}, [[85718.5, 447211.6]])
print("result: ", zhat[0])
