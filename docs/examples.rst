
Examples
============


Reading a LAZ file
------------------

.. code-block:: python

   import startinpy

   dt = startinpy.DT()
   dt.read_las("/home/elvis/myfile.laz", classification=[2,6])
   print("# vertices:", dt.number_of_vertices())
   

Exporting the DT to GeoJSON
---------------------------

.. code-block:: python

   import startinpy
   import numpy as np
   
   #-- generate 100 points randomly in the plane
   rng = np.random.default_rng(seed=42)
   pts = rng.random((100, 3))
   dt = startinpy.DT()
   dt.insert(pts, insertionstrategy="AsIs")
   dt.write_geojson("/home/elvis/myfile.geojson")


Exporting the DT to QGIS (MDAL Mesh)
------------------------------------

.. code-block:: python

   import startinpy
      
   dt = startinpy.DT()
   dt.read_geotiff("/home/elvis/mydem.tif")
   #-- exaggerate the elevation by a factor 2.0
   dt.vertical_exaggeration(2.0)
   dt.write_ply("/home/elvis/mydem.ply")


Plotting the DT with matplotlib
-------------------------------

.. code-block:: python

   import startinpy
   import numpy as np
   
   #-- generate 100 points randomly in the plane
   rng = np.random.default_rng(seed=42)
   pts = rng.random((100, 3))
   pts = pts * 100 #-- scale to [0, 100]
   t = startinpy.DT()
   t.insert(pts)
   pts = t.points
   trs = t.triangles
   #-- plot
   import matplotlib.pyplot as plt
   plt.triplot(pts[:,0], pts[:,1], trs)
   #-- the vertex "0" shouldn't be plotted, so start at 1
   plt.plot(pts[1:,0], pts[1:,1], 'o')
   plt.show()
