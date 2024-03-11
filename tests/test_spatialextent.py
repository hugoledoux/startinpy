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

def test_bbox_empty():
    dt = startinpy.DT()
    bbox = dt.get_bbox()
    assert np.isinf(bbox).all() == True

def test_bbox():
    pts = random(100)
    dt = startinpy.DT()
    dt.insert(pts)
    bbox = dt.get_bbox()
    assert np.isinf(bbox).any() == False
    assert bbox[2] > bbox[0]
    assert bbox[3] > bbox[1]

def test_bbox_square():
    dt = dt_5_points()
    bbox = dt.get_bbox()
    assert bbox[0] == pytest.approx(0.0)
    assert bbox[1] == pytest.approx(0.0)
    assert bbox[2] == pytest.approx(10.0)
    assert bbox[3] == pytest.approx(10.0)

def test_convexhull_empty():
    dt = startinpy.DT()
    ch = dt.convex_hull()
    assert len(ch) == 0
    dt.insert_one_pt([1., 1., 1.])
    ch = dt.convex_hull()
    assert len(ch) == 0
    dt.insert_one_pt([2., 3., 4.])
    ch = dt.convex_hull()
    assert len(ch) == 0
    dt.insert_one_pt([1., 6., 4.])
    ch = dt.convex_hull()
    assert len(ch) == 3

def test_convexhull():
    dt = dt_5_points()
    ch = dt.convex_hull()
    assert len(ch) == 4
    assert dt.is_vertex_convex_hull(1) == True
    with pytest.raises(Exception):
        dt.is_vertex_convex_hull(-1) 
    assert dt.is_vertex_convex_hull(5) == False
    assert dt.is_inside_convex_hull([7.1, 2.1]) == True
    assert dt.is_inside_convex_hull([-7.1, 2.1]) == False

def test_locate():
    dt = dt_5_points()
    assert (dt.locate([7.1, 2.1]) == np.array([5, 1, 2])).all()
    with pytest.raises(Exception):
        dt.locate(-1., 9.0)   
    