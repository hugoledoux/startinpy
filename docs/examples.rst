
Examples
============


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
   vs = t.all_vertices()
   trs = t.all_triangles()
   #-- plot
   import matplotlib.pyplot as plt
   plt.triplot(vs2[:,0], vs2[:,1], trs2)
   #-- the vertex "0" shouldn't be plotted, so start at 1
   plt.plot(vs2[1:,0], vs2[1:,1], 'o')
   plt.show()
