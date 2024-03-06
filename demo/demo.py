import startinpy
import numpy as np
import json
import laspy

las = laspy.read("/Users/hugo/data/ahn4/crop.laz")
d = np.vstack((las.x, las.y, las.z, las.classification)).transpose()
d100 = d[::10000] #-- thinning

dt = startinpy.DT(extra_attributes=True)
for each in d100:
    dt.insert_one_pt(each[0], each[1], each[2], classification=each[3])

# a = {'intensity': 155.5, 'reflectance': 111, 'hugo': True}
# dt.set_attribute(50, json.dumps(a))

# print("# vertices:", dt.number_of_vertices())
# print("# triangles:", dt.number_of_triangles())

print(dt)

# a = dt.get_attribute(41)
# print("=>", json.loads(a))

i = dt.attributes('classification').astype(int)
# print(np.nanmean(i))
print(i)

# print("all extra attributes:", dt.list_all_attributes())