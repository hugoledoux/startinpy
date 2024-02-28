import pytest
import startinpy
import numpy as np

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

def test_is_triangle():
    dt = startinpy.DT()
    assert dt.is_triangle([0, 1, 2]) == False
    with pytest.raises(Exception):
        dt.is_triangle([0, -1, 2])
    dt = dt_5_points()
    assert dt.is_triangle([0, 2, 1]) == True
    assert dt.is_triangle([0, 1, 1]) == False

def test_points():
    dt = startinpy.DT()
    assert dt.points.shape == (1, 3)
    dt = dt_5_points()
    assert dt.points.shape == (6, 3)

def test_triangles():
    dt = startinpy.DT()
    assert dt.triangles.shape == (0, 0)
    dt = dt_5_points()
    assert dt.triangles.shape == (4, 3)