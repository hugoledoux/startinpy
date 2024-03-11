import startinpy
import numpy as np

#-- generate 100 points randomly in the plane
rng = np.random.default_rng(seed=42)
pts = rng.random((100, 3))
dt = startinpy.DT()
dt.insert(pts, insertionstrategy="AsIs")
dt.write_geojson("myfile.geojson")