import laspy
import numpy as np
import rerun as rr  # -- pip install rerun-sdk (https://rerun.io)

import startinpy

las = laspy.read("../data/small.laz")

dt = startinpy.DT(np.dtype([("nx", np.float64), ("ny", float), ("nz", float)]))
dt.snap_tolerance = 0.10
dt.duplicates_handling = "Lowest"

# -- create the DTM (points classified as ground==2)
ground = las.classification == 2
pts = las.xyz[ground][::10]
dt.insert(pts)

vs = dt.points
vs[0] = vs[1]  # -- first vertex has inf and could mess things
trs = dt.triangles

# -- add normals as extra attribute
for vi in range(len(vs)):
    n = dt.normal_vertex(vi)
    dt.set_vertex_attributes(vi, nx=n[0], ny=n[1], nz=n[2])
a = dt.attributes
ns = np.column_stack((a["nx"], a["ny"], a["nz"]))

# -- rerun
rr.init("mydataset", spawn=True)
rr.log(
    "tin",
    rr.Mesh3D(
        vertex_positions=vs,
        vertex_colors=[212, 197, 112],
        vertex_normals=ns,
        triangle_indices=trs,
    ),
)
rr.log("ground_pts", rr.Points3D(las.xyz[ground], colors=[250, 150, 50], radii=0.5))
non_ground = las.classification != 2
rr.log(
    "non_ground_pts", rr.Points3D(las.xyz[non_ground], colors=[50, 150, 250], radii=0.5)
)
