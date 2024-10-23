import numpy as np
import startinpy

dt = startinpy.DT()
dt.set_attributes_schema(np.dtype([("classification", int)]))

dt.insert_one_pt([0.0, 0.0, 1.0], classification=1)
dt.insert_one_pt([0.0, 0.0, 1.0], classification=1)
dt.insert_one_pt([10.0, 0.0, 2.0], classification=2)
dt.insert_one_pt([10.0, 10.0, 3.0], classification=3)
dt.insert_one_pt([0.0, 10.0, 4.0], classification=4)
dt.insert_one_pt([5.0, 5.0, 10.0], classification=5)

dt.insert_one_pt([5.0, 5.0, 11.0], classification=11)
assert dt.points[5][2] == 10.0
assert dt.attributes[5][0] == 5

dt.duplicates_handling = "Highest"
dt.insert_one_pt([5.0, 5.0, 11.0], classification=11)
assert dt.points[5][2] == 11.0
assert dt.attributes[5][0] == 11
