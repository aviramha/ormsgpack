# Changelog
## Next Version
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
