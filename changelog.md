

# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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

