import startinpy
import numpy as np

#-- generate 100 points randomly in the plane
rng = np.random.default_rng()
pts = rng.random((100, 3))
pts = pts * 100

dt = startinpy.DT()
dt.insert(pts)

#-- remove vertex #4
try:
    dt.remove(4)
except Exception as e:
    print(e)

print("# vertices:", dt.number_of_vertices())
print("# triangles:", dt.number_of_triangles())

print("CH: ", dt.convex_hull())

print(dt.is_triangle([4, 12, 6]) )
print(dt.is_triangle([5, 12, 6]) )

print("--- /Points ---")
for each in dt.points:
    print(each)
print("--- Points/ ---")

alltr = dt.triangles
print(alltr[3])

zhat = dt.interpolate({"method": "TIN"}, [[55.2, 33.1]])
print("result: ", zhat[0])
