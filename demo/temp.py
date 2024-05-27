import startinpy
import numpy as np
import laspy
import math
import sys
import json


dt = startinpy.DT()
dt.add_attribute_map([("intensity", "f64"), ("visited", "i64")])

dt.insert_one_pt([20.0, 30.0, 1.1], intensity=111.2, visited=4);
dt.insert_one_pt([120.0, 33.0, 12.5], intensity=111.2, visited=5);
dt.insert_one_pt([124.0, 222.0, 7.65], intensity=111.2, visited=6);
dt.insert_one_pt([20.0, 133.0, 21.0], intensity=111.2, visited=7);
dt.insert_one_pt([23.0, 13.0, 11.0], intensity=111.2, visited=8);
dt.insert_one_pt([60.0, 60.0, 33.0], intensity=111.2, visited=9);


print(dt)

print(dt.all_attributes()[1:])
