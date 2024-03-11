import pytest
import startinpy
import numpy as np
import math

def random(n=20):
    rng = np.random.default_rng()
    pts = rng.random((n, 3))
    pts = pts * 100
    return pts

def test_remove_insert():
    pts = random(25)
    dt = startinpy.DT()
    dt.insert(pts)
    dt.remove(17)
    assert dt.is_vertex_removed(17) == True
    assert dt.has_garbage() == True
    dt.insert_one_pt([88., 88., 88.])
    assert dt.is_vertex_removed(17) == False
    assert dt.has_garbage() == False

def test_remove_valid():
    total = 25
    pts = random(total)
    dt = startinpy.DT()
    dt.insert(pts)
    assert dt.number_of_vertices() == total
    dt.remove(17)
    assert dt.number_of_vertices() == (total - 1)
    assert dt.is_vertex_removed(17) == True
    dt.remove(12)
    assert dt.number_of_vertices() == (total - 2)
    assert dt.is_vertex_removed(12) == True

def test_remove_invalid():
    pts = random()
    dt = startinpy.DT()
    dt.insert(pts)
    with pytest.raises(IndexError):
        dt.remove(21)
    with pytest.raises(IndexError):
        dt.remove(0)
    assert dt.number_of_vertices() == 20
    dt.remove(11)
    with pytest.raises(IndexError):
        dt.remove(11)
    
def test_cocircular():
    dt = startinpy.DT()
    dt.insert_one_pt([0.0, 0.0, 12.5]);
    dt.insert_one_pt([1.0, 0.0, 7.65]);
    dt.insert_one_pt([1.0, 1.0, 33.0]);
    dt.insert_one_pt([0.0, 1.0, 21.0]);
    y = 0.5 + math.sqrt(0.5 * 0.5 + 0.5 * 0.5)
    dt.insert_one_pt([0.5, y, 21.0]);
    dt.insert_one_pt([0.5, 0.5, 33.0]);
    dt.remove(6)
    assert dt.number_of_vertices() == 5
    assert dt.number_of_triangles() == 3

def test_convexhull():
    dt = startinpy.DT()
    dt.insert_one_pt([0.0, 0.0, 12.5]);
    dt.insert_one_pt([1.0, 0.0, 7.65]);
    dt.insert_one_pt([1.0, 1.0, 33.0]);
    dt.insert_one_pt([0.0, 1.0, 21.0]);
    dt.remove(3)
    assert dt.number_of_vertices() == 3
    assert dt.number_of_triangles() == 1    
    dt.remove(2)
    assert dt.number_of_vertices() == 2
    assert dt.number_of_triangles() == 0 

def test_garbagecollection():
    total = 100
    pts = random(total)
    dt = startinpy.DT()
    dt.insert(pts)
    dt.remove(31)
    dt.remove(61)
    dt.remove(91)
    assert dt.number_of_vertices() == 97
    assert dt.has_garbage() == True
    notr = dt.number_of_triangles()
    dt.collect_garbage()
    assert dt.number_of_vertices() == 97
    assert dt.number_of_triangles() == notr

def test_get_point():
    total = 100
    pts = random(total)
    dt = startinpy.DT()
    dt.insert(pts)
    dt.remove(77)
    with pytest.raises(IndexError):
        p = dt.get_point(77)


