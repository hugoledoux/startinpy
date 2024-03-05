import pytest
import startinpy
import numpy as np


def test_duplicates():
    dt = startinpy.DT()
    dt.insert_one_pt(0.0, 0.0, 1.0);
    dt.insert_one_pt(10.0, 0.0, 2.0);
    dt.insert_one_pt(10.0, 10.0, 3.0);
    dt.insert_one_pt(0.0, 10.0, 4.0);
    dt.insert_one_pt(5.0, 5.0, 10.0);

    (i, b) = dt.insert_one_pt(5.0, 5.0, 20.0);
    assert i == 5
    assert b == False
    assert dt.get_point(5)[2] == 10.0

    dt.duplicates_handling = 'Highest'
    (i, b) = dt.insert_one_pt(5.0, 5.0, 20.0);
    assert dt.get_point(5)[2] == 20.0

    dt.duplicates_handling = 'Lowest'
    (i, b) = dt.insert_one_pt(5.0, 5.0, 10.0);
    assert dt.get_point(5)[2] == 10.0

    dt.duplicates_handling = 'Last'
    (i, b) = dt.insert_one_pt(5.0, 5.0, 5.0);
    assert dt.get_point(5)[2] == 5.0

    dt.duplicates_handling = 'First'
    (i, b) = dt.insert_one_pt(5.0, 5.0, 15.0);
    assert dt.get_point(5)[2] == 5.0

def test_snap_tolerance():
    dt = startinpy.DT()
    dt.insert_one_pt(0.0, 0.0, 1.0);
    dt.insert_one_pt(10.0, 0.0, 2.0);
    dt.insert_one_pt(10.0, 10.0, 3.0);
    dt.insert_one_pt(0.0, 10.0, 4.0);
    dt.insert_one_pt(5.0, 5.0, 10.0);

    assert dt.snap_tolerance == pytest.approx(0.001)
    (i, b) = dt.insert_one_pt(5.0001, 5.0, 20.0);
    assert b == False
    (i, b) = dt.insert_one_pt(5.000999, 5.0, 20.0);
    assert b == False
    (i, b) = dt.insert_one_pt(5.001, 5.0, 20.01);
    assert b == True

    dt.snap_tolerance = 0.1
    assert dt.snap_tolerance == pytest.approx(0.1)
    (i, b) = dt.insert_one_pt(10.0, 0.0, 20.0);
    assert b == False
    (i, b) = dt.insert_one_pt(10.09, 0.0, 20.0);
    assert b == False
    (i, b) = dt.insert_one_pt(10.10, 0.0, 20.0);
    assert b == False
    (i, b) = dt.insert_one_pt(10.11, 0.0, 20.0);
    assert b == True

    