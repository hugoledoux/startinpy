import pytest
import startinpy
import numpy as np
import laspy
import json

def dt_5_points():
    dt = startinpy.DT()
    pts = np.array([
        [0.0, 0.0, 1.0],
        [10.0, 0.0, 2.0],
        [10.0, 10.0, 3.0],
        [0.0, 10.0, 4.0],
        [5.0, 5.0, 5.0]
     ])
    dt.insert(pts)
    return dt

def random(n=20):
    rng = np.random.default_rng()
    pts = rng.random((n, 3))
    pts = pts * 100
    return pts 

def small_laz():
    las = laspy.read("data/small.laz")
    d = np.vstack((las.x, las.y, las.z)).transpose()
    dt = startinpy.DT()
    dt.insert(d)
    return dt

def small_laz_intensity():
    las = laspy.read("data/small.laz")
    d = np.vstack((las.x, las.y, las.z, las.intensity)).transpose()
    dt = startinpy.DT(extra_attributes=True)
    for each in d:
        dt.insert_one_pt(each[0], each[1], each[2], intensity=each[3])
    return dt

def test_las_reading():
    dt = small_laz_intensity()
    a = json.loads(dt.get_vertex_attributes(11))
    assert a['intensity'] == pytest.approx(533.0)
    with pytest.raises(KeyError):
        assert a['blue'] 
    with pytest.raises(Exception):
        dt.get_vertex_attributes(55555)

def test_set_vertex_attributes_1by1():
    dt = startinpy.DT(extra_attributes=True)
    dt.insert_one_pt(0.0, 0.0, 12.5, humidity=33.3);
    dt.insert_one_pt(1.0, 0.0, 7.65);
    dt.insert_one_pt(1.0, 1.0, 33.0);
    dt.insert_one_pt(0.0, 1.0, 21.0);
    a = json.loads(dt.get_vertex_attributes(1))
    assert a['humidity'] == pytest.approx(33.3)
    i = dt.attribute('humidity')
    assert i.shape[0] == 5
    i = dt.attribute('smthelse')
    assert i.shape[0] == 0

def test_list_attributes():
    dt = startinpy.DT(extra_attributes=True)
    dt.insert_one_pt(0.0, 0.0, 12.5, a1=33.3);
    dt.insert_one_pt(1.0, 0.0, 7.65, a2=33.3);
    dt.insert_one_pt(1.0, 0.0, 7.65);
    dt.insert_one_pt(1.0, 1.0, 33.0, a3=33.3);
    dt.insert_one_pt(0.0, 1.0, 21.0, a4=33.3, a1=33.3);
    l = dt.list_attributes()
    assert len(l) == 4

def test_set_vertex_attributes():
    dt = small_laz_intensity()
    new_a = {'intensity': 155.5, 'reflectance': 222.2, 'extra': 3}
    assert dt.set_vertex_attributes(11, json.dumps(new_a)) == True
    new2 = json.loads(dt.get_vertex_attributes(11))
    assert new2['intensity'] == pytest.approx(155.5) 
    assert new2['reflectance'] == pytest.approx(222.2) 
    assert new2['extra'] == 3 
    with pytest.raises(KeyError):
        new2['hugo'] == 3

def test_no_attribute():
    dt = dt_5_points()
    with pytest.raises(Exception):
        a = json.loads(dt.get_vertex_attributes(2))
    with pytest.raises(Exception):
        a = json.loads(dt.get_vertex_attributes(12))

