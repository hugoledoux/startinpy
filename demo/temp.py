import numpy as np

import startinpy

dt = startinpy.DT()
# d = np.dtype('i8, bool, f4, U3')
d = np.dtype([("classification", np.float32), ("visited", bool), ("intensity", "<U8")])
print(d)
dt.set_attributes_schema(d)

dt.insert_one_pt(
    [20.0, 30.0, 1.1], classification=11.1, visited=True, intensity="12345678901"
)
(vi, bNewVertex, bZUpdated) = dt.insert_one_pt([120.0, 33.0, 12.5])
dt.set_vertex_attributes(vi, classification=22.2, visited=True, intensity="a")
# dt.insert_one_pt([120.0, 33.0, 12.5], classification=22.2, visited=True, intensity='a');
dt.insert_one_pt([124.0, 222.0, 7.65], classification=33.3, visited=True, intensity="b")
dt.insert_one_pt([20.0, 133.0, 21.0], classification=44.4, visited=True, intensity="c")
dt.insert_one_pt([23.0, 13.0, 11.0], classification=55.5, visited=False, intensity="d")
dt.insert_one_pt([60.0, 60.0, 33.0], classification=66.6, visited=False, intensity="e")


print(dt)
print(dt.attributes)
print("=>", dt.get_attributes_schema())
print(dt.attributes.dtype)
