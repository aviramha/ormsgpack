# Changelog
## 1.2.1 - 1/3/2022
### Misc
- Release 3.10 wheels
- Update dependencies
## 1.2.0 - 14/2/2022
### Changed
- Extended int passthrough to support u64. by [@pejter](https://github.com/aviramha/ormsgpack/pull/77)
### Misc
- Updated README to include new options. by [@ThomasTNO](https://github.com/aviramha/ormsgpack/pull/70)
- Updated dependencies
- Renamed in `numpy.rs` `from_parent` to `to_children` to fix new lint rules
## 1.1.0 - 8/1/2022
### Added
- Add optional passthrough for tuples. by [@TariqTNO](https://github.com/aviramha/ormsgpack/pull/64)
- Add optional passthrough for ints, that do not fit into an i64. by [@TariqTNO](https://github.com/aviramha/ormsgpack/pull/64)
### Changed
- `opt` parameter can be `None`.
### Misc
- Updated dependencies.
- Dropped 3.6 CI/CD.
- Added macOS universal builds (M1)
## 1.0.3 - 18/12/2021
- Update dependencies
## 1.0.2 - 26/10/2021
- Update dependencies
## 1.0.1 - 13/10/2021
### Fixed
- Decrement refcount for numpy `PyArrayInterface`. by [@ilj](https://github.com/ijl/orjson/commit/4c312a82f5215ff71eed5bd09d28fa004999299b).
- Fix serialization of dataclass inheriting from `abc.ABC` and using `__slots__`. by [@ilj](https://github.com/ijl/orjson/commit/4c312a82f5215ff71eed5bd09d28fa004999299b)
### Changed
- Updated dependencies.
- `find_str_kind` test for 4-byte before latin1. by [@ilj](https://github.com/ijl/orjson/commit/05860e1a2ea3e8f90823d6a59e5fc9929a8692b5)
## 1.0.0 - 31/8/2021
### Changed
-  Aligned to orjson's flags and features of SIMD. Didn't include the stable compilation part as seems unnecessary.
### Misc
- Bumped serde, pyo3.
- Fixed pyproject.toml to work with newest maturin version.
## 0.3.6 - 24/8/2021
### Misc
- Update dependencies.
## 0.3.5 - 5/8/2021
### Fixed
- Fixed clippy warnings for semicolon in macro.
### Misc
- Bumped serde.rs
## 0.3.4 - 23/7/2021
### Fixed
- Fixed `ormsgpack.pyi` support of str as input for `unpackb`.
### Misc
- Fixed Windows CI/CD.
## 0.3.3 - 23/7/2021
### Misc
- Refactored adding objects to the module, creating a `__all__` object similar to the way PyO3 creates. This solves an issue with upgrading to new maturin version.
- Changed < Py3.7 implementation to use automatic range inclusion.
- Added test to validate correct Python method flags are used on declare.
- Changed to use PyO3 configurations instead of our own. PR [#25](https://github.com/aviramha/ormsgpack/pull/25) by [@pejter](https://github.com/pejter).
## 0.3.2 - 13/7/2021
### Fixed
- Fix memory leak serializing `datetime.datetime` with `tzinfo`. (Copied from orjson)
### Changed
- Update dependencies, PyO3 -> 0.14.1.
### Misc
- Setup dependabot.
## 0.3.1 - 19/6/2021
### Changed
- `packb` of maps and sequences is now almost 2x faster as it leverages known size. PR [#18](https://github.com/aviramha/ormsgpack/pull/18) by [@ijl](https://github.com/ijl).
### Misc
- Added `scripts/bench_target.py` and `scripts/profile.sh` for easily benchmarking and profiling. Works only on Linux. PR [#17](https://github.com/aviramha/ormsgpack/pull/17) by [@ijl](https://github.com/ijl).
## 0.3.0 - 13/6/2021
### Added
- `unpackb` now accepts keyword argument `option` with argument `OPT_NON_STR_KEYS`. This option will let ormsgpack
    unpack dictionaries with non-str keys.
    Be aware that this option is considered unsafe and disabled by default in msgpack due to possibility of HashDoS.
- `packb` now is able to pack dictionaries with tuples as keys. `unpackb` is able to unpack such dictionaries. Both requires
    `OPT_NON_STR_KEYS`.
### Misc
- Grouped benchmarks in a pattern that should make more sense.
- Added pydantic docs to `README.md`
- Added graphs and benchmark results.
## 0.2.1 - 12/6/2021
### Fixed
- Depth limit is now enforced for `ormsgpack.unpackb` - function should be safe for use now.
### Removed
- Removed `OPT_SERIALIZE_UUID` from ormsgpack.pyi as it doesn't exist.
### Misc
- Added `scripts/test.sh` for running tests.
- Added benchmarks, modified scripts to match new layout.
## 0.2.0 - 10/6/2021
### Added
- Add support for serializing pydantic's `BaseModel` instances using `ormsgpack.OPT_SERIALIZE_PYDANTIC`.
### Fixed
- `orjson.packb` with `option` argument as `ormsgpack.OPT_NON_STR_KEYS` serializes bytes key into tuple of integers
    instead of using bin type. This also resulted in assymetrical packb/unpackb.
### Misc
- Added `--no-index` to `pip install ormsgpack` to avoid installing from PyPI on CI.
## 0.1.0 - 9/6/2021

First version, changed orjson to ormsgpack.
