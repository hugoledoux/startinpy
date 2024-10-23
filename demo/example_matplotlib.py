import numpy as np
import startinpy

# -- generate 100 points randomly in the plane
rng = np.random.default_rng(seed=42)
pts = rng.random((100, 3))
# -- scale to [0, 100]
pts = pts * 100
t = startinpy.DT()
t.insert(pts)
pts = t.points
trs = t.triangles
# -- plot
import matplotlib.pyplot as plt

plt.triplot(pts[:, 0], pts[:, 1], trs)
# -- the vertex "0" shouldn't be plotted, so start at 1
plt.plot(pts[1:, 0], pts[1:, 1], "o")
plt.show()
