import pytest
import startinpy
import numpy as np
import math

def four_points():
    return np.array([
        [0.0, 0.0, 1.0],
        [10.0, 0.0, 2.0],
        [10.0, 10.0, 3.0],
        [0.0, 10.0, 4.0]
     ])

def random(n=20):
    rng = np.random.default_rng()
    pts = rng.random((n, 3))
    pts = pts * 100
    return pts    

def test_empty():
    locs = np.array([[50.0, 41.1], [101.1, 33.2], [80.0, 66.0]])
    dt = startinpy.DT()
    re = dt.interpolate({"method": "IDW", "radius": 20, "power": 2.0}, locs)
    assert np.isnan(re).all() == True
    re = dt.interpolate({"method": "Laplace"}, locs)
    assert np.isnan(re).all() == True
    re = dt.interpolate({"method": "NN"}, locs)
    assert np.isnan(re).all() == True
    re = dt.interpolate({"method": "NNI"}, locs)
    assert np.isnan(re).all() == True
    re = dt.interpolate({"method": "TIN"}, locs)
    assert np.isnan(re).all() == True
    with pytest.raises(Exception):
        re = dt.interpolate({"method": "TIN"}, locs, strict=True)
    with pytest.raises(Exception):
        re = dt.interpolate({"method": "IDW", "radius": 20, "power": 2.0}, locs, strict=True)


def test_outside_convexhull():
    pts = four_points()
    dt = startinpy.DT()
    dt.insert(pts)
    locs = [[5.0, -0.1]]
    re = dt.interpolate({"method": "Laplace"}, locs)
    assert np.isnan(re).all() == True
    re = dt.interpolate({"method": "NN"}, locs)
    assert np.isnan(re).all() == True
    re = dt.interpolate({"method": "NNI"}, locs)
    assert np.isnan(re).all() == True
    re = dt.interpolate({"method": "TIN"}, locs)
    assert np.isnan(re).all() == True


def test_nn():
    pts = four_points()
    dt = startinpy.DT()
    dt.insert(pts)    
    dt.insert_one_pt(5.0, 5.0, 11.1)
    locs = [ [5.1, 5.1] ]
    re = dt.interpolate({"method": "NN"}, locs)
    assert re[0] == pytest.approx(11.1)

def test_tin_linear_random():
    pts = random(500)
    dt = startinpy.DT()
    dt.insert(pts)   
    locs = [[144.0, 48.0], [44.0, 48.0]]
    re = dt.interpolate({"method": "TIN"}, locs)
    assert np.isnan(re[0]) == True
    assert np.isnan(re[1]) == False
    assert re[1] > 0.0 and re[1] < 100.0



def test_middle():
    pts = four_points()
    dt = startinpy.DT()
    dt.insert(pts)    
    locs = [[5.0, 5.0]]
    re = dt.interpolate({"method": "Laplace"}, locs)
    assert re[0] == pytest.approx(2.5)
    re = dt.interpolate({"method": "NNI"}, locs)
    assert re[0] == pytest.approx(2.5)
    re = dt.interpolate({"method": "TIN"}, locs)
    assert re[0] == pytest.approx(3.0)

def test_exisiting_point():
    pts = four_points()
    dt = startinpy.DT()
    dt.insert(pts)    
    dt.insert_one_pt(5.0, 5.0, 11.1)
    locs = [[5.0, 5.0]]
    re = dt.interpolate({"method": "Laplace"}, locs)
    assert re[0] == pytest.approx(11.1)
    re = dt.interpolate({"method": "NN"}, locs)
    assert re[0] == pytest.approx(11.1)
    re = dt.interpolate({"method": "NNI"}, locs)
    assert re[0] == pytest.approx(11.1)
    re = dt.interpolate({"method": "TIN"}, locs)
    assert re[0] == pytest.approx(11.1)


def test_idw():
    pts = four_points()
    dt = startinpy.DT()
    dt.insert(pts)
    locs = [[5.0, 5.0], [9.0, 9.0]]
    re = dt.interpolate({"method": "IDW", "radius": 3.0, "power": 2.0}, locs)
    assert np.isnan(re[0])
    assert re[1] == pytest.approx(3.0)
