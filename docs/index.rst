.. startinpy documentation master file, created by
   sphinx-quickstart on Thu Dec 16 17:10:24 2021.
   You can adapt this file completely to your liking, but it should at least
   contain the root `toctree` directive.

startinpy 
=========

A Delaunay triangulator where the input are 2.5D points, the DT is computed in 2D but the elevation of the vertices are kept.
This is used mostly for the modelling of terrains.

Originally written in `Rust <https://www.rust-lang.org/>`_ (so it's rather fast), and `robust arithmetic <https://crates.io/crates/robust>`_ is used (so it shouldn't crash).

Delaunay triangulations can be incrementally constructed, vertices can be deleted efficiently, and a few spatial interpolation methods are implemented.

.. code-block:: python

   import startinpy

   t = startinpy.DT()
   t.insert_one_pt(1.1, 3.3, 4.1)
   t.insert_one_pt(15.1, 13.8, 2.9)
   ...
   t.insert_one_pt(4.6, 9.3, 1.2)
   t.remove(4) 
   print("# vertices:", t.number_of_vertices())
   print("# triangles:", t.number_of_triangles())
   t.write_geojson("/home/elvis/temp/mydt.geojson")


Table of content
================

.. toctree::
   :maxdepth: 1


   installation
   api
   examples
   howitworks


Indices and tables
==================

* :ref:`genindex`