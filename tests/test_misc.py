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

def test_is_triangle():
    dt = startinpy.DT()
    assert dt.is_triangle([0, 1, 2]) == False
    assert dt.is_triangle([0, 11, 2]) == False
    with pytest.raises(OverflowError):
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

def test_vertical_exaggeration():
    dt = dt_5_points()
    dt.vertical_exaggeration(2.0)
    assert dt.points[1][2] == pytest.approx(1.0)
    assert dt.points[5][2] == pytest.approx(9.0)

def test_update_vertex_z_value():
    dt = dt_5_points()
    re = dt.update_vertex_z_value(3, 5.55)
    assert re == True
    assert dt.points[3][2] == pytest.approx(5.55)
    re = dt.update_vertex_z_value(9, 5.55)
    assert re == False
