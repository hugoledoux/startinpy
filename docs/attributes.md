# Extra attributes

```{image} figs/extra_attributes.png
:width: 75%
```

## Attaching extra attributes

It is possible to store extra attributes with each vertex, to do so the class `startin.DT` must be initialised with the option `extra_attributes=True`.

```python
dt = startinpy.DT(extra_attributes=True)
```

Each attribute is stored as a JSON object/dictionary, a key-value pair where the key is a string and the value is one of these 3 options: (1) an integer, (2) a float, or (3) a boolean.

An attribute can be passed with the function `insert_one_pt()` using the parameter `extra_attributes`, observe that the function accepts a string, therefore `json.dumps()` must be used.
```python
a = {'intensity': 155.5, 'classification': 2, 'visited': False}
dt.insert_one_pt(31.1, 24.2, 1.8, extra_attributes=json.dumps(a))
```

Alternatively, any key/attribute can be added by using it as an extra parameter:
```python
dt.insert_one_pt(31.1, 24.2, 1.8, intensity=155.5, reflectance=111.0)
```

Also, we can set/overwrite the extra attributes for a specific vertex:
```python
dt.insert_one_pt(85000.0, 444003.2, 2.2, intensity=111.1, reflectance=29.9)
new_a = {'intensity': 155.5, 'classification': 3}
dt.set_attribute(17, json.dumps(new_a))
```

## Retrieving the extra attributes

It is possible to obtain the attributes attached to a single vertex, eg for the vertex with ID 50:
```python
a = dt.get_attribute(50)
print("=>", json.loads(a))
```

Notice that the vertices can have different attributes attached to them, or some can have attributes and some not.
You obtain the list of all the attributes stored in the triangulation with `list_attributes()`:
```python
dt = startinpy.DT(extra_attributes=True)
dt.insert_one_pt(85000.0, 444003.2, 2.2, intensity=111.1)
dt.list_attributes()
#-- ['intensity']
```

To retrieve all the values for all the vertices, for a given attribute, use the function `attributes()`. 
For example for all the values named `"intensity"`:
```python
i = dt.attributes('intensity')
#-- array([nan, 111.1, 22.2, 46.4, nan, ...,   77.8, 111.1])
```
This returns an array (or type `np.float64`) for all the vertices (including the infinity vertex!), and if there is no attribute `"intensity"` then `np.nan` is added.
It is your responsibility as user to cast the values if floats are not wanted, eg:
```python
i = dt.attributes('classification').astype(int)
#-- [0 2 2 2 1 3 2 6 2]
```


## One full Python example

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

a = {'intensity': 155.5, 'reflectance': 111, 'isvisited': True}
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