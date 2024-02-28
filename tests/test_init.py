import pytest
import startinpy
import numpy as np


def test_init():
    dt = startinpy.DT()
    assert dt.number_of_vertices() == 0
    assert dt.number_of_triangles() == 0

def test_before_first_triangle():
    dt = startinpy.DT()
    dt.insert_one_pt(0., 0., 0.)
    assert dt.number_of_vertices() == 1
    assert dt.number_of_triangles() == 0
    dt.insert_one_pt(1., 0., 0.)
    assert dt.number_of_vertices() == 2
    assert dt.number_of_triangles() == 0
    dt.insert_one_pt(1., 1., 0.)
    assert dt.number_of_vertices() == 3
    assert dt.number_of_triangles() == 1

def test_init_phase_duplicates_remove():
    dt = startinpy.DT()
    dt.insert_one_pt(0., 0., 0.)
    dt.insert_one_pt(1., 0., 0.)
    dt.insert_one_pt(1., 1., 0.)
    assert dt.number_of_vertices() == 3
    assert dt.number_of_triangles() == 1
    dt.insert_one_pt(1., 0., 0.)
    assert dt.number_of_vertices() == 3
    assert dt.number_of_triangles() == 1
    dt.remove(3)
    assert dt.number_of_vertices() == 2
    assert dt.number_of_triangles() == 0        

def test_grid():
    dt = startinpy.DT()
    for i in range(10):
        for j in range(10):
            dt.insert_one_pt(float(i), float(j), 1.0)
    assert dt.number_of_vertices() == 100

def test_collinear():
    dt = startinpy.DT()
    dt.insert_one_pt(0., 0., 0.)
    dt.insert_one_pt(1., 0., 0.)
    dt.insert_one_pt(2., 0., 0.)
    assert dt.number_of_vertices() == 3
    assert dt.number_of_triangles() == 0
    dt.insert_one_pt(2., 1., 0.)
    assert dt.number_of_vertices() == 4
    assert dt.number_of_triangles() == 2
    dt.remove(4)
    assert dt.number_of_vertices() == 3
    assert dt.number_of_triangles() == 0