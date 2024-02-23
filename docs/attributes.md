# Extra attributes

It is possible to store extra attributes with each vertex, to do so the class `startin.DT` must be initialised with the option `extra_attributes=True`.

```python
dt = startinpy.DT(extra_attributes=True)
```

Each attribute is stored as a JSON object/dictionary, a key-value pair where the key is a string and the value is an integer, a float, or a boolean.

An attribute can be passed with the function `insert_one_pt()` using the parameter `extra_attributes`:
```python
a = {'intensity': 155.5, 'reflectance': 111, 'something': True}
dt.insert_one_pt(31.1, 24.2, 1.8, extra_attributes=json.dumps(a))
```
Observe that the function accepts a string so `json.dumps()` must be used.

Alternatively, any key/attribute can be added by using it as an extra parameter:
```python
dt.insert_one_pt(31.1, 24.2, 1.8, intensity=155.5, reflectance=111.0)
```

It is possible to get the value of an attribute, eg for the vertex with ID 50:
```python
a = dt.get_attribute(50)
print("=>", json.loads(a))
```

And we can set/overwrite the extra attributes for a specific vertex:
```python
dt.insert_one_pt(85000.0, 444003.2, 2.2, intensity=111.1, reflectance=29.9)
new_a = {'intensity': 155.5, 'reflectance': 222.2, 'extra': 3}
dt.set_attribute(17, json.dumps(new_a))
dt.get_attribute(17)
```

All together:

```python
import startinpy
import numpy as np
import json
import laspy

las = laspy.read("../data/small.laz")

#-- read intensity and store it as extra_attribute in the startinpy DT
d = np.vstack((las.x, las.y, las.z, las.intensity)).transpose()

dt = startinpy.DT(extra_attributes=True)
for each in d:
    dt.insert_one_pt(each[0], each[1], each[2], intensity=each[3])

a = {'intensity': 155.5, 'reflectance': 111, 'something': True}
dt.set_attribute(50, json.dumps(a))

print(dt)

print("all extra attributes:", dt.list_attributes())

a = dt.get_attribute(50)
print("=>", json.loads(a))
a = dt.get_attribute(49)
print("=>", json.loads(a))

i = dt.attributes('intensity')
print(i.shape)
```