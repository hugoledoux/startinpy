import startinpy
import numpy as np
import json
import laspy

las = laspy.read("../data/small.laz")

#-- read intensity and store it as extra_attribute in the startinpy DT
d = np.vstack((las.x, las.y, las.z, las.classification)).transpose()
d = d[::10] #-- thinning to speed up, put ::1 to keep all the points

print(d)

dt = startinpy.DT()
dt.set_attributes_schema(np.dtype([("classification", np.uint64)]))


for each in d:
    dt.insert_one_pt(each[:3], classification=int(each[3]))

print("done")
# a = {'intensity': 155.5, 'reflectance': 111, 'something': True}
dt.set_vertex_attributes(50, classification=int(112.2))

print(dt)
print(dt.get_attributes_schema())
print(dt.attributes[1:])

a = dt.get_vertex_attributes(50)
print("=>", a)
a = dt.get_vertex_attributes(49)
print("=>", a)

