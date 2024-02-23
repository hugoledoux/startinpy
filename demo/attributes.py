import startinpy
import numpy as np
import json
import laspy

las = laspy.read("../data/small.laz")

#-- read intensity and store it as extra_attribute in the startinpy DT
d = np.vstack((las.x, las.y, las.z, las.intensity)).transpose()
d = d[::1] #-- thinning to speed up, put ::1 to keep all the points

dt = startinpy.DT(extra_attributes=True)
for each in d:
    dt.insert_one_pt(each[0], each[1], each[2], intensity=each[3])

a = {'intensity': 155.5, 'reflectance': 111, 'something': True}
dt.set_attribute(50, json.dumps(a))

print(dt)

print("all extra attributes:", dt.list_attributes())

a = dt.get_attribute(50)
print("=>", json.loads(a))
a = dt.get_attribute(49)
print("=>", json.loads(a))

i = dt.attributes('intensity')
print(i.shape)
