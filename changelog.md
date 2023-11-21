

# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.10.1] - 2023-11-21
### Changed
- improve the docs to explain how startinpy works
- added Python v3.12 support

## [0.10.0] - 2023-08-16
### Changed
- uses the new uses startin v0.7:
  - new interpolation mechanism
  - fix bugs
- outputs to GeoJSON and CityJSON was added
- has a new doc, better info and admittedly looks nicer


## [0.9.2] - 2022-11-14
### Changed
- bindings are built for the new python 3.11 as well now, from 3.7+

## [0.9.1] - 2022-09-28
### Changed
- uses startin = v0.6.1
- the much improved docs in `docs/` is now automatically built put at https://startinpy.rtfd.io/
### Added
- 2 functions related to garbage collection

## [0.9.0] - 2022-09-22
### Changed
- uses startin >= v0.6
- functions returns Exceptions if outside convexhull or wrong IDs
- general bug fixes
- PLY output so that QGIS can read directly with MDAL mesh
- new functions: `get_bbox()`, `__str()__`
### Added
- the much improved docs in `docs/` is now automatically built and put at https://hugoledoux.github.io/startinpy/


## [0.8.0] - 2021-12-22
### Changed
- the API is now using numpy for the arrays, you can use numpy arrays and the function that were returning lists are not returning numpy arrays
### Added
- a proper docs in `/docs`


## [0.7.0] - 2021-04-16
### Added
- startin v0.5 new features: 
  - interplation with natural neighbour (nni, or Sibson's method) is added. 
  - saving of the triangulation to GeoJSON is added
### Changed
- new name "startinpy" to avoid clashes with the rust lib "startin" on which this project is based on. Since I use a Rust builder (maturin), there were clashes and probably a better idea to call it "startinpy" anyway.


[0.10.0]: https://github.com/hugoledoux/startin/compare/0.9.2...0.10.0
[0.9.2]: https://github.com/hugoledoux/startin/compare/0.9.1...0.9.2
[0.9.1]: https://github.com/hugoledoux/startin/compare/0.9.0...0.9.1
[0.9.0]: https://github.com/hugoledoux/startin/compare/0.8.0...0.9.0
