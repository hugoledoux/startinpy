import json

import numpy as np
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


def test_obj(tmp_path):
    dt = dt_5_points()
    d = tmp_path
    ofile = d / "out.obj"
    dt.write_obj(str(ofile))
    with open(ofile) as f:
        lines = f.readlines()
        nov = 0
        nof = 0
        for l in lines:
            if l[0] == "v":
                nov += 1
            if l[0] == "f":
                nof += 1
        assert nov == 5
        assert nof == 4


def test_ply(tmp_path):
    dt = dt_5_points()
    d = tmp_path
    ofile = d / "out.ply"
    dt.write_ply(str(ofile))
    nov = 0
    nof = 0
    with open(ofile) as f:
        lines = f.readlines()
        for l in lines:
            a = l.split(" ")
            if a[0] == "element":
                if a[1] == "vertex":
                    nov = int(a[2])
                if a[1] == "face":
                    nof = int(a[2])
    assert nov == 5
    assert nof == 4


def test_cityjson(tmp_path):
    dt = dt_5_points()
    d = tmp_path
    ofile = d / "out.city.json"
    dt.write_cityjson(str(ofile))
    with open(ofile) as f:
        j = json.load(f)
        assert len(j["vertices"]) == 5
        assert len(j["CityObjects"]) == 1
        assert len(j["CityObjects"]["myterrain"]["geometry"][0]["boundaries"]) == 4


def test_geojson(tmp_path):
    dt = dt_5_points()
    d = tmp_path
    ofile = d / "out.geojson"
    dt.write_geojson(str(ofile))
    with open(ofile) as f:
        j = json.load(f)
        assert j["type"] == "FeatureCollection"
        nov = 0
        nof = 0
        fs = j["features"]
        for f in fs:
            if f["geometry"]["type"] == "Point":
                nov += 1
            if f["geometry"]["type"] == "Polygon":
                nof += 1
        assert nov == 5
        assert nof == 4
