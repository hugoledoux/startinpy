
## Build

```shell
cargo build
python setup.py install
```

Move to another folder, and

```python
import startin
dt = startin.DT()
dt.insert_one_pt(1.0, 2.0, 33.2)
```

## A full simple example
```python
import startin

pts = []
pts.append([0.0, 0.0, 11.11])
pts.append([1.0, 0.0, 22.22])
pts.append([1.0, 1.0, 33.33])
pts.append([0.0, 1.0, 44])
pts.append([0.5, 0.49, 44])
pts.append([0.45, 0.69, 44])
pts.append([0.65, 0.49, 44])
pts.append([0.75, 0.29, 44])
pts.append([1.5, 1.49, 44])
pts.append([0.6, 0.2, 44])
pts.append([0.45, 0.4, 44])
pts.append([0.1, 0.8, 44])

t = startin.DT()
t.insert(pts)

print("# vertices:", t.number_of_vertices())
print("# triangles:", t.number_of_triangles())

print("CH: ", t.convex_hull())

itrs = t.incident_triangles_to_vertex(4);
print(itrs)

print(t.is_triangle([4, 12, 6]) )
print(t.is_triangle([5, 12, 6]) )

print("--- /Vertices ---")
for each in t.all_vertices():
    print(each)
print("--- Vertices/ ---")


alltr = t.all_triangles()
print(alltr[3])
```





