import startinpy
import numpy as np
import polyscope as ps
import laspy

las = laspy.read("../data/small.laz")
pts = np.vstack((las.x, las.y, las.z)).transpose()
pts = pts[::10] #-- thinning to speed up, put ::10 to keep 1/10 of the points
dt = startinpy.DT()
dt.insert(pts)

pts = dt.points
pts[0] = pts[1] #-- first vertex has inf and could mess things
trs = dt.triangles

ps.init()
ps.set_program_name("mydt")
ps.set_up_dir("z_up")
ps.set_ground_plane_mode("shadow_only")
ps.set_ground_plane_height_factor(0.01, is_relative=True)
ps.set_autocenter_structures(True)
ps.set_autoscale_structures(True)
pc = ps.register_point_cloud("mypoints", pts[1:], radius=0.0015, point_render_mode='sphere')
ps_mesh = ps.register_surface_mesh("mysurface", pts, trs)
ps_mesh.reset_transform()
pc.reset_transform()
ps.show()