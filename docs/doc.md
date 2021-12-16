
### adjacent_vertices_to_vertex(i)
  returns a list of vertex indices that are adjacent to vertex i

### all_triangles()                 
  returns a list of (finite) Triangles (which is a list with 3 indices)

### all_vertices()                  
  returns a list of all vertices in the DT (including the infinite one, vertex "0")

### closest_point(x, y)                 
  returns the closest vertex index to (x,y) (distance in 2D)

### convex_hull()                   
  returns the convex hull as a list of vertex indices

### get_point(i)
  returns the point at the index i                     

### get_snap_tolerance()            
  returns the snap tolerance (2 vertices closer will be the same)

### incident_triangles_to_vertex(i)  
  returns a list of Triangles incident to vertex i

### insert([ [ax, ay, az], [bx, by, bz] ])
  calls insert_one_pt() for each vertex in the list
  returns nothing                        

### insert_one_pt(x, y, z)   
  returns the index of the vertex inserted (an already exisiting one is possible)             

### interpolate_laplace(x, y)
  returns the value, interpolated with the Laplace method, at location (x, y)  
  an error is thrown if outside the DT         

### interpolate_nn(x, y)       
  returns the value, interpolated with the nearest neighbour method, at location (x, y)  
  an error is thrown if outside the DT         

### interpolate_tin_linear(x, y) 
  returns the value, interpolated with the linear interpolation in TIN, at location (x, y)  
  an error is thrown if outside the DT         

### is_triangle([a, b, c])
  returns true if triangle abc exists, false if not

### is_vertex_convex_hull(i)
  returns true if vertex i is on the boundary of the convex hull, false if not   

### locate(x, y)
  returns the Triangle containing the point (x, y)

### number_of_triangles()
  returns the number of (finite) Triangles in the DT

### number_of_vertices()
  returns the number of (finite) vertices in the DT

### read_las(path_file)
  reads the LAS/LAZ file "path_file" (a string) and inserts all the points in the DT
  throws an error if the path is invalid

### remove(i)
  removes/delete the vertex i from the DT, and updates it for the Delaunay criterion
  returns 1 if the operation was successful; and -1 if the vertex doesn't exist

### set_snap_tolerance(value)
  sets the snap tolerance (for insertion of points in the DT) to this value 
  (default=0.001)
  returns nothing

### write_obj(path)
  writes an OBJ of the DT to the path (a string)
  throws an error if the path is invalid
