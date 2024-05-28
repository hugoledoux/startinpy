import startinpy
import numpy as np
import laspy
import math
import sys
import json


dt = startinpy.DT()
dt.add_attribute_map([("intensity", "f64")])
# dt.add_attribute_map([("intensity", "f64"), ("visited", "bool")])

dt.insert_one_pt([20.0, 30.0, 1.1], intensity=11.1, visited=True);
dt.insert_one_pt([120.0, 33.0, 12.5], intensity=22.2, visited=True);
dt.insert_one_pt([124.0, 222.0, 7.65], intensity=33.3, visited=True);
dt.insert_one_pt([20.0, 133.0, 21.0], intensity=44.4, visited=True);
dt.insert_one_pt([23.0, 13.0, 11.0], intensity=55.5, visited=False);
dt.insert_one_pt([60.0, 60.0, 33.0], intensity=66.6, visited=False);


print(dt)
print(dt.get_attribute_map())
print(dt.attributes)
