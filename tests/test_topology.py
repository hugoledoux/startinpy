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


def test_incident_triangles_to_vertex():
    dt = dt_5_points()
    trs = dt.incident_triangles_to_vertex(5)
    assert len(trs) == 4
    for tr in trs:
        assert np.isin(5, tr)
    with pytest.raises(IndexError):
        trs = dt.incident_triangles_to_vertex(6)
    trs = dt.incident_triangles_to_vertex(0)
    assert len(trs) == 4
    for tr in trs:
        assert np.isin(0, tr)


def test_adjacent_triangles_to_triangle():
    dt = dt_5_points()
    t1 = [1, 2, 5]
    assert dt.is_triangle(t1) == True
    trs = dt.adjacent_triangles_to_triangle(t1)
    assert len(trs) == 3
    finite = 0
    for tr in trs:
        if dt.is_finite(tr):
            finite += 1
    assert finite == 2


def test_adjacent_vertices_to_vertex():
    dt = dt_5_points()
    vs = dt.adjacent_vertices_to_vertex(5)
    assert len(vs) == 4
    with pytest.raises(IndexError):
        vs = dt.adjacent_vertices_to_vertex(15)
    vs = dt.adjacent_vertices_to_vertex(0)
    assert len(vs) == 4
    # -- includes the infinity vertex
    vs = dt.adjacent_vertices_to_vertex(1)
    assert len(vs) == 4
