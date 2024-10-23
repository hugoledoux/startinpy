import laspy
import numpy as np
import pytest
import startinpy


def dt_5_points():
    dt = startinpy.DT()
    pts = np.array(
        [
            [0.0, 0.0, 1.0],
            [10.0, 0.0, 2.0],
            [10.0, 10.0, 3.0],
            [0.0, 10.0, 4.0],
            [5.0, 5.0, 5.0],
        ]
    )
    dt.insert(pts)
    return dt


def small_laz_intensity():
    las = laspy.read("data/small.laz")
    d = np.vstack((las.x, las.y, las.z, las.intensity)).transpose()
    dt = startinpy.DT(np.dtype([("intensity", np.float64)]))
    for each in d:
        dt.insert_one_pt(each[:3], intensity=each[3])
    return dt


def test_schema():
    dt = small_laz_intensity()
    dtype = np.dtype([("intensity", np.float64)])
    assert dtype == dt.get_attributes_schema()


def test_las_reading():
    dt = small_laz_intensity()
    a = dt.get_vertex_attributes(11)
    assert a["intensity"] == pytest.approx(533.0)
    with pytest.raises(KeyError):
        assert a["blue"]
    with pytest.raises(IndexError):
        dt.get_vertex_attributes(55555)


def test_set_vertex_attributes_1by1():
    dt = startinpy.DT(np.dtype([("humidity", np.float64)]))
    dt.insert_one_pt([0.0, 0.0, 12.5], humidity=33.3)
    dt.insert_one_pt([1.0, 0.0, 7.65])
    dt.insert_one_pt([1.0, 1.0, 33.0])
    dt.insert_one_pt([0.0, 1.0, 21.0])
    a = dt.get_vertex_attributes(1)
    assert a["humidity"] == pytest.approx(33.3)
    i = dt.attributes
    assert i.shape[0] == 5


def test_set_vertex_attributes():
    dt = small_laz_intensity()
    dt.set_vertex_attributes(11, intensity=66.6)
    assert dt.attributes["intensity"][11] == 66.6
    dt.set_vertex_attributes(11, allo=22.2)
    assert dt.attributes["intensity"][11] == 66.6


def test_no_attribute():
    dt = dt_5_points()
    with pytest.raises(Exception):
        a = dt.get_vertex_attributes(2)
    with pytest.raises(Exception):
        a = dt.get_vertex_attributes(12)
